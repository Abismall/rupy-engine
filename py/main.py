# main.py

import os
from Application import App
from Application.signal import SignalBus
from Input import InputManager, WinNativeKeyboardHandler, WinNativeMouseHandler
from Utils.environment import EnvManager


def main():
    env_file_path = os.path.join(os.getcwd(), "py", ".env")
    EnvManager.set_env_from_file(env_file_path)
    app = App(env_file=env_file_path, logger="console", signal_bus=SignalBus(), input_manager=InputManager(
        mouse=WinNativeKeyboardHandler(), keyboard=WinNativeMouseHandler(), pressed_only_mode=True))

    app.start()


if __name__ == "__main__":
    main()
