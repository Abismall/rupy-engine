
import logging
import time
from typing import Any, Optional
import pygame
from Input.manager import InputManager
from Error.base import StatusText, create_error
from .signal import SignalBus, Signals


class App:
    def __init__(self, logger: logging.Logger, signal_bus: SignalBus, input_manager: InputManager):
        """
        Initializes the Application with the necessary components such as logger, message bus,
        and input manager with the listener setup.
        """
        self.logger = logger
        self.signal_bus = signal_bus
        self.inputs = input_manager
        self.running = False

    def start(self):
        """Starts the application."""
        self.running = True
        self.logger.info('Application start')
        self.init()
        self.run()
        self.shutdown()

    def init(self):
        """Initializes the engine and game resources."""
        self.publish_signal(Signals.APP_INIT)
        pygame.init()

    def run(self):
        """Main loop of the engine."""
        self.publish_signal(Signals.APP_START)
        while self.running:
            self.update()
            self.render()
            time.sleep(1 / 60)

    def update(self):
        """Update game state."""
        self.publish_signal(Signals.APP_UPDATE)

        self.publish_signal(Signals.INPUT_UPDATE_START)
        self.inputs.update()

        captured_inputs = self.inputs.get_state()
        self.publish_signal(
            Signals.INPUT_UPDATE_MOUSE, captured_inputs.get("keyboard", self.inputs.keyboard_handler.initial_state))
        self.publish_signal(
            Signals.INPUT_UPDATE_BUTTONS, captured_inputs.get("mouse", self.inputs.mouse_handler.initial_state))

        self.publish_signal(Signals.INPUT_UPDATE_END)

    def publish_signal(self, signal: Signals | Any, message: Optional[str] | Any = None):
        self.logger.info(f"{signal} {message}")
        self.signal_bus.publish(signal, message)

    def render(self):
        """Render the game."""
        self.publish_signal(Signals.APP_RENDER)

    def shutdown(self):
        """Cleans up resources and exits the application."""
        self.publish_signal(Signals.APP_SHUTDOWN)
        self.running = False
