import argparse
from Application.signal import SignalBus
from Utils.environment import EnvManager
from Input.handlers import WinNativeKeyboardHandler, WinNativeMouseHandler
from Input.manager import InputManager
from Application.application import App
from Utils.constants import DEFAULT_LOG_FILE, DEFAULT_LOG_FORMAT, DEFAULT_LOG_LEVEL, DEFAULT_SLEEP_AFTER_CYCLE, LOG_METHOD_CONSOLE, LOGGING_METHOD_OPTIONS, PYENGINE
from Utils.log import Logger


def parse_arguments():
    parser = argparse.ArgumentParser(
        description=PYENGINE)

    parser.add_argument(
        "-lo", "--lo", action="store_true")

    return parser.parse_args()


def main():
    args = parse_arguments()
    env_file_path = "py/.env"

    EnvManager.set_env_from_file(env_file_path)

    sleep_after_cycle = EnvManager.env_or_default(
        "SLEEP_AFTER_CYCLE", DEFAULT_SLEEP_AFTER_CYCLE, float)
    logging_method = EnvManager.env_or_default(
        "LOGGING_METHOD", LOG_METHOD_CONSOLE, str)
    logging_level = EnvManager.env_or_default(
        "LOGGING_LEVEL", DEFAULT_LOG_LEVEL, str)
    logging_format = EnvManager.env_or_default(
        "LOGGING_FORMAT", DEFAULT_LOG_FORMAT, str)
    logging_file_path = EnvManager.env_or_default(
        "LOGGING_FILE_PATH", DEFAULT_LOG_FILE, str)
    app = App(
        env_file=env_file_path,
        logger=Logger(name=PYENGINE, log_file=logging_file_path, log_level=logging_level, log_format=logging_format).get_console_logger(
        ) if logging_method == LOGGING_METHOD_OPTIONS[0] else Logger(name=PYENGINE, log_file=logging_file_path, log_level=logging_level, log_format=logging_format).get_file_logger(),
        signal_bus=SignalBus(),
        sleep_after_cycle=sleep_after_cycle,
        input_manager=InputManager(
            mouse=WinNativeMouseHandler(),
            keyboard=WinNativeKeyboardHandler(),
            pressed_only_mode=True
        ))

    app.start(vars(args))


if __name__ == "__main__":
    main()
