
class BaseElement:
    def __init__(self, parent, **kwargs):
        self.parent = parent
        self.element = None
        self.config = kwargs

    def pack(self, **kwargs):
        if self.element:
            self.element.pack(**kwargs)

    def grid(self, **kwargs):
        if self.element:
            self.element.grid(**kwargs)

    def set_config(self, **kwargs):
        if self.element:
            self.element.config(**kwargs)
