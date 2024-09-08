import os
from typing import Any, Dict
from dotenv import load_dotenv


class SetEnvironment:
    """Class to manage environment variables from different sources."""

    @staticmethod
    def _verify_path_is_string(path: Any):
        """Verifies that the provided path is a string."""
        if not isinstance(path, str):
            raise TypeError(f"Expected a string for path, but got {
                            type(path).__name__}")

    @staticmethod
    def load_env(file_path: str):
        """
        Loads environment variables from the specified file path.

        Args:
            file_path (str): The path to the .env file.
        """
        SetEnvironment._verify_path_is_string(file_path)
        try:
            load_dotenv(file_path)
            print(f"Loaded environment variables from {file_path}")
        except Exception as e:
            print(f"Error: Failed to load environment variables from {
                  file_path}. Details: {e}")

    @staticmethod
    def from_file(env_path: str = ".env"):
        """
        Loads environment variables from a .env file.

        Args:
            env_path (str): The path to the .env file. Default is ".env".
        """
        SetEnvironment._verify_path_is_string(env_path)
        if os.path.exists(env_path):
            load_dotenv(env_path)
            print(f"Loaded environment variables from {env_path}")
        else:
            print(f"Warning: {env_path} file not found.")

    @staticmethod
    def from_dict(env_dict: Dict[str, str]):
        """
        Sets environment variables from a dictionary.

        Args:
            env_dict (Dict[str, str]): A dictionary of environment variables to set.
        """
        if not isinstance(env_dict, dict):
            raise TypeError(f"Expected a dictionary for environment variables, but got {
                            type(env_dict).__name__}")

        for key, value in env_dict.items():
            if not isinstance(key, str) or not isinstance(value, str):
                raise TypeError(
                    f"Environment keys and values must be strings. Got: {key}={value}")
            os.environ[key] = value
            print(f"Set environment variable {key}.")


# def init_environment():
#     """Initializes the environment by loading .env variables and setting up the Python environment."""
#     # Load environment variables
#     load_env_file()

#     # Retrieve and check the required Python version from environment variables
#     min_python_version = os.getenv(
#         "MIN_PYTHON_VERSION", "3.8")  # Default to 3.8 if not set
#     if not check_python_version(min_python_version):
#         sys.exit(1)

#     # Create and activate virtual environment
#     venv_dir = "venv"
#     if not create_virtual_environment(venv_dir):
#         sys.exit(1)

#     # Activate the virtual environment
#     if not activate_virtual_environment(venv_dir):
#         sys.exit(1)

#     # Install required packages
#     requirements_file = os.getenv("REQUIREMENTS_FILE", "requirements.txt")
#     if not install_requirements(requirements_file):
#         sys.exit(1)

#     print("Environment initialized successfully.")
