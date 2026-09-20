"""Partial registry jobs resume only when the existing source/artifacts agree."""

import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch
from urllib.error import HTTPError

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
spec = importlib.util.spec_from_file_location("publish_registries", Path(__file__).resolve().parents[1] / "publish-registries.py")
registry = importlib.util.module_from_spec(spec)
spec.loader.exec_module(registry)
SHA = "a" * 40


def crate_archive(*, sha=SHA, dirty=False, path="crates/qimen-calendar", repository=registry.REPOSITORY,
                  name="qimen-calendar", version="0.1.0"):
    data = {
        ".cargo_vcs_info.json": json.dumps({"git": {"sha1": sha, "dirty": dirty}, "path_in_vcs": path}).encode(),
        "Cargo.toml": f'[package]\nname = "{name}"\nversion = "{version}"\nrepository = "{repository}"\n'.encode(),
    }
    archive = io.BytesIO()
    with tarfile.open(fileobj=archive, mode="w:gz") as output:
        for name, value in data.items():
            info = tarfile.TarInfo(f"qimen-calendar-0.1.0/{name}")
            info.size = len(value)
            output.addfile(info, io.BytesIO(value))
    return archive.getvalue()


class CrateResumeTests(unittest.TestCase):
    def test_only_same_clean_source_and_package_identity_can_be_skipped(self):
        original = crate_archive()
        registry.verify_crate("qimen-calendar", "0.1.0", SHA, original, hashlib.sha256(original).hexdigest())
        for change in ({"sha": "b" * 40}, {"dirty": True}, {"path": "other"},
                       {"repository": "https://example.invalid"}, {"name": "other"}, {"version": "0.2.0"}):
            data = crate_archive(**change)
            with self.subTest(change=change), self.assertRaisesRegex(ValueError, "provenance"):
                registry.verify_crate("qimen-calendar", "0.1.0", SHA, data, hashlib.sha256(data).hexdigest())
        with self.assertRaisesRegex(ValueError, "checksum"):
            registry.verify_crate("qimen-calendar", "0.1.0", SHA, original, "0" * 64)

    def test_partial_crate_job_skips_completed_package_and_publishes_remaining_in_order(self):
        data = crate_archive()
        registered = json.dumps({"version": {"checksum": hashlib.sha256(data).hexdigest()}}).encode()
        with patch.object(registry, "version", return_value="0.1.0"), patch.dict(os.environ, {"GITHUB_SHA": SHA}), \
                patch.object(registry.subprocess, "check_output", return_value=SHA), \
                patch.object(registry, "fetch", side_effect=[registered, data, None, None, None]), \
                patch.object(registry.subprocess, "run") as publish:
            registry.crates()
            self.assertEqual([call.args[0][-1] for call in publish.call_args_list], ["qimen-core", "qimen-cli", "qimen-mcp"])

    def test_network_and_permission_errors_do_not_mean_missing(self):
        for status in (404, 401, 429, 500):
            error = HTTPError("https://example.invalid", status, "error", None, None)
            with self.subTest(status=status), patch.object(registry, "urlopen", side_effect=error):
                if status == 404:
                    self.assertIsNone(registry.fetch("https://example.invalid", missing_ok=True))
                else:
                    with self.assertRaises(HTTPError):
                        registry.fetch("https://example.invalid", missing_ok=True)
                with self.assertRaises(HTTPError):
                    registry.fetch("https://example.invalid")


class PythonResumeTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.wheel = self.root / "qimen_rs-0.1.0-cp310-abi3-manylinux_2_28_x86_64.whl"
        self.source = self.root / "qimen_rs-0.1.0.tar.gz"
        self.wheel.write_bytes(b"original wheel")
        self.source.write_bytes(b"original source")
        patcher = patch.object(registry, "version", return_value="0.1.0")
        patcher.start()
        self.addCleanup(patcher.stop)

    def response(self, files):
        return json.dumps({"urls": [{"filename": path.name, "digests": {"sha256": digest}}
                                     for path, digest in files]}).encode()

    def test_partial_publish_keeps_only_missing_original_artifacts(self):
        data = self.response([(self.wheel, hashlib.sha256(self.wheel.read_bytes()).hexdigest())])
        with patch.object(registry, "fetch", return_value=data):
            self.assertTrue(registry.prepare_pypi(self.root))
        self.assertEqual(list(self.root.iterdir()), [self.source])
        self.assertEqual(self.source.read_bytes(), b"original source")

    def test_completed_publish_has_nothing_to_upload(self):
        data = self.response([(path, hashlib.sha256(path.read_bytes()).hexdigest()) for path in (self.wheel, self.source)])
        with patch.object(registry, "fetch", return_value=data):
            self.assertFalse(registry.prepare_pypi(self.root))
        self.assertEqual(list(self.root.iterdir()), [])

    def test_content_mismatch_leaves_original_staging_files_intact(self):
        data = self.response([(self.source, hashlib.sha256(self.source.read_bytes()).hexdigest()), (self.wheel, "wrong")])
        with patch.object(registry, "fetch", return_value=data), self.assertRaisesRegex(ValueError, "differs"):
            registry.prepare_pypi(self.root)
        self.assertTrue(self.wheel.exists())
        self.assertTrue(self.source.exists())

    def test_wrong_version_prefix_does_not_publish(self):
        self.wheel.rename(self.root / self.wheel.name.replace("0.1.0-", "0.1.01-"))
        with self.assertRaisesRegex(ValueError, "workspace version"):
            registry.prepare_pypi(self.root)


if __name__ == "__main__":
    unittest.main()
