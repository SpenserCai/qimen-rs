#!/usr/bin/env python3
"""Optional calendar differential check; no production dependency or CI gate.

Run from the repository root in a separate virtual environment:
    python -m venv .venv-calendar
    .venv-calendar/bin/python -m pip install lunar_python==1.4.8
    cargo build -p qimen-cli --locked
    .venv-calendar/bin/python scripts/compare-calendar.py --output calendar-check.json
On Windows, use .venv-calendar/Scripts/python.exe instead.

The two libraries share 6tail/Shou Xing ancestry. Agreement is cross-implementation
regression evidence, not independent proof of astronomical accuracy.
Exit codes: 0 = all match, 1 = comparison failures, 2 = setup/report errors.
"""

import argparse
import json
import os
import subprocess
from datetime import datetime
from importlib.metadata import PackageNotFoundError, version
from pathlib import Path

REFERENCE_VERSION = "1.4.8"
YEARS = (1900, 1901, 1912, 1930, 1949, 1960, 1978, 1999, 2000, 2024, 2026, 2050, 2100)
TIME_FIELDS = ("year", "month", "day", "hour", "minute", "second")
STEMS, BRANCHES = "甲乙丙丁戊己庚辛壬癸", "子丑寅卯辰巳午未申酉戌亥"
LIMITATION = (
    "lunar_python and tyme4rs share 6tail/Shou Xing astronomy ancestry; agreement "
    "is interoperability/regression evidence, not independent astronomical proof."
)


def compare(binary, civil, rule, sect, lunar):
    """Compare one civil timestamp and convention, preserving useful differences."""
    args = [str(binary), "bazi", "--json", "--day-boundary", rule]
    for field, value in zip(TIME_FIELDS, civil):
        args.extend([f"--{field}", str(value)])
    key = f"{datetime(*civil).isoformat()}+08:00 {rule}"
    try:
        process = subprocess.run(args, capture_output=True, text=True, encoding="utf-8", timeout=10)
        if process.returncode:
            return {"case": key, "matches": False, "error": process.stderr.strip()}
        actual = json.loads(process.stdout)
        eight = lunar.getEightChar()
        eight.setSect(sect)
        expected_pillars = [eight.getYear(), eight.getMonth(), eight.getDay(), eight.getTime()]
        indices = [actual["four_pillars"][p]["index"] for p in ("year", "month", "day", "hour")]
        actual_pillars = [STEMS[i % 10] + BRANCHES[i % 12] for i in indices]
        expected_lunar = [lunar.getYear(), abs(lunar.getMonth()), lunar.getDay(), lunar.getMonth() < 0]
        actual_lunar = [actual["lunar_date"][p] for p in ("year", "month", "day", "is_leap_month")]
        previous = lunar.getPrevJieQi()
        solar = previous.getSolar()
        expected_time = datetime(solar.getYear(), solar.getMonth(), solar.getDay(),
                                 solar.getHour(), solar.getMinute(), solar.getSecond())
        term = actual["solar_term"]
        actual_time = datetime(*(term["start"][p] for p in TIME_FIELDS))
        delta = int((actual_time - expected_time).total_seconds())
        return {
            "case": key,
            "matches": (actual_pillars == expected_pillars and actual_lunar == expected_lunar
                        and term["name"] == previous.getName() and delta == 0),
            "actual_pillars": actual_pillars, "expected_pillars": expected_pillars,
            "actual_lunar": actual_lunar, "expected_lunar": expected_lunar,
            "actual_term": term["name"], "expected_term": previous.getName(),
            "term_delta_seconds": delta,
        }
    except (OSError, subprocess.TimeoutExpired, ValueError, KeyError, TypeError) as error:
        return {"case": key, "matches": False, "error": str(error)}


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    executable = "qimen.exe" if os.name == "nt" else "qimen"
    parser.add_argument("--binary", type=Path,
                        default=Path(__file__).resolve().parents[1] / "target/debug" / executable)
    parser.add_argument("--output", type=Path, help="Optional full JSON report path")
    options = parser.parse_args()
    try:
        installed = version("lunar_python")
    except PackageNotFoundError:
        parser.error("Install the reference in a separate venv: python -m pip install lunar_python==1.4.8")
    if installed != REFERENCE_VERSION:
        parser.error(f"Expected lunar_python=={REFERENCE_VERSION}, found {installed}")
    if not options.binary.is_file():
        parser.error(f"Binary does not exist: {options.binary}; run cargo build -p qimen-cli --locked")
    from lunar_python import Solar

    records = []
    for year in YEARS:
        for month in range(1, 13):
            day = (1, 5, 15, 28)[(year + month) % 4]
            for hour in (0, 23):
                minute = second = 0 if hour == 0 else 59
                civil = (year, month, day, hour, minute, second)
                lunar = Solar.fromYmdHms(*civil).getLunar()
                for rule, sect in (("zi-start", 1), ("midnight", 2)):
                    records.append(compare(options.binary.resolve(), civil, rule, sect, lunar))
    failures = [row for row in records if not row["matches"]]
    deltas = [row["term_delta_seconds"] for row in records if "term_delta_seconds" in row]
    summary = {
        "reference": f"lunar_python {installed}", "binary": str(options.binary.resolve()),
        "distinct_civil_timestamps": len(YEARS) * 12 * 2, "cases": len(records),
        "passed": len(records) - len(failures), "failures": failures,
        "term_delta_min_seconds": min(deltas, default=None),
        "term_delta_max_seconds": max(deltas, default=None),
        "term_delta_nonzero": sum(delta != 0 for delta in deltas), "limitation": LIMITATION,
    }
    if options.output:
        try:
            options.output.parent.mkdir(parents=True, exist_ok=True)
            options.output.write_text(json.dumps({"summary": summary, "records": records},
                                                 ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        except OSError as error:
            parser.error(f"Cannot write report: {error}")
    print(json.dumps(summary, indent=2))
    return int(bool(failures))


if __name__ == "__main__":
    raise SystemExit(main())
