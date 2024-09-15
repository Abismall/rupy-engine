import sys
import threading
import tkinter as tk
import time
from typing import Optional
from Input.manager import InputManager
from Application.menu_manager import MenuManager
from Utils.constants import LAUNCH_OPTIONS_FLAG, LAUNCH_OPTIONS_MENU, FROZEN_EXEC
from Utils.environment import EnvManager
from Error.base import PyEngineError
from .signal import SignalBus, Signals


class App:
    def __init__(self, env_file: str, signal_bus: SignalBus, input_manager: InputManager, logger, sleep_after_cycle: float):
        self.logger = logger
        self.env_file = env_file
        self.signal_bus = signal_bus
        self.inputs = input_manager
        self.sleep_after_cycle = sleep_after_cycle
        self.tk_root = tk.Tk()
        self.running = False
        self.menu_manager = MenuManager(
            root=self.tk_root, signal_bus=self.signal_bus, env_file_path=self.env_file)
        self.signal_bus.subscribe(
            Signals.ENV_RELOAD.value, self.load_environment)

    def start(self, args: dict):
        self.init(args)
        self.shutdown()

    def consume_signals(self):
        for signal in Signals:
            self.signal_bus.consume(signal.value)

    def init(self, args):
        self.publish_signal(Signals.APP_INIT)

        if hasattr(sys, FROZEN_EXEC):
            self.signal_bus.publish(
                Signals.MENU_OPEN.value, LAUNCH_OPTIONS_MENU)
        elif args.get(LAUNCH_OPTIONS_FLAG):
            self.signal_bus.publish(
                Signals.MENU_OPEN.value, LAUNCH_OPTIONS_MENU)
        self.run()

    def load_environment(self, channel: str, message: str):
        try:
            EnvManager.load_env(self.env_file)
            for key, value in EnvManager.get_dotenv_values(self.env_file).items():
                print(key, value)
                print(isinstance(key, str) and hasattr(self, key.lower()))
                if isinstance(key, str) and hasattr(self, key.lower()):
                    if int(value):
                        setattr(self,  key.lower(), int(value))
                    else:
                        setattr(self,  key.lower(), value)
        except Exception as e:
            raise PyEngineError("CONFIG_LOAD_FAILED") from e

    def run(self):
        self.publish_signal(Signals.APP_START)
        self.running = True
        while self.running:
            self.publish_signal(Signals.APP_UPDATE)
            self.update_inputs()
            self.consume_signals()
            self.logger.info(f"Sleeping for: {self.sleep_after_cycle}")
            time.sleep(self.sleep_after_cycle)

    def update_inputs(self):
        self.publish_signal(Signals.INPUT_UPDATE_START)
        self.inputs.update()
        captured_inputs = self.inputs.get_state()
        self.publish_signal(Signals.INPUT_UPDATE_BUTTONS, captured_inputs.get(
            "keyboard", self.inputs.keyboard_handler.initial_state))
        self.publish_signal(Signals.INPUT_UPDATE_MOUSE, captured_inputs.get(
            "mouse", self.inputs.mouse_handler.initial_state))
        self.publish_signal(Signals.INPUT_UPDATE_END)

    def publish_signal(self, signal: Signals, message: Optional[str] = None):
        self.logger.info(f"{signal.value} {message}")
        self.signal_bus.publish(signal.value, message)

    def print_running_threads(self):
        threads = threading.enumerate()
        self.logger.info(f"Total running threads: {len(threads)}")
        for thread in threads:
            self.logger.info(f"{thread.name} (Daemon: {thread.daemon})")

    def render(self):
        self.publish_signal(Signals.APP_RENDER)

    def shutdown(self):
        self.publish_signal(Signals.APP_SHUTDOWN)
        self.menu_manager.close_all_menus()
        self.running = False
