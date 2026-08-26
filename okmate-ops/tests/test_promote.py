from okmate_ops.ghutil import DEFAULT_CHECKS
from okmate_ops.promote import (
    PROMOTE_TAG_USAGE,
    PROMOTE_USAGE,
    promote_command,
    promote_tag,
    wait_for_promote_ci,
)


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


def test_promote_tag_annotates_and_pushes_origin_main(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    waited: list[str] = []
    monkeypatch.setattr("okmate_ops.promote.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.promote.subprocess.run",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr("okmate_ops.promote.wait_for_promote_ci", waited.append)

    assert promote_tag("v1.2.3") == 0
    assert waited == ["abc"]
    assert calls == [
        ["git", "fetch", "origin"],
        ["git", "tag", "-a", "v1.2.3", "-m", "v1.2.3", "origin/main"],
        ["git", "push", "origin", "v1.2.3"],
    ]


def test_promote_tag_force_moves_dev(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.setattr("okmate_ops.promote.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.promote.subprocess.run",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr("okmate_ops.promote.wait_for_promote_ci", lambda sha: None)

    assert promote_tag("dev") == 0
    assert calls == [
        ["git", "fetch", "origin"],
        ["git", "tag", "-a", "-f", "dev", "-m", "dev", "origin/main"],
        ["git", "push", "--force", "origin", "dev"],
    ]


def test_promote_tag_does_not_push_when_ci_fails(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.setattr("okmate_ops.promote.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.promote.subprocess.run",
        lambda *args, **kwargs: type("Result", (), {"returncode": 0, "stdout": "abc\n"})(),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.run",
        lambda argv, cwd=None: calls.append(list(argv)),
    )
    monkeypatch.setattr(
        "okmate_ops.promote.wait_for_promote_ci",
        lambda sha: (_ for _ in ()).throw(SystemExit(f"CI failed for {sha}")),
    )
    try:
        promote_tag("v1.2.3")
    except SystemExit as exc:
        assert "abc" in str(exc)
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


def test_promote_tag_requires_v_prefix_or_dev() -> None:
    try:
        promote_tag("1.2.3")
    except SystemExit as exc:
        assert "dev" in str(exc)
    else:
        raise AssertionError("expected SystemExit")
