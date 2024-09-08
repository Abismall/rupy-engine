from .base import BaseElement
import tkinter as tk


class EntryElement(BaseElement):
    def __init__(self, parent, **kwargs):
        super().__init__(parent, **kwargs)
        self.element = tk.Entry(parent, **self.config)

    def get_value(self):
        return self.element.get()
