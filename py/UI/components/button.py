from .base import BaseElement
import tkinter as tk


class ButtonElement(BaseElement):
    def __init__(self, parent, text, command=None, **kwargs):

        super().__init__(parent, **kwargs)
        self.element = tk.Button(
            parent, text=text, command=command, **self.config)
