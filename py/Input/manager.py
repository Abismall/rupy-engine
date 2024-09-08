
from typing import Optional
from .interfaces import InputListenerKeyboardInterface, InputListenerMouseInterface
from .handlers import WinNativeKeyboardHandler, WinNativeMouseHandler


class InputManager:
    default_mouse = WinNativeMouseHandler
    default_keyboard = WinNativeKeyboardHandler

    def __init__(
        self,
        mouse: Optional[InputListenerMouseInterface] = None,
        keyboard: Optional[InputListenerKeyboardInterface] = None,
        pressed_only_mode: bool = False,
    ):

        self.mouse_handler: InputListenerMouseInterface = (
            mouse if isinstance(
                mouse, InputListenerMouseInterface) else self.default_mouse()
        )
        self.keyboard_handler: InputListenerKeyboardInterface = (
            keyboard if isinstance(
                keyboard, InputListenerKeyboardInterface) else self.default_keyboard()
        )
        self.pressed_only_mode = pressed_only_mode

    def update(self):
        """
        Updates the state of the mouse and keyboard handlers.
        """
        self.mouse_handler.update()
        self.keyboard_handler.update()

    def get_state(self):
        """
        Gets the current state of the mouse and keyboard handlers.

        :return: A dictionary containing the state of the mouse and keyboard.
        """
        mouse_state = self.mouse_handler.get_state()
        keyboard_state = self.keyboard_handler.get_state()

        # If pressed_only_mode is enabled, filter the keyboard state to include only pressed keys
        if self.pressed_only_mode:
            keyboard_state = {
                key: state for key, state in keyboard_state.items() if state == True}

        return {
            "mouse": mouse_state,
            "keyboard": keyboard_state,
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
