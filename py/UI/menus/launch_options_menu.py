import tkinter as tk
from tkinter import ttk
from UI.components.button import ButtonElement
from UI.components.label import LabelElement
from UI.components.entry import EntryElement
from Utils.environment import EnvManager
from Application.signal import SignalBus, Signals
from Utils.constants import LAUNCH_OPTIONS_MENU
from Utils.strings import convert_underscore_to_title
from Error.base import PyEngineError


class LaunchOptionsMenu:
    name = LAUNCH_OPTIONS_MENU

    def __init__(self, root, signal_bus: SignalBus, env_file_path: str = ".env"):
        self.root = root
        self.signal_bus = signal_bus
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
            self.frame, text="Save configurations", command=self.save_config
        )
        self.save_button.pack(pady=5)
        self.launch_button = ButtonElement(
            self.frame, text="Launch", command=self.launch_engine
        )
        self.launch_button.pack(pady=5)
        self.close_button = ButtonElement(
            self.frame, text=f"Close {convert_underscore_to_title(self.name)}", command=self.publish_close_signal
        )
        self.close_button.pack(pady=10)

        LabelElement(
            self.frame, text="Current configurations:").pack(pady=15)

        self.create_env_entries()

    def __str__(self) -> str:
        return self.name

    def display_error(self, message: str):
        self.error_label.set_config(text=message)

    def clear_error(self):
        self.error_label.set_config(text="")

    def display_success(self, message: str):
        self.success_label.set_config(text=message)

    def clear_success(self):
        self.success_label.set_config(text="")

    def clear_status_messages(self):
        self.clear_error()
        self.clear_success()

    def load_env_variables(self):
        try:
            self.clear_status_messages()
            self.env_vars = EnvManager.get_dotenv_values(self.env_file_path)
        except FileNotFoundError:
            self.display_error("The environment file was not found.")
            raise PyEngineError("FILE_NOT_FOUND")
        except PermissionError:
            self.display_error(
                "Permission denied when accessing the environment file.")
            raise PyEngineError("PERMISSION_DENIED")
        except ValueError:
            self.display_error(
                "Invalid value found in the environment variables.")
            raise PyEngineError("VALUE_ERROR")
        except Exception as e:
            self.display_error(f"Failed to load environment variables: {e}")
            raise PyEngineError("UNKNOWN_ERROR") from e

    def create_env_entries(self):
        try:
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
        except Exception as e:
            self.display_error(f"Failed to create environment entries: {e}")
            raise PyEngineError("UI_ERROR") from e

    def save_config(self):
        try:
            self.clear_status_messages()
            new_entries = {key: entry.element.get()
                           for key, entry in self.env_entries.items()}
            EnvManager.set_env_keys(self.env_file_path, new_entries)
            self.signal_bus.publish(
                channel=Signals.ENV_RELOAD.value, message=self.name)
            success_message = "Environment variables updated successfully."
            self.display_success(success_message)

        except FileNotFoundError:
            self.display_error("The environment file was not found.")
            raise PyEngineError("FILE_NOT_FOUND")
        except PermissionError:
            self.display_error(
                "Permission denied when writing to the environment file.")
            raise PyEngineError("PERMISSION_DENIED")
        except ValueError:
            self.display_error(
                "Invalid value provided in the environment variables.")
            raise PyEngineError("VALUE_ERROR")
        except Exception as e:
            error_message = "Failed to save environment variables."
            self.display_error(error_message)
            raise PyEngineError("UNKNOWN_ERROR") from e

    def launch_engine(self):
        try:
            pass
        except Exception as e:
            self.display_error("Failed to launch the engine.")
            raise PyEngineError("UNKNOWN_ERROR") from e

    def publish_close_signal(self):
        try:
            self.signal_bus.publish(
                channel=Signals.MENU_CLOSE.value, message=self.name)
        except Exception as e:
            self.display_error("Failed to publish close signal.")
            raise PyEngineError("UNKNOWN_ERROR") from e
