# main.py

import os
from Application import App
from Application.signal import SignalBus
from Utils.log import Logger
from Utils.environment import EnvManager
from Input import InputManager, WinNativeKeyboardHandler, WinNativeMouseHandler


def main():

    EnvManager.set_env_from_file(os.path.join(os.getcwd(), "py", ".env"))
    app = App(logger=Logger('RupyLogger').get_file_logger(), signal_bus=SignalBus(), input_manager=InputManager(
        mouse=WinNativeKeyboardHandler(), keyboard=WinNativeMouseHandler()))

    app.start()


if __name__ == "__main__":
    main()
