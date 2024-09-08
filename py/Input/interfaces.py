

class InputListenerBaseInterface:
    """
    Interface for input listeners. Defines methods to update and get the states of mouse and keyboard inputs.
    """
    state: dict = {}

    def update(self) -> None:
        """
        Updates the current state of mouse and keyboard inputs.
        """
        raise NotImplementedError(
            "update method must be implemented by the input listener.")

    def get_state(self):
        return self.state


class InputListenerMouseInterface(InputListenerBaseInterface):
    """
    Interface for input listeners. Defines methods to update and get the states of mouse and keyboard inputs.
    """
    initial_state = {
        "pos_x": 0,
        "pos_y": 0,
        "left_click": False,
        "right_click": False
    }

    def is_left_clicked(self):
        """
        Checks if the left mouse button is clicked.

        :return: True if the left mouse button is clicked, otherwise False.
        """
        raise NotImplementedError(
            "update method must be implemented by the input listener.")

    def is_right_clicked(self):
        """
        Checks if the right mouse button is clicked.

        :return: True if the right mouse button is clicked, otherwise False.
        """
        raise NotImplementedError(
            "update method must be implemented by the input listener.")

    def get_position(self):
        """
        Gets the current position of the mouse.

        :return: Tuple (x, y) representing the mouse position.
        """
        raise NotImplementedError(
            "update method must be implemented by the input listener.")


class InputListenerKeyboardInterface(InputListenerBaseInterface):
    initial_state = {}

    def is_key_pressed(self, key):
        """
        Checks if a specific key is currently pressed.

        :param key: The key code (e.g., pygame.K_SPACE).
        :return: True if the key is pressed, otherwise False.
        """
        raise NotImplementedError(
            "update method must be implemented by the input listener.")
