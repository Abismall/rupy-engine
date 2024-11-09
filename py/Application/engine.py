import os
from pathlib import Path
import subprocess
import threading
import platform
from Error.base import PyEngineError
from Application.signal import SignalBus, Signals
from Utils.constants import LAUNCHER_MENU


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


def launch_rupy_engine(width=800, height=600, x=300, y=100):
    try:
        script_path = resolve_launch_file_path()
        os.environ["RUPY_ENGINE_WINDOW_WIDTH"] = str(width)
        os.environ["RUPY_ENGINE_WINDOW_HEIGHT"] = str(height)
        os.environ["RUPY_ENGINE_WINDOW_X_ANCHOR"] = str(x)
        os.environ["RUPY_ENGINE_WINDOW_Y_ANCHOR"] = str(y)
        process = subprocess.Popen(
            ["cmd.exe", "/c", script_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            shell=True,
            cwd=os.path.dirname(script_path)
        )

        thread = threading.Thread(
            target=monitor_process, args=([process]))
        thread.start()
    except FileNotFoundError:
        print("Engine script not found.")
        raise PyEngineError("FILE_NOT_FOUND")
    except PermissionError:
        print("Permission denied when trying to launch the engine.")
        raise PyEngineError("PERMISSION_DENIED")
    except Exception as e:
        print("Failed to launch the engine.")
        raise PyEngineError("UNKNOWN_ERROR") from e


def monitor_process(process: subprocess):
    process.wait()
