from enum import Enum
from typing import Any, Type


class PythonTypeEnum(Enum):
    String = str
    Integer = int
    Float = float
    Boolean = bool
    List = list
    Dict = dict
    Tuple = tuple
    Set = set
    none = type(None)


def is_key_of(var_name: str, mapping: dict[str, Any]) -> bool:
    return var_name in mapping


def is_not_none(var: Any) -> bool:
    return var is not None


def match_type(to_match: str, to_evaluate: Any) -> bool:
    return to_match in PythonTypeEnum.__members__ and isinstance(to_evaluate, PythonTypeEnum[to_match].value)


def match_type_or_raise_exception(expected_type: Type, to_evaluate: Any) -> bool:
    if to_evaluate is None:
        raise UnboundLocalError(
            f"Unbound or undefined variable '{to_evaluate}'")

    if not isinstance(expected_type, type):
        raise ValueError(f"Invalid or unknown type '{expected_type}'")

    try:
        if isinstance(to_evaluate, expected_type):
            return True
        else:
            raise TypeError(f"Expected type '{expected_type.__name__}', but received '{
                            type(to_evaluate).__name__}'")
    except Exception as e:
        raise RuntimeError(f"Warning: Unknown exception caught when matching types for '{
                           expected_type.__name__}', '{type(to_evaluate).__name__}'")


def is_callable(method: Any) -> bool:
    return callable(method)
