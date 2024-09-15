

import ctypes
from typing import Dict, Tuple

from Utils.validation import is_key_of
from Utils.constants import HEX_MAP
from .interfaces import InputListenerKeyboardInterface, InputListenerMouseInterface


class PygameKeyboardHandler(InputListenerKeyboardInterface):
    def __init__(self):
        """
        Initializes the keyboard handler to keep track of key states.
        """
        super().__init__()
        self.state = {key: False for key in HEX_MAP}
        if not is_key_of("pygame", globals()):
            try:
                global pygame
                import pygame
            except ImportError as e:
                raise ImportError(
                    "Pygame is required for this handler.") from e

    def update(self):
        """
        Updates the pressed keys state.
        """
        keys = pygame.key.get_pressed()

        self.state = {key: keys[key]
                      for key in range(len(keys)) if keys[key]}

    def get_state(self):
        return self.state

    def is_key_pressed(self, key):
        """
        Checks if a specific key is currently pressed.

        :param key: The key code (e.g., pygame.K_SPACE).
        :return: True if the key is pressed, otherwise False.
        """
        return self.state.get(key, False)


class PygameMouseHandler(InputListenerMouseInterface):
    def __init__(self):

        super().__init__()
        self.state = InputListenerMouseInterface.initial_state
        if not is_key_of("pygame", globals()):
            try:
                global pygame
                import pygame
            except ImportError as e:
                raise ImportError(
                    "Pygame is required for this handler.") from e

    def update(self):

        mouse_pos = pygame.mouse.get_pos()
        mouse_buttons = pygame.mouse.get_pressed()

        self.state.update({"pos_x": mouse_pos[0] if isinstance(
            mouse_pos, list) else 0, "pos_y": mouse_pos[1] if isinstance(
            mouse_pos, list) and len(mouse_pos) >= 1 else 0})

        self.state.update(
            {"right_click": mouse_buttons[0], "left_click": mouse_buttons[1]})

    def get_state(self):
        return self.state

    def is_left_clicked(self):

        return self.state.get('left_click', False)

    def is_right_clicked(self):

        return self.state.get('right_click', False)

    def get_position(self):

        return {"pos_x": self.state.get("pos_x", 0), "pos_y": self.state.get("pos_y", 0)}


class WinNativeKeyboardHandler(InputListenerKeyboardInterface):
    """
    Handles keyboard input for Windows using native API calls.
    """

    def __init__(self):
        """
        Initializes the keyboard handler to keep track of key states.
        """
        super().__init__()

    def update(self):
        """
        Updates the state of keyboard keys.
        """

        self.state = {
            key: bool(ctypes.windll.user32.GetAsyncKeyState(
                hex_val) & 0x8000)
            for key, hex_val in HEX_MAP.items()
        }

    def get_state(self) -> Dict[int, bool]:
        """
        Returns the current state of keyboard keys.

        :return: A dictionary containing the states of all keyboard keys.
        """
        return self.state

    def is_key_pressed(self, key):
        """
        Checks if a specific key is currently pressed.

        :param key: The key code (e.g., hex value).
        :return: True if the key is pressed, otherwise False.
        """
        return self.state.get(key, False)


class WinNativeMouseHandler(InputListenerMouseInterface):
    """
    Handles mouse input for Windows using native API calls.
    """
    initial_state = InputListenerMouseInterface.initial_state

    def __init__(self):
        """
        Initializes the mouse handler to keep track of mouse position.
        """
        super().__init__()
        self.state = WinNativeMouseHandler.initial_state

    def update(self):
        """
        Updates the mouse position state.
        """
        class POINT(ctypes.Structure):
            _fields_ = [("x", ctypes.c_long), ("y", ctypes.c_long)]

        pt = POINT()
        ctypes.windll.user32.GetCursorPos(ctypes.byref(pt))
        self.state["pos_x"] = pt.x
        self.state["pos_y"] = pt.y

    def get_state(self) -> Tuple[int, int]:
        """
        Returns the current mouse position.

        :return: A tuple representing the mouse position (x, y).
        """
        return self.state["pos_x"], self.state["pos_y"]

    def get_position(self):
        """
        Gets the current position of the mouse.

        :return: A dictionary with keys 'pos_x' and 'pos_y' representing the mouse position.
        """
        return {"pos_x": self.state.get("pos_x", 0), "pos_y": self.state.get("pos_y", 0)}

    def is_left_clicked(self):
        """
        Dummy implementation since this handler does not handle button states.

        :return: False as no mouse button tracking is implemented.
        """
        return False

    def is_right_clicked(self):
        """
        Dummy implementation since this handler does not handle button states.

        :return: False as no mouse button tracking is implemented.
        """
        return False

