#!/usr/bin/env python3
"""Run the portable Rust quality gate and retain diagnostics for failed CI runs."""

import argparse
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def run(name: str, command: list[str], directory: Path) -> bool:
    print(f"\n::group::{name}", flush=True)
    print("+ " + " ".join(command), flush=True)
    with (directory / f"{name}.log").open("w", encoding="utf-8") as log:
        process = subprocess.Popen(
            command, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
            text=True, encoding="utf-8", errors="replace",
        )
        assert process.stdout is not None
        for line in process.stdout:
            print(line, end="", flush=True)
            log.write(line)
        success = process.wait() == 0
    print("::endgroup::", flush=True)
    return success


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--diagnostics", type=Path, default=ROOT / "quality-diagnostics")
    args = parser.parse_args()
    args.diagnostics.mkdir(parents=True, exist_ok=True)
    os.environ["RUSTFLAGS"] = (os.environ.get("RUSTFLAGS", "") + " -D warnings").strip()
    os.environ["RUSTDOCFLAGS"] = (os.environ.get("RUSTDOCFLAGS", "") + " -D warnings").strip()
    os.environ["CARGO_TERM_COLOR"] = "never"
    results = []
    results.append(run("versions", ["rustc", "--version", "--verbose"], args.diagnostics))
    results.append(run("format", ["cargo", "fmt", "--all", "--", "--check"], args.diagnostics))
    for name, command in [
        ("check", ["cargo", "check", "--workspace", "--all-targets", "--locked"]),
        ("clippy", ["cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"]),
        ("test", ["cargo", "test", "--workspace", "--locked", "--exclude", "qimen-python", "--exclude", "qimen-node", "--exclude", "qimen-wasm"]),
        ("docs", ["cargo", "doc", "--workspace", "--no-deps", "--locked"]),
        ("schemas", [sys.executable, str(ROOT / "scripts" / "check-schemas.py")]),
    ]:
        results.append(run(name, command, args.diagnostics))
    # A failed format gate stays failed; the patch merely makes remote repair practical.
    if not results[1] and os.environ.get("CI") == "true":
        if run("format-repair", ["cargo", "fmt", "--all"], args.diagnostics):
            diff = subprocess.run(["git", "diff", "--", "*.rs"], cwd=ROOT,
                                  capture_output=True, check=True)
            (args.diagnostics / "rustfmt.patch").write_bytes(diff.stdout)
    return 0 if all(results) else 1


if __name__ == "__main__":
    sys.exit(main())
