import logging
from collections import deque
from enum import Enum
from typing import Any, Callable, Dict
import threading

from Utils.constants import (APP_INIT_SIGNAL, APP_INTERRUPT_SIGNAL, APP_LAUNCH_ENGINE_SIGNAL, APP_SHUTDOWN_SIGNAL, APP_START_SIGNAL, APP_UPDATE_SIGNAL,
                             INPUT_UPDATE_BUTTONS_SIGNAL, INPUT_UPDATE_END_SIGNAL, INPUT_UPDATE_MOUSE_SIGNAL, INPUT_UPDATE_START_SIGNAL, MENU_CLOSE_SIGNAL, MENU_OPEN_SIGNAL, ENV_RELOAD)


class Signals(Enum):
    APP_INIT = APP_INIT_SIGNAL
    APP_START = APP_START_SIGNAL
    APP_UPDATE = APP_UPDATE_SIGNAL
    LAUNCH_ENGINE = APP_LAUNCH_ENGINE_SIGNAL
    APP_SHUTDOWN = APP_SHUTDOWN_SIGNAL
    ENV_RELOAD = ENV_RELOAD
    INPUT_UPDATE_START = INPUT_UPDATE_START_SIGNAL
    INPUT_UPDATE_MOUSE = INPUT_UPDATE_MOUSE_SIGNAL
    INPUT_UPDATE_BUTTONS = INPUT_UPDATE_BUTTONS_SIGNAL
    INPUT_UPDATE_END = INPUT_UPDATE_END_SIGNAL
    MENU_OPEN = MENU_OPEN_SIGNAL
    MENU_CLOSE = MENU_CLOSE_SIGNAL
    APP_INTERRUPT = APP_INTERRUPT_SIGNAL


class SignalBus:
    def __init__(self):
        self.channels: Dict[str, Dict[str, Any]] = {}
        self.lock = threading.Lock()
        self.logger = logging.getLogger("SignalBus")
        self.logger.setLevel(logging.DEBUG)
        handler = logging.StreamHandler()
        handler.setFormatter(logging.Formatter(
            '%(asctime)s - %(levelname)s - %(message)s'))
        self.logger.addHandler(handler)

    def subscribe(self, signal: Signals, listener: Callable[[str, Any], None]):
        with self.lock:
            if signal.value not in self.channels:
                self.channels[signal.value] = {
                    'listeners': [], 'queue': deque()}
                self.logger.debug(f"Created channel: {signal}")
            self.channels[signal.value]['listeners'].append(listener)
            self.logger.debug(
                f"Subscribed {listener.__name__} to channel: {signal}")

    def unsubscribe(self, signal: Signals, listener: Callable[[str, Any], None]):
        with self.lock:
            if signal.value in self.channels and listener in self.channels[signal.value]['listeners']:
                self.channels[signal.value]['listeners'].remove(listener)
                self.logger.debug(
                    f"Unsubscribed {listener.__name__} from channel: {signal}")
                if not self.channels[signal.value]['listeners']:
                    del self.channels[signal.value]
                    self.logger.debug(f"Deleted channel: {signal}")

    def publish(self, signal: Signals, message: Any = None):
        with self.lock:
            if signal.value in self.channels:
                self.channels[signal.value]['queue'].append(message)
                self.logger.debug(f"Published message to channel: {
                                  signal} with message: {message}")

    def consume(self, signal: Signals):
        with self.lock:
            if signal.value in self.channels:
                while self.channels[signal.value]['queue']:
                    message = self.channels[signal.value]['queue'].popleft()
                    listeners = list(self.channels[signal.value]['listeners'])
                    self.logger.debug(f"Consuming message from channel: {
                                      signal} with message: {message}")
                    for listener in listeners:
                        try:
                            self.logger.debug(f"Invoking listener {
                                              listener.__name__} on channel: {signal}")
                            listener(signal, message)
                        except Exception as e:
                            self.logger.error(f"Error invoking listener {
                                              listener.__name__} on channel: {signal}: {e}")
                            continue
