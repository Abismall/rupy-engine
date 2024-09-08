from enum import Enum

from Error.trace import Trace

GenericErrorMessage = "an exception occurred"


class StatusText(Enum):
    UNKNOWN_ERROR = "[UnknownError]"
    IMPORT_ERROR = "[ImportError]"
    TYPE_ERROR = "[TypeError]"
    VALUE_ERROR = "[ValueError]"
    UNBOUND_LOCAL_ERROR = "[UnboundLocalError]"
    ARGUMENT_ERROR = "[ArgumentError]"
    RUNTIME_ERROR = "[RuntimeError]"


def create_error(
    status: str = StatusText.UNKNOWN_ERROR.value,
    message: str = GenericErrorMessage,
    trace: bool = False,
    **args
):
    trace_lines = Trace().source_lines if trace else ""
    error_dict = {
        "status": status,
        "message": message,
        "trace": trace,
        "trace_lines": trace_lines
    }

    error_dict.update(args)
    return str(error_dict)
