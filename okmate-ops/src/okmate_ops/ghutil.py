from __future__ import annotations

import subprocess
import sys
import time
from typing import IO, Callable

from tqdm import tqdm

DEFAULT_CHECKS = ("Code Formatting & Lints", "Test")
DEFAULT_BAR_WINDOW_S = 600.0


def parse_check_line(result: str) -> tuple[str, str] | None:
    line = result.strip().splitlines()[0] if result.strip() else ""
    if not line:
        return None
    status, _, conclusion = line.partition(" ")
    return status, conclusion or "pending"


def wait_for_check(
    *,
    repo: str,
    sha: str,
    check: str,
    gh: Callable[..., str],
    sleep: Callable[[float], None],
    deadline_s: float | None = None,
    out: IO[str] | None = None,
) -> None:
    started = time.monotonic()
    stream = out or sys.stdout
    window = deadline_s if deadline_s is not None else DEFAULT_BAR_WINDOW_S
    bar = tqdm(
        total=int(window),
        desc=check,
        unit="s",
        file=stream,
        ascii=True,
        ncols=80,
        mininterval=0,
        dynamic_ncols=False,
        bar_format="{desc}: {bar} {elapsed} {postfix}",
    )
    try:
        while True:
            if deadline_s is not None and time.monotonic() - started > deadline_s:
                raise SystemExit(f"timed out waiting for {check}")
            raw = gh(
                [
                    "api",
                    f"repos/{repo}/commits/{sha}/check-runs",
                    "--jq",
                    f'.check_runs[] | select(.name == "{check}") | .status + " " + (.conclusion // "pending")',
                ]
            )
            parsed = parse_check_line(raw)
            if parsed is None:
                bar.set_postfix_str("waiting", refresh=True)
            else:
                status, conclusion = parsed
                if status == "completed":
                    if conclusion == "success":
                        bar.set_postfix_str("passed", refresh=True)
                        return
                    bar.set_postfix_str(f"failed ({conclusion})", refresh=True)
                    raise SystemExit(f"{check} failed ({conclusion})")
                bar.set_postfix_str(f"{status} {conclusion}", refresh=True)
            sleep(10)
            bar.update(10)
    finally:
        bar.close()


def gh_run(args: list[str], check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(["gh", *args], check=check, capture_output=True, text=True)
