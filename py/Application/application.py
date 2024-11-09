import sys
import threading
import tkinter as tk
import time
from tkinter import ttk
from typing import Optional
from Input.manager import InputManager
from Application.menu_manager import MenuManager
from Utils.constants import LAUNCH_OPTIONS_FLAG, LAUNCHER_MENU, FROZEN_EXEC, SCENE_MENU
from Utils.environment import EnvManager
from Error.base import PyEngineError
from .engine import launch_rupy_engine
from .signal import SignalBus, Signals


class App:
    def __init__(self, env_file: str, signal_bus: SignalBus, input_manager: InputManager, logger, sleep_after_cycle: float):
        self.logger = logger
        self.env_file = env_file
        self.signal_bus = signal_bus
        self.inputs = input_manager
        self.sleep_after_cycle = sleep_after_cycle
        self.tk_root = tk.Tk()
        self.tk_root.geometry("800x600")
        self.tk_root.title("Rupy Engine")

        style = ttk.Style()
        style.configure("Dark.TButton", background="#5a5a5a",
                        foreground="#ffffff", padding=(5, 5))
        style.map("Dark.TButton", background=[("active", "#6f6f6f")])

        self.running = False
        self.menu_manager = MenuManager(
            root=self.tk_root, signal_bus=self.signal_bus, env_file_path=self.env_file)

        self.signal_bus.subscribe(Signals.ENV_RELOAD, self.load_environment)
        self.signal_bus.subscribe(Signals.LAUNCH_ENGINE, self.launch_engine)

    def start(self, args: dict):
        self.init(args)
        self.tk_root.mainloop()
        self.shutdown()

    def consume_signals(self):
        for signal in Signals:
            self.signal_bus.consume(signal)

    def init(self, args):
        self.publish_signal(Signals.APP_INIT)

        self.signal_bus.publish(
            Signals.MENU_OPEN, SCENE_MENU)
        self.signal_bus.publish(
            Signals.MENU_OPEN, LAUNCHER_MENU)
        self.run()

    def load_environment(self, channel: str, message: str):
        try:
            EnvManager.load_env(self.env_file)
            for key, value in EnvManager.get_dotenv_values(self.env_file).items():
                if isinstance(key, str) and hasattr(self, key.lower()):
                    if int(value):
                        setattr(self,  key.lower(), int(value))
                    else:
                        setattr(self,  key.lower(), value)
        except Exception as e:
            raise PyEngineError("CONFIG_LOAD_FAILED") from e

    def start_tkinter_loop(self):
        self.tk_root.mainloop()

    def run(self):
        self.publish_signal(Signals.APP_START)
        self.running = True
        while self.running:
            self.publish_signal(Signals.APP_UPDATE)
            self.update_inputs()
            self.consume_signals()

            self.tk_root.update_idletasks()
            self.tk_root.update()

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
        self.signal_bus.publish(signal, message)

    def print_running_threads(self):
        threads = threading.enumerate()
        self.logger.info(f"Total running threads: {len(threads)}")
        for thread in threads:
            self.logger.info(f"{thread.name} (Daemon: {thread.daemon})")

    def launch_engine(self, channel: str, message: str):
        launch_rupy_engine(
        )

    def shutdown(self):
        self.publish_signal(Signals.APP_SHUTDOWN)
        self.running = False
