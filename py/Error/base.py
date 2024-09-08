from enum import Enum

from Error.trace import Trace

GenericErrorMessage = "an exception occurred"


class StatusText(Enum):
    UNKNOWN_ERROR = "[UnknownError]"
    TYPE_ERROR = "[TypeError]"
    VALUE_ERROR = "[ValueError]"
    UNBOUND_LOCAL_ERROR = "[UnboundLocalError]"
    ARGUMENT_ERROR = "[ArgumentError]"
    RUNTIME_ERROR = "[RuntimeError]"


def create_error(status: StatusText = StatusText.UNKNOWN_ERROR.value, message: str = GenericErrorMessage, trace: bool = True):
    return "\n".join([f"{status}:{message}\n".format(status, message), Trace().source_lines if trace else ""])
