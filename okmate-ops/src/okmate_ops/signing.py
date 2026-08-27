from __future__ import annotations

import base64
import os
import shutil
import subprocess
import tempfile
from collections.abc import Iterator
from contextlib import contextmanager
from pathlib import Path

P12_SECRET = "APPLE_CERTIFICATE_P12"
P12_PASSWORD_SECRET = "APPLE_CERTIFICATE_PASSWORD"
CI_MISSING_P12 = (
    "okmate-ops package sign: GitHub-hosted runners have no Developer ID "
    "in the keychain. Set APPLE_CERTIFICATE_P12 (base64 PKCS#12) and "
    f"{P12_PASSWORD_SECRET} on the release environment."
)


def _security(*args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["security", *args],
        check=check,
        capture_output=True,
        text=True,
    )


def _user_keychains() -> list[str]:
    result = _security("list-keychains", "-d", "user")
    found: list[str] = []
    for line in result.stdout.splitlines():
        path = line.strip().strip('"')
        if path:
            found.append(path)
    return found


def decode_p12(blob: str) -> bytes:
    try:
        return base64.b64decode(blob, validate=True)
    except Exception as exc:
        raise SystemExit(
            f"okmate-ops package sign: {P12_SECRET} must be standard base64 of a .p12"
        ) from exc


@contextmanager
def imported_developer_id_keychain() -> Iterator[None]:
    blob = os.environ.get(P12_SECRET, "").strip()
    if not blob:
        if os.environ.get("GITHUB_ACTIONS") == "true" and os.environ.get("SIGN_DRY_RUN") != "1":
            raise SystemExit(CI_MISSING_P12)
        yield
        return
    p12 = decode_p12("".join(blob.split()))
    password = os.environ.get(P12_PASSWORD_SECRET, "")
    scratch = Path(tempfile.mkdtemp(prefix="okmate-sign-"))
    p12_path = scratch / "developer-id.p12"
    keychain = scratch / "signing.keychain-db"
    kc_pass = os.urandom(24).hex()
    previous = _user_keychains()
    p12_path.write_bytes(p12)
    try:
        _security("create-keychain", "-p", kc_pass, str(keychain))
        _security("set-keychain-settings", "-lut", "21600", str(keychain))
        _security("unlock-keychain", "-p", kc_pass, str(keychain))
        _security(
            "import",
            str(p12_path),
            "-k",
            str(keychain),
            "-P",
            password,
            "-T",
            "/usr/bin/codesign",
            "-T",
            "/usr/bin/security",
        )
        _security("list-keychains", "-d", "user", "-s", str(keychain), *previous)
        _security(
            "set-key-partition-list",
            "-S",
            "apple-tool:,apple:,codesign:",
            "-s",
            "-k",
            kc_pass,
            str(keychain),
        )
        yield
    finally:
        _security("delete-keychain", str(keychain), check=False)
        if previous:
            _security("list-keychains", "-d", "user", "-s", *previous, check=False)
        shutil.rmtree(scratch, ignore_errors=True)
