
from typing import Optional
from .handlers import KeyboardHandler, MouseHandler


class InputManager:
    default_mouse = MouseHandler
    default_keyboard = KeyboardHandler

    def __init__(self, mouse: Optional[MouseHandler] = None, keyboard: Optional[KeyboardHandler] = None):
        """
        Initializes the input manager with mouse and keyboard handlers.
        """
        self.mouse_handler: MouseHandler = mouse if isinstance(
            mouse, MouseHandler) else self.default_mouse()
        self.keyboard_handler: KeyboardHandler = keyboard if isinstance(
            keyboard, KeyboardHandler) else self.default_keyboard()

    def update(self):
        """
        Updates the state of the mouse and keyboard handlers.
        """
        self.mouse_handler.update()
        self.keyboard_handler.update()

    def get_state(self):
        """
        Updates the state of the mouse and keyboard handlers.
        """
        return {
            "mouse": self.mouse_handler.get_state(),
            "keyboard":  self.keyboard_handler.get_state()
        }

    # Mouse-related methods
    def get_mouse_position(self):
        """
        Gets the current position of the mouse.

        :return: Tuple (x, y) representing the mouse position.
        """
        return self.mouse_handler.get_position()

    def is_left_mouse_clicked(self):
        """
        Checks if the left mouse button is clicked.

        :return: True if the left mouse button is clicked, otherwise False.
        """
        return self.mouse_handler.is_left_clicked()

    def is_right_mouse_clicked(self):
        """
        Checks if the right mouse button is clicked.

        :return: True if the right mouse button is clicked, otherwise False.
        """
        return self.mouse_handler.is_right_clicked()

    def is_key_pressed(self, key):
        """
        Checks if a specific key is pressed.

        :param key: The key code (e.g., pygame.K_SPACE).
        :return: True if the key is pressed, otherwise False.
        """

        return self.keyboard_handler.is_key_pressed(key)
