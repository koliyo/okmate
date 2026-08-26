from __future__ import annotations

import os
import platform
import shlex
import shutil
import subprocess
import time
import tomllib
from pathlib import Path

from okmate_ops.paths import repo_root

CLI_CRATE = "okmate"
CLI_BINARY = "okmate"
APP_NAME = "Okmate"
BUNDLE_IDENTIFIER = "dev.okmate.app"

BUILD_USAGE = "usage: okmate-ops build"
INSTALL_USAGE = "usage: okmate-ops install cli"
PACKAGE_USAGE = "usage: okmate-ops package desktop"


def run(argv: list[str], *, cwd: Path | None = None, env: dict[str, str] | None = None) -> None:
    started = time.monotonic()
    print(
        f"[okmate-ops] phase=command status=start command={shlex.join(argv)}",
        flush=True,
    )
    try:
        subprocess.run(argv, cwd=cwd or repo_root(), env=env, check=True)
    except subprocess.CalledProcessError:
        elapsed_ms = int((time.monotonic() - started) * 1000)
        print(f"[okmate-ops] phase=command status=failed elapsed_ms={elapsed_ms}", flush=True)
        raise
    elapsed_ms = int((time.monotonic() - started) * 1000)
    print(f"[okmate-ops] phase=command status=done elapsed_ms={elapsed_ms}", flush=True)


def release_binary(root: Path, name: str) -> Path:
    target = Path(os.environ.get("CARGO_TARGET_DIR") or root / "target")
    return target / "release" / name


def require_darwin(kind: str) -> None:
    if platform.system() != "Darwin":
        raise SystemExit(f"{kind} can only be built on macOS.")


def crate_version(root: Path) -> str:
    data = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    return str(data["package"]["version"])


def app_bundle_dir(root: Path) -> Path:
    target = Path(os.environ.get("CARGO_TARGET_DIR") or root / "target")
    return target / "release" / "bundle" / "macos" / f"{APP_NAME}.app"


def info_plist(version: str, executable: str) -> str:
    return f"""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>{APP_NAME}</string>
  <key>CFBundleExecutable</key>
  <string>{executable}</string>
  <key>CFBundleIdentifier</key>
  <string>{BUNDLE_IDENTIFIER}</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>{APP_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>{version}</string>
  <key>CFBundleVersion</key>
  <string>{version}</string>
  <key>LSMinimumSystemVersion</key>
  <string>12.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>NSSupportsAutomaticGraphicsSwitching</key>
  <true/>
</dict>
</plist>
"""


def package_desktop() -> int:
    require_darwin("The macOS app bundle")
    root = repo_root()
    run(["cargo", "build", "--release", "-p", CLI_CRATE], cwd=root)
    src = release_binary(root, CLI_BINARY)
    if not src.is_file():
        raise SystemExit(f"  Error: expected binary not found at '{src}'")
    bundle_dir = app_bundle_dir(root)
    if bundle_dir.exists():
        shutil.rmtree(bundle_dir)
    macos = bundle_dir / "Contents" / "MacOS"
    resources = bundle_dir / "Contents" / "Resources"
    macos.mkdir(parents=True)
    resources.mkdir(parents=True)
    dest = macos / CLI_BINARY
    shutil.copy2(src, dest)
    dest.chmod(0o755)
    icns = root / "packaging" / "macos" / "AppIcon.icns"
    if icns.is_file():
        shutil.copy2(icns, resources / "AppIcon.icns")
    version = crate_version(root)
    (bundle_dir / "Contents" / "Info.plist").write_text(
        info_plist(version, CLI_BINARY), encoding="utf-8"
    )
    (bundle_dir / "Contents" / "PkgInfo").write_bytes(b"APPL????")
    run(["codesign", "--force", "--deep", "--sign", "-", str(bundle_dir)], cwd=root)
    print(bundle_dir)
    return 0


def package_command(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        raise SystemExit(PACKAGE_USAGE)
    sub, rest = argv[0], argv[1:]
    if rest:
        raise SystemExit(PACKAGE_USAGE)
    if sub == "desktop":
        return package_desktop()
    raise SystemExit(PACKAGE_USAGE)


def build_cli() -> int:
    root = repo_root()
    run(["cargo", "build", "--release", "-p", CLI_CRATE], cwd=root)
    src = release_binary(root, CLI_BINARY)
    if not src.is_file():
        raise SystemExit(f"  Error: expected binary not found at '{src}'")
    print(f"Built {src}")
    return 0


def install_cli(*, dest: Path | None = None) -> int:
    root = repo_root()
    dest = dest or Path.home() / ".local" / "bin"
    print(f"Okmate CLI installer\n  Source: {root}\n  Destination: {dest}\n")
    if not dest.is_dir():
        answer = input(f"  '{dest}' does not exist. Create it? [y/N] ")
        if answer.strip().lower() not in {"y", "yes"}:
            print("  Aborted.")
            return 1
        dest.mkdir(parents=True)
    if not os.access(dest, os.W_OK):
        raise SystemExit(f"  Error: '{dest}' is not writable.")
    run(["cargo", "build", "--release", "-p", CLI_CRATE], cwd=root)
    src = release_binary(root, CLI_BINARY)
    if not src.is_file():
        raise SystemExit(f"  Error: expected binary not found at '{src}'")
    shutil.copy2(src, dest / CLI_BINARY)
    (dest / CLI_BINARY).chmod(0o755)
    print(f"\nInstalled:\n  {dest / CLI_BINARY}")
    path = os.environ.get("PATH", "")
    if str(dest) not in path.split(os.pathsep):
        print(f"\n  Note: '{dest}' is not on your PATH.")
    return 0


def build_command(argv: list[str]) -> int:
    if argv:
        raise SystemExit(BUILD_USAGE)
    return build_cli()


def install_command(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        raise SystemExit(INSTALL_USAGE)
    sub, rest = argv[0], argv[1:]
    if rest:
        raise SystemExit(INSTALL_USAGE)
    if sub == "cli":
        return install_cli()
    raise SystemExit(INSTALL_USAGE)
