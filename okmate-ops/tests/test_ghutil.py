from io import StringIO

from okmate_ops.ghutil import wait_for_check


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
    assert "waiting" in written
    assert "in_progress pending" in written
    assert "passed" in written
    assert "Status:" not in written


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
    assert "failed (cancelled)" in written
    assert "Status:" not in written
