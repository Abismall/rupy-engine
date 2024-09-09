from enum import Enum
# Updated to use the new Status and create_error
from Error.base import Status, create_error
from typing import Any

# Enum to define Python types for type matching


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
    """Checks if a variable is defined in the global scope."""
    return var_name in mapping


def is_not_none(var: Any) -> bool:
    """Checks if a variable is defined and is not None."""
    return var is not None


def match_type(to_match: str, to_evaluate: Any) -> bool:
    """
    Checks if a variable is of a specific type without raising exceptions.

    Args:
        to_match (str): The type name to match against.
        to_evaluate (Any): The variable to check.

    Returns:
        bool: True if the type matches, False otherwise.
    """
    return to_match in PythonTypeEnum.__members__ and isinstance(to_evaluate, PythonTypeEnum[to_match].value)


def match_type_or_raise_exception(to_match: str, to_evaluate: Any) -> bool:
    """
    Checks if a variable is of a specific type and raises an exception if not.

    Args:
        to_match (str): The type name to match against.
        to_evaluate (Any): The variable to check.

    Returns:
        bool: True if the type matches.

    Raises:
        UnboundLocalError: If the variable is not defined in the current context.
        ValueError: If the provided type name is not recognized.
        RuntimeError: If an unexpected error occurs during type checking.
    """
    if not is_not_none(to_evaluate):
        raise UnboundLocalError(create_error(
            status=Status.UnboundLocalError,
            details=f"Unbound or undefined variable '{to_evaluate}'"
        ))

    if to_match not in PythonTypeEnum.__members__:
        raise ValueError(create_error(
            status=Status.ValueError,
            details=f"Invalid or unknown type '{to_match}'"
        ))

    try:
        if match_type(to_match, to_evaluate):
            return True
        else:
            raise TypeError(create_error(
                status=Status.TypeError,
                details=f"Expected type '{to_match}', but received '{
                    type(to_evaluate).__name__}'"
            ))
    except Exception as e:
        raise RuntimeError(create_error(
            status=Status.RuntimeError,
            details=f"Warning: Unknown exception caught when matching types for '{
                to_match}', '{type(to_evaluate).__name__}'"
        )) from e


def is_callable(method: Any) -> bool:
    """Checks if the provided method is callable."""
    return callable(method)
