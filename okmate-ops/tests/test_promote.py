import subprocess
from pathlib import Path

from okmate_ops.ghutil import DEFAULT_CHECKS
from okmate_ops.promote import (
    PROMOTE_TAG_USAGE,
    PROMOTE_USAGE,
    promote_command,
    promote_tag,
    push_version_update,
    wait_for_promote_ci,
)
from okmate_ops.version import first_package_version, release_files_match


def test_promote_usage() -> None:
    try:
        promote_command(["preview"])
    except SystemExit as exc:
        assert str(exc) == PROMOTE_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_promote_tag_usage() -> None:
    try:
        promote_command(["tag"])
    except SystemExit as exc:
        assert str(exc) == PROMOTE_TAG_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_promote_command_routes(monkeypatch) -> None:
    called: list[str] = []
    monkeypatch.setattr(
        "okmate_ops.promote.promote_tag",
        lambda tag, from_ref="main": called.append(f"{tag}:{from_ref}") or 0,
    )
    assert promote_command(["tag", "v1.2.3"]) == 0
    assert promote_command(["tag", "v1.2.3", "--from", "release"]) == 0
    assert called == ["v1.2.3:main", "v1.2.3:release"]


def test_promote_tag_pushes_version_then_tags(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    waited: list[str] = []
    monkeypatch.setattr("okmate_ops.promote.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.promote.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.push_version_update",
        lambda version, from_ref, remote_sha: f"{version}:{from_ref}:{remote_sha}",
    )
    monkeypatch.setattr("okmate_ops.promote.wait_for_promote_ci", waited.append)

    assert promote_tag("v1.2.3") == 0
    assert waited == ["1.2.3:main:abc"]
    assert calls == [
        ["git", "fetch", "origin"],
        ["git", "tag", "-a", "v1.2.3", "-m", "v1.2.3", "1.2.3:main:abc"],
        ["git", "push", "origin", "v1.2.3"],
    ]


def test_promote_tag_force_moves_dev(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.setattr("okmate_ops.promote.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.promote.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr("okmate_ops.promote.wait_for_promote_ci", lambda sha: None)
    monkeypatch.setattr(
        "okmate_ops.promote.push_version_update",
        lambda *args, **kwargs: (_ for _ in ()).throw(AssertionError("dev must not bump versions")),
    )

    assert promote_tag("dev") == 0
    assert calls == [
        ["git", "fetch", "origin"],
        ["git", "tag", "-a", "-f", "dev", "-m", "dev", "abc"],
        ["git", "push", "--force", "origin", "dev"],
    ]


def test_promote_tag_does_not_push_when_ci_fails(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.setattr("okmate_ops.promote.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.promote.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.push_version_update",
        lambda version, from_ref, remote_sha: "newsha",
    )
    monkeypatch.setattr(
        "okmate_ops.promote.wait_for_promote_ci",
        lambda sha: (_ for _ in ()).throw(SystemExit(f"CI failed for {sha}")),
    )
    try:
        promote_tag("v1.2.3")
    except SystemExit as exc:
        assert "newsha" in str(exc)
    else:
        raise AssertionError("expected SystemExit")
    assert calls == [["git", "fetch", "origin"]]


def test_wait_for_promote_ci_waits_default_checks(monkeypatch) -> None:
    seen: list[str] = []
    monkeypatch.setattr("okmate_ops.promote.github_repo", lambda: "koliyo/okmate")
    monkeypatch.setattr("okmate_ops.promote.gh_run", lambda args: type("R", (), {"stdout": ""})())
    monkeypatch.setattr(
        "okmate_ops.promote.wait_for_check",
        lambda **kwargs: seen.append(kwargs["check"]),
    )
    wait_for_promote_ci("abc")
    assert seen == list(DEFAULT_CHECKS)
    assert DEFAULT_CHECKS == ("Test",)


def _git(cwd: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(["git", *args], cwd=cwd, check=True, text=True, capture_output=True)


def test_push_version_update_commits_and_pushes(tmp_path: Path, monkeypatch) -> None:
    origin = tmp_path / "origin.git"
    repo = tmp_path / "repo"
    subprocess.run(["git", "init", "--bare", "-b", "main", str(origin)], check=True)
    subprocess.run(["git", "clone", str(origin), str(repo)], check=True)
    _git(repo, "config", "user.email", "ops@example.com")
    _git(repo, "config", "user.name", "ops")
    _git(repo, "config", "commit.gpgsign", "false")
    (repo / "okf").mkdir()
    (repo / "Casks").mkdir()
    (repo / "Cargo.toml").write_text('[workspace.package]\nversion = "0.1.0"\n', encoding="utf-8")
    (repo / "okf" / "Cargo.toml").write_text('[package]\nversion = "0.1.0"\n', encoding="utf-8")
    (repo / "Cargo.lock").write_text(
        '[[package]]\nname = "okf"\nversion = "0.1.0"\n\n[[package]]\nname = "okmate"\nversion = "0.1.0"\n',
        encoding="utf-8",
    )
    (repo / "Casks" / "okmate.rb").write_text(
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
    _git(repo, "add", ".")
    _git(repo, "commit", "-m", "seed")
    _git(repo, "push", "origin", "HEAD:main")
    sha = _git(repo, "rev-parse", "HEAD").stdout.strip()
    monkeypatch.setattr("okmate_ops.promote.repo_root", lambda: repo)

    new_sha = push_version_update("2.3.4", "main", sha)
    assert new_sha != sha
    _git(repo, "fetch", "origin")
    checkout = tmp_path / "check"
    subprocess.run(["git", "clone", str(origin), str(checkout)], check=True)
    assert release_files_match(checkout, "2.3.4")
    assert first_package_version((checkout / "Cargo.toml").read_text(encoding="utf-8")) == "2.3.4"
    same = push_version_update("2.3.4", "main", new_sha)
    assert same == new_sha


def test_promote_tag_requires_v_prefix_or_dev() -> None:
    try:
        promote_tag("1.2.3")
    except SystemExit as exc:
        assert "dev" in str(exc)
    else:
        raise AssertionError("expected SystemExit")
