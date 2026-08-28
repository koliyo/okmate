from okmate_ops.cli import USAGE, main


def test_usage_lists_build_install_and_package() -> None:
    assert "build         cargo release build of okmate" in USAGE
    assert "install       cli" in USAGE
    assert "package       desktop, sign, appcast" in USAGE
    assert "release       vX.Y.Z or dev after Test" in USAGE


def test_main_routes_release(monkeypatch) -> None:
    seen: list[list[str]] = []
    monkeypatch.setattr(
        "okmate_ops.promote.release_command",
        lambda argv: seen.append(list(argv)) or 0,
    )
    try:
        main(["release", "v1.2.3"])
    except SystemExit as exc:
        assert exc.code == 0
    else:
        raise AssertionError("expected SystemExit")
    assert seen == [["v1.2.3"]]
