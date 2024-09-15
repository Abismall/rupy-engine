from .category import ErrorCategory

ERROR_MESSAGES = {
    "UNKNOWN_ERROR": {
        "category": ErrorCategory.GENERAL,
        "message": "An unexpected error occurred.",
    },
    "FILE_NOT_FOUND": {
        "category": ErrorCategory.IO,
        "message": "The specified file was not found.",
    },
    "PERMISSION_DENIED": {
        "category": ErrorCategory.IO,
        "message": "Permission denied: Access to the resource is restricted.",
    },
    "INPUT_VALIDATION_FAILED": {
        "category": ErrorCategory.INPUT,
        "message": "Input validation failed.",
    },
    "VALUE_ERROR": {
        "category": ErrorCategory.GENERAL,
        "message": "An invalid value was provided.",
    },
    "TYPE_ERROR": {
        "category": ErrorCategory.GENERAL,
        "message": "A type mismatch was encountered.",
    },
    "ARGUMENT_ERROR": {
        "category": ErrorCategory.GENERAL,
        "message": "Invalid arguments were supplied.",
    },
    "UNBOUND_LOCAL_ERROR": {
        "category": ErrorCategory.GENERAL,
        "message": "Referenced a variable before assignment.",
    },
}


def get_error_message(error_code: str):
    return ERROR_MESSAGES.get(error_code, ERROR_MESSAGES["UNKNOWN_ERROR"])
