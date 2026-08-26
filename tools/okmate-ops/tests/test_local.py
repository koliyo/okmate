from okmate_ops.local import (
    APP_NAME,
    BUILD_USAGE,
    BUNDLE_IDENTIFIER,
    CLI_BINARY,
    CLI_CRATE,
    DEFAULT_SU_FEED_URL,
    INSTALL_USAGE,
    PACKAGE_APPCAST_USAGE,
    PACKAGE_SIGN_USAGE,
    PACKAGE_USAGE,
    app_bundle_dir,
    build_command,
    install_cli,
    install_command,
    package_appcast,
    package_command,
    package_desktop,
    package_sign,
    release_binary,
)


def test_cli_crate() -> None:
    assert CLI_CRATE == "okmate"
    assert CLI_BINARY == "okmate"


def test_build_usage() -> None:
    try:
        build_command(["--release"])
    except SystemExit as exc:
        assert str(exc) == BUILD_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_install_usage() -> None:
    try:
        install_command([])
    except SystemExit as exc:
        assert str(exc) == INSTALL_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_install_rejects_unknown_target() -> None:
    try:
        install_command(["vscode"])
    except SystemExit as exc:
        assert str(exc) == INSTALL_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_build_runs_cargo_release(monkeypatch, tmp_path) -> None:
    calls: list[list[str]] = []
    monkeypatch.delenv("CARGO_TARGET_DIR", raising=False)
    binary = tmp_path / "target" / "release" / "okmate"
    binary.parent.mkdir(parents=True)
    binary.write_bytes(b"okmate")
    monkeypatch.setattr("okmate_ops.local.repo_root", lambda: tmp_path)
    monkeypatch.setattr(
        "okmate_ops.local.run",
        lambda argv, cwd=None, env=None: calls.append(list(argv)),
    )
    assert build_command([]) == 0
    assert calls == [["cargo", "build", "--release", "-p", "okmate"]]


def test_install_cli_copies_release_binary(monkeypatch, tmp_path) -> None:
    monkeypatch.delenv("CARGO_TARGET_DIR", raising=False)
    dest = tmp_path / "bin"
    dest.mkdir()
    root = tmp_path / "repo"
    binary = release_binary(root, "okmate")
    calls: list[list[str]] = []

    def fake_run(argv, cwd=None, env=None) -> None:
        calls.append(list(argv))
        binary.parent.mkdir(parents=True)
        binary.write_bytes(b"okmate")

    monkeypatch.setattr("okmate_ops.local.repo_root", lambda: root)
    monkeypatch.setattr("okmate_ops.local.run", fake_run)

    assert install_cli(dest=dest) == 0
    assert calls == [["cargo", "build", "--release", "-p", "okmate"]]
    installed = dest / "okmate"
    assert installed.is_file()
    assert installed.read_bytes() == b"okmate"
    assert installed.stat().st_mode & 0o111


def test_package_usage() -> None:
    try:
        package_command([])
    except SystemExit as exc:
        assert str(exc) == PACKAGE_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_package_rejects_unknown_target() -> None:
    try:
        package_command(["cli"])
    except SystemExit as exc:
        assert str(exc) == PACKAGE_USAGE
    else:
        raise AssertionError("expected SystemExit")


def _h35_macos(tmp_path, *names: str):
    root = tmp_path / "okmate"
    h35 = tmp_path / "h35-desktop"
    macos = h35 / "packaging" / "macos"
    macos.mkdir(parents=True)
    scripts = {}
    for name in names:
        script = macos / name
        script.write_text("#!/bin/sh\n", encoding="utf-8")
        scripts[name] = script
    root.mkdir()
    return root, h35, scripts


