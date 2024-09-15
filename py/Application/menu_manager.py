import threading
import time
from UI.menus.launch_options_menu import LaunchOptionsMenu
from Utils.strings import convert_underscore_to_title
from Error.base import PyEngineError
from Utils.constants import LAUNCH_OPTIONS_MENU
from Utils.constants import LAUNCH_OPTIONS_MENU_GEOMETRY
from .signal import Signals, SignalBus
from typing import Any, Callable, Dict
import tkinter as tk


class MenuThread(threading.Thread):
    def __init__(self, target: Callable, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._stop_event = threading.Event()
        self._target_function = target

    def run(self):
        try:
            while not self._stop_event.is_set():
                self._target_function()
                time.sleep(0.1)
        except Exception as e:
            # Log error and raise a general threading error
            raise PyEngineError("THREADING_ERROR") from e

    def stop(self):
        self._stop_event.set()


class MenuManager:
    def __init__(self, root: tk.Tk, signal_bus: SignalBus, env_file_path: str = ".env"):
        self.root = root
        self.signal_bus = signal_bus
        self.env_file_path = env_file_path
        self.menus: Dict[str, Dict[str, Any]] = {}

        self.subscribe_signals()

    def subscribe_signals(self):
        try:

            self.signal_bus.subscribe(
                Signals.MENU_OPEN.value, self.handle_menu_open)
            self.signal_bus.subscribe(
                Signals.MENU_CLOSE.value, self.handle_menu_close)
        except Exception as e:
            raise PyEngineError("CONFIG_LOAD_FAILED") from e

    def handle_menu_open(self, channel: str, menu_name: str):
        try:
            if menu_name not in self.menus or not self.menus[menu_name]["open"]:
                self.open_menu(menu_name)
        except KeyError:
            raise PyEngineError("RESOURCE_NOT_AVAILABLE")
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    def handle_menu_close(self, channel: str, menu_name: str):
        try:
            if menu_name in self.menus and self.menus[menu_name]["open"]:
                self.menus[menu_name]["open"] = False
                if hasattr(self.menus[menu_name]["thread"], "stop") and callable(self.menus[menu_name]["thread"].stop):
                    self.menus[menu_name]["thread"].stop()
                else:
                    raise PyEngineError("CHILD_PROCESS_ERROR")
        except KeyError:
            raise PyEngineError("RESOURCE_NOT_AVAILABLE")
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    def open_menu(self, menu_name: str):
        try:
            if menu_name == LAUNCH_OPTIONS_MENU:
                self.launch_menu_thread(menu_name, self.run_options_menu)
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    def launch_menu_thread(self, menu_name: str, target_function: Callable):
        try:
            thread = MenuThread(target=target_function, daemon=True)
            self.menus.update({
                menu_name: {
                    "open": True,
                    "thread": thread
                }
            })
            if hasattr(self.menus[menu_name]["thread"], "start") and callable(self.menus[menu_name]["thread"].start):
                self.menus[menu_name]["thread"].start()
            else:
                raise PyEngineError("CHILD_PROCESS_ERROR")
        except Exception as e:
            raise PyEngineError("THREADING_ERROR") from e

    def run_options_menu(self):
        try:
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
        except Exception as e:
            raise PyEngineError("UI_ERROR") from e
        finally:
            tk_root.destroy()

    def close_all_menus(self):
        try:
            for menu_name in self.menus.keys():
                self.signal_bus.publish(Signals.MENU_CLOSE.value, menu_name)
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e
