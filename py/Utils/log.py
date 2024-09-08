from enum import Enum
import logging
from logging.handlers import RotatingFileHandler
from typing import Any
from Error.base import StatusText, create_error
from Utils.environment import EnvManager


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

    @classmethod
    def name_to_format(cls):
        """
        Creates a dictionary that maps format names to their corresponding enum values.

        Returns:
            dict: A dictionary mapping string format names to enum values.
        """
        return {
            'SIMPLE': cls.SIMPLE.value,
            'DETAILED': cls.DETAILED.value,
            'VERBOSE': cls.VERBOSE.value,
            'WITH_FUNC_NAME': cls.WITH_FUNC_NAME.value,
            'TIMESTAMP_ONLY': cls.TIMESTAMP_ONLY.value
        }

    @classmethod
    def from_name(cls, name: str):
        """
        Gets the log format string from its name.

        Args:
            name (str): The name of the log format (e.g., 'SIMPLE').

        Returns:
            str: The log format string associated with the given name.

        Raises:
            ValueError: If the format name is not recognized.
        """
        name = name.upper()
        name_to_format = cls.name_to_format()
        if name not in name_to_format:
            raise ValueError(f"Invalid log format name: {name}. Must be one of {
                             list(name_to_format.keys())}.")
        return name_to_format[name]


class Logger:
    def __init__(self, name: None, log_file: str = None, log_level=None, log_format=None):
        """
        Initializes the custom logger with separate loggers for console and file.

        Args:
            name (str): The name of the logger.
            log_file (str): The file where logs will be saved.
            log_level (int): The logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL).
            log_format (LogFormat): The format of the log messages.
        """
        self.name = self._get_arg_or_env('name', name, str)
        self.log_file = self._get_arg_or_env('log_file', log_file, str)
        self.log_level = self._get_arg_or_env(
            'log_level', log_level, self._validate_log_level)
        self.log_format = self._get_arg_or_env(
            'log_format', log_format, self._validate_log_format)

        self.console_logger = self._create_console_logger()
        self.file_logger = self._create_file_logger()

    def _get_arg_or_env(self, arg_name, arg_value, validation_func):

        if arg_value is not None:
            return validation_func(arg_value)

        env_value = EnvManager.os_getenv(arg_name)
        if env_value is not None:
            return validation_func(env_value)

        raise ValueError(
            f"'{arg_name}' must be provided either as an argument or an environment variable.")

    def _validate_log_level(self, log_level):

        if isinstance(log_level, int):
            if log_level in [logging.DEBUG, logging.INFO, logging.WARNING, logging.ERROR, logging.CRITICAL]:
                return log_level
        elif isinstance(log_level, str):
            log_level = log_level.upper()
            if log_level in logging._nameToLevel:
                return logging._nameToLevel[log_level]

    def _validate_log_format(self, log_format):
        if isinstance(log_format, LogFormat):
            return log_format
        elif isinstance(log_format, str):
            log_format = log_format.upper()
            if log_format in LogFormat.__members__:
                return LogFormat[log_format]
        raise ValueError(f"Invalid log format: {log_format}. Must be one of {
            ', '.join(LogFormat.__members__.keys())}.")

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
