import tkinter as tk
import time
from UI.menus.options_menu import OptionsMenu
from typing import Any, Optional
from Input.manager import InputManager
from Utils.validation import is_key_of
from Utils.environment import EnvManager
from Error.base import StatusText, create_error
from .signal import SignalBus, Signals
from Utils.log import Logger


class App:
    env_attribute_transpiles = {
        "APP_SLEEP_AFTER_CYCLE": float
    }

    def __init__(self, env_file: str, logger: str, signal_bus: SignalBus, input_manager: InputManager):
        """
        Initializes the Application with the necessary components such as logger, message bus,
        and input manager with the listener setup.
        """
        if logger not in ["file", "console"]:
            raise ValueError(create_error(status=StatusText.VALUE_ERROR,
                             message=f"Error: expected logger to be a string of 'console' or 'file', but received '{logger}' instead"))
        else:
            if logger == "file":
                self.logger = Logger('RupyLogger').get_file_logger()
            else:
                self.logger = Logger('RupyLogger').get_console_logger()
        self.env_file = env_file
        self.signal_bus = signal_bus
        self.inputs = input_manager
        self.tk_root = tk.Tk()
        self.running = False
        self.options_window_open = False

    def start(self):
        """Starts the application."""
        self.running = True
        self.logger.info('Application start')
        self.init()
        self.run()
        self.shutdown()

    def set_attributes_from_env(self, attributes: list = ['APP_SLEEP_AFTER_CYCLE']):
        """
        Sets class attributes based on environment variables.

        Args:
            attributes (list): A list of attribute names that correspond to environment variable names.
        """
        for attr in attributes:
            env_value = EnvManager.os_getenv(attr)
            if env_value is None:
                raise ValueError(create_error(status=StatusText.VALUE_ERROR,
                                 message=f"Error: could not locate {attr} or value is None"))
            else:
                if attr in self.env_attribute_transpiles:
                    setattr(self, attr.lower(),
                            self.env_attribute_transpiles[attr](env_value))
                else:
                    setattr(self, attr.lower(),
                            env_value)

    def init(self):
        """Initializes the engine, game resources, and launches the options menu."""
        self.publish_signal(Signals.APP_INIT)
        self.set_attributes_from_env()
        self.launch_options_menu()

        if is_key_of("pygame", globals()):
            pygame_module = globals().get("pygame", None)
            if pygame_module and hasattr(pygame_module, "init") and callable(pygame_module.init):
                pygame_module.init()

    def launch_options_menu(self, menu_title: str = "RupyEngine Options Menu", menu_geometry: str = "400x600"):

        def on_menu_close():
            self.options_window_open = False

        self.tk_root.title(menu_title)
        self.tk_root.geometry(menu_geometry)
        self.options_window_open = True
        _ = OptionsMenu(
            root=self.tk_root, close_menu_callback=on_menu_close, env_file_path=self.env_file, logger=self.logger)

        self.tk_root.protocol("WM_DELETE_WINDOW", on_menu_close)

        while self.options_window_open:
            self.tk_root.update()
            self.tk_root.update_idletasks()
            time.sleep(0.1)

        self.tk_root.destroy()

    def run(self):
        """Main loop of the engine."""
        self.publish_signal(Signals.APP_START)
        while self.running:
            self.update()
            self.render()
            time.sleep(self.app_sleep_after_cycle)

    def update(self):
        """Update game state."""
        self.publish_signal(Signals.APP_UPDATE)

        self.publish_signal(Signals.INPUT_UPDATE_START)
        self.inputs.update()

        captured_inputs = self.inputs.get_state()
        self.publish_signal(
            Signals.INPUT_UPDATE_BUTTONS, captured_inputs.get("keyboard", self.inputs.keyboard_handler.initial_state))
        self.publish_signal(
            Signals.INPUT_UPDATE_MOUSE, captured_inputs.get("mouse", self.inputs.mouse_handler.initial_state))

        self.publish_signal(Signals.INPUT_UPDATE_END)

    def publish_signal(self, signal: Signals | Any, message: Optional[str] | Any = None):
        self.logger.info(f"{signal} {message}")
        self.signal_bus.publish(signal, message)

    def render(self):
        """Render the game."""
        self.publish_signal(Signals.APP_RENDER)

    def shutdown(self):
        """Cleans up resources and exits the application."""
        self.publish_signal(Signals.APP_SHUTDOWN)
        self.running = False
