from tkinter import ttk
from Utils.environment import EnvManager
from Utils.constants import LAUNCHER_MENU
from Application.signal import SignalBus, Signals
import tkinter as tk
import asyncio
import os
from pathlib import Path
import subprocess
import threading
import platform
from Error.base import PyEngineError


def resolve_launch_file_path():
    file_name = "run.bat" if platform.system(
    ).lower().startswith("windows") else "run.sh"

    current_dir = Path(__file__).resolve().parent

    for parent in current_dir.parents:
        potential_ru_dir = parent / "ru"
        if potential_ru_dir.is_dir():
            script_path = potential_ru_dir / file_name
            return str(script_path.resolve())

    raise PyEngineError("RU_DIRECTORY_NOT_FOUND")


def launch_rust_process(script_path):
    try:
        DETACHED_PROCESS = 0x00000008  # Windows-specific flag
        process = subprocess.Popen(
            ["cmd.exe", "/c", script_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            shell=True,
            cwd=os.path.dirname(script_path),
            creationflags=DETACHED_PROCESS  # Detach the process
        )
        print(f"Rust process started with PID: {process.pid}")
    except Exception as e:
        print(f"Failed to launch Rust process: {e}")


def rust_process_runner(script_path):
    try:
        process = subprocess.Popen(
            ["cmd.exe", "/c", script_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            shell=True,
            cwd=os.path.dirname(script_path)
        )
        process.wait()
    except Exception as e:
        print(f"Error running Rust process: {e}")


async def launch_rust_process(script_path):
    try:
        process = await asyncio.create_subprocess_exec(
            "cmd.exe", "/c", script_path,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
            cwd=os.path.dirname(script_path),
            shell=True
        )

        async for line in process.stdout:
            print(f"[Rust]: {line.decode().strip()}")

        await process.wait()
    except Exception as e:
        print(f"Failed to run Rust process: {e}")


def run_rust_engine(script_path):
    asyncio.run(launch_rust_process(script_path))


def start_and_monitor_process(script_path):
    try:
        process = subprocess.Popen(
            ["cmd.exe", "/c", script_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            shell=True,
            cwd=os.path.dirname(script_path)
        )
        monitor_process(process)
    except Exception as e:
        print(f"Error in subprocess: {e}")


def monitor_process(process: subprocess):
    process.wait()


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

    def on_closing(self):
        self.signal_bus.publish(Signals.MENU_CLOSE, LAUNCHER_MENU)
