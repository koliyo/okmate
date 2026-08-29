from __future__ import annotations

import os
import platform
import shlex
import shutil
import subprocess
import tempfile
import time
from pathlib import Path

from okmate_ops.paths import repo_root
from okmate_ops.signing import imported_developer_id_keychain

CLI_CRATE = "okmate"
CLI_BINARY = "okmate"
APP_NAME = "OKMate"
BUNDLE_IDENTIFIER = "com.koliyo.okmate"
APP_ICON = "assets/brand/okmate.icns"
INFO_PLIST = "packaging/macos/Info.plist"
SPARKLE_FEED_BRANCH = "sparkle"
DEFAULT_SPARKLE_FEED_REMOTE = "https://github.com/koliyo/okmate.git"
DEFAULT_SU_FEED_URL = (
    "https://raw.githubusercontent.com/koliyo/okmate/sparkle/appcast.xml"
)
DEFAULT_SU_PUBLIC_ED_KEY = "lSXTvcKDK7P4DEjd+o/k2BM6OPTNGyYdvhIk2DxJyao="

BUILD_USAGE = "usage: okmate-ops build"
INSTALL_USAGE = "usage: okmate-ops install cli"
PACKAGE_USAGE = "usage: okmate-ops package <desktop|sign|appcast|publish-feed>"
PACKAGE_SIGN_USAGE = "usage: okmate-ops package sign [App.app]"
PACKAGE_APPCAST_USAGE = (
    "usage: okmate-ops package appcast <inbox-dir> <download-url-prefix>"
)
PACKAGE_PUBLISH_FEED_USAGE = "usage: okmate-ops package publish-feed <appcast.xml>"


def run(
    argv: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    capture: bool = False,
) -> str:
    started = time.monotonic()
    print(
        f"[okmate-ops] phase=command status=start command={shlex.join(argv)}",
        flush=True,
    )
    try:
        result = subprocess.run(
            argv,
            cwd=cwd or repo_root(),
            env=env,
            check=True,
            text=True,
            capture_output=capture,
        )
    except subprocess.CalledProcessError:
        elapsed_ms = int((time.monotonic() - started) * 1000)
        print(f"[okmate-ops] phase=command status=failed elapsed_ms={elapsed_ms}", flush=True)
        raise
    elapsed_ms = int((time.monotonic() - started) * 1000)
    print(f"[okmate-ops] phase=command status=done elapsed_ms={elapsed_ms}", flush=True)
    return result.stdout if capture else ""


def release_binary(root: Path, name: str) -> Path:
    target = Path(os.environ.get("CARGO_TARGET_DIR") or root / "target")
    return target / "release" / name


def require_darwin(kind: str) -> None:
    if platform.system() != "Darwin":
        raise SystemExit(f"{kind} can only be built on macOS.")


def app_bundle_dir(root: Path) -> Path:
    return root / "dist" / f"{APP_NAME}.app"


def resolve_repo_path(root: Path, path: Path) -> Path:
    resolved = path.expanduser()
    if not resolved.is_absolute():
        resolved = root / resolved
    return resolved.resolve()


def resolve_app_bundle(root: Path, app: Path) -> Path:
    return resolve_repo_path(root, app)


def h35_desktop_root(root: Path) -> Path:
    env = os.environ.get("H35_DESKTOP")
    if env:
        return Path(env)
    return (root / ".." / "h35-desktop").resolve()


def h35_ops_argv(root: Path, args: list[str]) -> list[str]:
    h35 = h35_desktop_root(root)
    if not (h35 / "h35-ops").is_dir():
        raise SystemExit(f"  Error: h35-desktop packager not found at '{h35}'")
    return ["uv", "run", "--directory", str(h35), "--no-dev", "h35-ops", *args]


def package_env(root: Path) -> dict[str, str]:
    env = os.environ.copy()
    env["APP_NAME"] = APP_NAME
    env["BUNDLE_ID"] = env.get("BUNDLE_ID") or BUNDLE_IDENTIFIER
    env["EXECUTABLE"] = CLI_BINARY
    env["CRATE"] = CLI_CRATE
    env["PRODUCT_ROOT"] = str(root)
    env["APP_ICON"] = str(root / APP_ICON)
    env["INFO_PLIST"] = str(root / INFO_PLIST)
    env.setdefault("SU_FEED_URL", DEFAULT_SU_FEED_URL)
    env.setdefault("SU_PUBLIC_ED_KEY", DEFAULT_SU_PUBLIC_ED_KEY)
    return env


