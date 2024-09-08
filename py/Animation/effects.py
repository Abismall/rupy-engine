import time


class FadeEffect:
    def __init__(self, start_time, fade_duration, initial_alpha=255):
        """
        Initializes a fade effect.

        :param start_time: The time when the fading starts.
        :param fade_duration: Duration over which the object fades out.
        :param initial_alpha: Initial alpha (transparency) value. Default is 255 (fully opaque).
        """
        self.start_time = start_time
        self.fade_duration = fade_duration
        self.initial_alpha = initial_alpha
        self.current_alpha = initial_alpha

    def update(self, current_time):
        """
        Updates the effect based on the elapsed time.

        :param current_time: The current time to calculate fade progress.
        :return: True if the effect is still active, False if it has completed.
        """
        elapsed_time = current_time - self.start_time
        if elapsed_time > self.fade_duration:
            self.current_alpha = 0
            return False

        fade_ratio = 1 - (elapsed_time / self.fade_duration)
        self.current_alpha = max(int(self.initial_alpha * fade_ratio), 0)
        return True

    def get_alpha(self):
        """
        Returns the current alpha value.

        :return: The current alpha (0 to 255).
        """
        return self.current_alpha


class Shape:
    def __init__(self, pos, color, effect=None):
        """
        Initializes a basic shape.

        :param pos: Position of the shape (x, y).
        :param color: Color of the shape (R, G, B).
        :param effect: Optional effect to be applied to the shape.
        """
        self.pos = pos
        self.color = color
        self.effect = effect

    def apply_effect(self, current_time):
        """
        Applies the effect if present and updates its state.

        :param current_time: The current time used to update the effect.
        """
        if self.effect:
            if not self.effect.update(current_time):
                self.color = (*self.color[:3], 0)
            else:
                self.color = (*self.color[:3], self.effect.get_alpha())


class Circle(Shape):
    def __init__(self, pos, radius, color, effect=None):
        super().__init__(pos, color, effect)
        self.radius = radius

    def get_draw_command(self):
        """Creates a draw command for rendering the circle."""
        return {'type': 'circle', 'pos': self.pos, 'radius': self.radius, 'color': self.color}


class Rectangle(Shape):
    def __init__(self, pos, width, height, color, effect=None):
        super().__init__(pos, color, effect)
        self.width = width
        self.height = height

    def get_draw_command(self):
        """Creates a draw command for rendering the rectangle."""
        return {'type': 'rect', 'pos': self.pos, 'width': self.width, 'height': self.height, 'color': self.color}


class Triangle(Shape):
    def __init__(self, vertices, color, effect=None):
        super().__init__(vertices[0], color, effect)
        self.vertices = vertices

    def get_draw_command(self):
        """Creates a draw command for rendering the triangle."""
        return {'type': 'polygon', 'vertices': self.vertices, 'color': self.color}
