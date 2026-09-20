"""Published API date-range checks using Python's independent Gregorian ordinals."""

from datetime import date
import unittest

from qimen_rs import calculate


class CalendarRangeTests(unittest.TestCase):
    def test_proleptic_gregorian_days_across_the_supported_range(self):
        anchor = date(2000, 1, 7)  # The documented Jia-Zi day anchor.
        for civil in (
            date(1, 1, 1), date(4, 2, 29), date(24, 1, 28),
            date(1070, 1, 1), date(1582, 10, 5),
            date(1582, 10, 14), date(1899, 12, 31), date(2101, 1, 1),
            date(9999, 12, 31),
        ):
            with self.subTest(date=civil):
                chart = calculate({
                    "year": civil.year, "month": civil.month,
                    "day": civil.day, "hour": 12,
                })
                self.assertEqual(chart["calendar"]["four_pillars"]["day"]["index"],
                                 (civil - anchor).days % 60)
                self.assertEqual(len(chart["palaces"]), 9)

    def test_input_endpoints_keep_valid_offsets_and_reject_outside_years(self):
        for request in (
            {"year": 1, "month": 1, "day": 1, "hour": 0, "utc_offset_minutes": 840},
            {"year": 9999, "month": 12, "day": 31, "hour": 23,
             "minute": 59, "second": 59, "utc_offset_minutes": -840},
        ):
            with self.subTest(request=request):
                self.assertEqual(len(calculate(request)["palaces"]), 9)
        for year in (0, 10000):
            with self.subTest(year=year), self.assertRaises(ValueError):
                calculate({"year": year, "month": 1, "day": 1, "hour": 0})


if __name__ == "__main__":
    unittest.main()
