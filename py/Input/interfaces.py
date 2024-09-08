
HEX_MAP = {
    1: 0x01,  # Left Mouse Button
    2: 0x02,  # Right Mouse Button
    3: 0x03,  # Control-Break Processing
    4: 0x04,  # Middle Mouse Button
    5: 0x05,  # X1 Mouse Button
    6: 0x06,  # X2 Mouse Button
    8: 0x08,  # Backspace
    9: 0x09,  # Tab
    12: 0x0C,  # Clear
    13: 0x0D,  # Enter
    16: 0x10,  # Shift
    17: 0x11,  # Ctrl
    18: 0x12,  # Alt
    19: 0x13,  # Pause
    20: 0x14,  # Caps Lock
    21: 0x15,  # IME Kana Mode
    27: 0x1B,  # Escape
    32: 0x20,  # Spacebar
    33: 0x21,  # Page Up
    34: 0x22,  # Page Down
    35: 0x23,  # End
    36: 0x24,  # Home
    37: 0x25,  # Left Arrow
    38: 0x26,  # Up Arrow
    39: 0x27,  # Right Arrow
    40: 0x28,  # Down Arrow
    44: 0x2C,  # Print Screen
    45: 0x2D,  # Insert
    46: 0x2E,  # Delete
    48: 0x30,  # 0
    49: 0x31,  # 1
    50: 0x32,  # 2
    51: 0x33,  # 3
    52: 0x34,  # 4
    53: 0x35,  # 5
    54: 0x36,  # 6
    55: 0x37,  # 7
    56: 0x38,  # 8
    57: 0x39,  # 9
    65: 0x41,  # A
    66: 0x42,  # B
    67: 0x43,  # C
    68: 0x44,  # D
    69: 0x45,  # E
    70: 0x46,  # F
    71: 0x47,  # G
    72: 0x48,  # H
    73: 0x49,  # I
    74: 0x4A,  # J
    75: 0x4B,  # K
    76: 0x4C,  # L
    77: 0x4D,  # M
    78: 0x4E,  # N
    79: 0x4F,  # O
    80: 0x50,  # P
    81: 0x51,  # Q
    82: 0x52,  # R
    83: 0x53,  # S
    84: 0x54,  # T
    85: 0x55,  # U
    86: 0x56,  # V
    87: 0x57,  # W
    88: 0x58,  # X
    89: 0x59,  # Y
    90: 0x5A,  # Z
    91: 0x5B,  # LWindows
    92: 0x5C,  # RWindows
    93: 0x5D,  # Applications
    96: 0x60,  # Numpad 0
    97: 0x61,  # Numpad 1
    98: 0x62,  # Numpad 2
    99: 0x63,  # Numpad 3
    100: 0x64,  # Numpad 4
    101: 0x65,  # Numpad 5
    102: 0x66,  # Numpad 6
    103: 0x67,  # Numpad 7
    104: 0x68,  # Numpad 8
    105: 0x69,  # Numpad 9
    106: 0x6A,  # Multiply
    107: 0x6B,  # Add
    108: 0x6C,  # Separator
    109: 0x6D,  # Subtract
    110: 0x6E,  # Decimal
    111: 0x6F,  # Divide
    112: 0x70,  # F1
    113: 0x71,  # F2
    114: 0x72,  # F3
    115: 0x73,  # F4
    116: 0x74,  # F5
    117: 0x75,  # F6
    118: 0x76,  # F7
    119: 0x77,  # F8
    120: 0x78,  # F9
    121: 0x79,  # F10
    122: 0x7A,  # F11
    123: 0x7B,  # F12
    144: 0x90,  # NumLock
    145: 0x91,  # ScrollLock
    160: 0xA0,  # LShift
    161: 0xA1,  # RShift
    162: 0xA2,  # LCtrl
    163: 0xA3,  # RCtrl
    164: 0xA4,  # LAlt
    165: 0xA5,  # RAlt
    186: 0xBA,  # ;:
    187: 0xBB,  # =+
    188: 0xBC,  # ,<
    189: 0xBD,  # -_
    190: 0xBE,  # .>
    191: 0xBF,  # /?
    192: 0xC0,  # `~
    219: 0xDB,  # [{
    220: 0xDC,  # \|
    221: 0xDD,  # ]}
    222: 0xDE,  # '"
}


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
    initial_state = {key: False for key in HEX_MAP}

    def is_key_pressed(self, key):
        """
        Checks if a specific key is currently pressed.

        :param key: The key code (e.g., pygame.K_SPACE).
        :return: True if the key is pressed, otherwise False.
        """
        raise NotImplementedError(
            "update method must be implemented by the input listener.")
