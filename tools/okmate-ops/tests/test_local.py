from okmate_ops.local import (
    APP_NAME,
    BUILD_USAGE,
    CLI_BINARY,
    CLI_CRATE,
    INSTALL_USAGE,
    PACKAGE_USAGE,
    app_bundle_dir,
    build_command,
    install_cli,
    install_command,
    package_command,
    package_desktop,
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


def test_package_desktop_assembles_app(monkeypatch, tmp_path) -> None:
    monkeypatch.delenv("CARGO_TARGET_DIR", raising=False)
    root = tmp_path / "repo"
    root.mkdir()
    (root / "Cargo.toml").write_text(
        '[package]\nname = "okmate"\nversion = "0.1.0"\n',
        encoding="utf-8",
    )
    binary = release_binary(root, "okmate")
    calls: list[list[str]] = []

    def fake_run(argv, cwd=None, env=None) -> None:
        calls.append(list(argv))
        if argv[:3] == ["cargo", "build", "--release"]:
            binary.parent.mkdir(parents=True)
            binary.write_bytes(b"okmate")

    monkeypatch.setattr("okmate_ops.local.repo_root", lambda: root)
    monkeypatch.setattr("okmate_ops.local.require_darwin", lambda kind: None)
    monkeypatch.setattr("okmate_ops.local.run", fake_run)

    assert package_desktop() == 0
    bundle = app_bundle_dir(root)
    assert bundle.name == f"{APP_NAME}.app"
    exe = bundle / "Contents" / "MacOS" / "okmate"
    assert exe.is_file()
    assert exe.read_bytes() == b"okmate"
    plist = (bundle / "Contents" / "Info.plist").read_text(encoding="utf-8")
    assert "dev.okmate.app" in plist
    assert "0.1.0" in plist
    assert (bundle / "Contents" / "PkgInfo").read_bytes() == b"APPL????"
    assert calls[0] == ["cargo", "build", "--release", "-p", "okmate"]
    assert calls[1][:4] == ["codesign", "--force", "--deep", "--sign"]
