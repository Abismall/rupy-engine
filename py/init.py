import os
import sys
import subprocess
from typing import Any, Dict
from Utils.constants import PYPROJECT, PYENGINE
from Utils.files import activate_virtual_environment, create_virtual_environment
from Error.base import PyEngineError

PYTHON_EXEC = sys.executable

PROJECT_ROOT_DIR = os.path.abspath(os.path.dirname(__file__))
VENV_DIR = os.path.join(PROJECT_ROOT_DIR, 'venv')


def find_project_root(starting_path: str) -> str:
    current_dir = os.path.abspath(starting_path)
    try:
        while current_dir != os.path.dirname(current_dir):
            print(os.path.exists(os.path.join(current_dir, "py")))
            print(os.path.exists(os.path.join(current_dir, PYPROJECT)))
            if os.path.exists(os.path.join(current_dir, "py")):
                return os.path.join(current_dir, "py")
            if os.path.exists(os.path.join(current_dir, PYPROJECT)):
                return current_dir
            current_dir = os.path.dirname(current_dir)
            print(current_dir)
        raise FileNotFoundError(
            f"Could not find project root containing {PYPROJECT}")
    except FileNotFoundError as e:
        raise PyEngineError("FILE_NOT_FOUND") from e
    except Exception as e:
        raise PyEngineError("UNKNOWN_ERROR") from e


def run_command(command: list, error_message: str) -> bool:
    try:
        subprocess.run(command, check=True)
        return True
    except subprocess.CalledProcessError:
        print(error_message)
        raise PyEngineError("CHILD_PROCESS_ERROR")


def load_project_config(root_dir: str, project: str) -> Dict[str, Any]:
    config_path = os.path.join(root_dir, project)
    config = {}
    try:
        with open(config_path, 'r') as file:
            current_section = None
            for line in file:
                line = line.strip()
                if line.startswith('[') and line.endswith(']'):
                    current_section = line[1:-1]
                    config[current_section] = {}
                elif '=' in line and current_section:
                    key, value = line.split('=', 1)
                    key = key.strip()
                    value = value.strip().strip('"').strip("'")
                    config[current_section][key] = value
        return config
    except FileNotFoundError:
        raise PyEngineError("FILE_NOT_FOUND")
    except PermissionError:
        raise PyEngineError("PERMISSION_DENIED")
    except Exception as e:
        raise PyEngineError("UNKNOWN_ERROR") from e


def load_min_python_version(cfg: Dict[str, Any], pyengine: str) -> str:
    try:
        return cfg.get(pyengine, {}).get('min_python_version_requirement', '3.8')
    except Exception as e:
        raise PyEngineError("CONFIG_LOAD_FAILED") from e


def create_dotenv(cfg: Dict[str, Any], pyengine: str):
    try:
        with open(".env", "w") as f:
            for key, value in cfg.get(pyengine, {}).items():
                f.write(f"{key.upper()}={value.lower() if isinstance(
                    value, str) else str(value).lower()}\n")
    except PermissionError:
        raise PyEngineError("PERMISSION_DENIED")
    except Exception as e:
        raise PyEngineError("UNKNOWN_ERROR") from e


def check_python_version(min_version: str):
    try:
        required_version = tuple(map(int, min_version.split('.')))
        if sys.version_info < required_version:
            raise PyEngineError("VALUE_ERROR", f"Python {
                                min_version} or higher is required.")
    except ValueError:
        raise PyEngineError("VALUE_ERROR")
    except Exception as e:
        raise PyEngineError("UNKNOWN_ERROR") from e


def install_dependencies(requirements_file: str):
    try:
        if not run_command([sys.executable, "-m", "pip", "install", "--upgrade", "pip"], "Failed to upgrade pip."):
            raise PyEngineError("CHILD_PROCESS_ERROR")
        if os.path.isfile(requirements_file):
            if not run_command([sys.executable, "-m", "pip", "install", "-r", requirements_file],
                               "Failed to install dependencies from requirements.txt."):
                raise PyEngineError("CHILD_PROCESS_ERROR")
        else:
            raise PyEngineError("FILE_NOT_FOUND")
    except FileNotFoundError:
        raise PyEngineError("FILE_NOT_FOUND")
    except PermissionError:
        raise PyEngineError("PERMISSION_DENIED")
    except Exception as e:
        raise PyEngineError("UNKNOWN_ERROR") from e


def start_application(args, main_script: str):
    try:
        if not run_command([sys.executable, main_script] + args, "The application encountered an error during execution."):
            raise PyEngineError("CHILD_PROCESS_ERROR")
    except Exception as e:
        raise PyEngineError("UNKNOWN_ERROR") from e


def main():
    global PROJECT_ROOT_DIR, VENV_DIR

    try:
        PROJECT_ROOT_DIR = find_project_root(__file__)
        VENV_DIR = os.path.join(PROJECT_ROOT_DIR, 'venv')

        requirements = os.path.join(PROJECT_ROOT_DIR, 'requirements.txt')
        config = load_project_config(PROJECT_ROOT_DIR, PYPROJECT)
        main_script = os.path.join(PROJECT_ROOT_DIR, "main.py")
        min_python_version = load_min_python_version(config, PYENGINE)

        # create_virtual_environment(VENV_DIR, PYTHON_EXEC)
        # activate_virtual_environment(VENV_DIR)

        check_python_version(min_python_version)
        # install_dependencies(requirements)
        create_dotenv(config, PYENGINE)
        start_application(sys.argv[1:], main_script)
    except PyEngineError as e:
        print(f"Error: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Unexpected Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
