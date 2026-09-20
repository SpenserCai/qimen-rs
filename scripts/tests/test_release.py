"""Release guards run without network access or a package registry token."""

from pathlib import Path
import os
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import release
from test_versions import repository

SHA = "a" * 40
OTHER = "b" * 40


class ReleaseDecisionTests(unittest.TestCase):
    def test_main_new_version_is_selected(self):
        self.assertTrue(release.decision("0.2.0", SHA, "refs/heads/main", "push", {"v0.1.0": OTHER})[0])

    def test_push_and_manual_same_version_skip(self):
        for event in ("push", "workflow_dispatch"):
            selected, reason = release.decision("0.2.0", SHA, "refs/heads/main", event, {"v0.2.0": OTHER})
            self.assertFalse(selected)
            self.assertIn("failed jobs", reason)

    def test_tag_compatibility_does_not_repeat_completed_release(self):
        self.assertTrue(release.decision("0.2.0", SHA, "refs/tags/v0.2.0", "push", {"v0.2.0": SHA})[0])
        self.assertFalse(release.decision("0.2.0", SHA, "refs/tags/v0.2.0", "push", {"v0.2.0": SHA}, True)[0])

    def test_rejects_other_refs_wrong_tag_sha_and_rollback(self):
        cases = [
            ("0.2.0", "refs/heads/feature", {}),
            ("0.2.0", "refs/tags/v0.3.0", {}),
            ("0.2.0", "refs/tags/v0.2.0", {"v0.2.0": OTHER}),
            ("0.2.0", "refs/heads/main", {"v0.10.0": OTHER}),
        ]
        for current, ref, tags in cases:
            with self.subTest(ref=ref, tags=tags), self.assertRaises(ValueError):
                release.decision(current, SHA, ref, "push", tags)


class GitTagTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        base = Path(self.temporary.name)
        self.root = base / "checkout"
        self.root.mkdir()
        repository(self.root)
        self.git("config", "user.name", "Release Test")
        self.git("config", "user.email", "test@example.invalid")
        self.git("add", ".")
        self.git("commit", "-qm", "Initial version")
        self.sha = self.git("rev-parse", "HEAD")
        subprocess.run(["git", "init", "--bare", "-q", str(base / "remote.git")], check=True)
        self.git("remote", "add", "origin", str(base / "remote.git"))
        self.git("push", "-q", "-u", "origin", "main")
        self.addCleanup(patch.stopall)
        patch.object(release, "ROOT", self.root).start()
        patch.dict(os.environ, {"GITHUB_SHA": self.sha}).start()

    def git(self, *args):
        return subprocess.check_output(["git", *args], cwd=self.root, text=True).strip()

    def test_creates_exact_tag_and_safely_reuses_it_on_job_retry(self):
        release.create_tag("v0.1.0")
        release.create_tag("v0.1.0")
        self.assertEqual(self.git("rev-parse", "v0.1.0"), self.sha)
        self.assertIn(self.sha, self.git("ls-remote", "origin", "refs/tags/v0.1.0"))

    def test_never_moves_an_existing_tag(self):
        self.git("tag", "v0.1.0")
        self.git("push", "-q", "origin", "v0.1.0")
        self.git("commit", "--allow-empty", "-qm", "Another commit")
        self.git("push", "-q", "origin", "main")
        with patch.dict(os.environ, {"GITHUB_SHA": self.git("rev-parse", "HEAD")}):
            with self.assertRaisesRegex(ValueError, "immutable"):
                release.create_tag("v0.1.0")
        self.assertEqual(self.git("rev-parse", "v0.1.0"), self.sha)

    def test_never_releases_a_commit_outside_main_history(self):
        self.git("checkout", "-qb", "feature")
        self.git("commit", "--allow-empty", "-qm", "Unmerged")
        with patch.dict(os.environ, {"GITHUB_SHA": self.git("rev-parse", "HEAD")}):
            with self.assertRaises(subprocess.CalledProcessError):
                release.create_tag("v0.1.0")
        self.assertEqual(self.git("ls-remote", "origin", "refs/tags/v0.1.0"), "")


if __name__ == "__main__":
    unittest.main()
