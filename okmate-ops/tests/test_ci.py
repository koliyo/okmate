from pathlib import Path

from okmate_ops.ci import JOB_NAMES, parse_ci_args, steps_for
from okmate_ops.paths import repo_root


def test_list_jobs_are_stable() -> None:
    assert JOB_NAMES == ("test", "knowledge")


def test_parse_list_flag() -> None:
    args = parse_ci_args(["--list"])
    assert args.list is True
    assert args.jobs == []


def test_test_job_matches_hosted_ci() -> None:
    argv_lists = [s.argv for s in steps_for("test", repo_root())]
    assert any(argv[:3] == ("cargo", "fmt", "--all") for argv in argv_lists)
    assert any(argv == ("cargo", "test", "-p", "okf") for argv in argv_lists)
    assert any(
        argv == ("cargo", "test", "-p", "okmate", "--no-default-features") for argv in argv_lists
    )


def test_knowledge_skips_without_bundle(tmp_path: Path) -> None:
    assert steps_for("knowledge", tmp_path) == []


def test_knowledge_checks_when_index_exists(tmp_path: Path) -> None:
    knowledge = tmp_path / "knowledge"
    knowledge.mkdir()
    (knowledge / "index.md").write_text("---\nokf_version: \"0.2\"\n---\n", encoding="utf-8")
    steps = steps_for("knowledge", tmp_path)
    assert steps
    assert "check" in steps[0].argv
    assert "knowledge" in steps[0].argv
