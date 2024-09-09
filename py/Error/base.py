from enum import Enum
from Error.trace import Trace
from constants import (
    ERROR_MESSAGE_TEMPLATE,
    UNKNOWN_ERROR_STATUS, IMPORT_ERROR_STATUS, TYPE_ERROR_STATUS, VALUE_ERROR_STATUS,
    UNBOUND_LOCAL_ERROR_STATUS, ARGUMENT_ERROR_STATUS, RUNTIME_ERROR_STATUS,
    CHILD_PROCESS_ERROR_STATUS, UNKNOWN_ERROR_MSG, IMPORT_ERROR_MSG, TYPE_ERROR_MSG,
    VALUE_ERROR_MSG, UNBOUND_LOCAL_ERROR_MSG, ARGUMENT_ERROR_MSG, RUNTIME_ERROR_MSG,
    CHILD_PROCESS_ERROR_MSG
)


class Status(Enum):
    UnknownError = UNKNOWN_ERROR_STATUS
    ImportError = IMPORT_ERROR_STATUS
    TypeError = TYPE_ERROR_STATUS
    ValueError = VALUE_ERROR_STATUS
    UnboundLocalError = UNBOUND_LOCAL_ERROR_STATUS
    ArgumentError = ARGUMENT_ERROR_STATUS
    RuntimeError = RUNTIME_ERROR_STATUS
    ChildProcessError = CHILD_PROCESS_ERROR_STATUS


class Message(Enum):
    UnknownError = UNKNOWN_ERROR_MSG
    ImportError = IMPORT_ERROR_MSG
    TypeError = TYPE_ERROR_MSG
    ValueError = VALUE_ERROR_MSG
    UnboundLocalError = UNBOUND_LOCAL_ERROR_MSG
    ArgumentError = ARGUMENT_ERROR_MSG
    RuntimeError = RUNTIME_ERROR_MSG
    ChildProcessError = CHILD_PROCESS_ERROR_MSG


def format_error_message(status: Status, details: str = "") -> str:
    """
    Formats the error message based on the status and details provided.

    Args:
        status (Status): The error status enum value.
        details (str): Additional details to format into the message.

    Returns:
        str: The formatted error message.
    """
    base_message = Message[status.name].value

    message = base_message.format(details=details)

    return ERROR_MESSAGE_TEMPLATE.format(status=status.value, message=message)


def create_error(
    status: Status = Status.UnknownError,
    message: str = "",
    trace: bool = False,
    **args
) -> str:
    """
    Creates a formatted error representation including status, message, and optional trace information.

    Args:
        status (Status): The error status.
        message (str): A custom error message, if provided; otherwise, it uses the formatted default message.
        trace (bool): If True, includes trace information.
        **args: Additional key-value pairs to include in the error dictionary.

    Returns:
        str: A string representation of the error dictionary.
    """

    trace_lines = Trace().source_lines if trace else ""
    error_message = message or format_error_message(
        status, args.get("details", ""))

    error_dict = {
        "status": status.value,
        "message": error_message,
        "trace": trace,
        "trace_lines": trace_lines
    }

    error_dict.update(args)
    return str(error_dict)
