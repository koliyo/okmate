from __future__ import annotations

import json
import subprocess
import time
from typing import Callable

CI_WORKFLOW = "ci.yml"
IN_PROGRESS = frozenset({"queued", "in_progress", "waiting", "requested", "pending"})


def parse_workflow_runs(raw: str) -> list[dict]:
    if not raw.strip():
        return []
    data = json.loads(raw)
    if not isinstance(data, list):
        return []
    return data


def wait_for_workflow_run(
    *,
    sha: str,
    workflow: str = CI_WORKFLOW,
    gh: Callable[..., str],
    sleep: Callable[[float], None],
    watch: Callable[[int], None] | None = None,
    deadline_s: float | None = None,
) -> None:
    started = time.monotonic()
    print(f"Waiting for: {workflow} on {sha}", flush=True)
    while True:
        if deadline_s is not None and time.monotonic() - started > deadline_s:
            raise SystemExit(f"timed out waiting for {workflow}")
        raw = gh(
            [
                "run",
                "list",
                "--commit",
                sha,
                "--workflow",
                workflow,
                "--json",
                "databaseId,status,conclusion,createdAt",
                "--limit",
                "20",
            ]
        )
        runs = parse_workflow_runs(raw)
        runs.sort(key=lambda run: run.get("createdAt") or "", reverse=True)
        if any(
            run.get("status") == "completed" and run.get("conclusion") == "success"
            for run in runs
        ):
            print(f"  {workflow} passed", flush=True)
            return
        inflight = [run for run in runs if run.get("status") in IN_PROGRESS]
        if inflight:
            run_id = int(inflight[0]["databaseId"])
            print(f"  watching run {run_id}", flush=True)
            if watch is not None:
                watch(run_id)
            else:
                gh(["run", "watch", str(run_id), "--exit-status"])
            sleep(0)
            continue
        completed = [run for run in runs if run.get("status") == "completed"]
        if completed:
            conclusion = completed[0].get("conclusion") or "failure"
            raise SystemExit(f"{workflow} failed ({conclusion})")
        raise SystemExit(
            f"CI has not run on {sha}; wait for the push or dispatch {workflow} yourself"
        )


def wait_for_existing_ci(
    sha: str,
    *,
    parent_sha: str | None = None,
    gh: Callable[..., str],
    sleep: Callable[[float], None],
    watch: Callable[[int], None] | None = None,
    deadline_s: float | None = None,
) -> None:
    try:
        wait_for_workflow_run(
            sha=sha,
            workflow=CI_WORKFLOW,
            gh=gh,
            sleep=sleep,
            watch=watch,
            deadline_s=deadline_s,
        )
    except SystemExit as exc:
        if parent_sha is None or not str(exc).startswith("CI has not run on"):
            raise
        print(f"no {CI_WORKFLOW} on {sha}; waiting on parent {parent_sha}", flush=True)
        wait_for_workflow_run(
            sha=parent_sha,
            workflow=CI_WORKFLOW,
            gh=gh,
            sleep=sleep,
            watch=watch,
            deadline_s=deadline_s,
        )


def gh_run(args: list[str], check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(["gh", *args], check=check, capture_output=True, text=True)
