"""BaZi and Qimen Dunjia charts computed by the shared Rust engine."""

from __future__ import annotations

import json
from collections.abc import Mapping
from typing import cast

from ._native import calculate_json
from .types import Chart, ChartRequest

__all__ = ["Chart", "ChartRequest", "calculate", "calculate_json"]


def calculate(request: ChartRequest | Mapping[str, object]) -> Chart:
    """Return the canonical chart as a plain dictionary.

    ``ValueError`` reports invalid calendar values, unsupported conventions, or
    invalid JSON. ``TypeError`` reports a non-mapping or non-JSON Python value.
    Date ranges, defaults, and field validation are shared with the Rust API.
    Computation releases the GIL. No local timezone is read implicitly.
    """
    if not isinstance(request, Mapping):
        raise TypeError("request must be a mapping containing year, month, day, hour")
    if any(not isinstance(key, str) for key in request):
        raise TypeError("request keys must be strings")
    payload = json.dumps(dict(request), ensure_ascii=False, allow_nan=False)
    return cast(Chart, json.loads(calculate_json(payload)))