def test_package_desktop_runs_h35_packager(monkeypatch, tmp_path) -> None:
    root, h35, scripts = _h35_macos(tmp_path, "package.sh")
    calls: list[list[str]] = []
    envs: list[dict[str, str]] = []

    def fake_run(argv, cwd=None, env=None, capture=False) -> str:
        calls.append(list(argv))
        envs.append(env or {})
        return ""

    monkeypatch.setenv("H35_DESKTOP", str(h35))
    monkeypatch.setattr("okmate_ops.local.repo_root", lambda: root)
    monkeypatch.setattr("okmate_ops.local.require_darwin", lambda kind: None)
    monkeypatch.setattr("okmate_ops.local.run", fake_run)

    assert package_desktop() == 0
    assert calls == [[str(scripts["package.sh"])]]
    assert envs[0]["APP_NAME"] == APP_NAME
    assert envs[0]["BUNDLE_ID"] == BUNDLE_IDENTIFIER
    assert envs[0]["EXECUTABLE"] == CLI_BINARY
    assert envs[0]["CRATE"] == CLI_CRATE
    assert envs[0]["PRODUCT_ROOT"] == str(root)
    assert envs[0]["SU_FEED_URL"] == DEFAULT_SU_FEED_URL
    assert app_bundle_dir(root) == root / "dist" / f"{APP_NAME}.app"


def test_package_desktop_requires_h35_packager(monkeypatch, tmp_path) -> None:
    monkeypatch.setenv("H35_DESKTOP", str(tmp_path / "missing"))
    monkeypatch.setattr("okmate_ops.local.repo_root", lambda: tmp_path)
    monkeypatch.setattr("okmate_ops.local.require_darwin", lambda kind: None)
    try:
        package_desktop()
    except SystemExit as exc:
        assert "h35-desktop packager" in str(exc)
    else:
        raise AssertionError("expected SystemExit")


def test_package_sign_runs_h35_signer(monkeypatch, tmp_path) -> None:
    root, h35, scripts = _h35_macos(tmp_path, "sign.sh")
    calls: list[list[str]] = []

    def fake_run(argv, cwd=None, env=None, capture=False) -> str:
        calls.append(list(argv))
        return ""

    monkeypatch.setenv("H35_DESKTOP", str(h35))
    monkeypatch.setattr("okmate_ops.local.repo_root", lambda: root)
    monkeypatch.setattr("okmate_ops.local.require_darwin", lambda kind: None)
    monkeypatch.setattr("okmate_ops.local.run", fake_run)

    assert package_sign([]) == 0
    assert calls == [[str(scripts["sign.sh"]), str(root / "dist" / f"{APP_NAME}.app")]]


def test_package_sign_usage() -> None:
    try:
        package_sign(["a.app", "b.app"])
    except SystemExit as exc:
        assert str(exc) == PACKAGE_SIGN_USAGE
    else:
        raise AssertionError("expected SystemExit")


def test_package_appcast_fetches_sparkle_then_generates(monkeypatch, tmp_path) -> None:
    root, h35, scripts = _h35_macos(tmp_path, "fetch-sparkle.sh", "generate-appcast.sh")
    framework = tmp_path / "Sparkle.framework"
    calls: list[list[str]] = []
    envs: list[dict[str, str]] = []

    def fake_run(argv, cwd=None, env=None, capture=False) -> str:
        calls.append(list(argv))
        envs.append(env or {})
        if capture:
            return f"{framework}\n"
        return ""

    monkeypatch.setenv("H35_DESKTOP", str(h35))
    monkeypatch.setattr("okmate_ops.local.repo_root", lambda: root)
    monkeypatch.setattr("okmate_ops.local.require_darwin", lambda kind: None)
    monkeypatch.setattr("okmate_ops.local.run", fake_run)

    inbox = str(tmp_path / "inbox")
    prefix = "https://example.test/download/"
    assert package_appcast([inbox, prefix]) == 0
    assert calls[0] == [str(scripts["fetch-sparkle.sh"])]
    assert calls[1] == [str(scripts["generate-appcast.sh"]), inbox, prefix]
    assert envs[1]["GENERATE_APPCAST"] == str(framework.parent / "bin" / "generate_appcast")


def test_package_appcast_usage() -> None:
    try:
        package_appcast(["only-inbox"])
    except SystemExit as exc:
        assert str(exc) == PACKAGE_APPCAST_USAGE
    else:
        raise AssertionError("expected SystemExit")
