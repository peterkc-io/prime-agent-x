#!/usr/bin/env python3
"""Rerun each failed `cargo test` test alone, for the fork's known-flake rule.

Reads a `cargo test` log, finds each failed test and the test binary that ran
it (the `Running ... (target/...)` line before its result), then runs that
binary with `<test> --exact` N times. Prints one line per test and writes a
Markdown table to $GITHUB_STEP_SUMMARY when it is set.

Rule (docs/fork/SYNC.md): at least 15 of 20 passes alone is a known flake;
more failures block. This script reports counts; it decides nothing.
Doc-test failures are listed but not rerun.
"""
import os
import re
import subprocess
import sys

RUNNING = re.compile(r"^\s*Running (?:unittests )?(?P<source>\S+) \((?P<binary>[^)]+)\)\s*$")
DOCTESTS = re.compile(r"^\s*Doc-tests ")
FAILED = re.compile(r"^test (?P<name>\S+) \.\.\. FAILED$")
ANSI = re.compile(r"\x1b\[[0-9;]*m")


def package_dir(source, binary):
    """The package folder cargo ran the binary from, else the workspace root.

    A unit-test binary is named after its crate (pa_daemon-<hash>); an
    integration-test binary is named after its tests/<file>.rs.
    """
    stem = os.path.basename(binary).rsplit("-", 1)[0]
    if source.startswith("tests/"):
        matches = [
            os.path.join("crates", crate)
            for crate in sorted(os.listdir("crates"))
            if os.path.isfile(os.path.join("crates", crate, source))
        ]
        return matches[0] if len(matches) == 1 else "."
    crate_dir = os.path.join("crates", stem.replace("_", "-"))
    return crate_dir if os.path.isdir(crate_dir) else "."


def failed_tests(log_text):
    binary = source = None
    found = []
    for raw in log_text.splitlines():
        line = ANSI.sub("", raw)
        if DOCTESTS.match(line):
            binary = source = None
            continue
        match = RUNNING.match(line)
        if match:
            binary, source = match.group("binary"), match.group("source")
            continue
        match = FAILED.match(line)
        if match and (binary, match.group("name")) not in [f[:2] for f in found]:
            found.append((binary, match.group("name"), source))
    return found


def main():
    log_path, runs = sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 20
    with open(log_path, encoding="utf-8", errors="replace") as handle:
        failures = failed_tests(handle.read())
    rows = []
    for binary, name, source in failures:
        if binary is None:
            rows.append((name, "-", "doc test, not rerun"))
            continue
        passed = 0
        for _ in range(runs):
            result = subprocess.run(
                [os.path.abspath(binary), name, "--exact"],
                cwd=package_dir(source, binary),
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
                check=False,
            )
            passed += result.returncode == 0
        rows.append((name, os.path.basename(binary), f"{passed}/{runs}"))
    for name, binary, result in rows:
        print(f"{result}\t{binary}\t{name}")
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a", encoding="utf-8") as handle:
            handle.write("### Failed tests rerun alone\n\n| Passes | Binary | Test |\n| --- | --- | --- |\n")
            for name, binary, result in rows:
                handle.write(f"| {result} | `{binary}` | `{name}` |\n")
    if not rows:
        print("no failed test found in the log")


if __name__ == "__main__":
    main()
