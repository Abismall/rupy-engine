
PYENGINE = "rupyengine"
PYPROJECT = "pyproject.toml"
DEFAULT_SLEEP_AFTER_CYCLE = 0.1
FROZEN_EXEC = 'frozen'
LAUNCH_OPTIONS_FLAG = 'lo'


SIZE_SMALL = "300x450"          # Small size: 300 pixels wide, 450 pixels high
SIZE_MEDIUM = "400x600"         # Medium size: 400 pixels wide, 600 pixels high
SIZE_LARGE = "500x750"          # Large size: 500 pixels wide, 750 pixels high
SIZE_EXTRA_LARGE = "600x900"    # Extra large size: 600 pixels wide, 900 pixels high


LAUNCHER_MENU = "launcher_menu"
SCENE_MENU = "scenes_menu"


LOGGER_NAME_BASE = "RupyLogger"
LOG_METHOD_FILE = "file"
LOG_METHOD_CONSOLE = "console"
LOGGING_METHOD_OPTIONS = [LOG_METHOD_CONSOLE, LOG_METHOD_FILE]
DEFAULT_LOG_FILE = "app.log"
DEFAULT_LOG_LEVEL = "info"
DEFAULT_LOG_FORMAT = "verbose"


UNKNOWN_ERROR_MSG = "An unexpected error occurred. Please try again."
IMPORT_ERROR_MSG = "Failed to import the required module."
TYPE_ERROR_MSG = "Type mismatch encountered."
VALUE_ERROR_MSG = "Invalid value provided."
UNBOUND_LOCAL_ERROR_MSG = "Referenced a variable before assignment."
ARGUMENT_ERROR_MSG = "Invalid arguments supplied."
RUNTIME_ERROR_MSG = "A runtime error has occurred."
CHILD_PROCESS_ERROR_MSG = "Error in child process execution."


APP_INIT_SIGNAL = "app:init"
APP_START_SIGNAL = "app:start"
APP_UPDATE_SIGNAL = "app:state:update"
APP_LAUNCH_ENGINE_SIGNAL = "app:engine:start"
APP_SHUTDOWN_SIGNAL = "app:shutdown"
ENV_RELOAD = "env:reload"
INPUT_UPDATE_START_SIGNAL = "input:update:start"
INPUT_UPDATE_MOUSE_SIGNAL = "input:update:mouse"
INPUT_UPDATE_BUTTONS_SIGNAL = "input:update:buttons"
INPUT_UPDATE_END_SIGNAL = "input:update:end"
MENU_OPEN_SIGNAL = "menu:open"
MENU_CLOSE_SIGNAL = "menu:close"
APP_INTERRUPT_SIGNAL = "app:interrupt"


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

KEYBOARD_INTERFACE_INITIAL_STATE = {key: False for key in HEX_MAP}
MOUSE_INTERFACE_INITIAL_STATE = {
    "pos_x": 0,
    "pos_y": 0,
    "left_click": False,
    "right_click": False
}
