from __future__ import annotations

import sys

from okmate_ops import ci, local, pr_checkout, release

USAGE = """\
usage: okmate-ops <command> [args...]

commands:
  build         cargo release build of okmate
  ci            run GitHub Actions validation jobs on this machine
  install       cli
  package       desktop, sign, appcast, publish-feed
  pr-checkout   list open PRs, or checkout one here as pr/<branch>
  release       patch, minor, major, v*, or dev
"""


def main(argv: list[str] | None = None) -> None:
    args = sys.argv[1:] if argv is None else argv
    if not args or args[0] in ("-h", "--help"):
        sys.stdout.write(USAGE)
        if not args:
            raise SystemExit(2)
        raise SystemExit(0)
    command, rest = args[0], args[1:]
    if command == "build":
        raise SystemExit(local.build_command(rest))
    if command == "ci":
        raise SystemExit(ci.main(rest))
    if command == "install":
        raise SystemExit(local.install_command(rest))
    if command == "package":
        raise SystemExit(local.package_command(rest))
    if command == "pr-checkout":
        raise SystemExit(pr_checkout.main(rest))
    if command == "release":
        raise SystemExit(release.release_command(rest))
    sys.stderr.write(f"unknown command: {command}\n")
    sys.stderr.write(USAGE)
    raise SystemExit(2)
