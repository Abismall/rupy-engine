import os
from typing import Any, Dict, Optional
from dotenv import dotenv_values, load_dotenv, set_key
from Error.base import PyEngineError


class EnvManager:
    @staticmethod
    def os_getenv(env_key: str):
        try:
            return os.getenv(env_key.upper())
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    @staticmethod
    def set_env_keys(env_file_path: str, new_entries: Dict[str, Any]):
        try:
            for key, entry in new_entries.items():
                new_value = entry.get_value() if hasattr(
                    entry, 'get_value') and callable(entry.get_value) else str(entry)
                if new_value != os.getenv(key):
                    set_key(env_file_path, key, new_value)
        except FileNotFoundError:
            raise PyEngineError("FILE_NOT_FOUND")
        except PermissionError:
            raise PyEngineError("PERMISSION_DENIED")
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    @staticmethod
    def load_env(env_file_path: str):
        if not os.path.exists(env_file_path):
            raise PyEngineError("FILE_NOT_FOUND")
        try:
            load_dotenv(env_file_path)
        except PermissionError:
            raise PyEngineError("PERMISSION_DENIED")
        except Exception as e:
            raise PyEngineError("VALUE_ERROR") from e

    @staticmethod
    def set_env_from_file(env_file_path: str = "py/.env"):
        try:
            if os.path.exists(env_file_path):
                if not load_dotenv(env_file_path):
                    raise PyEngineError("VALUE_ERROR")
            else:
                raise PyEngineError("FILE_NOT_FOUND")
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    @staticmethod
    def set_env_from_dict(env_dict: Dict[str, str]):
        try:
            for key, value in env_dict.items():
                os.environ[key] = value
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    @staticmethod
    def env_or_default(env_key: str, default: Any, cast: Optional[str | int | float], casts=[str, int, float]):
        try:
            env_value = EnvManager.os_getenv(env_key)
            if env_value is None:
                return default
            if cast in casts:
                return cast(env_value)
            return casts[0](env_value)
        except ValueError:
            raise PyEngineError("VALUE_ERROR")
        except TypeError:
            raise PyEngineError("TYPE_ERROR")
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

    @staticmethod
    def get_dotenv_values(env_file_path: str) -> Dict[str, str]:
        try:
            if os.path.exists(env_file_path):
                return dotenv_values(env_file_path)
            raise PyEngineError("FILE_NOT_FOUND")
        except Exception as e:
            raise PyEngineError("UNKNOWN_ERROR") from e

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
