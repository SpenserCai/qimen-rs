"""Read the workspace version and synchronize metadata required by package registries."""

import json
from pathlib import Path
import re
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parents[1]
STABLE = re.compile(r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)")


def version_key(value: str) -> tuple[int, ...]:
    if not STABLE.fullmatch(value):
        raise ValueError(f"Expected a stable X.Y.Z version without leading zeros; got {value!r}")
    return tuple(map(int, value.split(".")))


def manifest_paths(root: Path) -> list[Path]:
    paths = [root / "Cargo.toml"]
    workspace = tomllib.loads(paths[0].read_text(encoding="utf-8"))["workspace"]
    for member in workspace["members"]:
        paths.extend(path / "Cargo.toml" for path in sorted(root.glob(member)))
    return list(dict.fromkeys(paths))


def version(root: Path = ROOT) -> str:
    value = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]["package"]["version"]
    version_key(value)
    return value


def local_dependencies(metadata: dict):
    for key, value in metadata.items():
        if not isinstance(value, dict):
            continue
        if key in ("dependencies", "dev-dependencies", "build-dependencies"):
            for name, dependency in value.items():
                if isinstance(dependency, dict) and "path" in dependency:
                    yield name, dependency
        else:
            yield from local_dependencies(value)


def validate(root: Path = ROOT, *, tracked: bool = True) -> None:
    expected = version(root)
    names = set()
    for path in manifest_paths(root):
        metadata = tomllib.loads(path.read_text(encoding="utf-8"))
        if "package" in metadata:
            if metadata["package"].get("version") != {"workspace": True}:
                raise ValueError(f"{path.relative_to(root)} must inherit version.workspace = true")
            names.add(metadata["package"]["name"])
        for name, dependency in local_dependencies(metadata):
            if dependency.get("version") != expected:
                raise ValueError(f"Local dependency {name} in {path.relative_to(root)} must use {expected}")
    node = json.loads((root / "bindings/node/package.json").read_text(encoding="utf-8"))
    lock = json.loads((root / "bindings/node/package-lock.json").read_text(encoding="utf-8"))
    if any(item.get("version") != expected for item in (node, lock, lock["packages"][""])):
        raise ValueError("Node package.json and package-lock.json must mirror the workspace version")
    python = tomllib.loads((root / "bindings/python/pyproject.toml").read_text(encoding="utf-8"))["project"]
    if "version" in python or "version" not in python.get("dynamic", []):
        raise ValueError("Python must derive its dynamic version from the Rust workspace")
    cargo_lock = tomllib.loads((root / "Cargo.lock").read_text(encoding="utf-8"))
    packages = {item["name"]: item for item in cargo_lock["package"] if "source" not in item}
    if any(name not in packages or packages[name]["version"] != expected for name in names):
        raise ValueError("Workspace package versions in Cargo.lock must match Cargo.toml")
    if tracked:
        subprocess.run(["git", "ls-files", "--error-unmatch", "Cargo.lock"], cwd=root,
                       check=True, stdout=subprocess.DEVNULL)


def synchronize(new: str, root: Path = ROOT) -> None:
    """Change only first-party versions; preserve resolved third-party dependencies."""
    if version_key(new) < version_key(version(root)):
        raise ValueError("The workspace version cannot move backwards")
    changes = {}
    names = set()
    old_versions = set()
    for path in manifest_paths(root):
        source = path.read_text(encoding="utf-8")
        metadata = tomllib.loads(source)
        if "package" in metadata:
            if metadata["package"].get("version") != {"workspace": True}:
                raise ValueError(f"{path.relative_to(root)} must inherit version.workspace = true")
            names.add(metadata["package"]["name"])
        if path == root / "Cargo.toml":
            source, count = re.subn(
                r'(\[workspace\.package\][^\[]*?\bversion\s*=\s*")[^"]+(\")',
                lambda match: match[1] + new + match[2], source, count=1,
            )
            if count != 1:
                raise ValueError("Unable to locate workspace.package.version")
        # Cargo's inline path dependencies keep their existing formatting and comments.
        source = re.sub(
            r'\{[^{}]*\bpath\s*=\s*"[^"\n]+"[^{}]*\}',
            lambda match: re.sub(r'(\bversion\s*=\s*")[^"]+(\")',
                                 lambda value: value[1] + new + value[2], match[0]),
            source,
        )
        for name, dependency in local_dependencies(tomllib.loads(source)):
            if dependency.get("version") != new:
                raise ValueError(f"Use an inline table with a version for local dependency {name} in {path}")
        changes[path] = source
    lock_path = root / "Cargo.lock"
    lock_source = lock_path.read_text(encoding="utf-8")

    def update_lock(match):
        source = match[0]
        package = tomllib.loads(source)["package"][0]
        if package["name"] in names and "source" not in package:
            old_versions.add((package["name"], package["version"]))
            source = re.sub(r'(?m)^version = "[^"]+"$', f'version = "{new}"', source, count=1)
        return source

    lock_source = re.sub(r'\[\[package\]\][\s\S]*?(?=\[\[package\]\]|\Z)', update_lock, lock_source)
    for name, old in old_versions:
        lock_source = lock_source.replace(f'"{name} {old}"', f'"{name} {new}"')
    changes[lock_path] = lock_source
    node_path = root / "bindings/node/package.json"
    node = json.loads(node_path.read_text(encoding="utf-8"))
    node["version"] = new
    changes[node_path] = json.dumps(node, ensure_ascii=False, indent=2) + "\n"
    npm_lock_path = root / "bindings/node/package-lock.json"
    npm_lock = json.loads(npm_lock_path.read_text(encoding="utf-8"))
    npm_lock["version"] = new
    npm_lock["packages"][""]["version"] = new
    changes[npm_lock_path] = json.dumps(npm_lock, ensure_ascii=False, indent=2) + "\n"
    original = {path: path.read_bytes() for path in changes}
    try:
        for path, content in changes.items():
            path.write_text(content, encoding="utf-8")
        validate(root)
    except Exception:
        for path, content in original.items():
            path.write_bytes(content)
        raise
    print(f"Workspace and all language package versions synchronized: {new}")
