import os
import sys
import subprocess
import toml
from typing import Any
from tqdm import tqdm
from colorama import Fore, Style, init

from constants import PYENGINE, PYPROJECT, PYPROJECT_TOOL

init(autoreset=True)
PYTHON_EXEC = sys.executable


def load_project_config() -> dict[str, Any]:
    try:
        print(f"{Fore.CYAN}Loading project configuration...")
        config = toml.load(os.path.join(os.path.dirname(__file__), PYPROJECT))
        print(f"{Fore.GREEN}Configuration loaded successfully.")
        return config
    except Exception as e:
        print(f"{Fore.RED}Error loading configuration: {e}")
        sys.exit(1)


def load_project_root_dir(cfg: dict[str, Any]):
    return cfg.get(PYPROJECT_TOOL, {}).get(PYENGINE, {}).get('root_dir', None)


def load_project_venv_dir(cfg: dict[str, Any]):
    return cfg.get(PYPROJECT_TOOL, {}).get(PYENGINE, {}).get('venv_dir', None)


def load_requirements_file(cfg: dict[str, Any]):
    return cfg.get(PYPROJECT_TOOL, {}).get(PYENGINE, {}).get('path_to_requirements_file', 'requirements.txt')


def load_min_python_version(cfg: dict[str, Any]):
    return cfg.get(PYPROJECT_TOOL, {}).get(PYENGINE, {}).get('min_python_version_requirement', '3.8')


def create_dotenv(cfg: dict[str, Any]):
    env_vars = cfg.get(PYPROJECT_TOOL, {}).get(PYENGINE, {})
    with open(".env", "w") as f:
        for key, value in env_vars.items():
            f.write(f"{key.upper()}={value.lower() if isinstance(
                value, str) else str(value).lower()}\n")
    print(f"{Fore.GREEN}.env file created with environment variables.")


def check_python_version(min_version: str):
    print(f"{Fore.CYAN}Checking Python version requirement...")
    required_version = tuple(map(int, min_version.split('.')))
    if sys.version_info < required_version:
        print(f"{Fore.RED}Error: Python {min_version} or higher is required.")
        sys.exit(1)
    print(f"{Fore.GREEN}Python version {sys.version_info.major}.{
          sys.version_info.minor} is compatible.")


def create_virtual_environment(venv_path: str):
    if not os.path.isdir(venv_path):
        print(f"{Fore.CYAN}Creating virtual environment...")
        if not run_command([PYTHON_EXEC, "-m", "venv", venv_path], "Failed to create the virtual environment."):
            sys.exit(1)
        print(f"{Fore.GREEN}Virtual environment created at {venv_path}")


def activate_virtual_environment(venv_path: str):
    print(f"{Fore.CYAN}Activating virtual environment...")
    if os.name == 'nt':  # Windows
        activate_script = os.path.join(venv_path, "Scripts", "activate")
    else:  # Unix-based systems
        activate_script = os.path.join(venv_path, "bin", "activate")

    activate_command = f"source {activate_script}"
    print(f"{Fore.YELLOW}({activate_command})")

    if os.name == 'nt':
        subprocess.call(activate_script, shell=True)
    else:
        os.system(f"source {activate_script}")


def install_dependencies(requirements_file: str):
    print(f"{Fore.CYAN}Upgrading pip...")
    if not run_command([sys.executable, "-m", "pip", "install", "--upgrade", "pip"], "Failed to upgrade pip."):
        sys.exit(1)
    requirements_file_path = os.path.join(
        os.path.dirname(__file__), requirements_file)
    if os.path.isfile(requirements_file_path):
        print(f"{Fore.CYAN}Installing Python dependencies from {
              requirements_file_path}...")
        with tqdm(total=100, desc="Installing dependencies", bar_format="{l_bar}{bar} [{elapsed}]") as pbar:
            if not run_command(
                [sys.executable, "-m", "pip", "install",
                    "-r", requirements_file_path],
                "Failed to install dependencies from requirements.txt."
            ):
                sys.exit(1)
            pbar.update(100)
        print(f"{Fore.GREEN}Dependencies installed successfully.")
    else:
        print(f"{Fore.RED}Error: {
              requirements_file_path} not found. Please provide a requirements file.")
        sys.exit(1)


def run_command(command, error_message):
    try:
        subprocess.run(command, check=True)
        return True
    except subprocess.CalledProcessError:
        print(f"{Fore.RED}Error: {error_message}")
        return False


def start_application(args):
    main_script = os.path.join(os.path.dirname(__file__), "main.py")
    print(f"{Fore.CYAN}Starting the application...")
    if not run_command([sys.executable, main_script] + args, "The application encountered an error during execution."):
        sys.exit(1)
    print(f"{Fore.GREEN}Application started successfully.")


def main():
    print(f"{Fore.BLUE}{Style.BRIGHT}Starting setup process...\n")
    config = load_project_config()
    root = load_project_root_dir(config)
    venv = load_project_venv_dir(config)
    venv_path = os.path.join(root, venv)
    requirements_file = load_requirements_file(config)
    print(requirements_file)
    min_python_version = load_min_python_version(config)

    check_python_version(min_python_version)
    create_virtual_environment(venv_path)
    activate_virtual_environment(venv_path)
    install_dependencies(requirements_file)
    create_dotenv(config)
    start_application(sys.argv[1:])
    print(f"{Fore.BLUE}{Style.BRIGHT}Setup process completed successfully.")


if __name__ == "__main__":
    main()
