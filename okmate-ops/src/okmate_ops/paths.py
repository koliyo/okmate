from __future__ import annotations

import os
from pathlib import Path


def repo_root() -> Path:
    env = os.environ.get("OKMATE_REPO_ROOT")
    if env:
        return Path(env)
    here = Path(__file__).resolve()
    for parent in here.parents:
        if (parent / "okmate-ops").is_dir() and (parent / "Cargo.toml").is_file():
            return parent
    raise SystemExit("could not find okmate repository root")
