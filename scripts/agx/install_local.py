#!/usr/bin/env python3
"""Build and install a local agx bundle without touching prime-agent."""

import argparse
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def install(skip_build=False, catalog_assets=None):
    home = Path.home()
    share = home / ".local" / "share"
    bins = home / ".local" / "bin"
    destination = share / "agx"
    for path in (home, home / ".local", share, bins, destination):
        if path.is_symlink():
            raise ValueError(f"Refusing a symlinked install path: {path}")
    if destination.exists() and not destination.is_dir():
        raise ValueError(f"Not an install directory: {destination}")
    if not skip_build:
        subprocess.run(
            ["cargo", "+1.98.1", "build", "--locked", "--release",
             "--workspace"],
            cwd=ROOT, check=True,
        )
    binary = ROOT / "target" / "release" / "agx"
    if not binary.is_file():
        raise FileNotFoundError(f"Build agx before --skip-build: {binary}")
    share.mkdir(parents=True, exist_ok=True)
    bins.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".agx-", dir=share) as temporary:
        temporary = Path(temporary)
        assets = catalog_assets
        if assets is None:
            assets = temporary / "catalog"
            subprocess.run(
                [sys.executable,
                 str(ROOT / "scripts/release/bundle_catalog.py"),
                 "--network", "--out", str(assets)],
                cwd=ROOT, check=True,
            )
        output = temporary / "package"
        subprocess.run(
            [sys.executable, str(ROOT / "scripts/package_release.py"),
             "--skip-build", "--binary", str(binary), "--binary-name", "agx",
             "--catalog-assets", str(assets), "--out-dir", str(output)],
            cwd=ROOT, check=True,
        )
        packages = [path for path in output.iterdir() if path.is_dir()]
        if len(packages) != 1:
            raise ValueError("The native packer must produce one bundle")
        backup = temporary / "previous"
        had_previous = destination.exists()
        if had_previous:
            destination.rename(backup)
        try:
            packages[0].rename(destination)
            with tempfile.NamedTemporaryFile(
                mode="w", prefix=".agx-", dir=bins, delete=False,
            ) as launcher:
                launcher_path = Path(launcher.name)
                launcher.write("#!/bin/sh\nexec " + shlex.quote(
                    str(destination / "agx")
                ) + ' "$@"\n')
            try:
                launcher_path.chmod(0o755)
                launcher_path.replace(bins / "agx")
            finally:
                launcher_path.unlink(missing_ok=True)
        except (OSError, ValueError):
            if destination.exists():
                shutil.rmtree(destination)
            if had_previous:
                backup.rename(destination)
            raise
    print(f"Installed {bins / 'agx'}")
    print("Settings and sessions stay shared with prime-agent.")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--catalog-assets", type=Path)
    args = parser.parse_args(argv)
    install(args.skip_build, args.catalog_assets)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
