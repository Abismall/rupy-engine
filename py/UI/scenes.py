import os
import tkinter as tk
from tkinter import ttk

import yaml

from Application.signal import SignalBus, Signals
from PIL import Image, ImageTk

from Utils.constants import SCENE_MENU


def load_scene_files(directory_path):

    scenes = []

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
                                scenes.append({
                                    "title": title,
                                    "photo_path": photo_path
                                })
                            else:
                                print(f"Warning: Photo file not found for scene '{
                                      title}' in {scene_folder}")

                    except yaml.YAMLError as e:
                        print(f"Error loading {filename}: {e}")
                    except FileNotFoundError:
                        print(f"File not found: {file_path}")
                    except Exception as e:
                        print(f"Unexpected error reading {filename}: {e}")

    return scenes


class SceneFilesMenu(tk.Frame):
    name = SCENE_MENU

    def __init__(self, parent: tk.Tk, signal_bus: SignalBus, scene_dir: str):
        super().__init__(parent)
        self.signal_bus = signal_bus
        self.scenes = load_scene_files(os.path.abspath(scene_dir))
        self.configure(background="#2b2b2b")

        self.notebook = ttk.Notebook(self)
        self.notebook.pack(expand=True, fill="both")

        self.setup_scene_tab()

    def setup_scene_tab(self):
        scenes_tab = ttk.Frame(self.notebook)
        self.notebook.add(scenes_tab, text="Scenes")

        canvas = tk.Canvas(scenes_tab, background="#2b2b2b")
        scrollbar = ttk.Scrollbar(
            scenes_tab, orient="vertical", command=canvas.yview)
        scrollable_frame = ttk.Frame(canvas)

        scrollable_frame.bind(
            "<Configure>",
            lambda e: canvas.configure(scrollregion=canvas.bbox("all"))
        )

        canvas.create_window((0, 0), window=scrollable_frame, anchor="nw")
        canvas.configure(yscrollcommand=scrollbar.set)

        canvas.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")

        for scene in self.scenes:
            self.display_scene(
                scrollable_frame, scene['title'], scene['photo_path'])

    def display_scene(self, parent_frame, title, photo_path):

        try:
            img = Image.open(photo_path)
            img.thumbnail((150, 150))
            img = ImageTk.PhotoImage(img)
            label = ttk.Label(parent_frame, image=img,
                              background="#2b2b2b", text=title)
            label.image = img
            label.pack(anchor="w", pady=5)
        except Exception as e:
            ttk.Label(parent_frame, text=f"Failed to load image: {
                e}", foreground="red", background="#2b2b2b").pack(anchor="w")

    def on_closing(self):
        self.signal_bus.publish(Signals.MENU_CLOSE, SCENE_MENU)
