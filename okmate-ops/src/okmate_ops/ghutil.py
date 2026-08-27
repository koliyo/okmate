from __future__ import annotations

import subprocess
import sys
import time
from typing import IO, Callable

DEFAULT_CHECKS = ("Test",)
BAR_WIDTH = 24
DEFAULT_BAR_WINDOW_S = 600.0


def format_elapsed(seconds: float) -> str:
    total = max(0, int(seconds))
    return f"{total // 60:02d}:{total % 60:02d}"


def render_bar(elapsed: float, window: float, width: int = BAR_WIDTH) -> str:
    frac = 0.0 if window <= 0 else min(1.0, elapsed / window)
    filled = int(frac * width)
    return f"[{'#' * filled}{'-' * (width - filled)}]"


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

    def tick(label: str) -> None:
        elapsed = time.monotonic() - started
        line = f"{check} {render_bar(elapsed, window)} {format_elapsed(elapsed)} {label}"
        stream.write(f"\r{line}\033[K")
        stream.flush()

    while True:
        if deadline_s is not None and time.monotonic() - started > deadline_s:
            stream.write("\n")
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
            tick("waiting")
        else:
            status, conclusion = parsed
            if status == "completed":
                if conclusion == "success":
                    tick("passed")
                    stream.write("\n")
                    return
                tick(f"failed ({conclusion})")
                stream.write("\n")
                raise SystemExit(f"{check} failed ({conclusion})")
            tick(f"{status} {conclusion}")
        sleep(10)


def gh_run(args: list[str], check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run(["gh", *args], check=check, capture_output=True, text=True)
