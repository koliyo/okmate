from pathlib import Path

from okmate_ops.paths import repo_root
from okmate_ops.version import (
    CASK,
    apply_release_version,
    cask_tracks_self_update,
    first_package_version,
    parse_release_version,
    release_files_match,
    replace_cask_version,
    replace_lock_crate_versions,
    replace_package_versions,
)


def test_parse_release_version() -> None:
    assert parse_release_version("v1.2.3") == "1.2.3"
    try:
        parse_release_version("1.2.3")
    except SystemExit as exc:
        assert "v*" in str(exc)
    else:
        raise AssertionError("expected SystemExit")
    try:
        parse_release_version("vnext")
    except SystemExit as exc:
        assert "vX.Y.Z" in str(exc)
    else:
        raise AssertionError("expected SystemExit")


def test_replace_package_versions_skips_dependency_tables() -> None:
    text = '[workspace.package]\nversion = "0.1.0"\n\n[package]\nversion = "0.1.0"\nclap = { version = "4.5" }\n'
    updated = replace_package_versions(text, "2.0.0")
    assert first_package_version(updated) == "2.0.0"
    assert updated.count('version = "2.0.0"') == 2
    assert 'clap = { version = "4.5" }' in updated


def test_replace_lock_and_cask() -> None:
    lock = '[[package]]\nname = "okf"\nversion = "0.1.0"\n\n[[package]]\nname = "okmate"\nversion = "0.1.0"\n'
    assert 'version = "3.1.4"' in replace_lock_crate_versions(lock, "3.1.4")
    cask = 'cask "okmate" do\n  version "0.1.0"\n  url "https://github.com/koliyo/okmate/releases/download/v#{version}/Okmate.zip"\nend\n'
    updated = replace_cask_version(cask, "3.1.4")
    assert 'version "3.1.4"' in updated
    assert "v#{version}/Okmate.zip" in updated


def test_apply_release_version(tmp_path: Path) -> None:
    (tmp_path / "okf").mkdir()
    (tmp_path / "Casks").mkdir()
    (tmp_path / "Cargo.toml").write_text('[workspace.package]\nversion = "0.1.0"\n', encoding="utf-8")
    (tmp_path / "okf" / "Cargo.toml").write_text('[package]\nversion = "0.1.0"\n', encoding="utf-8")
    (tmp_path / "Cargo.lock").write_text(
        '[[package]]\nname = "okf"\nversion = "0.1.0"\n\n[[package]]\nname = "okmate"\nversion = "0.1.0"\n',
        encoding="utf-8",
    )
    (tmp_path / "Casks" / "okmate.rb").write_text(
        'cask "okmate" do\n'
        '  version "0.1.0"\n'
        "  sha256 :no_check\n"
        '  url "https://github.com/koliyo/okmate/releases/download/v#{version}/Okmate.zip"\n'
        '  livecheck do\n    url "https://github.com/koliyo/okmate/releases/latest"\n    strategy :github_latest\n  end\n'
        "  auto_updates true\n"
        '  app "Okmate.app"\n'
        '  binary "#{appdir}/Okmate.app/Contents/MacOS/okmate", target: "okmate"\n'
        "end\n",
        encoding="utf-8",
    )
    apply_release_version(tmp_path, "9.8.7")
    assert release_files_match(tmp_path, "9.8.7")
    assert not release_files_match(tmp_path, "0.1.0")


def test_checked_in_cask_matches_crate_and_sparkle_channel() -> None:
    root = repo_root()
    version = first_package_version((root / "Cargo.toml").read_text(encoding="utf-8"))
    assert version
    assert release_files_match(root, version)
    cask = (root / CASK).read_text(encoding="utf-8")
    assert cask_tracks_self_update(cask, version)
    assert "sha256 :no_check" in cask
