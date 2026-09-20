#!/usr/bin/env python3
"""Resume registry jobs using original artifacts or exact Rust source provenance."""

import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile
import tomllib
from urllib.error import HTTPError
from urllib.request import Request, urlopen

from versions import ROOT, version

REPOSITORY = "https://github.com/SpenserCai/qimen-rs"
CRATES = {"qimen-calendar": "crates/qimen-calendar", "qimen-core": "crates/qimen-core",
          "qimen-cli": "apps/qimen-cli", "qimen-mcp": "apps/qimen-mcp"}


def fetch(url: str, *, missing_ok: bool = False) -> bytes | None:
    try:
        request = Request(url, headers={"User-Agent": "qimen-rs-release (+https://github.com/SpenserCai/qimen-rs)"})
        with urlopen(request, timeout=60) as response:
            return response.read()
    except HTTPError as error:
        if missing_ok and error.code == 404:
            return None
        raise


def verify_crate(package: str, expected: str, sha: str, archive: bytes, checksum: str) -> None:
    if hashlib.sha256(archive).hexdigest() != checksum:
        raise ValueError(f"Registry checksum mismatch for {package}")
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:gz") as contents:
        def read(name: str) -> bytes:
            entry = contents.getmember(f"{package}-{expected}/{name}")
            if not entry.isfile():
                raise ValueError(f"Expected a regular {name} in {package}")
            with contents.extractfile(entry) as source:
                return source.read()
        provenance = json.loads(read(".cargo_vcs_info.json"))
        metadata = tomllib.loads(read("Cargo.toml").decode("utf-8"))["package"]
    if (provenance.get("git", {}).get("sha1") != sha
            or provenance["git"].get("dirty", False) is not False
            or provenance.get("path_in_vcs") != CRATES[package]
            or metadata.get("name") != package or metadata.get("version") != expected
            or metadata.get("repository") != REPOSITORY):
        raise ValueError(f"Published {package}@{expected} has different source provenance; refusing to skip it")


def crates() -> None:
    expected = version()
    sha = os.environ["GITHUB_SHA"]
    if not re.fullmatch(r"[0-9a-f]{40}", sha) or subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True,
    ).strip() != sha:
        raise ValueError("Cargo publication requires the exact workflow commit")
    for package in CRATES:
        existing = fetch(f"https://crates.io/api/v1/crates/{package}/{expected}", missing_ok=True)
        if existing is None:
            subprocess.run(["cargo", "publish", "--locked", "-p", package], cwd=ROOT, check=True)
        else:
            published = json.loads(existing)["version"]
            if published.get("yanked", False):
                raise ValueError(f"{package}@{expected} was yanked; choose a new workspace version")
            checksum = published["checksum"]
            archive = fetch(f"https://static.crates.io/crates/{package}/{package}-{expected}.crate")
            verify_crate(package, expected, sha, archive, checksum)
            print(f"Verified same Rust source commit; skipping {package}@{expected}", flush=True)


def prepare_pypi(directory: Path) -> bool:
    expected = version()
    paths = sorted(path for path in directory.iterdir() if path.is_file())
    if not paths or any(not (path.name == f"qimen_rs-{expected}.tar.gz"
                            or (path.name.startswith(f"qimen_rs-{expected}-") and path.name.endswith(".whl")))
                        for path in paths):
        raise ValueError("Expected original wheel/source artifacts for the workspace version")
    existing = fetch(f"https://pypi.org/pypi/qimen-rs/{expected}/json", missing_ok=True)
    registered = {} if existing is None else {item["filename"]: item["digests"]["sha256"] for item in json.loads(existing)["urls"]}
    complete = []
    for path in paths:
        if path.name in registered:
            if hashlib.sha256(path.read_bytes()).hexdigest() != registered[path.name]:
                raise ValueError(f"Published PyPI artifact differs from the original: {path.name}")
            complete.append(path)
    # Validate all matches before removing any original artifacts from this staging copy.
    for path in complete:
        path.unlink()
        print(f"Verified identical PyPI artifact; skipping {path.name}", flush=True)
    return len(complete) != len(paths)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("crates")
    commands.add_parser("pypi").add_argument("directory", type=Path)
    args = parser.parse_args()
    if args.command == "crates":
        crates()
    else:
        pending = prepare_pypi(args.directory)
        with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
            output.write(f"pending={str(pending).lower()}\n")


if __name__ == "__main__":
    main()
