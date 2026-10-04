#!/usr/bin/env python3
"""Check the upstream seam ledger against a Git base and working tree."""

import argparse
import os
from pathlib import Path
import re
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]


def git_names(repo, *args):
    result = subprocess.run(
        ["git", "-C", str(repo), *args], check=True, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return {os.fsdecode(name) for name in result.stdout.split(b"\0") if name}


def ledger_rows(path):
    rows = set()
    for number, line in enumerate(path.read_text().splitlines(), 1):
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.split("|")[1:-1]]
        if cells == ["Upstream file", "Reason", "Verification"]:
            continue
        if cells and all(re.fullmatch(r":?-+:?", cell) for cell in cells):
            continue
        if len(cells) != 3 or not all(cells):
            raise ValueError(f"Invalid seam table row at line {number}")
        name = cells[0].removeprefix("`").removesuffix("`")
        if name in rows:
            raise ValueError(f"Duplicate seam row: {name}")
        rows.add(name)
    return rows


def check(repo, base, ledger, revision=None):
    upstream = git_names(repo, "ls-tree", "-r", "-z", "--name-only", base)
    args = ["diff", "--no-renames", "--name-only", "-z", base]
    if revision:
        args.append(revision)
    changed = git_names(repo, *args, "--") & upstream
    rows = ledger_rows(ledger)
    return sorted(changed - rows), sorted(rows - changed), len(changed)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=ROOT)
    parser.add_argument("--base", default="upstream/main")
    parser.add_argument("--revision", help="compare a commit instead of working tree")
    parser.add_argument("--ledger", type=Path)
    args = parser.parse_args(argv)
    ledger = args.ledger or args.repo / "docs/fork/SEAMS.md"
    try:
        missing, stale, count = check(
            args.repo, args.base, ledger, args.revision,
        )
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(f"Seam check failed: {error}", file=sys.stderr)
        return 1
    for name in missing:
        print(f"Missing seam row: {name}", file=sys.stderr)
    for name in stale:
        print(f"Stale seam row: {name}", file=sys.stderr)
    if missing or stale:
        return 1
    print(f"Seam ledger matches all {count} changed upstream files.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
