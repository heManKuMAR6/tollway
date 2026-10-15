"""``tollway`` command-line interface."""

import argparse
from collections.abc import Sequence

from tollway import __version__


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="tollway", description="tollway agent gateway")
    parser.add_argument("--version", action="version", version=f"tollway {__version__}")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    parser.parse_args(argv)
    parser.print_help()
    return 0
