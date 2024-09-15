from enum import Enum, auto


class ErrorCategory(Enum):
    GENERAL = auto()
    IO = auto()
    INPUT = auto()
    RENDERING = auto()
    PHYSICS = auto()
    NETWORK = auto()
    RESOURCE = auto()
    CONFIG = auto()
    SCRIPT = auto()
    THREADING = auto()
