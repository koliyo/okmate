from io import StringIO

from okmate_ops.ghutil import format_elapsed, render_bar, wait_for_check


def test_render_bar_and_elapsed() -> None:
    assert format_elapsed(0) == "00:00"
    assert format_elapsed(75) == "01:15"
    assert render_bar(0, 100, width=10) == "[----------]"
    assert render_bar(50, 100, width=10) == "[#####-----]"
    assert render_bar(100, 100, width=10) == "[##########]"


def test_wait_for_check_rewrites_one_line() -> None:
    replies = ["", "in_progress pending", "completed success"]
    out = StringIO()
    wait_for_check(
        repo="koliyo/okmate",
        sha="abc",
        check="Test",
        gh=lambda args: replies.pop(0),
        sleep=lambda seconds: None,
        out=out,
    )
    written = out.getvalue()
    assert written.count("\n") == 1
    assert written.endswith("\n")
    assert written.count("\r") == 3
    assert "Status:" not in written
    assert "waiting" in written
    assert "in_progress pending" in written
    assert "passed" in written


def test_wait_for_check_failed_keeps_single_line() -> None:
    out = StringIO()
    try:
        wait_for_check(
            repo="koliyo/okmate",
            sha="abc",
            check="Test",
            gh=lambda args: "completed cancelled",
            sleep=lambda seconds: None,
            out=out,
        )
    except SystemExit as exc:
        assert "cancelled" in str(exc)
    else:
        raise AssertionError("expected SystemExit")
    written = out.getvalue()
    assert written.count("\n") == 1
    assert "failed (cancelled)" in written
    assert "Status:" not in written
