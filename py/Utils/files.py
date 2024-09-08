import os
from Error.base import StatusText, create_error
from .validation import match_type_or_raise_exception


class FilePath:
    @staticmethod
    def path_exists(file_path: str) -> bool:
        """
        Checks if the specified path exists.

        Args:
            file_path (str): The path to check.

        Returns:
            bool: True if the path exists, False otherwise.
        """
        match_type_or_raise_exception("string", file_path)
        try:
            return os.path.exists(file_path)
        except Exception as exec:
            raise RuntimeError(
                status=StatusText.RUNTIME_ERROR.value,
                message=(str(exec) or None),
            )

    @staticmethod
    def create_path_if_not_exists(path_string: str):
        """
        Checks if a directory or file path exists, and creates the directories if they do not exist.

        Args:
            path_string (str): The path to the directory or file.

        Raises:
            TypeError: If the `path_string` is not a string.
            RuntimeError: If an error occurs while creating the directory.
        """
        return FilePath._create_if_not_exists(file_path_arg=path_string)

    @staticmethod
    def create_non_existing_paths(paths_list: list[str]):
        """
        Checks if a list of directory or file paths exist, and creates the directories if they do not exist.

        Args:
            paths_list (list[str]): List of paths to the directories or files.

        Raises:
            TypeError: If the `paths_list` is not a list.
            RuntimeError: If an error occurs while creating the directory.
        """
        match_type_or_raise_exception(
            to_mach="array", to_check=paths_list)
        for path in paths_list:
            FilePath._create_if_not_exists(file_path_arg=path)

    @staticmethod
    def _create_if_not_exists(file_path_arg: str) -> None:
        """
        Helper function to create a directory if it does not exist.

        Args:
            file_path_arg (str): The path to check and create if not exists.

        Raises:
            RuntimeError: If an error occurs while creating the directory.
        """
        match_type_or_raise_exception(
            to_mach="string", to_check=file_path_arg)
        directory = os.path.dirname(file_path_arg) if os.path.isfile(
            file_path_arg) else file_path_arg
        if not FilePath.path_exists(directory):
            try:
                os.makedirs(directory, exist_ok=True)
            except Exception as exec:
                raise RuntimeError(create_error(
                    status=StatusText.RUNTIME_ERROR.value,
                    message=str(exec) or None,
                ))


# def get_file_suffix(file_path: str):
#     """
#     Extracts the file suffix from a file path.

#     :param file_path: The path of the file.
#     :return: The file suffix (extension) in lowercase.
#     :raises TypeError: If file_path is not a string.
#     :raises ValueError: If the suffix cannot be parsed.
#     """
#     try:
#         if not file_path or not isinstance(file_path, str):
#             raise TypeError(
#                 'Invalid or missing value for file_path, expected a non-empty string.')

#         file_path_lower = file_path.lower()
#         splitext = os.path.splitext(file_path_lower)

#         if not splitext or len(splitext) < 2:
#             raise ValueError('Failed to split file path into components.')

#         file_suffix = splitext[1]

#         if not file_suffix or not isinstance(file_suffix, str):
#             raise ValueError('Invalid file suffix parsed from the file path.')

#         return file_suffix
#     except (TypeError, ValueError) as e:
#         log_error(f"Error in get_file_suffix: {e}")
#         raise


# def is_image_file(file_name, accepted=FILE_SUFFIX.get('image')):
#     """
#     Checks if a file is an image based on its suffix.

#     :param file_name: The name of the file to check.
#     :param accepted: A list of accepted image suffixes.
#     :return: True if the file is an image, False otherwise.
#     """
#     try:
#         return get_file_suffix(file_name) in accepted
#     except ValueError as e:
#         log_error(f"Error checking if file is an image: {e}")
#         return False


# def is_sound_file(file_name, accepted=FILE_SUFFIX.get('sound')):
#     """
#     Checks if a file is a sound based on its suffix.

#     :param file_name: The name of the file to check.
#     :param accepted: A list of accepted sound suffixes.
#     :return: True if the file is a sound, False otherwise.
#     """
#     try:
#         return get_file_suffix(file_name) in accepted
#     except ValueError as e:
#         log_error(f"Error checking if file is a sound: {e}")
#         return False


# def transform_scale(surface, width, height):
#     """
#     Scales a pygame surface to the given width and height.

#     :param surface: The pygame surface to scale.
#     :param width: The new width of the surface.
#     :param height: The new height of the surface.
#     :return: The scaled surface.
#     :raises RuntimeError: If scaling fails.
#     """
#     try:
#         return pygame.transform.scale(surface, (width, height))
#     except pygame.error as e:
#         log_error(f"Failed to transform scale for surface: {e}")
#         raise RuntimeError(f"Failed to transform scale: {e}")


# def load_file(path):
#     """
#     Loads a file based on its type (image or sound).

#     :param path: The path of the file to load.
#     :return: The loaded file object.
#     :raises FileNotFoundError: If the file does not exist.
#     :raises ValueError: If the file type is unsupported.
#     """
#     try:
#         if not os.path.exists(path):
#             raise FileNotFoundError(f"File not found: {path}")
#         if is_image_file(path):
#             return load_image_file(path)
#         elif is_sound_file(path):
#             return load_sound_file(path)
#         else:
#             raise ValueError(f"Unsupported file type: {path}")
#     except (FileNotFoundError, ValueError) as e:
#         log_error(e)
#         raise


# def load_image_file(path):
#     """
#     Loads an image file as a pygame surface.

#     :param path: The path of the image file.
#     :return: The loaded pygame surface.
#     :raises RuntimeError: If the image cannot be loaded.
#     """
#     try:
#         return pygame.image.load(path)
#     except pygame.error as e:
#         log_error(f"Failed to load image file: {path}, error: {e}")
#         raise RuntimeError(f'Failed to load image file: {path}, error: {e}')


# def load_sound_file(path):
#     """
#     Loads a sound file as a pygame sound object.

#     :param path: The path of the sound file.
#     :return: The loaded pygame sound object.
#     :raises RuntimeError: If the sound cannot be loaded.
#     """
#     try:
#         return pygame.mixer.Sound(path)
#     except pygame.error as e:
#         log_error(f"Failed to load sound file: {path}, error: {e}")
#         raise RuntimeError(f'Failed to load sound file: {path}, error: {e}')


# def transform_scale(frames, width, height):
#     return [pygame.transform.scale(f, (width, height)) for f in frames]

# def draw_loading_screen(win, font, message, progress_bar_width, progress_bar_height):
#     win.fill((0, 0, 0))
#     text_surface = font.render(message, True, (255, 255, 255))
#     win.blit(
#         text_surface,
#         (
#             (win.get_width() - progress_bar_width or text_surface.get_width()) // 2,
#             (win.get_height() - progress_bar_height or text_surface.get_height()) // 2,
#         )
#     )
#     return pygame.display.update()
