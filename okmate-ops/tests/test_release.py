import subprocess
from pathlib import Path

from okmate_ops.ghutil import DEFAULT_CHECKS
from okmate_ops.release import (
    RELEASE_USAGE,
    run_release,
    push_tap_version,
    push_version_update,
    release_command,
    wait_for_release_ci,
)
from okmate_ops.version import CASK, first_package_version, release_files_match, tap_files_match


def test_release_usage() -> None:
    try:
        release_command([])
    except SystemExit as exc:
        assert str(exc) == RELEASE_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_release_command_routes(monkeypatch) -> None:
    called: list[str] = []
    monkeypatch.setattr(
        "okmate_ops.release.run_release",
        lambda tag, from_ref="main", force=False, dry_run=False: called.append(
            f"{tag}:{from_ref}:{force}:{dry_run}"
        )
        or 0,
    )
    assert release_command(["v1.2.3"]) == 0
    assert release_command(["v1.2.3", "--from", "release"]) == 0
    assert release_command(["v1.2.3", "--force"]) == 0
    assert release_command(["patch"]) == 0
    assert release_command(["patch", "--dry-run"]) == 0
    assert called == [
        "v1.2.3:main:False:False",
        "v1.2.3:release:False:False",
        "v1.2.3:main:True:False",
        "patch:main:False:False",
        "patch:main:False:True",
    ]


def test_run_release_dispatches_release_from_actions(monkeypatch, tmp_path) -> None:
    released: list[str] = []
    monkeypatch.setenv("GITHUB_ACTIONS", "true")
    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.release.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr("okmate_ops.release.run", lambda argv, cwd=None: None)
    monkeypatch.setattr(
        "okmate_ops.release.push_version_update",
        lambda version, from_ref, remote_sha: "newsha",
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_release_ci",
        lambda sha, from_ref="main": None,
    )
    monkeypatch.setattr("okmate_ops.release.dispatch_hosted_release", released.append)
    monkeypatch.setattr("okmate_ops.release.push_tap_version", lambda version: None)
    assert run_release("v1.2.3") == 0
    assert released == ["v1.2.3"]


def test_run_release_pushes_version_then_tags(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    waited: list[str] = []
    monkeypatch.delenv("GITHUB_ACTIONS", raising=False)
    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.release.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.release.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_version_update",
        lambda version, from_ref, remote_sha: f"{version}:{from_ref}:{remote_sha}",
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_release_ci",
        lambda sha, from_ref="main": waited.append(sha),
    )
    taps: list[str] = []
    monkeypatch.setattr("okmate_ops.release.push_tap_version", taps.append)

    assert run_release("v1.2.3") == 0
    assert waited == ["1.2.3:main:abc"]
    assert taps == ["1.2.3"]
    assert calls == [
        ["git", "fetch", "origin", "refs/heads/main:refs/remotes/origin/main"],
        ["git", "tag", "-a", "v1.2.3", "-m", "v1.2.3", "1.2.3:main:abc"],
        ["git", "push", "origin", "v1.2.3"],
    ]


def test_run_release_force_overwrites_versioned_tag(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.delenv("GITHUB_ACTIONS", raising=False)
    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.release.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.release.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_version_update",
        lambda version, from_ref, remote_sha: "newsha",
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_release_ci",
        lambda sha, from_ref="main": None,
    )
    monkeypatch.setattr("okmate_ops.release.push_tap_version", lambda version: None)

    assert run_release("v1.2.3", force=True) == 0
    assert calls == [
        ["git", "fetch", "origin", "refs/heads/main:refs/remotes/origin/main"],
        ["git", "tag", "-a", "-f", "v1.2.3", "-m", "v1.2.3", "newsha"],
        ["git", "push", "--force", "origin", "v1.2.3"],
    ]


def test_run_release_force_moves_dev(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.delenv("GITHUB_ACTIONS", raising=False)
    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.release.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.release.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_release_ci",
        lambda sha, from_ref="main": None,
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_version_update",
        lambda *args, **kwargs: (_ for _ in ()).throw(AssertionError("dev must not bump versions")),
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_tap_version",
        lambda *args, **kwargs: (_ for _ in ()).throw(AssertionError("dev must not bump the tap")),
    )

    assert run_release("dev") == 0
    assert calls == [
        ["git", "fetch", "origin", "refs/heads/main:refs/remotes/origin/main"],
        ["git", "tag", "-a", "-f", "dev", "-m", "dev", "abc"],
        ["git", "push", "--force", "origin", "dev"],
    ]


def test_run_release_does_not_push_when_ci_fails(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.release.git_capture",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.release.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_version_update",
        lambda version, from_ref, remote_sha: "newsha",
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_release_ci",
        lambda sha, from_ref="main": (_ for _ in ()).throw(SystemExit(f"CI failed for {sha}")),
    )
    try:
        run_release("v1.2.3")
    except SystemExit as exc:
        assert "newsha" in str(exc)
    else:
        raise AssertionError("expected SystemExit")
    assert calls == [["git", "fetch", "origin", "refs/heads/main:refs/remotes/origin/main"]]


