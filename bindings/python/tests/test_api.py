"""Installed-wheel contract tests; run with unittest discover, not Rust's runner."""

import json
import unittest
from concurrent.futures import ThreadPoolExecutor

from qimen_rs import Chart, calculate, calculate_json

from schema_assertions import assert_schema


class ChartApiTests(unittest.TestCase):
    request = {"year": 2024, "month": 2, "day": 10, "hour": 12}

    def test_dictionary_and_json_interfaces_agree(self):
        chart = calculate(self.request)
        self.assertEqual(chart, json.loads(calculate_json(json.dumps(self.request))))
        self.assertEqual(len(chart["palaces"]), 9)

    def test_invalid_date_is_value_error(self):
        with self.assertRaises(ValueError):
            calculate({**self.request, "month": 2, "day": 30})

    def test_malformed_and_unknown_fields_are_rejected(self):
        for payload in ["not JSON", "{}", json.dumps({**self.request, "typo": 1})]:
            with self.subTest(payload=payload), self.assertRaises(ValueError):
                calculate_json(payload)

    def test_non_mapping_and_non_string_keys_are_rejected(self):
        for request in [[], None, {1: 2024}]:
            with self.subTest(request=request), self.assertRaises(TypeError):
                calculate(request)

    def test_floats_bools_and_nonfinite_numbers_are_rejected(self):
        for hour in [1.5, True, float("nan"), float("inf")]:
            with self.subTest(hour=hour), self.assertRaises(ValueError):
                calculate({**self.request, "hour": hour})

    def test_parallel_calls_do_not_share_mutable_state(self):
        expected = calculate(self.request)
        with ThreadPoolExecutor(max_workers=4) as pool:
            results = list(pool.map(calculate, [self.request] * 12))
        self.assertTrue(all(result == expected for result in results))

    def test_public_typed_dictionary_matches_rust_output_recursively(self):
        for month in (2, 8):
            with self.subTest(month=month):
                assert_schema(calculate({**self.request, "month": month}), Chart)


if __name__ == "__main__":
    unittest.main()
