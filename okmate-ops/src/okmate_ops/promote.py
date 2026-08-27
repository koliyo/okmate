from __future__ import annotations

import subprocess
import tempfile
import time
from pathlib import Path

from okmate_ops.ghutil import DEFAULT_CHECKS, gh_run, wait_for_check
from okmate_ops.paths import repo_root
from okmate_ops.version import apply_release_version, parse_release_version, release_files_match

PROMOTE_USAGE = "usage: okmate-ops promote tag"
PROMOTE_TAG_USAGE = "usage: okmate-ops promote tag <tag> [--from BRANCH] [--force]"


def run(argv: list[str], *, cwd: Path | None = None) -> None:
    subprocess.run(argv, cwd=cwd or repo_root(), check=True)


def git_capture(argv: list[str], *, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        argv,
        cwd=cwd or repo_root(),
        capture_output=True,
        text=True,
    )


def github_repo() -> str:
    result = subprocess.run(
        ["gh", "repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"],
        cwd=repo_root(),
        capture_output=True,
        text=True,
        check=True,
    )
    return result.stdout.strip()


def wait_for_promote_ci(sha: str) -> None:
    repo = github_repo()

    def gh(args: list[str]) -> str:
        return gh_run(args).stdout

    for check in DEFAULT_CHECKS:
        wait_for_check(repo=repo, sha=sha, check=check, gh=gh, sleep=time.sleep)


def push_version_update(version: str, from_ref: str, remote_sha: str) -> str:
    root = repo_root()
    with tempfile.TemporaryDirectory() as tmp:
        worktree = Path(tmp) / "wt"
        run(["git", "worktree", "add", "--detach", str(worktree), remote_sha], cwd=root)
        try:
            if release_files_match(worktree, version):
                return remote_sha
            paths = apply_release_version(worktree, version)
            run(["git", "add", *[str(path) for path in paths]], cwd=worktree)
            status = git_capture(["git", "status", "--porcelain"], cwd=worktree)
            if status.returncode != 0:
                raise SystemExit("promote tag could not read worktree status")
            if not status.stdout.strip():
                return remote_sha
            run(["git", "commit", "-m", f"chore(release): set version {version}"], cwd=worktree)
            run(["git", "push", "origin", f"HEAD:{from_ref}"], cwd=worktree)
            pushed = git_capture(["git", "rev-parse", "HEAD"], cwd=worktree)
            if pushed.returncode != 0:
                raise SystemExit("promote tag could not read version commit")
            return pushed.stdout.strip()
        finally:
            run(["git", "worktree", "remove", "--force", str(worktree)], cwd=root)


def promote_tag(tag: str, from_ref: str = "main", *, force: bool = False) -> int:
    movable = tag == "dev"
    if not movable:
        parse_release_version(tag)
    elif len(tag) < 2:
        raise SystemExit("promote tag requires a v* name or the movable dev tag")
    run(
        [
            "git",
            "fetch",
            "origin",
            f"refs/heads/{from_ref}:refs/remotes/origin/{from_ref}",
        ]
    )
    remote_ref = f"origin/{from_ref}"
    verify = git_capture(["git", "rev-parse", "--verify", remote_ref])
    if verify.returncode != 0:
        raise SystemExit(f"promote tag requires {remote_ref}")
    sha = verify.stdout.strip()
    if not movable:
        sha = push_version_update(parse_release_version(tag), from_ref, sha)
    wait_for_promote_ci(sha)
    tag_argv = ["git", "tag", "-a", tag, "-m", tag, sha]
    push_argv = ["git", "push", "origin", tag]
    if movable or force:
        tag_argv = ["git", "tag", "-a", "-f", tag, "-m", tag, sha]
        push_argv = ["git", "push", "--force", "origin", tag]
    run(tag_argv)
    run(push_argv)
    return 0


def promote_tag_command(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        raise SystemExit(PROMOTE_TAG_USAGE)
    from_ref = "main"
    tag: str | None = None
    force = False
    i = 0
    while i < len(argv):
        if argv[i] == "--from":
            if i + 1 >= len(argv):
                raise SystemExit(PROMOTE_TAG_USAGE)
            from_ref = argv[i + 1]
            i += 2
            continue
        if argv[i] == "--force":
            force = True
            i += 1
            continue
        if tag is not None:
            raise SystemExit(PROMOTE_TAG_USAGE)
        tag = argv[i]
        i += 1
    if tag is None:
        raise SystemExit(PROMOTE_TAG_USAGE)
    return promote_tag(tag, from_ref=from_ref, force=force)


def promote_command(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        raise SystemExit(PROMOTE_USAGE)
    if argv[0] == "tag":
        return promote_tag_command(argv[1:])
    raise SystemExit(PROMOTE_USAGE)
