#!/usr/bin/env python3
"""Synchronize package versions, plan guarded releases and archive applications."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
from urllib.error import HTTPError
from urllib.request import Request, urlopen
import zipfile

from versions import synchronize, validate as validate_versions, version, version_key

ROOT = Path(__file__).resolve().parents[1]


def validate(tag: str | None = None) -> None:
    expected = f"v{version(ROOT)}"
    if tag is not None and tag != expected:
        raise ValueError(f"Release tag must equal workspace version {expected}; got {tag!r}")
    validate_versions(ROOT)
    print(f"Release metadata valid: {expected}")


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def release_exists(tag: str) -> bool:
    repository = os.environ["GITHUB_REPOSITORY"]
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository):
        raise ValueError("Invalid GitHub repository")
    request = Request(f"https://api.github.com/repos/{repository}/releases/tags/{tag}", headers={
        "Accept": "application/vnd.github+json",
        "Authorization": f"Bearer {os.environ['GH_TOKEN']}",
        "X-GitHub-Api-Version": "2022-11-28",
    })
    try:
        with urlopen(request, timeout=30) as response:
            return bool(json.load(response))
    except HTTPError as error:
        if error.code == 404:
            return False
        raise


def stable_tags() -> dict[str, str]:
    result = {}
    for tag in git("tag", "--list", "v*").splitlines():
        try:
            version_key(tag[1:])
        except ValueError:
            continue
        result[tag] = git("rev-parse", f"refs/tags/{tag}^{{commit}}")
    return result


def decision(current: str, sha: str, ref: str, event: str, tags: dict[str, str],
             published: bool = False) -> tuple[bool, str]:
    """Decide without writes, including duplicate, downgrade and tag-conflict cases."""
    expected = f"v{current}"
    if ref != "refs/heads/main" and ref != f"refs/tags/{expected}":
        raise ValueError(f"Release only from main or the workspace tag {expected}; got {ref!r}")
    if event not in ("push", "workflow_dispatch"):
        raise ValueError(f"Unsupported release event: {event}")
    if tags and version_key(current) < max(version_key(tag[1:]) for tag in tags):
        raise ValueError("Workspace version is older than an existing release tag; refusing a rollback")
    if ref.startswith("refs/tags/"):
        if tags.get(expected) != sha:
            raise ValueError(f"{expected} does not point at this exact commit; tags are immutable")
        if published:
            return False, f"{expected} already has a release; re-run failed jobs in its original workflow run"
        return True, f"Release existing tag {expected}"
    if expected in tags:
        return False, f"{expected} already exists; same-version commits do not publish. Re-run failed jobs in the original release run"
    return True, f"Validate and publish new workspace version {expected}"


def check_history(sha: str) -> None:
    if not re.fullmatch(r"[0-9a-f]{40}", sha) or git("rev-parse", "HEAD") != sha:
        raise ValueError("Checkout must match the exact workflow commit")
    subprocess.run(["git", "merge-base", "--is-ancestor", sha, "origin/main"], cwd=ROOT, check=True)


def plan() -> None:
    validate()
    sha = os.environ["GITHUB_SHA"]
    check_history(sha)
    current = version(ROOT)
    tag = f"v{current}"
    tags = stable_tags()
    ref = os.environ["GITHUB_REF"]
    if tag in tags and ref == "refs/heads/main":
        # A normal same-version descendant is safe to skip; an unrelated tag is not.
        subprocess.run(["git", "merge-base", "--is-ancestor", tags[tag], sha], cwd=ROOT, check=True)
    published = release_exists(tag) if ref.startswith("refs/tags/") and tags.get(tag) == sha else False
    publish, reason = decision(current, sha, ref, os.environ["GITHUB_EVENT_NAME"], tags, published)
    print(reason)
    with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as output:
        output.write(f"publish={str(publish).lower()}\nversion={current}\ntag={tag}\n")
    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with Path(summary).open("a", encoding="utf-8") as output:
            output.write(reason + "\n")


def create_tag(tag: str) -> None:
    """Create a tag only after verified artifacts exist; never move an existing tag."""
    validate(tag)
    sha = os.environ["GITHUB_SHA"]
    subprocess.run(["git", "fetch", "origin", "main", "--tags"], cwd=ROOT, check=True)
    check_history(sha)
    tags = stable_tags()
    if tags and version_key(tag[1:]) < max(version_key(item[1:]) for item in tags):
        raise ValueError("A newer version was tagged while this release waited; refusing a rollback")
    if tag in tags:
        if tags[tag] != sha:
            raise ValueError(f"{tag} already points to a different commit; tags are immutable")
        print(f"Using existing immutable tag {tag} at {sha}")
        return
    # No --force: a concurrent creation fails safely instead of replacing its commit.
    subprocess.run(["git", "push", "origin", f"{sha}:refs/tags/{tag}"], cwd=ROOT, check=True)
    print(f"Created {tag} at verified commit {sha}")


def wasm_metadata(directory: Path) -> None:
    path = directory / "package.json"
    package = json.loads(path.read_text(encoding="utf-8"))
    if package["version"] != version(ROOT):
        raise ValueError("Generated WASM package version differs from the workspace")
    package["repository"] = {
        "type": "git", "url": "git+https://github.com/SpenserCai/qimen-rs.git",
        "directory": "bindings/wasm",
    }
    path.write_text(json.dumps(package, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def archive(target: str) -> None:
    distribution = ROOT / "dist"
    distribution.mkdir(exist_ok=True)
    base = f"qimen-rs-v{version(ROOT)}-{target}"
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
    commands.add_parser("validate").add_argument("tag", nargs="?")
    commands.add_parser("set-version").add_argument("version")
    commands.add_parser("plan")
    commands.add_parser("tag").add_argument("tag")
    commands.add_parser("wasm-metadata").add_argument("directory", type=Path)
    commands.add_parser("archive").add_argument("target")
    args = parser.parse_args()
    try:
        if args.command == "validate":
            validate(args.tag)
        elif args.command == "set-version":
            synchronize(args.version, ROOT)
        elif args.command == "plan":
            plan()
        elif args.command == "tag":
            create_tag(args.tag)
        elif args.command == "wasm-metadata":
            wasm_metadata(args.directory)
        else:
            archive(args.target)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error)) from error


if __name__ == "__main__":
    main()
