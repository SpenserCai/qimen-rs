"""BaZi and Qimen Dunjia charts computed by the shared Rust engine."""

from __future__ import annotations

import json
from collections.abc import Mapping
from typing import cast

from ._native import calculate_json
from .types import Chart, ChartRequest, ExtensionOptions

__all__ = ["Chart", "ChartRequest", "ExtensionOptions", "calculate", "calculate_json"]


def calculate(
    request: ChartRequest | Mapping[str, object],
    *,
    extensions: ExtensionOptions | None = None,
) -> Chart:
    """Return the canonical chart as a plain dictionary.

    ``ValueError`` reports invalid calendar values, unsupported conventions, or
    invalid JSON. ``TypeError`` reports a non-mapping or non-JSON Python value.
    Date ranges, defaults, and field validation are shared with the Rust API.
    Computation releases the GIL. No local timezone is read implicitly.
    Extensions default to disabled. Select them either in the request or with
    the keyword argument; supplying both raises ``ValueError``.
    """
    if not isinstance(request, Mapping):
        raise TypeError("request must be a mapping containing year, month, day, hour")
    if any(not isinstance(key, str) for key in request):
        raise TypeError("request keys must be strings")
    values = dict(request)
    if extensions is not None:
        if "extensions" in values:
            raise ValueError("supply extensions in the request or as a keyword, not both")
        values["extensions"] = extensions
    payload = json.dumps(values, ensure_ascii=False, allow_nan=False)
    return cast(Chart, json.loads(calculate_json(payload)))
