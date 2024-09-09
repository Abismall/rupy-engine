import threading
import tkinter as tk
import time
from typing import Optional
from Input.manager import InputManager
from Application.menu_manager import MenuManager
from constants import LAUNCH_OPTIONS_MENU, LOGGER_NAME_BASE, LOGGING_METHOD_OPTIONS
from Error.base import Status, create_error
from .signal import SignalBus, Signals
from Utils.log import Logger


class App:
    def __init__(self, env_file: str, logging_method: str, signal_bus: SignalBus, input_manager: InputManager, sleep_after_cycle: float):
        """
        Initializes the application with environment, logging, signals, inputs, and menu management.

        Args:
            env_file (str): The path to the environment file.
            logging_method (str): The logging method to use ('file' or 'console').
            signal_bus (SignalBus): The signal bus for inter-component communication.
            input_manager (InputManager): The input manager for handling user inputs.
            sleep_after_cycle (float): The sleep time between each application update cycle.
        """
        self.logger: Logger
        self._set_logger(method=logging_method)
        self.env_file = env_file
        self.signal_bus = signal_bus
        self.inputs = input_manager
        self.sleep_after_cycle = sleep_after_cycle
        self.tk_root = tk.Tk()
        self.running = False
        self.menu_manager = MenuManager(
            root=self.tk_root, signal_bus=self.signal_bus, env_file_path=self.env_file)

    def _set_logger(self, method: str):
        """
        Sets up the logger based on the specified logging method.

        Args:
            method (str): The logging method ('file' or 'console').

        Raises:
            TypeError: If the logging method is invalid or not recognized.
        """
        if not method or not isinstance(method, str) or method not in LOGGING_METHOD_OPTIONS:
            raise TypeError(create_error(
                status=Status.TypeError,
                details=f"Expected logger method to be one of 'file' or 'console', but received {
                    method}."
            ))
        if method == LOGGING_METHOD_OPTIONS[0]:
            self.logger = Logger(LOGGER_NAME_BASE).get_file_logger()
        elif method == LOGGING_METHOD_OPTIONS[1]:
            self.logger = Logger(LOGGER_NAME_BASE).get_console_logger()

    def start(self, args: dict):
        """
        Starts the application by initializing and running it.

        Args:
            args (dict): Arguments for initializing the application.
        """
        self.init(args)
        self.shutdown()

    def consume_signals(self):
        """
        Consumes signals for opening and closing menus.
        """
        self.signal_bus.consume(Signals.MENU_CLOSE.value)
        self.signal_bus.consume(Signals.MENU_OPEN.value)

    def init(self, args):
        """
        Initializes the application and runs the launch options menu if specified.

        Args:
            args (dict): Arguments specifying whether to open certain menus.
        """
        self.publish_signal(Signals.APP_INIT)
        if args.get('lo'):
            self.signal_bus.publish(
                Signals.MENU_OPEN.value, LAUNCH_OPTIONS_MENU)
        self.run()

    def run(self):
        """
        Runs the main application loop, updating inputs and processing signals.
        """
        self.publish_signal(Signals.APP_START)
        self.running = True
        while self.running:
            self.publish_signal(Signals.APP_UPDATE)
            self.update_inputs()
            self.consume_signals()
            time.sleep(self.sleep_after_cycle)

    def update_inputs(self):
        """
        Updates the state of the inputs and publishes relevant signals.
        """
        self.publish_signal(Signals.INPUT_UPDATE_START)
        self.inputs.update()
        captured_inputs = self.inputs.get_state()
        self.publish_signal(Signals.INPUT_UPDATE_BUTTONS, captured_inputs.get(
            "keyboard", self.inputs.keyboard_handler.initial_state))
        self.publish_signal(Signals.INPUT_UPDATE_MOUSE, captured_inputs.get(
            "mouse", self.inputs.mouse_handler.initial_state))
        self.publish_signal(Signals.INPUT_UPDATE_END)

    def publish_signal(self, signal: Signals, message: Optional[str] = None):
        """
        Publishes a signal with an optional message and logs it.

        Args:
            signal (Signals): The signal to publish.
            message (Optional[str]): The message associated with the signal.
        """
        self.logger.info(f"{signal.value} {message}")
        self.signal_bus.publish(signal.value, message)

    def print_running_threads(self):
        """
        Logs the currently running threads in the application.
        """
        threads = threading.enumerate()
        self.logger.info(f"Total running threads: {len(threads)}")
        for thread in threads:
            self.logger.info(f"{thread.name} (Daemon: {thread.daemon})")

    def render(self):
        """
        Publishes the render signal.
        """
        self.publish_signal(Signals.APP_RENDER)

    def shutdown(self):
        """
        Shuts down the application by closing all menus and stopping the main loop.
        """
        self.publish_signal(Signals.APP_SHUTDOWN)
        self.menu_manager.close_all_menus()
        self.running = False