def test_wait_for_release_ci_waits_default_checks(monkeypatch) -> None:
    seen: list[str] = []
    dispatched: list[list[str]] = []
    monkeypatch.delenv("GITHUB_ACTIONS", raising=False)
    monkeypatch.setattr("okmate_ops.release.github_repo", lambda: "koliyo/okmate")
    monkeypatch.setattr("okmate_ops.release.gh_run", lambda args: type("R", (), {"stdout": ""})())
    monkeypatch.setattr(
        "okmate_ops.release.dispatch_hosted_ci",
        lambda from_ref: dispatched.append([from_ref]),
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_check",
        lambda **kwargs: seen.append(kwargs["check"]),
    )
    wait_for_release_ci("abc")
    assert dispatched == []
    assert seen == list(DEFAULT_CHECKS)
    assert DEFAULT_CHECKS == ("Code Formatting & Lints", "Test")


def test_wait_for_release_ci_dispatches_from_actions(monkeypatch) -> None:
    seen: list[str] = []
    dispatched: list[str] = []
    monkeypatch.setenv("GITHUB_ACTIONS", "true")
    monkeypatch.setattr("okmate_ops.release.github_repo", lambda: "koliyo/okmate")
    monkeypatch.setattr("okmate_ops.release.gh_run", lambda args: type("R", (), {"stdout": ""})())
    monkeypatch.setattr(
        "okmate_ops.release.dispatch_hosted_ci",
        lambda from_ref: dispatched.append(from_ref),
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_check",
        lambda **kwargs: seen.append(kwargs["check"]),
    )
    wait_for_release_ci("abc", from_ref="release")
    assert dispatched == ["release"]
    assert seen == list(DEFAULT_CHECKS)


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
    (repo / "Cargo.toml").write_text('[workspace.package]\nversion = "0.1.0"\n', encoding="utf-8")
    (repo / "okf" / "Cargo.toml").write_text('[package]\nversion = "0.1.0"\n', encoding="utf-8")
    (repo / "Cargo.lock").write_text(
        '[[package]]\nname = "okf"\nversion = "0.1.0"\n\n[[package]]\nname = "okmate"\nversion = "0.1.0"\n',
        encoding="utf-8",
    )
    _git(repo, "add", ".")
    _git(repo, "commit", "-m", "seed")
    _git(repo, "push", "origin", "HEAD:main")
    sha = _git(repo, "rev-parse", "HEAD").stdout.strip()
    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: repo)

    new_sha = push_version_update("2.3.4", "main", sha)
    assert new_sha != sha
    _git(repo, "fetch", "origin")
    checkout = tmp_path / "check"
    subprocess.run(["git", "clone", str(origin), str(checkout)], check=True)
    assert release_files_match(checkout, "2.3.4")
    assert first_package_version((checkout / "Cargo.toml").read_text(encoding="utf-8")) == "2.3.4"
    same = push_version_update("2.3.4", "main", new_sha)
    assert same == new_sha


def test_push_tap_version_commits_and_pushes(tmp_path: Path, monkeypatch) -> None:
    origin = tmp_path / "tap.git"
    seed = tmp_path / "seed"
    subprocess.run(["git", "init", "--bare", "-b", "main", str(origin)], check=True)
    subprocess.run(["git", "clone", str(origin), str(seed)], check=True)
    _git(seed, "config", "user.email", "ops@example.com")
    _git(seed, "config", "user.name", "ops")
    _git(seed, "config", "commit.gpgsign", "false")
    (seed / "Casks").mkdir()
    (seed / CASK).write_text(
        'cask "okmate" do\n'
        '  version "0.1.0"\n'
        "  sha256 :no_check\n"
        '  url "https://github.com/koliyo/okmate/releases/download/v#{version}/OKMate.zip"\n'
        '  livecheck do\n    url "https://github.com/koliyo/okmate/releases/latest"\n    strategy :github_latest\n  end\n'
        "  auto_updates true\n"
        '  app "OKMate.app"\n'
        '  binary "#{appdir}/OKMate.app/Contents/MacOS/okmate", target: "okmate"\n'
        "end\n",
        encoding="utf-8",
    )
    _git(seed, "add", ".")
    _git(seed, "commit", "-m", "seed")
    _git(seed, "push", "origin", "HEAD:main")
    monkeypatch.setenv("HOMEBREW_TAP", str(origin))
    monkeypatch.setenv("GIT_AUTHOR_NAME", "ops")
    monkeypatch.setenv("GIT_AUTHOR_EMAIL", "ops@example.com")
    monkeypatch.setenv("GIT_COMMITTER_NAME", "ops")
    monkeypatch.setenv("GIT_COMMITTER_EMAIL", "ops@example.com")
    monkeypatch.setenv("GIT_CONFIG_GLOBAL", "/dev/null")
    monkeypatch.setenv("GIT_CONFIG_SYSTEM", "/dev/null")
    monkeypatch.setattr(
        "okmate_ops.release.run",
        lambda argv, cwd=None: subprocess.run(argv, cwd=cwd, check=True),
    )

    def capture(argv, cwd=None):
        return subprocess.run(argv, cwd=cwd, capture_output=True, text=True)

    monkeypatch.setattr("okmate_ops.release.git_capture", capture)
    push_tap_version("2.3.4")
    checkout = tmp_path / "check"
    subprocess.run(["git", "clone", str(origin), str(checkout)], check=True)
    assert tap_files_match(checkout, "2.3.4")
    push_tap_version("2.3.4")


