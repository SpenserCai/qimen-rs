"""Check that installed Rust results satisfy the shipped Python type contract."""

import types
import typing


def assert_schema(value, annotation, path="chart"):
    """Validate TypedDict fields and primitive/literal/list/union annotations."""
    origin = typing.get_origin(annotation)
    arguments = typing.get_args(annotation)
    if typing.is_typeddict(annotation):
        assert isinstance(value, dict), f"{path}: expected dict"
        fields = typing.get_type_hints(annotation)
        assert annotation.__required_keys__ <= value.keys(), f"{path}: missing fields"
        assert value.keys() <= fields.keys(), f"{path}: undocumented fields"
        for key, item in value.items():
            assert_schema(item, fields[key], f"{path}.{key}")
    elif origin is list:
        assert isinstance(value, list), f"{path}: expected list"
        for index, item in enumerate(value):
            assert_schema(item, arguments[0], f"{path}[{index}]")
    elif origin is typing.Literal:
        assert value in arguments, f"{path}: unknown literal {value!r}"
    elif origin in (typing.Union, types.UnionType):
        for variant in arguments:
            try:
                assert_schema(value, variant, path)
                return
            except AssertionError:
                pass
        raise AssertionError(f"{path}: no matching union variant for {value!r}")
    else:
        assert type(value) is annotation, f"{path}: expected {annotation}, got {type(value)}"
