#!/usr/bin/env python3
"""Validate release metadata and create reproducible-layout application archives."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[1]


def version() -> str:
    with (ROOT / "Cargo.toml").open("rb") as source:
        return tomllib.load(source)["workspace"]["package"]["version"]


def validate(tag: str) -> None:
    expected = version()
    if not re.fullmatch(r"v\d+\.\d+\.\d+", tag) or tag != f"v{expected}":
        raise SystemExit(f"Release tag must equal stable workspace version v{expected}; got {tag!r}")
    node = json.loads((ROOT / "bindings/node/package.json").read_text(encoding="utf-8"))
    if node["version"] != expected:
        raise SystemExit("Node package version must equal the workspace version")
    manifests = [ROOT / "Cargo.toml"]
    for directory in ("crates", "apps", "bindings"):
        manifests.extend(sorted((ROOT / directory).glob("*/Cargo.toml")))
    for manifest in manifests:
        with manifest.open("rb") as source:
            metadata = tomllib.load(source)
        declared = metadata.get("package", {}).get("version", {"workspace": True})
        if declared != {"workspace": True} and declared != expected:
            raise SystemExit(f"Package version mismatch in {manifest.relative_to(ROOT)}")
        dependencies = metadata.get("dependencies", {})
        dependencies = {**dependencies, **metadata.get("workspace", {}).get("dependencies", {})}
        for name, dependency in dependencies.items():
            if isinstance(dependency, dict) and "path" in dependency:
                if dependency.get("version") != expected:
                    raise SystemExit(f"Local dependency {name} in {manifest.relative_to(ROOT)} must use version {expected}")
    if not (ROOT / "Cargo.lock").is_file():
        raise SystemExit("Cargo.lock must be committed before a release")
    tracked = subprocess.run(["git", "ls-files", "--error-unmatch", "Cargo.lock"],
                             cwd=ROOT, capture_output=True, check=False)
    if tracked.returncode:
        raise SystemExit("Cargo.lock must be tracked before a release")
    print(f"Release metadata valid: {tag}")


def archive(target: str) -> None:
    distribution = ROOT / "dist"
    distribution.mkdir(exist_ok=True)
    base = f"qimen-rs-v{version()}-{target}"
    windows = "windows" in target
    with tempfile.TemporaryDirectory(prefix="qimen-release-") as temporary:
        staging = Path(temporary) / base
        staging.mkdir()
        for binary in ("qimen", "qimen-mcp"):
            filename = binary + (".exe" if windows else "")
            shutil.copy2(ROOT / "target" / target / "release" / filename, staging / filename)
        for filename in ("LICENSE", "README.md", "README.en.md"):
            shutil.copy2(ROOT / filename, staging / filename)
        if windows:
            output = distribution / f"{base}.zip"
            with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED) as package:
                for entry in sorted(staging.iterdir()):
                    package.write(entry, f"{base}/{entry.name}")
        else:
            output = distribution / f"{base}.tar.gz"
            with tarfile.open(output, "w:gz") as package:
                package.add(staging, arcname=base)
    checksum = hashlib.sha256(output.read_bytes()).hexdigest()
    output.with_name(output.name + ".sha256").write_text(
        f"{checksum}  {output.name}\n", encoding="utf-8"
    )
    print(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("validate").add_argument("tag")
    commands.add_parser("archive").add_argument("target")
    args = parser.parse_args()
    if args.command == "validate":
        validate(args.tag)
    else:
        archive(args.target)


if __name__ == "__main__":
    main()
