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


def create_error(status: StatusText, message: str = GenericErrorMessage, trace: bool = True):
    return "{status}:{message}\n".format(status, message).join(Trace() if trace else "")
