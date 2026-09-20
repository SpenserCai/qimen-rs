"""Version changes must preserve third-party resolutions and registry compatibility."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import tomllib
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import versions


def repository(root: Path) -> None:
    files = {
        "Cargo.toml": '[workspace]\nmembers = ["crates/*", "bindings/*"]\n'
                      '[workspace.package]\nversion = "0.1.0"\n'
                      '[workspace.dependencies]\nqimen-core = { path = "crates/core", version = "0.1.0" }\n',
        "Cargo.lock": 'version = 4\n\n[[package]]\nname = "external"\nversion = "0.1.0"\n'
                      'source = "registry+https://example.invalid/index"\n',
        "bindings/python/pyproject.toml": '[project]\nname = "qimen-rs"\ndynamic = ["version"]\n',
        "bindings/node/package.json": json.dumps({"name": "@spensercai/qimen-rs", "version": "0.1.0"}),
        "bindings/node/package-lock.json": json.dumps({
            "version": "0.1.0", "packages": {"": {"version": "0.1.0"},
            "node_modules/external": {"version": "0.1.0", "integrity": "sha512-example"}}}),
    }
    for directory, name in (("crates/core", "qimen-core"), ("bindings/node", "qimen-node"),
                            ("bindings/python", "qimen-python"), ("bindings/wasm", "qimen-wasm")):
        files[f"{directory}/Cargo.toml"] = f'[package]\nname = "{name}"\nversion.workspace = true\n'
        files["Cargo.lock"] += f'\n[[package]]\nname = "{name}"\nversion = "0.1.0"\n'
    files["bindings/node/Cargo.toml"] += '[dependencies]\nqimen-core = { path = "../../crates/core", version = "0.1.0" }\n'
    for filename, source in files.items():
        path = root / filename
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(source, encoding="utf-8")
    subprocess.run(["git", "init", "-q", "-b", "main"], cwd=root, check=True)
    subprocess.run(["git", "add", "Cargo.lock"], cwd=root, check=True)


class VersionsTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        repository(self.root)

    def test_synchronizes_all_first_party_metadata_without_resolving_dependencies(self):
        node_before = json.loads((self.root / "bindings/node/package-lock.json").read_text())["packages"]["node_modules/external"]
        versions.synchronize("0.2.0", self.root)
        versions.validate(self.root)
        self.assertEqual(versions.version(self.root), "0.2.0")
        rust = tomllib.loads((self.root / "Cargo.lock").read_text())["package"]
        self.assertEqual([item["version"] for item in rust if "source" not in item], ["0.2.0"] * 4)
        self.assertEqual(rust[0]["version"], "0.1.0")
        node = json.loads((self.root / "bindings/node/package-lock.json").read_text())
        self.assertEqual(node["packages"]["node_modules/external"], node_before)

    def test_invalid_or_backward_versions_do_not_modify_files(self):
        before = (self.root / "Cargo.toml").read_bytes()
        for value in ("0.0.9", "01.2.0", "0.2.0-rc.1", "v0.2.0", "0.2.0\nother=value"):
            with self.subTest(version=value), self.assertRaises(ValueError):
                versions.synchronize(value, self.root)
            self.assertEqual((self.root / "Cargo.toml").read_bytes(), before)

    def test_failed_validation_restores_every_changed_file(self):
        python = self.root / "bindings/python/pyproject.toml"
        python.write_text('[project]\nname = "qimen-rs"\nversion = "0.1.0"\n')
        paths = versions.manifest_paths(self.root) + [self.root / "Cargo.lock", self.root / "bindings/node/package.json", self.root / "bindings/node/package-lock.json"]
        before = {path: path.read_bytes() for path in paths}
        with self.assertRaisesRegex(ValueError, "Python"):
            versions.synchronize("0.2.0", self.root)
        self.assertEqual({path: path.read_bytes() for path in paths}, before)

    def test_ci_rejects_independent_binding_version(self):
        path = self.root / "bindings/wasm/Cargo.toml"
        path.write_text(path.read_text().replace("version.workspace = true", 'version = "0.1.0"'))
        with self.assertRaisesRegex(ValueError, "inherit"):
            versions.validate(self.root)

    def test_ci_checks_both_lockfiles_and_local_requirements(self):
        for filename, old, new in (
            ("Cargo.lock", 'name = "qimen-core"\nversion = "0.1.0"', 'name = "qimen-core"\nversion = "0.2.0"'),
            ("bindings/node/package-lock.json", '"version": "0.1.0"', '"version": "0.2.0"'),
            ("bindings/node/Cargo.toml", 'version = "0.1.0"', 'version = "0.2.0"'),
        ):
            with self.subTest(file=filename):
                path = self.root / filename
                original = path.read_text()
                path.write_text(original.replace(old, new))
                with self.assertRaises(ValueError):
                    versions.validate(self.root)
                path.write_text(original)

    def test_reapplying_version_is_idempotent(self):
        versions.synchronize("0.2.0", self.root)
        first = (self.root / "Cargo.lock").read_bytes()
        versions.synchronize("0.2.0", self.root)
        self.assertEqual((self.root / "Cargo.lock").read_bytes(), first)


if __name__ == "__main__":
    unittest.main()
