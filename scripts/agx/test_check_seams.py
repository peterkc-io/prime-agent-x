#!/usr/bin/env python3
"""Standard-library tests for exact seam set comparison."""

from pathlib import Path
import subprocess
import tempfile
import unittest

import check_seams


class SeamChecks(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.repo = Path(self.temp.name)
        self.git("init", "--quiet")
        (self.repo / "one.rs").write_text("upstream one\n")
        (self.repo / "two.rs").write_text("upstream two\n")
        self.git("add", "one.rs", "two.rs")
        # A Git tree has the same diff interface without fixture identities.
        self.base = self.git("write-tree").strip()
        self.ledger = self.repo / "SEAMS.md"
        self.write_rows([])

    def git(self, *args):
        return subprocess.check_output(
            ["git", "-C", str(self.repo), *args], text=True,
        )

    def write_rows(self, names):
        self.ledger.write_text(
            "| Upstream file | Reason | Verification |\n"
            "| --- | --- | --- |\n" + "".join(
                f"| `{name}` | required fork change | focused check |\n"
                for name in names
            )
        )

    def test_exact_changed_set_passes(self):
        (self.repo / "one.rs").write_text("fork one\n")
        self.write_rows(["one.rs"])
        self.assertEqual(check_seams.check(
            self.repo, self.base, self.ledger,
        ), ([], [], 1))

    def test_missing_row_names_changed_file(self):
        (self.repo / "two.rs").write_text("fork two\n")
        self.assertEqual(check_seams.check(
            self.repo, self.base, self.ledger,
        ), (["two.rs"], [], 1))

    def test_unchanged_row_is_stale(self):
        self.write_rows(["one.rs"])
        self.assertEqual(check_seams.check(
            self.repo, self.base, self.ledger,
        ), ([], ["one.rs"], 0))

    def test_deletion_requires_a_row(self):
        (self.repo / "one.rs").unlink()
        self.assertEqual(check_seams.check(
            self.repo, self.base, self.ledger,
        ), (["one.rs"], [], 1))

    def test_fork_added_file_does_not_require_an_upstream_row(self):
        (self.repo / "fork.rs").write_text("fork-owned\n")
        self.git("add", "fork.rs")
        self.assertEqual(check_seams.check(
            self.repo, self.base, self.ledger,
        ), ([], [], 0))

    def test_duplicate_row_is_rejected(self):
        self.write_rows(["one.rs", "one.rs"])
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            check_seams.ledger_rows(self.ledger)


if __name__ == "__main__":
    unittest.main()
