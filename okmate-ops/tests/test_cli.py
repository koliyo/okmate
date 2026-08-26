from okmate_ops.cli import USAGE


def test_usage_lists_build_install_and_package() -> None:
    assert "build         cargo release build of okmate" in USAGE
    assert "install       cli" in USAGE
    assert "package       desktop, sign, appcast" in USAGE
