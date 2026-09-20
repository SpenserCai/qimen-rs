"""A retry may skip only the identical package, never a merely matching version."""

import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
spec = importlib.util.spec_from_file_location("publish_npm", Path(__file__).resolve().parents[1] / "publish-npm.py")
publish_npm = importlib.util.module_from_spec(spec)
spec.loader.exec_module(publish_npm)


class PublishedPackageTests(unittest.TestCase):
    def test_retrying_an_older_release_cannot_downgrade_latest(self):
        with patch.object(publish_npm, "npm_view", return_value="0.10.0"):
            with self.assertRaisesRegex(ValueError, "roll it back"):
                publish_npm.protect_latest("example", "0.2.0")
        for latest in (None, "0.1.0", "0.2.0"):
            with patch.object(publish_npm, "npm_view", return_value=latest):
                publish_npm.protect_latest("example", "0.2.0")

    def test_skips_only_identical_package_integrity(self):
        with patch.object(publish_npm.subprocess, "run", return_value=subprocess.CompletedProcess([], 0, '"sha512-same"')):
            self.assertTrue(publish_npm.already_published("example@0.1.0", "sha512-same"))
            with self.assertRaisesRegex(ValueError, "different contents"):
                publish_npm.already_published("example@0.1.0", "sha512-different")

    def test_only_explicit_not_found_allows_publish(self):
        for code in ("E404", "E401", "E429", "ECONNRESET"):
            result = subprocess.CompletedProcess([], 1, json.dumps({"error": {"code": code}}))
            with self.subTest(code=code), patch.object(publish_npm.subprocess, "run", return_value=result):
                if code == "E404":
                    self.assertFalse(publish_npm.already_published("example@0.1.0", "sha512-content"))
                else:
                    with self.assertRaises(RuntimeError):
                        publish_npm.already_published("example@0.1.0", "sha512-content")


if __name__ == "__main__":
    unittest.main()
