
from enum import Enum
from py.Error.base import StatusText, create_error
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


def is_defined_local(var_name: str) -> bool:
    """Checks if a variable is defined in the local scope."""
    local_vars = locals()
    return var_name in local_vars


def is_defined_global(var_name: str) -> bool:
    """Checks if a variable is defined in the global scope."""
    global_vars = globals()
    return var_name in global_vars


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
    # Check if the type exists in the Enum and match the type
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
            status=StatusText.UNBOUND_LOCAL_ERROR.value,
            message=f"unbound or undefined variable '{to_evaluate}'"
        ))

    if to_match not in PythonTypeEnum.__members__:
        raise ValueError(create_error(
            status=StatusText.VALUE_ERROR.value,
            message=f"invalid or unknown type '{type(to_match).__name__}'",
        ))

    try:
        if match_type(to_match, to_evaluate):
            return True
        else:
            raise TypeError(create_error(
                status=StatusText.TYPE_ERROR.value,
                message=f"Expected type '{type(to_match).__name__}', but received '{type(to_evaluate).__name__}'"))
    except Exception as fatal_err:
        raise RuntimeError(create_error(
            status=StatusText.RUNTIME_ERROR.value,
            message=str(fatal_err) or None
        ))


def is_callable(method: Any) -> bool:
    """Checks if the provided method is callable."""
    return callable(method)
