#!/usr/bin/env python3
"""Prefix root-absolute URLs so GitHub project Pages serve under /okmate/."""

from __future__ import annotations

import sys
from pathlib import Path

SEGMENT = "okmate"


def rewrite(text: str) -> str:
    out = text
    for needle in ('href="/', 'src="/', 'content="/', "](/", '"route": "/'):
        out = out.replace(needle, f"{needle}{SEGMENT}/")
    out = out.replace(f"/{SEGMENT}/{SEGMENT}/", f"/{SEGMENT}/")
    return out


def main(root: Path) -> None:
    for path in root.rglob("*"):
        if not path.is_file() or path.suffix not in {".html", ".json", ".txt", ".xml"}:
            continue
        original = path.read_text(encoding="utf-8")
        updated = rewrite(original)
        if updated != original:
            path.write_text(updated, encoding="utf-8")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.stderr.write("usage: gh-pages-prefix.py DIST_DIR\n")
        raise SystemExit(2)
    main(Path(sys.argv[1]))
