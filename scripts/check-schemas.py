#!/usr/bin/env python3
"""Check committed public JSON schemas against the Rust API's generated schemas."""

import json
from pathlib import Path
import shlex
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def canonical(value: object) -> str:
    """Ignore whitespace and object key order while retaining JSON value types."""
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def main() -> int:
    valid = True
    for kind in ("request", "chart"):
        command = [
            "cargo", "run", "--quiet", "--locked", "-p", "qimen-core",
            "--example", "schema", "--features", "schema", "--", kind,
        ]
        relative = Path("docs") / "schema" / f"{kind}.schema.json"
        try:
            generated = subprocess.run(
                command, cwd=ROOT, capture_output=True, check=True,
                text=True, encoding="utf-8",
            )
            actual = json.loads(generated.stdout)
            expected = json.loads((ROOT / relative).read_text(encoding="utf-8"))
        except subprocess.CalledProcessError as error:
            print(f"Failed to generate {kind} schema:\n{error.stderr}", file=sys.stderr)
            valid = False
            continue
        except (OSError, json.JSONDecodeError) as error:
            print(f"Cannot compare {relative.as_posix()}: {error}", file=sys.stderr)
            valid = False
        else:
            if canonical(actual) == canonical(expected):
                print(f"Verified {relative.as_posix()}")
                continue
            print(f"Public schema drift: {relative.as_posix()}", file=sys.stderr)
            valid = False
        print(
            "After reviewing the API change, regenerate and commit with:\n"
            f"  {shlex.join(command)} > {relative.as_posix()}",
            file=sys.stderr,
        )
    return 0 if valid else 1


if __name__ == "__main__":
    sys.exit(main())
