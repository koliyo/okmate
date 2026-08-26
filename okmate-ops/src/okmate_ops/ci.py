from __future__ import annotations

import argparse
import subprocess
from dataclasses import dataclass
from pathlib import Path

from okmate_ops.paths import repo_root

JOB_NAMES = (
    "test",
    "knowledge",
)


@dataclass(frozen=True)
class Step:
    argv: tuple[str, ...]


def steps_for(job: str, root: Path) -> list[Step]:
    if job == "test":
        return [
            Step(("cargo", "fmt", "--all", "--", "--check")),
            Step(("cargo", "test", "-p", "okf")),
            Step(("cargo", "test", "-p", "okmate", "--no-default-features")),
        ]
    if job == "knowledge":
        if not (root / "knowledge" / "index.md").is_file():
            return []
        return [
            Step(
                (
                    "cargo",
                    "run",
                    "-q",
                    "--no-default-features",
                    "-p",
                    "okmate",
                    "--",
                    "check",
                    "knowledge",
                    "--profile",
                    "strict",
                    "--format",
                    "terminal",
                )
            )
        ]
    raise ValueError(f"unknown job: {job}")


def run_step(step: Step, cwd: Path) -> int:
    result = subprocess.run(list(step.argv), cwd=cwd, check=False)
    return result.returncode


def run_job(job: str, cwd: Path) -> int:
    print(f"==> {job}", flush=True)
    steps = steps_for(job, cwd)
    if not steps:
        print(f"skip {job}: knowledge/index.md is missing", flush=True)
        return 0
    for step in steps:
        print("+ " + " ".join(step.argv), flush=True)
        code = run_step(step, cwd)
        if code != 0:
            return code
    return 0


def parse_ci_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(prog="okmate-ops ci")
    parser.add_argument("-k", "--keep-going", action="store_true")
    parser.add_argument("-l", "--list", action="store_true")
    parser.add_argument("jobs", nargs="*", choices=JOB_NAMES)
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_ci_args(argv)
    if args.list:
        for name in JOB_NAMES:
            print(name)
        return 0
    jobs = args.jobs or list(JOB_NAMES)
    cwd = repo_root()
    failed: list[str] = []
    for job in jobs:
        code = run_job(job, cwd)
        if code != 0:
            failed.append(job)
            if not args.keep_going:
                return code
    return 1 if failed else 0
