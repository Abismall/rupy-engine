from enum import Enum
import logging
from logging.handlers import RotatingFileHandler
from typing import Any

from Error.base import StatusText, create_error


def canonical_repr(var: Any) -> str:
    """Returns the canonical string representation of a variable."""
    if isinstance(var, str):
        return f'"{var}"'
    elif isinstance(var, (int, float, bool)):
        return str(var)
    elif var is None:
        return 'None'
    else:
        return repr(var)


class LogFormat(Enum):
    """
    Enum class for different log formats.
    """
    SIMPLE = '%(asctime)s - %(levelname)s - %(message)s'
    DETAILED = '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    VERBOSE = '%(asctime)s - %(name)s - %(levelname)s - [%(filename)s:%(lineno)d] - %(message)s'
    WITH_FUNC_NAME = '%(asctime)s - %(name)s - %(levelname)s - %(funcName)s - %(message)s'
    TIMESTAMP_ONLY = '%(asctime)s - %(message)s'


class LogFormat(Enum):
    """
    Enum class for different log formats.
    """
    SIMPLE = '%(asctime)s - %(levelname)s - %(message)s'
    DETAILED = '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    VERBOSE = '%(asctime)s - %(name)s - %(levelname)s - [%(filename)s:%(lineno)d] - %(message)s'
    WITH_FUNC_NAME = '%(asctime)s - %(name)s - %(levelname)s - %(funcName)s - %(message)s'
    TIMESTAMP_ONLY = '%(asctime)s - %(message)s'


class Logger:
    def __init__(self, name: str, log_file: str = "app.log", log_level=logging.INFO, log_format=LogFormat.DETAILED):
        """
        Initializes the custom logger with separate loggers for console and file.

        Args:
            name (str): The name of the logger.
            log_file (str): The file where logs will be saved.
            log_level (int): The logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL).
            log_format (LogFormat): The format of the log messages.
        """
        self.name = name
        self.log_file = log_file
        self.log_level = log_level
        self.log_format = log_format

        # Initialize separate loggers
        self.console_logger = self._create_console_logger()
        self.file_logger = self._create_file_logger()

    def _has_existing_handler(self):
        if hasattr(self, "console_logger"):
            if (isinstance(self.console_logger, logging.StreamHandler) or isinstance(self.console_logger, RotatingFileHandler)):
                if hasattr(self.console_logger, 'hasHandlers') and callable(self.console_logger.hasHandlers):
                    return self.console_logger.hasHandlers()
                raise ValueError(create_error(status=StatusText.RUNTIME_ERROR.value,
                                 message="Logger is missing method 'hasHandlers' or method is not callable"))
            else:
                raise
        else:
            raise

    def _create_console_logger(self) -> logging.Logger:
        """
        Creates and configures a logger for console output.

        Returns:
            logging.Logger: Configured logger for console output.
        """
        console_logger = logging.getLogger(f"{self.name}_console")
        console_logger.setLevel(self.log_level)

        formatter = logging.Formatter(self.log_format.value)
        console_handler = logging.StreamHandler()
        console_handler.setLevel(self.log_level)
        console_handler.setFormatter(formatter)

        if not console_logger.hasHandlers():
            console_logger.addHandler(console_handler)

        return console_logger

    def _create_file_logger(self) -> logging.Logger:
        file_logger = logging.getLogger(f"{self.name}_file")
        file_logger.setLevel(self.log_level)

        formatter = logging.Formatter(self.log_format.value)
        file_handler = RotatingFileHandler(
            self.log_file, maxBytes=5 * 1024 * 1024, backupCount=3
        )
        file_handler.setLevel(self.log_level)
        file_handler.setFormatter(formatter)
        if not file_logger.hasHandlers():
            file_logger.addHandler(file_handler)

        return file_logger

    def log_to_console(self, level: int, message: str):
        self.console_logger.log(level, message)

    def log_to_file(self, level: int, message: str):
        self.file_logger.log(level, message)

    def get_console_logger(self):
        return self.console_logger

    def get_file_logger(self):
        return self.file_logger
