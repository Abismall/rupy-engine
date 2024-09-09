from collections import deque
from enum import Enum
from typing import Any, Callable, Dict
import threading

from constants import (APP_INIT_SIGNAL, APP_INTERRUPT_SIGNAL, APP_RENDER_SIGNAL, APP_SHUTDOWN_SIGNAL, APP_START_SIGNAL, APP_UPDATE_SIGNAL,
                       INPUT_UPDATE_BUTTONS_SIGNAL, INPUT_UPDATE_END_SIGNAL, INPUT_UPDATE_MOUSE_SIGNAL, INPUT_UPDATE_START_SIGNAL, MENU_CLOSE_SIGNAL, MENU_OPEN_SIGNAL)


class Signals(Enum):
    APP_INIT = APP_INIT_SIGNAL
    APP_START = APP_START_SIGNAL
    APP_UPDATE = APP_UPDATE_SIGNAL
    APP_RENDER = APP_RENDER_SIGNAL
    APP_SHUTDOWN = APP_SHUTDOWN_SIGNAL
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

    def subscribe(self, channel: str, listener: Callable[[str, Any], None]):
        with self.lock:
            if channel not in self.channels:
                self.channels[channel] = {'listeners': [], 'queue': deque()}
            self.channels[channel]['listeners'].append(listener)

    def unsubscribe(self, channel: str, listener: Callable[[str, Any], None]):
        with self.lock:
            if channel in self.channels and listener in self.channels[channel]['listeners']:
                self.channels[channel]['listeners'].remove(listener)
                if not self.channels[channel]['listeners']:
                    del self.channels[channel]

    def publish(self, channel: str, message: Any = None):
        with self.lock:
            if channel in self.channels:
                self.channels[channel]['queue'].append(message)

    def consume(self, channel: str):
        with self.lock:
            if channel in self.channels:
                while self.channels[channel]['queue']:
                    message = self.channels[channel]['queue'].popleft()
                    listeners = list(self.channels[channel]['listeners'])
                    for listener in listeners:
                        try:
                            listener(channel, message)
                        except:
                            continue
