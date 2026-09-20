#!/usr/bin/env python3
"""Publish packed npm artifacts, safely resuming a partially published native set."""

import argparse
import json
from pathlib import Path
import subprocess
import tempfile

from versions import version, version_key

REGISTRY = "https://registry.npmjs.org"


def package_set(directory: Path, native: bool) -> list[Path]:
    manifest_path = directory / "package.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest["version"] != version():
        raise ValueError("npm package version must match the workspace")
    if not native:
        return [directory]
    packages = sorted((directory / "npm").glob("*/package.json"))
    if len(packages) != len(manifest["napi"]["targets"]):
        raise ValueError("Native package set is incomplete")
    dependencies = {}
    for path in packages:
        platform = json.loads(path.read_text(encoding="utf-8"))
        if platform["version"] != manifest["version"] or not platform["name"].startswith(manifest["name"] + "-"):
            raise ValueError(f"Unexpected native package identity in {path}")
        dependencies[platform["name"]] = manifest["version"]
    if len(dependencies) != len(packages):
        raise ValueError("Duplicate native package names")
    manifest["optionalDependencies"] = dependencies
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return [path.parent for path in packages] + [directory]


def npm_view(spec: str, field: str) -> str | None:
    result = subprocess.run(
        ["npm", "view", spec, field, "--json", "--registry", REGISTRY],
        capture_output=True, text=True, check=False,
    )
    if result.returncode:
        try:
            missing = json.loads(result.stdout).get("error", {}).get("code") == "E404"
        except (ValueError, AttributeError):
            missing = False
        if missing:
            return None
        raise RuntimeError(f"Unable to verify {spec} in npm; refusing to assume it is unpublished")
    value = json.loads(result.stdout)
    if not isinstance(value, str) or not value:
        raise ValueError(f"npm returned invalid {field} for {spec}")
    return value


def already_published(spec: str, integrity: str) -> bool:
    existing = npm_view(spec, "dist.integrity")
    if existing is None:
        return False
    if existing != integrity:
        raise ValueError(f"{spec} already exists with different contents; bump the workspace version")
    return True


def protect_latest(name: str, expected: str) -> None:
    latest = npm_view(name, "dist-tags.latest")
    if latest is not None and version_key(latest) > version_key(expected):
        raise ValueError(f"{name} latest is already {latest}; refusing to roll it back to {expected}")


def publish(directory: Path) -> None:
    with tempfile.TemporaryDirectory(prefix="qimen-npm-") as temporary:
        packed = json.loads(subprocess.check_output(
            ["npm", "pack", "--json", "--ignore-scripts", "--pack-destination", temporary],
            cwd=directory, text=True,
        ))[0]
        spec = f"{packed['name']}@{packed['version']}"
        if already_published(spec, packed["integrity"]):
            print(f"Verified identical npm package; skipping {spec}", flush=True)
            return
        protect_latest(packed["name"], packed["version"])
        subprocess.run(
            ["npm", "publish", str(Path(temporary) / packed["filename"]), "--access", "public",
             "--provenance", "--ignore-scripts", "--registry", REGISTRY],
            cwd=directory, check=True,
        )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("--native", action="store_true")
    args = parser.parse_args()
    for directory in package_set(args.directory.resolve(), args.native):
        publish(directory)


if __name__ == "__main__":
    main()
