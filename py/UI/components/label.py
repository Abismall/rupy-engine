from .base import BaseElement
import tkinter as tk


class LabelElement(BaseElement):
    def __init__(self, parent, text, **kwargs):

        super().__init__(parent, **kwargs)
        self.element = tk.Label(parent, text=text, **self.config)
