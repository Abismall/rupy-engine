import os
import subprocess
import sys


def check_python_version(min_version):
    """Check if the current Python version meets the minimum requirement."""
    current_version = sys.version_info
    required_version = tuple(map(int, min_version.split(".")))

    if current_version < required_version:
        print(f"Error: Python {min_version} or higher is required. Found Python {
              current_version.major}.{current_version.minor}.")
        return False

    print(f"Python version {current_version.major}.{
          current_version.minor} is compatible.")
    return True


def create_virtual_environment(venv_dir="venv"):
    """Creates a virtual environment if it does not exist."""
    if not os.path.exists(venv_dir):
        try:
            subprocess.run(["python3", "-m", "venv", venv_dir], check=True)
            print(f"Virtual environment created at {venv_dir}")
        except subprocess.CalledProcessError:
            print(f"Error: Failed to create virtual environment at {
                  venv_dir}.")
            return False
    else:
        print(f"Virtual environment already exists at {venv_dir}.")
    return True


def activate_virtual_environment(venv_dir="venv"):
    """Activates the virtual environment."""
    activate_script = os.path.join(venv_dir, "bin", "activate_this.py")
    if os.path.exists(activate_script):
        exec(open(activate_script).read(), {'__file__': activate_script})
        print(f"Activated virtual environment at {venv_dir}.")
    else:
        print(f"Error: Virtual environment activation script not found at {
              activate_script}.")
        return False
    return True


def install_requirements(requirements_file="requirements.txt"):
    """Installs Python dependencies from a requirements file."""
    if os.path.exists(requirements_file):
        try:
            subprocess.run(
                ["pip", "install", "-r", requirements_file], check=True)
            print(f"Installed dependencies from {requirements_file}.")
        except subprocess.CalledProcessError:
            print(f"Error: Failed to install dependencies from {
                  requirements_file}.")
            return False
    else:
        print(f"Error: {requirements_file} not found.")
        return False
    return True
