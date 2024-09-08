import pygame
from .interfaces import InputListenerKeyboardInterface, InputListenerMouseInterface


class KeyboardHandler(InputListenerKeyboardInterface):
    initial_state = {}

    def __init__(self):
        """
        Initializes the keyboard handler to keep track of key states.
        """
        super().__init__()
        self.state = KeyboardHandler.initial_state

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


class MouseHandler(InputListenerMouseInterface):
    """
    Handles mouse input, capturing mouse movements and button states.
    """
    initial_state = {
        "pos_x": 0,
        "pos_y": 0,
        "left_click": False,
        "right_click": False
    }

    def __init__(self):
        """
        Initializes the mouse handler to keep track of mouse clicks and positions.
        """
        super().__init__()
        self.state = MouseHandler.initial_state

    def update(self):
        """
        Updates the mouse state including position and click status.
        """
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
        """
        Checks if the left mouse button is clicked.

        :return: True if the left mouse button is clicked, otherwise False.
        """
        return self.state.get('left_click', False)

    def is_right_clicked(self):
        """
        Checks if the right mouse button is clicked.

        :return: True if the right mouse button is clicked, otherwise False.
        """
        return self.state.get('right_click', False)

    def get_position(self):
        """
        Gets the current position of the mouse.

        :return: Tuple (x, y) representing the mouse position.
        """
        return {"pos_x": self.state.get("pos_x", 0), "pos_y": self.state.get("pos_y", 0)}
