
import time

from Application.signal import SignalBus, Signals
from Input.handlers import KeyboardHandler, MouseHandler
from Input.manager import InputManager
from Utils.log import Logger


class Application:
    def __init__(self):
        """
        Initializes the Application with the necessary components such as logger, message bus,
        and input manager with the listener setup.
        """
        self.logger = Logger()
        self.signal_bus = SignalBus()
        self.inputs = InputManager(MouseHandler(), KeyboardHandler())
        self.running = False

    def start(self):
        """Starts the application."""
        self.running = True
        self.init()
        self.run()
        self.shutdown()

    def init(self):
        """Initializes the engine and game resources."""
        self.signal_bus.publish(Signals.APP_INIT)

    def run(self):
        """Main loop of the engine."""
        self.signal_bus.publish(Signals.APP_START)
        while self.running:
            self.update()
            self.render()
            time.sleep(1 / 60)

    def update(self):
        """Update game state."""
        self.signal_bus.publish(Signals.APP_UPDATE)

        self.signal_bus.publish(Signals.INPUT_UPDATE_START)
        self.inputs.update()

        captured_inputs = self.inputs.get_state()

        self.signal_bus.publish(
            Signals.INPUT_UPDATE_MOUSE, captured_inputs.get("keyboard", KeyboardHandler.initial_state))
        self.signal_bus.publish(
            Signals.INPUT_UPDATE_BUTTONS, captured_inputs.get("mouse", MouseHandler.initial_state))

        self.signal_bus.publish(Signals.INPUT_UPDATE_END)

    def render(self):
        """Render the game."""
        self.signal_bus.publish(Signals.APP_RENDER)

    def shutdown(self):
        """Cleans up resources and exits the application."""
        self.signal_bus.publish(Signals.APP_SHUTDOWN)
        self.running = False
