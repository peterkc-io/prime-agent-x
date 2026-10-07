#!/usr/bin/env python3
"""Installer isolation, symlink refusal, and atomic rollback."""

from pathlib import Path
import tempfile
import unittest
from unittest import mock

import install_local


class InstallerChecks(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.home = root / "home"
        self.home.mkdir()
        self.repo = root / "repo"
        binary = self.repo / "target/release/agx"
        binary.parent.mkdir(parents=True)
        binary.write_text("source binary")
        self.catalog = root / "catalog"
        self.catalog.mkdir()
        for patch in [mock.patch.object(install_local, "ROOT", self.repo),
                      mock.patch.object(Path, "home", return_value=self.home),
                      mock.patch.object(install_local.subprocess, "run",
                                        side_effect=self.package)]:
            patch.start()
            self.addCleanup(patch.stop)

    def package(self, args, **kwargs):
        self.assertIn("--binary-name", args)
        self.assertEqual(args[args.index("--binary-name") + 1], "agx")
        output = Path(args[args.index("--out-dir") + 1])
        stage = output / "agx-fixture"
        stage.mkdir(parents=True)
        (stage / "agx").write_text("new binary")

    def install(self):
        install_local.install(skip_build=True, catalog_assets=self.catalog)

    def test_only_fork_bundle_and_launcher_change(self):
        share = self.home / ".local/share"
        bins = self.home / ".local/bin"
        upstream = share / "prime-agent"
        upstream.mkdir(parents=True)
        bins.mkdir(parents=True)
        (upstream / "sentinel").write_bytes(b"upstream bundle")
        (bins / "prime-agent").write_bytes(b"upstream launcher")
        self.install()
        self.assertEqual((upstream / "sentinel").read_bytes(), b"upstream bundle")
        self.assertEqual((bins / "prime-agent").read_bytes(), b"upstream launcher")
        self.assertEqual((share / "agx/agx").read_text(), "new binary")
        launcher = bins / "agx"
        self.assertIn(str(share / "agx/agx"), launcher.read_text())
        self.assertTrue(launcher.stat().st_mode & 0o111)

    def test_symlinked_ancestors_and_destination_are_rejected(self):
        outside = self.home.parent / "outside"
        outside.mkdir()
        (outside / "sentinel").write_bytes(b"unchanged")
        for relative in [".local", ".local/share", ".local/bin",
                         ".local/share/agx"]:
            with self.subTest(path=relative):
                path = self.home / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.symlink_to(outside, target_is_directory=True)
                try:
                    with self.assertRaisesRegex(ValueError, "symlinked"):
                        self.install()
                finally:
                    path.unlink()
        self.assertEqual((outside / "sentinel").read_bytes(), b"unchanged")
        self.assertEqual(list(outside.iterdir()), [outside / "sentinel"])

    def test_home_symlink_is_rejected(self):
        alias = self.home.parent / "alias"
        alias.symlink_to(self.home, target_is_directory=True)
        with mock.patch.object(Path, "home", return_value=alias):
            with self.assertRaisesRegex(ValueError, "symlinked"):
                self.install()
        self.assertFalse((self.home / ".local").exists())

    def test_failed_launcher_replacement_restores_previous_bundle(self):
        destination = self.home / ".local/share/agx"
        destination.mkdir(parents=True)
        (destination / "agx").write_text("previous binary")
        launcher = self.home / ".local/bin/agx"
        launcher.parent.mkdir(parents=True)
        launcher.write_text("previous launcher")
        with mock.patch.object(Path, "replace", side_effect=OSError("fixture")):
            with self.assertRaisesRegex(OSError, "fixture"):
                self.install()
        self.assertEqual((destination / "agx").read_text(), "previous binary")
        self.assertEqual(launcher.read_text(), "previous launcher")
        self.assertEqual(list(destination.parent.iterdir()), [destination])
        self.assertEqual(list(launcher.parent.iterdir()), [launcher])


if __name__ == "__main__":
    unittest.main()
