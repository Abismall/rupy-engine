import os
from typing import Any, Dict
from dotenv import dotenv_values, load_dotenv, set_key

from Error.base import StatusText, create_error


class EnvManager:
    """Class to manage environment variables from different sources."""

    @staticmethod
    def _verify_path_is_string(path: Any):
        if not isinstance(path, str):
            raise TypeError(create_error(status=StatusText.TYPE_ERROR,
                                         message=f"Expected a string for path, but got {type(path).__name__}"))

    @staticmethod
    def os_getenv(env_key: str):
        if not env_key or not isinstance(env_key, str):
            raise TypeError(create_error(status=StatusText.TYPE_ERROR,
                                         message=f"Expected a string for env key, but got {type(env_key).__name__}"))
        else:
            return os.getenv(env_key.upper())

    @staticmethod
    def set_env_keys(env_file_path: str, new_entries: Dict[str, Any]):
        """
        Saves the modified environment variables back to the .env file.

        Args:
            env_file_path (str): The path to the .env file.
            new_entries (dict): A dictionary of environment variable names and their new values.
        """
        EnvManager._verify_path_is_string(env_file_path)

        if not isinstance(new_entries, dict):
            raise TypeError(create_error(status=StatusText.VALUE_ERROR,
                                         message="Expected a dictionary for environment entries."))

        try:
            for key, entry in new_entries.items():
                new_value = entry.get_value() if hasattr(entry, 'get_value') else str(entry)
                if new_value != os.getenv(key):
                    set_key(env_file_path, key, new_value)
        except Exception as e:
            raise RuntimeError(create_error(status=StatusText.RUNTIME_ERROR,
                                            message="Error: Failed to save environment variables.")) from e

    @staticmethod
    def load_env(file_path: str):
        EnvManager._verify_path_is_string(file_path)
        try:
            load_dotenv(file_path)
        except Exception as e:
            raise ValueError(create_error(status=StatusText.VALUE_ERROR,
                                          message=f"Error: Failed to load environment variables from {file_path}.")) from e

    @staticmethod
    def set_env_from_file(env_path: str = ".env"):
        EnvManager._verify_path_is_string(env_path)
        if os.path.exists(env_path):
            load_dotenv(env_path)
        else:
            raise ValueError(create_error(status=StatusText.VALUE_ERROR,
                                          message=f"Warning: {env_path} file not found."))

    @staticmethod
    def set_env_from_dict(env_dict: Dict[str, str]):
        if not isinstance(env_dict, dict):
            raise TypeError(create_error(status=StatusText.VALUE_ERROR,
                                         message=f"Expected a dictionary for environment variables, but got {type(env_dict).__name__}"))

        for key, value in env_dict.items():
            if not isinstance(key, str) or not isinstance(value, str):
                raise TypeError(create_error(status=StatusText.VALUE_ERROR,
                                             message=f"Environment keys and values must be strings. Got: {key}={value}"))
            else:
                os.environ[key] = value

    @staticmethod
    def get_env_variables(env_file_path: str) -> Dict[str, str]:
        """
        Retrieves environment variables from a specified .env file.

        Args:
            env_file_path (str): The path to the .env file.

        Returns:
            dict: A dictionary of environment variables from the .env file.
        """
        EnvManager._verify_path_is_string(env_file_path)

        if not os.path.exists(env_file_path):
            raise FileNotFoundError(create_error(
                status=StatusText.VALUE_ERROR, message=f"The specified .env file '{env_file_path}' does not exist."))

        return dotenv_values(env_file_path)


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
