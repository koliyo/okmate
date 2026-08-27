from __future__ import annotations

import re
from pathlib import Path

PACKAGE_VERSION_RE = re.compile(r'^version = "([^"]+)"', re.MULTILINE)
LOCK_PACKAGE_RE = re.compile(
    r'(^\[\[package\]\]\nname = "(?:okf|okmate)"\n)version = "[^"]+"',
    re.MULTILINE,
)
CASK_VERSION_RE = re.compile(r'^(\s*version ")[^"]+(")', re.MULTILINE)
SEMVER_RE = re.compile(r"^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$")

CARGO_TOML = Path("Cargo.toml")
OKF_CARGO_TOML = Path("okf") / "Cargo.toml"
CARGO_LOCK = Path("Cargo.lock")
CASK = Path("Casks") / "okmate.rb"

CASK_TEMPLATE = r"""\
cask "okmate" do
  version "{release}"
  sha256 :no_check

  url "https://github.com/koliyo/okmate/releases/download/v#{version}/Okmate.zip"
  name "Okmate"
  desc "Open knowledge mate for OKF bundles"
  homepage "https://github.com/koliyo/okmate"

  livecheck do
    url "https://github.com/koliyo/okmate/releases/latest"
    strategy :github_latest
  end

  auto_updates true

  app "Okmate.app"
  binary "#{appdir}/Okmate.app/Contents/MacOS/okmate", target: "okmate"
end
"""

VERSION_PATHS = (CARGO_TOML, OKF_CARGO_TOML, CARGO_LOCK)
DEFAULT_HOMEBREW_TAP = "https://github.com/koliyo/homebrew-okmate.git"
RELEASE_ZIP = "Okmate.zip"
RELEASE_DOWNLOAD = "https://github.com/koliyo/okmate/releases/download/"
APPCAST_LATEST = "https://github.com/koliyo/okmate/releases/latest"


def parse_release_version(tag: str) -> str:
    if not tag.startswith("v") or len(tag) < 2:
        raise SystemExit("promote tag requires a v* name or the movable dev tag")
    version = tag[1:]
    if not SEMVER_RE.fullmatch(version):
        raise SystemExit(f"promote tag {tag} is not a vX.Y.Z version")
    return version


def first_package_version(text: str) -> str | None:
    match = PACKAGE_VERSION_RE.search(text)
    return match.group(1) if match else None


def replace_package_versions(text: str, version: str) -> str:
    updated, count = PACKAGE_VERSION_RE.subn(f'version = "{version}"', text)
    if count == 0:
        raise SystemExit("could not find package version")
    return updated


def replace_lock_crate_versions(text: str, version: str) -> str:
    updated, count = LOCK_PACKAGE_RE.subn(rf'\1version = "{version}"', text)
    if count == 0:
        raise SystemExit("could not find okf/okmate versions in Cargo.lock")
    return updated


def replace_cask_version(text: str, version: str) -> str:
    updated, count = CASK_VERSION_RE.subn(rf"\g<1>{version}\2", text)
    if count != 1:
        raise SystemExit("could not find Homebrew cask version")
    return updated


def apply_release_version(root: Path, version: str) -> list[Path]:
    if not SEMVER_RE.fullmatch(version):
        raise SystemExit(f"invalid release version: {version}")
    changed: list[Path] = []

    cargo = root / CARGO_TOML
    cargo.write_text(replace_package_versions(cargo.read_text(encoding="utf-8"), version), encoding="utf-8")
    changed.append(CARGO_TOML)

    okf_cargo = root / OKF_CARGO_TOML
    okf_cargo.write_text(
        replace_package_versions(okf_cargo.read_text(encoding="utf-8"), version),
        encoding="utf-8",
    )
    changed.append(OKF_CARGO_TOML)

    lock = root / CARGO_LOCK
    lock.write_text(replace_lock_crate_versions(lock.read_text(encoding="utf-8"), version), encoding="utf-8")
    changed.append(CARGO_LOCK)
    return changed


def apply_cask_version(root: Path, version: str) -> Path:
    if not SEMVER_RE.fullmatch(version):
        raise SystemExit(f"invalid release version: {version}")
    cask = root / CASK
    if cask.is_file():
        cask.write_text(replace_cask_version(cask.read_text(encoding="utf-8"), version), encoding="utf-8")
    else:
        cask.parent.mkdir(parents=True, exist_ok=True)
        cask.write_text(CASK_TEMPLATE.replace("{release}", version), encoding="utf-8")
    return CASK


def cask_tracks_self_update(text: str, version: str) -> bool:
    return (
        f'version "{version}"' in text
        and "auto_updates true" in text
        and RELEASE_ZIP in text
        and RELEASE_DOWNLOAD in text
        and APPCAST_LATEST in text
        and "strategy :github_latest" in text
        and 'app "Okmate.app"' in text
        and "Contents/MacOS/okmate" in text
        and 'target: "okmate"' in text
        and text.count("binary ") == 1
    )


def tap_files_match(root: Path, version: str) -> bool:
    cask = root / CASK
    if not cask.is_file():
        return False
    return cask_tracks_self_update(cask.read_text(encoding="utf-8"), version)


def release_files_match(root: Path, version: str) -> bool:
    cargo = first_package_version((root / CARGO_TOML).read_text(encoding="utf-8"))
    okf = first_package_version((root / OKF_CARGO_TOML).read_text(encoding="utf-8"))
    lock = (root / CARGO_LOCK).read_text(encoding="utf-8") if (root / CARGO_LOCK).is_file() else ""
    return (
        cargo == version
        and okf == version
        and f'name = "okmate"\nversion = "{version}"' in lock
        and f'name = "okf"\nversion = "{version}"' in lock
    )
