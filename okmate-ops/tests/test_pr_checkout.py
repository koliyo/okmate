from pathlib import Path
from types import SimpleNamespace

import pytest

from okmate_ops.pr_checkout import checkout_pr, list_open_prs, local_pr_branch, main, parse_pr_ref


def test_parse_pr_number_and_hash() -> None:
    assert parse_pr_ref("39").number == 39
    assert parse_pr_ref("#39").number == 39
    assert parse_pr_ref("  #39  ").branch is None


def test_parse_github_pr_url() -> None:
    url = "https://github.com/koliyo/okmate/pull/39"
    assert parse_pr_ref(url).number == 39
    assert parse_pr_ref(f"{url}/files").number == 39


def test_parse_branch_path() -> None:
    ref = parse_pr_ref("feat/example")
    assert ref.number is None
    assert ref.branch == "feat/example"
    assert parse_pr_ref("refs/heads/feat/foo").branch == "feat/foo"


def test_local_pr_branch_prefixes_once() -> None:
    assert local_pr_branch("feat/example") == "pr/feat/example"
    assert local_pr_branch("pr/feat/example") == "pr/feat/example"


def test_checkout_switches_prefixed_branch(monkeypatch, tmp_path: Path, capsys) -> None:
    calls: list[tuple[str, ...]] = []

    def git(root: Path, *args: str, check: bool = True):
        calls.append(args)
        if args[:2] == ("status", "--porcelain"):
            return SimpleNamespace(returncode=0, stdout="", stderr="")
        if args[:2] == ("fetch", "origin"):
            return SimpleNamespace(returncode=0, stdout="", stderr="")
        if args[:2] == ("rev-parse", "--verify"):
            return SimpleNamespace(returncode=0, stdout="abc123\n", stderr="")
        if args[:2] == ("switch", "-C"):
            return SimpleNamespace(returncode=0, stdout="", stderr="")
        if args[0] == "branch":
            return SimpleNamespace(returncode=0, stdout="", stderr="")
        if check:
            raise AssertionError(args)
        return SimpleNamespace(returncode=1, stdout="", stderr="")

    monkeypatch.setattr("okmate_ops.pr_checkout._git", git)
    monkeypatch.setattr(
        "okmate_ops.pr_checkout._gh_head_ref",
        lambda _root, number: "feat/example",
    )

    branch = checkout_pr(parse_pr_ref("#39"), root=tmp_path)
    assert branch == "pr/feat/example"
    assert ("fetch", "origin", "pull/39/head") in calls
    assert ("switch", "-C", "pr/feat/example", "abc123") in calls
    assert "pr/feat/example" in capsys.readouterr().out


def test_checkout_refuses_dirty_worktree(monkeypatch, tmp_path: Path) -> None:
    def git(root: Path, *args: str, check: bool = True):
        if args[:2] == ("status", "--porcelain"):
            return SimpleNamespace(returncode=0, stdout=" M README.md\n", stderr="")
        raise AssertionError(args)

    monkeypatch.setattr("okmate_ops.pr_checkout._git", git)
    monkeypatch.setattr("okmate_ops.pr_checkout._gh_head_ref", lambda *_: "feat/example")

    with pytest.raises(SystemExit, match="uncommitted"):
        checkout_pr(parse_pr_ref("39"), root=tmp_path)


def test_list_open_prs_runs_gh(monkeypatch, tmp_path: Path) -> None:
    calls: list[object] = []

    def run(cmd, **kwargs):
        calls.append((cmd, kwargs.get("cwd")))
        return SimpleNamespace(returncode=0)

    monkeypatch.setattr("okmate_ops.pr_checkout.subprocess.run", run)
    assert list_open_prs(root=tmp_path) == 0
    assert calls == [(["gh", "pr", "list", "--state", "open"], tmp_path)]


def test_main_without_ref_lists_prs(monkeypatch) -> None:
    monkeypatch.setattr("okmate_ops.pr_checkout.list_open_prs", lambda: 0)
    monkeypatch.setattr(
        "okmate_ops.pr_checkout.checkout_pr",
        lambda *args, **kwargs: (_ for _ in ()).throw(AssertionError("checkout")),
    )
    assert main([]) == 0


def test_dry_run_skips_git_mutators(monkeypatch, tmp_path: Path, capsys) -> None:
    monkeypatch.setattr(
        "okmate_ops.pr_checkout._git",
        lambda *args, **kwargs: (_ for _ in ()).throw(AssertionError(args)),
    )
    monkeypatch.setattr("okmate_ops.pr_checkout._gh_head_ref", lambda *_: "feat/example")

    branch = checkout_pr(parse_pr_ref("39"), root=tmp_path, dry_run=True)
    assert branch == "pr/feat/example"
    assert "#39" in capsys.readouterr().out
