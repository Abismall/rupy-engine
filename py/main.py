import os
import argparse
from typing import Any
from Application import App
from Application.signal import SignalBus
from Input import InputManager, WinNativeKeyboardHandler, WinNativeMouseHandler
from Utils.environment import EnvManager
from Error.base import Status, create_error


def parse_arguments():
    """
    Parses command-line arguments for the application.

    Returns:
        argparse.Namespace: Parsed arguments.
    """
    parser = argparse.ArgumentParser(
        description="Application CLI argument parser.")
    parser.add_argument(
        "-lo", "--lo", action="store_true", help="Launch options menu on start if this flag is present.")
    args = parser.parse_args()
    return args


def get_env_value(env_key: str, default: Any, expected_type: type) -> Any:
    """
    Retrieves an environment variable, validates its type, and provides a default if not set or invalid.

    Args:
        env_key (str): The environment variable key.
        default (Any): The default value to return if the environment variable is not set or invalid.
        expected_type (type): The expected type of the environment variable.

    Returns:
        Any: The value of the environment variable or the default if not set or invalid.
    """
    try:
        value = EnvManager.os_getenv(env_key)
        if value is None:
            return default
        value = expected_type(value)
        return value
    except ValueError:
        print(create_error(
            status=Status.ValueError,
            details=f"Invalid type for environment variable '{
                env_key}'. Expected {expected_type.__name__}."
        ))
        return default


def main():
    """
    Main entry point for the application.
    """
    args = parse_arguments()

    env_file_path = ".env"
    try:
        EnvManager.set_env_from_file(env_file_path)
    except Exception as e:
        print(create_error(
            status=Status.RuntimeError,
            details=f"Failed to load environment file: {env_file_path}.",
            trace=True
        ))

    sleep_after_cycle = get_env_value("SLEEP_AFTER_CYCLE", 0.1, float)

    app = App(
        env_file=env_file_path,
        logging_method=get_env_value("LOGGING_METHOD", "console", str),
        signal_bus=SignalBus(),
        sleep_after_cycle=sleep_after_cycle,
        input_manager=InputManager(
            mouse=WinNativeMouseHandler(),
            keyboard=WinNativeKeyboardHandler(),
            pressed_only_mode=True
        )
    )
    app.start(vars(args))


if __name__ == "__main__":
    main()
