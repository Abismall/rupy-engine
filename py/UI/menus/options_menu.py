import logging
import tkinter as tk
from tkinter import ttk
from typing import Callable, Optional
from UI.components.button import ButtonElement
from UI.components.label import LabelElement
from UI.components.entry import EntryElement
from Utils.environment import EnvManager
from Error.base import StatusText, create_error


class OptionsMenu:
    def __init__(self, root, close_menu_callback, env_file_path: str = ".env", logger: Optional[logging.Logger] = None):
        self.root = root
        self.logger = logger
        self.set_close_menu_callback(close_menu_callback)
        self.frame = tk.Frame(self.root)
        self.frame.pack(padx=5, pady=5)

        self.env_file_path = env_file_path
        self.env_vars = {}

        self.error_label = LabelElement(self.frame, text="", fg="red")
        self.error_label.pack(pady=5)
        self.error_label.element.config(wraplength=350)

        self.success_label = LabelElement(self.frame, text="", fg="green")
        self.success_label.pack(pady=5)
        self.success_label.element.config(wraplength=350)

        self.load_env_variables()

        self.save_button = ButtonElement(
            self.frame, text="Save Configuration", command=self.save_config
        )
        self.save_button.pack(pady=5)
        self.launch_button = ButtonElement(
            self.frame, text="Launch Engine", command=self.launch_engine
        )
        self.launch_button.pack(pady=5)
        self.close_button = ButtonElement(
            self.frame, text="Close", command=self.close_menu_callback
        )
        self.close_button.pack(pady=10)

        LabelElement(self.frame, text="Environment Variables:").pack(pady=15)

        self.create_env_entries()

    def display_error(self, message: str):
        """Displays an error message in the options menu."""
        self.error_label.set_config(text=message)

    def clear_error(self):
        """Clears the displayed error message."""
        self.error_label.set_config(text="")

    def display_success(self, message: str):
        """Displays an success message in the options menu."""
        self.success_label.set_config(text=message)

    def clear_success(self):
        """Clears the displayed success message."""
        self.success_label.set_config(text="")

    def clear_status_messages(self):
        self.clear_error()
        self.clear_success()

    def load_env_variables(self):
        """Loads environment variables from the specified file."""
        try:
            self.clear_status_messages()
            self.env_vars = EnvManager.get_env_variables(self.env_file_path)
            if self.logger:
                self.logger.info(f"Environment variables loaded from {
                    self.env_file_path}.")
            self.clear_error()
        except Exception as e:
            error_message = f"Failed to load environment variables: {e}"
            self.display_error(error_message)

    def create_env_entries(self):
        """Creates entry fields for each environment variable."""
        self.env_entries = {}

        canvas = tk.Canvas(self.frame)
        scrollbar = ttk.Scrollbar(
            self.frame, orient="vertical", command=canvas.yview)
        scrollable_frame = ttk.Frame(canvas)

        scrollable_frame.bind(
            "<Configure>",
            lambda e: canvas.configure(
                scrollregion=canvas.bbox("all")
            )
        )

        canvas.create_window((0, 0), window=scrollable_frame, anchor="nw")
        canvas.configure(yscrollcommand=scrollbar.set)

        canvas.pack(side="left", fill="both", expand=True)
        scrollbar.pack(side="right", fill="y")

        for key, value in self.env_vars.items():
            LabelElement(scrollable_frame, text=key).pack(anchor="w")
            entry = EntryElement(scrollable_frame)
            entry.set_config(width=50)
            entry.pack(anchor="w", pady=2)
            entry.element.insert(0, value or "")
            self.env_entries[key] = entry

    def save_config(self):
        """Saves the modified environment variables using EnvManager."""
        try:
            self.clear_status_messages()
            EnvManager.set_env_keys(self.env_file_path, self.env_entries)
            success_message = "Environment variables updated successfully."
            if self.logger:
                self.logger.info(success_message)
            self.display_success(success_message)
        except Exception as e:
            error_message = f"Failed to save environment variables: {e}"
            self.display_error(error_message)

    def launch_engine(self):
        if self.logger:
            self.logger.info("Launching engine...")

    def set_close_menu_callback(self, callback: Callable, on_error_message="close menu callback is None or is not a callable method"):
        if callback and callable(callback):
            self.close_menu_callback = callback
        else:
            if self.logger:
                self.logger.error(
                    on_error_message)
            raise TypeError(create_error(status=StatusText.TYPE_ERROR,
                                         message=f"Error: {on_error_message}"))
