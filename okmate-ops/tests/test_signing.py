import base64

from okmate_ops.signing import (
    CI_MISSING_P12,
    decode_p12,
    imported_developer_id_keychain,
)


def test_decode_p12_rejects_garbage() -> None:
    try:
        decode_p12("not base64!!!")
    except SystemExit as exc:
        assert "APPLE_CERTIFICATE_P12" in str(exc)
    else:
        raise AssertionError("expected SystemExit")


def test_hosted_ci_requires_p12(monkeypatch) -> None:
    monkeypatch.setenv("GITHUB_ACTIONS", "true")
    monkeypatch.delenv("APPLE_CERTIFICATE_P12", raising=False)
    monkeypatch.delenv("SIGN_DRY_RUN", raising=False)
    try:
        with imported_developer_id_keychain():
            pass
    except SystemExit as exc:
        assert str(exc) == CI_MISSING_P12
    else:
        raise AssertionError("expected SystemExit")


def test_hosted_dry_run_skips_p12(monkeypatch) -> None:
    monkeypatch.setenv("GITHUB_ACTIONS", "true")
    monkeypatch.setenv("SIGN_DRY_RUN", "1")
    monkeypatch.delenv("APPLE_CERTIFICATE_P12", raising=False)
    with imported_developer_id_keychain():
        pass


def test_import_p12_into_ephemeral_keychain(monkeypatch) -> None:
    calls: list[list[str]] = []
    payload = base64.b64encode(b"pkcs12-bytes").decode("ascii")
    monkeypatch.setenv("APPLE_CERTIFICATE_P12", payload)
    monkeypatch.setenv("APPLE_CERTIFICATE_PASSWORD", "p12-pass")
    monkeypatch.delenv("GITHUB_ACTIONS", raising=False)

    class Result:
        stdout = '    "/Users/me/login.keychain-db"\n'

    def fake_security(*args: str, check: bool = True):
        calls.append(list(args))
        return Result()

    monkeypatch.setattr("okmate_ops.signing._security", fake_security)
    with imported_developer_id_keychain():
        pass
    verbs = [call[0] for call in calls]
    assert verbs[0] == "list-keychains"
    assert "create-keychain" in verbs
    assert "import" in verbs
    assert "set-key-partition-list" in verbs
    assert verbs[-2] == "delete-keychain"
    import_args = next(call for call in calls if call[0] == "import")
    assert "-P" in import_args
    assert import_args[import_args.index("-P") + 1] == "p12-pass"
    restore = next(
        call
        for call in calls
        if call[:4] == ["list-keychains", "-d", "user", "-s"] and call[-1].endswith("login.keychain-db")
    )
    assert restore[-1] == "/Users/me/login.keychain-db"