def test_run_release_dry_run_does_not_push(monkeypatch, tmp_path, capsys) -> None:
    calls: list[list[str]] = []
    cargo = 'version = "1.2.3"\n'
    lock = (
        '[[package]]\nname = "okf"\nversion = "1.2.3"\n\n'
        '[[package]]\nname = "okmate"\nversion = "1.2.3"\n'
    )

    def capture(argv, cwd=None):
        if argv[:2] == ["git", "rev-parse"]:
            return type("Result", (), {"returncode": 0, "stdout": "abc\n"})()
        path = argv[2].split(":", 1)[1]
        if path == "Cargo.lock":
            return _show_result(lock)
        return _show_result(cargo)

    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr("okmate_ops.release.git_capture", capture)
    monkeypatch.setattr(
        "okmate_ops.release.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_version_update",
        lambda *args, **kwargs: (_ for _ in ()).throw(AssertionError("dry-run must not bump")),
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_release_ci",
        lambda sha: (_ for _ in ()).throw(AssertionError("dry-run must not wait")),
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_tap_version",
        lambda version: (_ for _ in ()).throw(AssertionError("dry-run must not bump the tap")),
    )

    assert run_release("v1.2.3", dry_run=True) == 0
    assert calls == [["git", "fetch", "origin", "refs/heads/main:refs/remotes/origin/main"]]
    out = capsys.readouterr().out
    assert "okmate-ops release v1.2.3" in out
    assert "dry-run: release files match=true" in out


def test_run_release_requires_v_prefix_or_dev() -> None:
    try:
        run_release("1.2.3")
    except SystemExit as exc:
        assert "dev" in str(exc)
    else:
        raise AssertionError("expected SystemExit")


def _show_result(text: str):
    return type("Result", (), {"returncode": 0, "stdout": text})()


def test_run_release_patch_resolves_from_sha(monkeypatch, tmp_path, capsys) -> None:
    calls: list[list[str]] = []
    waited: list[str] = []
    monkeypatch.delenv("GITHUB_ACTIONS", raising=False)
    cargo = 'version = "0.1.2"\n'
    lock = (
        '[[package]]\nname = "okf"\nversion = "0.1.2"\n\n'
        '[[package]]\nname = "okmate"\nversion = "0.1.2"\n'
    )

    def capture(argv, cwd=None):
        if argv[:2] == ["git", "rev-parse"]:
            return type("Result", (), {"returncode": 0, "stdout": "abc\n"})()
        path = argv[2].split(":", 1)[1]
        if path == "Cargo.lock":
            return _show_result(lock)
        return _show_result(cargo)

    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr("okmate_ops.release.git_capture", capture)
    monkeypatch.setattr(
        "okmate_ops.release.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.release.push_version_update",
        lambda version, from_ref, remote_sha: f"{version}:{from_ref}:{remote_sha}",
    )
    monkeypatch.setattr(
        "okmate_ops.release.wait_for_release_ci",
        lambda sha, from_ref="main": waited.append(sha),
    )
    taps: list[str] = []
    monkeypatch.setattr("okmate_ops.release.push_tap_version", taps.append)

    assert run_release("patch") == 0
    assert waited == ["0.1.3:main:abc"]
    assert taps == ["0.1.3"]
    assert calls == [
        ["git", "fetch", "origin", "refs/heads/main:refs/remotes/origin/main"],
        ["git", "tag", "-a", "v0.1.3", "-m", "v0.1.3", "0.1.3:main:abc"],
        ["git", "push", "origin", "v0.1.3"],
    ]
    assert "okmate-ops release v0.1.3" in capsys.readouterr().out


def test_run_release_mismatched_crates_exit(monkeypatch, tmp_path) -> None:
    def capture(argv, cwd=None):
        if argv[:2] == ["git", "rev-parse"]:
            return type("Result", (), {"returncode": 0, "stdout": "abc\n"})()
        path = argv[2].split(":", 1)[1]
        if path == "okf/Cargo.toml":
            return _show_result('version = "0.2.0"\n')
        if path == "Cargo.lock":
            return _show_result(
                '[[package]]\nname = "okf"\nversion = "0.2.0"\n\n'
                '[[package]]\nname = "okmate"\nversion = "0.1.2"\n'
            )
        return _show_result('version = "0.1.2"\n')

    monkeypatch.setattr("okmate_ops.release.repo_root", lambda: tmp_path)
    monkeypatch.setattr("okmate_ops.release.git_capture", capture)
    monkeypatch.setattr("okmate_ops.release.run", lambda argv, cwd=None: None)
    try:
        run_release("minor")
    except SystemExit as exc:
        assert "differ" in str(exc)
    else:
        raise AssertionError("expected SystemExit")
