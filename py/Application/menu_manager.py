import threading
import time
from UI.menus.launch_options_menu import LaunchOptionsMenu
from constants import LAUNCH_OPTIONS_MENU_GEOMETRY, LAUNCH_OPTIONS_MENU
from Utils.strings import convert_underscore_to_title
from Error.base import Status, create_error
from .signal import Signals, SignalBus
from typing import Any, Callable, Dict
import tkinter as tk


class MenuThread(threading.Thread):
    def __init__(self, target: Callable, *args, **kwargs):
        """
        Initializes a new thread to run the menu target function.

        Args:
            target (Callable): The target function to be run in the thread.
            *args: Additional positional arguments.
            **kwargs: Additional keyword arguments.
        """
        super().__init__(*args, **kwargs)
        self._stop_event = threading.Event()
        self._target_function = target

    def run(self):
        """
        Runs the target function repeatedly until the stop event is set.
        """
        while not self._stop_event.is_set():
            self._target_function()
            time.sleep(0.1)

    def stop(self):
        """
        Sets the stop event, signaling the thread to stop running.
        """
        self._stop_event.set()


class MenuManager:
    def __init__(self, root: tk.Tk, signal_bus: SignalBus, env_file_path: str = ".env"):
        """
        Initializes the MenuManager to manage menu states and signals.

        Args:
            root (tk.Tk): The root Tkinter window.
            signal_bus (SignalBus): The signal bus for handling inter-component signals.
            env_file_path (str): The path to the environment configuration file.
        """
        self.root = root
        self.signal_bus = signal_bus
        self.env_file_path = env_file_path
        self.menus: Dict[str, Dict[str, Any]] = {}

        self.subscribe_signals()

    def subscribe_signals(self):
        """
        Subscribes to signal events for opening and closing menus.
        """
        self.signal_bus.subscribe(
            Signals.MENU_OPEN.value, self.handle_menu_open)
        self.signal_bus.subscribe(
            Signals.MENU_CLOSE.value, self.handle_menu_close)

    def handle_menu_open(self, channel: str, menu_name: str):
        """
        Handles the signal to open a menu.

        Args:
            channel (str): The signal channel.
            menu_name (str): The name of the menu to open.
        """
        if menu_name not in self.menus or not self.menus[menu_name]["open"]:
            self.open_menu(menu_name)

    def handle_menu_close(self, channel: str, menu_name: str):
        """
        Handles the signal to close a menu.

        Args:
            channel (str): The signal channel.
            menu_name (str): The name of the menu to close.

        Raises:
            ChildProcessError: If the menu thread cannot be stopped.
        """
        if menu_name in self.menus and self.menus[menu_name]["open"]:
            self.menus[menu_name]["open"] = False
            if hasattr(self.menus[menu_name]["thread"], "stop") and callable(self.menus[menu_name]["thread"].stop):
                self.menus[menu_name]["thread"].stop()
            else:
                raise ChildProcessError(create_error(
                    status=Status.ChildProcessError,
                    details=f"Unable to 'stop()' thread for {
                        menu_name}, attribute is missing or is not callable.",
                    trace=True
                ))

    def open_menu(self, menu_name: str):
        """
        Opens the specified menu.

        Args:
            menu_name (str): The name of the menu to open.
        """
        if menu_name == LAUNCH_OPTIONS_MENU:
            self.launch_menu_thread(menu_name, self.run_options_menu)

    def launch_menu_thread(self, menu_name: str, target_function: Callable):
        """
        Launches a new thread to run the target function for the specified menu.

        Args:
            menu_name (str): The name of the menu.
            target_function (Callable): The function to run in the thread.

        Raises:
            ChildProcessError: If the thread cannot be started.
        """
        thread = MenuThread(target=target_function, daemon=True)
        self.menus.update({menu_name: {
            "open": True,
            "thread": thread
        }})
        if hasattr(self.menus[menu_name]["thread"], "start") and callable(self.menus[menu_name]["thread"].start):
            self.menus[menu_name]["thread"].start()
        else:
            raise ChildProcessError(create_error(
                status=Status.ChildProcessError,
                details=f"Unable to call 'start()' on thread for {
                    menu_name}, attribute is missing or is not callable.",
                trace=True
            ))

    def run_options_menu(self):
        """
        Runs the launch options menu, updating the GUI until the menu is closed.
        """
        tk_root = tk.Tk()
        tk_root.title(convert_underscore_to_title(LAUNCH_OPTIONS_MENU))
        tk_root.geometry(LAUNCH_OPTIONS_MENU_GEOMETRY)

        tk_root.protocol("WM_DELETE_WINDOW", lambda: self.signal_bus.publish(
            Signals.MENU_CLOSE.value, LAUNCH_OPTIONS_MENU))

        _ = LaunchOptionsMenu(
            root=tk_root, signal_bus=self.signal_bus, env_file_path=self.env_file_path)

        while self.menus[LAUNCH_OPTIONS_MENU]["open"]:
            try:
                tk_root.update_idletasks()
                tk_root.update()
                time.sleep(0.1)
            except tk.TclError:
                break

        tk_root.destroy()

    def close_all_menus(self):
        """
        Closes all open menus by publishing a close signal for each.
        """
        for menu_name in self.menus.keys():
            self.signal_bus.publish(Signals.MENU_CLOSE.value, menu_name)
