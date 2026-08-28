#!/usr/bin/env python3
"""Build the macOS-masked PNG and ICNS from the full-bleed master.

Run from the repository root:

    uv run --with pillow assets/brand/generate_icons.py
"""

from __future__ import annotations

import shutil
import subprocess
from pathlib import Path

from PIL import Image, ImageDraw

BRAND = Path(__file__).resolve().parent
MASTER = BRAND / "okmate-app-icon.png"
MASKED = BRAND / "okmate-app-icon-macos.png"
ICNS = BRAND / "okmate.icns"

CANVAS = 1024
CONTENT = 824
RADIUS = 185.4
SUPERSAMPLE = 4

ICONSET_SIZES = (
    ("icon_16x16.png", 16),
    ("icon_16x16@2x.png", 32),
    ("icon_32x32.png", 32),
    ("icon_32x32@2x.png", 64),
    ("icon_128x128.png", 128),
    ("icon_128x128@2x.png", 256),
    ("icon_256x256.png", 256),
    ("icon_256x256@2x.png", 512),
    ("icon_512x512.png", 512),
    ("icon_512x512@2x.png", 1024),
)


def macos_icon(master: Image.Image) -> Image.Image:
    square = master.convert("RGBA")
    if square.size != (CANVAS, CANVAS):
        square = square.resize((CANVAS, CANVAS), Image.Resampling.LANCZOS)

    margin = (CANVAS - CONTENT) / 2
    scale = SUPERSAMPLE
    mask_size = CANVAS * scale
    mask = Image.new("L", (mask_size, mask_size), 0)
    draw = ImageDraw.Draw(mask)
    inset = margin * scale
    box = (inset, inset, mask_size - inset, mask_size - inset)
    draw.rounded_rectangle(box, radius=RADIUS * scale, fill=255)
    mask = mask.resize((CANVAS, CANVAS), Image.Resampling.LANCZOS)

    content = square.resize((CONTENT, CONTENT), Image.Resampling.LANCZOS)
    placed = Image.new("RGBA", (CANVAS, CANVAS), (0, 0, 0, 0))
    placed.paste(content, (round(margin), round(margin)))
    placed.putalpha(mask)
    return placed


def write_iconset(masked: Image.Image, iconset: Path) -> None:
    iconset.mkdir(parents=True, exist_ok=True)
    for name, size in ICONSET_SIZES:
        resized = masked.resize((size, size), Image.Resampling.LANCZOS)
        resized.save(iconset / name, format="PNG")


def main() -> None:
    if not MASTER.is_file():
        raise SystemExit(f"missing master icon: {MASTER}")
    with Image.open(MASTER) as master:
        masked = macos_icon(master)
    masked.save(MASKED, format="PNG")

    iconset = BRAND / ".iconset-build" / "AppIcon.iconset"
    if iconset.exists():
        shutil.rmtree(iconset)
    write_iconset(masked, iconset)
    produced = iconset.parent / "AppIcon.icns"
    subprocess.run(
        ["iconutil", "-c", "icns", str(iconset), "-o", str(produced)],
        check=True,
    )
    shutil.copy2(produced, ICNS)
    shutil.rmtree(iconset.parent)

    print(f"wrote {MASKED}")
    print(f"wrote {ICNS}")


if __name__ == "__main__":
    main()
