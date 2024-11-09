
from tkinter import ttk
import tkinter as tk
from Application.engine import launch_rupy_engine
from Application.signal import SignalBus, Signals
from Error.base import PyEngineError

from Utils.constants import LAUNCHER_MENU
from Utils.environment import EnvManager


class LaunchOptionsMenu(tk.Frame):
    name = LAUNCHER_MENU

    def __init__(self, parent: tk.Tk, signal_bus: SignalBus, env_file_path: str = ".env"):
        super().__init__(parent)
        self.signal_bus = signal_bus
        self.env_file_path = env_file_path
        self.env_vars = {}
        self.is_rust_window_open = False
        self.configure(background="#2b2b2b")

        self.notebook = ttk.Notebook(self)
        self.notebook.pack(expand=True, fill="both")

        self.setup_main_tab()
        self.setup_cfg_tab()

    def setup_main_tab(self):
        main_tab = ttk.Frame(self.notebook)
        self.notebook.add(main_tab, text="Main")

        self.launch_button = ttk.Button(
            main_tab, text="Launch", command=self.toggle_rust_window,
        )
        self.launch_button.pack(pady=5)

    def setup_cfg_tab(self):
        self.load_env_variables()
        settings_tab = ttk.Frame(self.notebook)
        self.notebook.add(settings_tab, text="Cfg")

        for key, value in self.env_vars.items():
            ttk.Label(settings_tab, text=key, foreground="white",
                      background="#2b2b2b").pack(anchor="w")
            entry = ttk.Entry(settings_tab)
            entry.pack(anchor="w", pady=2)
            entry.insert(0, value or "")
            self.env_vars[key] = entry

    def load_env_variables(self):
        try:
            self.env_vars = EnvManager.get_dotenv_values(self.env_file_path)
        except Exception as e:
            ttk.Label(self, text=f"Failed to load environment variables: {
                e}", background="red").pack()

    def toggle_rust_window(self):
        if not self.is_rust_window_open:
            self.launch_rust_engine()
            self.launch_button.config(text="Close")
        else:
            self.close_rust_engine()
            self.launch_button.config(text="Launch")

    def launch_rust_engine(self):
        try:
            self.is_rust_window_open = True
            launch_rupy_engine()
        except PyEngineError as e:
            print(f"Failed to launch engine: {e}")

    def close_rust_engine(self):
        if self.is_rust_window_open:
            self.is_rust_window_open = False

    def on_closing(self):
        self.signal_bus.publish(Signals.MENU_CLOSE, LAUNCHER_MENU)
