"""Optional-annotation behavior through an installed Python wheel."""

import json
import unittest

from qimen_rs import Chart, ExtensionOptions, calculate, calculate_json

from schema_assertions import assert_schema


class ExtensionApiTests(unittest.TestCase):
    # User-supplied software screenshot: 2026-09-18 18:15, UTC+08:00.
    request = {"year": 2026, "month": 9, "day": 18, "hour": 18, "minute": 15}
    all_rules: ExtensionOptions = {
        "hidden_stems": "duty_door_hour_stem_with_center_fallback",
        "strength": "classical_stars_and_five_elements",
        "growth_stages": "yang_forward_yin_reverse_fire_earth",
        "punishments": "six_instrument_branches",
        "tombs": "growth_stage_fire_earth",
        "day_horse": "day_branch_three_harmony",
        "door_pressure": "door_controls_palace",
    }

    def test_default_and_empty_options_omit_extensions(self):
        base = calculate(self.request)
        self.assertNotIn("extensions", base)
        self.assertEqual(base["schema_version"], "1.1")
        self.assertEqual(calculate(self.request, extensions={}), base)
        self.assertEqual(calculate(self.request, extensions={"day_horse": None}), base)

    def test_day_horse_can_be_selected_without_changing_hour_horse(self):
        options: ExtensionOptions = {"day_horse": "day_branch_three_harmony"}
        chart = calculate(self.request, extensions=options)
        self.assertEqual(chart["horse"], {"branch": "hai", "palace": 6})
        self.assertEqual(chart["extensions"]["day_horse"]["horse"], {"branch": "si", "palace": 4})
        self.assertEqual(set(chart["extensions"]), {"day_horse"})
        assert_schema(chart, Chart)

    def test_all_options_match_json_interface_and_public_types(self):
        request = {**self.request, "extensions": self.all_rules}
        chart = calculate(self.request, extensions=self.all_rules)
        self.assertEqual(chart, calculate(request))
        self.assertEqual(chart, json.loads(calculate_json(json.dumps(request))))
        annotations = chart["extensions"]
        self.assertEqual({name: result["rule"] for name, result in annotations.items()}, self.all_rules)
        self.assertEqual(
            [item["stem"] for item in annotations["hidden_stems"]["palaces"]],
            ["ren", "xin", "geng", "ji", "wu", "yi", "bing", "ding", "gui"],
        )
        assert_schema(chart, Chart)

    def test_three_wonders_tombs_preserve_not_applicable_values(self):
        chart = calculate(self.request, extensions={"tombs": "traditional_three_wonders"})
        stems = chart["extensions"]["tombs"]["stems"]
        instrument = next(item for item in stems if item["placement"]["stem"] == "geng")
        self.assertIsNone(instrument["tomb_branch"])
        self.assertIsNone(instrument["is_in_tomb"])
        assert_schema(chart, Chart)

    def test_conflicting_option_locations_are_rejected(self):
        with self.assertRaisesRegex(ValueError, "not both"):
            calculate({**self.request, "extensions": {}}, extensions=self.all_rules)

    def test_unknown_names_rules_and_invalid_shapes_are_rejected(self):
        for options in (
            {"typo": "day_branch_three_harmony"},
            {"day_horse": "automatic"},
            {"day_horse": {"day_branch_three_harmony": None}},
            {"hidden_stems": True},
            [],
            True,
            "all",
            None,
        ):
            with self.subTest(options=options), self.assertRaises(ValueError):
                calculate({**self.request, "extensions": options})


if __name__ == "__main__":
    unittest.main()