def package_desktop() -> int:
    require_darwin("The macOS app bundle")
    root = repo_root()
    run(h35_ops_argv(root, ["package"]), cwd=root, env=package_env(root))
    print(app_bundle_dir(root))
    return 0


def package_sign(argv: list[str]) -> int:
    if argv and argv[0] in ("-h", "--help"):
        raise SystemExit(PACKAGE_SIGN_USAGE)
    if len(argv) > 1:
        raise SystemExit(PACKAGE_SIGN_USAGE)
    require_darwin("Signing")
    root = repo_root()
    app = resolve_app_bundle(root, Path(argv[0]) if argv else app_bundle_dir(root))
    with imported_developer_id_keychain():
        run(h35_ops_argv(root, ["sign", str(app)]), cwd=root, env=package_env(root))
    return 0


def package_appcast(argv: list[str]) -> int:
    if len(argv) != 2 or argv[0] in ("-h", "--help"):
        raise SystemExit(PACKAGE_APPCAST_USAGE)
    require_darwin("Appcast generation")
    root = repo_root()
    inbox = resolve_repo_path(root, Path(argv[0]))
    prefix = argv[1]
    run(h35_ops_argv(root, ["appcast", str(inbox), prefix]), cwd=root, env=package_env(root))
    return 0


def write_sparkle_feed(dest: Path, appcast: Path) -> bool:
    dest.mkdir(parents=True, exist_ok=True)
    target = dest / "appcast.xml"
    text = appcast.read_text(encoding="utf-8")
    if target.is_file() and target.read_text(encoding="utf-8") == text:
        return False
    target.write_text(text, encoding="utf-8")
    return True


def checkout_sparkle_feed_branch(dest: Path, remote: str, branch: str) -> bool:
    cloned = subprocess.run(
        ["git", "clone", "--depth", "1", "--branch", branch, remote, str(dest)],
        text=True,
        capture_output=True,
    )
    if cloned.returncode == 0:
        return True
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    run(["git", "init", "-b", branch], cwd=dest)
    run(["git", "remote", "add", "origin", remote], cwd=dest)
    return False


def commit_and_push_sparkle_feed(dest: Path, branch: str) -> None:
    run(
        ["git", "config", "user.email", "41898282+github-actions[bot]@users.noreply.github.com"],
        cwd=dest,
    )
    run(["git", "config", "user.name", "github-actions[bot]"], cwd=dest)
    run(["git", "add", "appcast.xml"], cwd=dest)
    run(["git", "commit", "-m", "Publish Sparkle appcast"], cwd=dest)
    run(["git", "push", "-u", "origin", f"HEAD:{branch}"], cwd=dest)


def publish_sparkle_feed(appcast: Path, dest: Path, *, remote: str, branch: str) -> None:
    cloned = checkout_sparkle_feed_branch(dest, remote, branch)
    if not write_sparkle_feed(dest, appcast) and cloned:
        return
    commit_and_push_sparkle_feed(dest, branch)


def package_publish_feed(argv: list[str]) -> int:
    if len(argv) != 1 or argv[0] in ("-h", "--help"):
        raise SystemExit(PACKAGE_PUBLISH_FEED_USAGE)
    root = repo_root()
    appcast = resolve_repo_path(root, Path(argv[0]))
    if not appcast.is_file():
        raise SystemExit(f"okmate-ops package publish-feed: missing {appcast}")
    remote = os.environ.get("SPARKLE_FEED_REMOTE") or DEFAULT_SPARKLE_FEED_REMOTE
    branch = os.environ.get("SPARKLE_FEED_BRANCH") or SPARKLE_FEED_BRANCH
    parent = Path(tempfile.mkdtemp(prefix="okmate-sparkle-feed-"))
    dest = parent / "feed"
    try:
        publish_sparkle_feed(appcast, dest, remote=remote, branch=branch)
    finally:
        shutil.rmtree(parent, ignore_errors=True)
    return 0


def package_command(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        raise SystemExit(PACKAGE_USAGE)
    sub, rest = argv[0], argv[1:]
    if sub == "desktop":
        if rest:
            raise SystemExit(PACKAGE_USAGE)
        return package_desktop()
    if sub == "sign":
        return package_sign(rest)
    if sub == "appcast":
        return package_appcast(rest)
    if sub == "publish-feed":
        return package_publish_feed(rest)
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
    print(f"okmate CLI installer\n  Source: {root}\n  Destination: {dest}\n")
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
