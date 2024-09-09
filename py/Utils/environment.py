import os
from typing import Any, Dict
from dotenv import dotenv_values, load_dotenv, set_key
from Error.base import Status, create_error
from .validation import match_type_or_raise_exception


class EnvManager:
    """Class to manage environment variables from different sources."""

    @staticmethod
    def _verify_path_is_string(path: Any):
        """
        Verifies that the provided path is a string.

        Args:
            path (Any): The path to verify.

        Raises:
            TypeError: If the path is not a string.
        """
        match_type_or_raise_exception("String", path)

    @staticmethod
    def os_getenv(env_key: str):
        """
        Retrieves an environment variable by key.

        Args:
            env_key (str): The environment variable key.

        Returns:
            str: The value of the environment variable.

        Raises:
            TypeError: If the key is not a string.
        """
        match_type_or_raise_exception("String", env_key)
        return os.getenv(env_key.upper())

    @staticmethod
    def set_env_keys(env_file_path: str, new_entries: Dict[str, Any]):
        """
        Saves the modified environment variables back to the .env file.

        Args:
            env_file_path (str): The path to the .env file.
            new_entries (dict): A dictionary of environment variable names and their new values.

        Raises:
            TypeError: If the `env_file_path` is not a string or `new_entries` is not a dictionary.
            RuntimeError: If an error occurs while setting the environment variables.
        """
        EnvManager._verify_path_is_string(env_file_path)
        match_type_or_raise_exception("Dict", new_entries)

        try:
            for key, entry in new_entries.items():
                new_value = entry.get_value() if hasattr(entry, 'get_value') else str(entry)
                if new_value != os.getenv(key):
                    set_key(env_file_path, key, new_value)
        except Exception as e:
            raise RuntimeError(create_error(
                status=Status.RuntimeError,
                details="Failed to save environment variables.",
                trace=True
            )) from e

    @staticmethod
    def load_env(file_path: str):
        """
        Loads environment variables from the specified .env file.

        Args:
            file_path (str): The path to the .env file.

        Raises:
            ValueError: If an error occurs while loading the environment variables.
        """
        EnvManager._verify_path_is_string(file_path)
        try:
            load_dotenv(file_path)
        except Exception as e:
            raise ValueError(create_error(
                status=Status.ValueError,
                details=f"Failed to load environment variables from {
                    file_path}.",
                trace=True
            )) from e

    @staticmethod
    def set_env_from_file(env_path: str = ".env"):
        """
        Loads environment variables from a specified file.

        Args:
            env_path (str): The path to the .env file.

        Raises:
            ValueError: If the .env file does not exist.
        """
        EnvManager._verify_path_is_string(env_path)
        if os.path.exists(env_path):
            load_dotenv(env_path)
        else:
            raise ValueError(create_error(
                status=Status.ValueError,
                details=f"{env_path} file not found.",
                trace=True
            ))

    @staticmethod
    def set_env_from_dict(env_dict: Dict[str, str]):
        """
        Sets environment variables from a dictionary.

        Args:
            env_dict (dict): A dictionary of environment variable names and values.

        Raises:
            TypeError: If the `env_dict` is not a dictionary or contains non-string keys/values.
        """
        match_type_or_raise_exception("Dict", env_dict)

        for key, value in env_dict.items():
            match_type_or_raise_exception("String", key)
            match_type_or_raise_exception("String", value)
            os.environ[key] = value

    @staticmethod
    def get_env_variables(env_file_path: str) -> Dict[str, str]:
        """
        Retrieves environment variables from a specified .env file.

        Args:
            env_file_path (str): The path to the .env file.

        Returns:
            dict: A dictionary of environment variables from the .env file.

        Raises:
            FileNotFoundError: If the .env file does not exist.
        """
        EnvManager._verify_path_is_string(env_file_path)

        if not os.path.exists(env_file_path):
            raise FileNotFoundError(create_error(
                status=Status.ValueError,
                details=f"The specified .env file '{
                    env_file_path}' does not exist.",
                trace=True
            ))

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
