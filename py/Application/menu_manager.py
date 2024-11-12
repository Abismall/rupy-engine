import time
from typing import Callable
import yaml
import os
import tkinter as tk
from tkinter import ttk
from queue import Queue
import threading
import yaml
from Error.base import PyEngineError
from Utils.constants import LAUNCHER_MENU, SCENE_MENU
from UI.launcher import LaunchOptionsMenu
from UI.scenes import SceneFilesMenu
from .signal import Signals, SignalBus


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
            raise PyEngineError("THREADING_ERROR") from e

    def stop(self):
        self._stop_event.set()


class MenuManager:
    def __init__(self, root: tk.Tk, signal_bus: SignalBus, scenes_directory: str = "./ru/static/scenes", env_file_path=".env"):
        self.root = root
        self.signal_bus = signal_bus
        self.scenes_directory = scenes_directory
        self.env_file_path = env_file_path
        self.menus = {}
        self.scene_queue = Queue()

        self.notebook = ttk.Notebook(self.root)
        self.notebook.grid(row=0, column=0, sticky="nsew")

        self.launch_options_menu = LaunchOptionsMenu(
            self.notebook, signal_bus, env_file_path)
        self.scene_files_menu = SceneFilesMenu(
            self.notebook, signal_bus, scenes_directory)

        self.notebook.add(self.launch_options_menu, text="Launcher")
        self.notebook.add(self.scene_files_menu, text="Scenes")

        self.subscribe_signals()
        self.check_scene_queue()

    def subscribe_signals(self):
        self.signal_bus.subscribe(Signals.MENU_OPEN, self.handle_menu_open)
        self.signal_bus.subscribe(Signals.MENU_CLOSE, self.handle_menu_close)

    def handle_menu_open(self, channel: str, menu_name: str):
        if menu_name == LAUNCHER_MENU:
            self.show_menu_tab(self.launch_options_menu)
        elif menu_name == SCENE_MENU:
            self.show_menu_tab(self.scene_files_menu)

    def handle_menu_close(self, channel: str, menu_name: str):
        if menu_name == LAUNCHER_MENU:
            self.notebook.hide(self.launch_options_menu)
        elif menu_name == SCENE_MENU:
            self.notebook.hide(self.scene_files_menu)

    def show_menu_tab(self, menu_frame):
        index = self.notebook.index(menu_frame)
        self.notebook.select(index)

    def check_scene_queue(self):
        if not self.scene_queue.empty():
            self.scene_files_menu.scenes = self.scene_queue.get_nowait()

        self.root.after(100, self.check_scene_queue)

    def load_scene_files(self, directory_path: str):
        scenes = []
        if not os.path.exists(directory_path):
            print(f"Directory not found: {directory_path}")
            return

        for scene_folder in os.listdir(directory_path):
            scene_path = os.path.join(directory_path, scene_folder)
            if os.path.isdir(scene_path):
                for filename in os.listdir(scene_path):
                    if filename.endswith(".rupy"):
                        file_path = os.path.join(scene_path, filename)
                        try:
                            with open(file_path, 'r') as file:
                                scene_data = yaml.safe_load(file)
                                title = scene_data.get("name", "Unnamed Scene")
                                photo_path = os.path.join(
                                    scene_path, scene_data.get("photo", ""))
                                if os.path.exists(photo_path):
                                    scenes.append(
                                        {"title": title, "photo_path": photo_path})
                                else:
                                    print(f"Warning: Photo not found for scene '{
                                          title}' in {scene_folder}")
                        except Exception as e:
                            print(f"Error loading {filename}: {e}")

        self.scene_queue.put(scenes)
