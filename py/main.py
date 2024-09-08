# main.py

import os
from Application import App
from Application.signal import SignalBus
from Utils.log import Logger
from Utils.environment import SetEnvironment
from Input import InputManager, MouseHandler, KeyboardHandler, WinNativeKeyboardHandler, WinNativeMouseHandler


def main():

    SetEnvironment.from_file(os.path.join(os.getcwd(), "py", ".env"))
    app = App(logger=Logger('RupyLogger').get_console_logger(), signal_bus=SignalBus(), input_manager=InputManager(
        mouse=WinNativeMouseHandler(), keyboard=WinNativeKeyboardHandler()))

    app.start()


if __name__ == "__main__":
    main()
