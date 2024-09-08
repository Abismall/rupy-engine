
from enum import Enum
from typing import Any, Callable, Dict, List


class Signals(Enum):
    APP_INIT = "app:init"
    APP_START = "app:start"
    APP_UPDATE = "app:state:update"
    APP_RENDER = "app:render"
    APP_SHUTDOWN = "app:shutdown"

    INPUT_UPDATE_START = "input:update:start"
    INPUT_UPDATE_MOUSE = "input:update:mouse"
    INPUT_UPDATE_BUTTONS = "input:update:buttons"
    INPUT_UPDATE_END = "input:update:end"


class SignalBus:
    def __init__(self, channels: Dict[str, List[Callable[[Any], None]]] = {}):
        self.channels = channels

    def subscribe(self, channel: str, listener: Callable[[Any], None]):
        if channel not in self.channels:
            self.channels[channel] = []
        self.channels[channel].append(listener)

    def unsubscribe(self, channel: str, listener: Callable[[Any], None]):
        if channel in self.channels:
            self.channels[channel] = [
                l for l in self.channels[channel] if l != listener]

    def publish(self, channel: str, message: Any = None):
        if channel in self.channels:
            for listener in self.channels[channel]:
                listener(channel, message)
