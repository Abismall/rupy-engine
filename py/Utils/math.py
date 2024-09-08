
import math
import random
from typing import Tuple, List

# Constants
PI = math.pi
TWO_PI = 2 * math.pi
HALF_PI = math.pi / 2
DEG_TO_RAD = PI / 180
RAD_TO_DEG = 180 / PI

# Vector Operations


def add_vectors(v1: Tuple[float, float], v2: Tuple[float, float]) -> Tuple[float, float]:
    """
    Adds two vectors.

    :param v1: First vector (x1, y1).
    :param v2: Second vector (x2, y2).
    :return: Resulting vector (x, y).
    """
    return v1[0] + v2[0], v1[1] + v2[1]


def subtract_vectors(v1: Tuple[float, float], v2: Tuple[float, float]) -> Tuple[float, float]:
    """
    Subtracts vector v2 from v1.

    :param v1: First vector (x1, y1).
    :param v2: Second vector (x2, y2).
    :return: Resulting vector (x, y).
    """
    return v1[0] - v2[0], v1[1] - v2[1]


def scale_vector(v: Tuple[float, float], scalar: float) -> Tuple[float, float]:
    """
    Scales a vector by a scalar.

    :param v: The vector (x, y).
    :param scalar: The scalar value.
    :return: Scaled vector (x, y).
    """
    return v[0] * scalar, v[1] * scalar


def vector_length(v: Tuple[float, float]) -> float:
    """
    Calculates the magnitude (length) of a vector.

    :param v: The vector (x, y).
    :return: The magnitude of the vector.
    """
    return math.sqrt(v[0] ** 2 + v[1] ** 2)


def normalize_vector(v: Tuple[float, float]) -> Tuple[float, float]:
    """
    Normalizes a vector to have a length of 1 (unit vector).

    :param v: The vector (x, y).
    :return: Normalized vector (x, y).
    """
    length = vector_length(v)
    if length == 0:
        return 0, 0
    return v[0] / length, v[1] / length

# Distance and Angle Calculations


def distance(p1: Tuple[float, float], p2: Tuple[float, float]) -> float:
    """
    Calculates the distance between two points.

    :param p1: First point (x1, y1).
    :param p2: Second point (x2, y2).
    :return: Distance between p1 and p2.
    """
    return math.sqrt((p2[0] - p1[0]) ** 2 + (p2[1] - p1[1]) ** 2)


def angle_between_vectors(v1: Tuple[float, float], v2: Tuple[float, float]) -> float:
    """
    Calculates the angle between two vectors in radians.

    :param v1: First vector (x1, y1).
    :param v2: Second vector (x2, y2).
    :return: Angle in radians.
    """
    dot = v1[0] * v2[0] + v1[1] * v2[1]
    lengths = vector_length(v1) * vector_length(v2)
    if lengths == 0:
        return 0.0
    return math.acos(max(-1, min(1, dot / lengths)))


def degrees_to_radians(degrees: float) -> float:
    """
    Converts an angle from degrees to radians.

    :param degrees: Angle in degrees.
    :return: Angle in radians.
    """
    return degrees * DEG_TO_RAD


def radians_to_degrees(radians: float) -> float:
    """
    Converts an angle from radians to degrees.

    :param radians: Angle in radians.
    :return: Angle in degrees.
    """
    return radians * RAD_TO_DEG

# Random Generators


def random_int(min_value: int, max_value: int) -> int:
    """
    Generates a random integer between min_value and max_value.

    :param min_value: Minimum value.
    :param max_value: Maximum value.
    :return: Random integer.
    """
    return random.randint(min_value, max_value)


def random_float(min_value: float, max_value: float) -> float:
    """
    Generates a random float between min_value and max_value.

    :param min_value: Minimum value.
    :param max_value: Maximum value.
    :return: Random float.
    """
    return random.uniform(min_value, max_value)


def random_choice(choices: List) -> any:
    """
    Selects a random element from a list.

    :param choices: List of elements to choose from.
    :return: Randomly selected element.
    """
    return random.choice(choices)

# Collision Detection


def point_in_rect(point: Tuple[float, float], rect: Tuple[float, float, float, float]) -> bool:
    """
    Checks if a point is inside a rectangle.

    :param point: The point (x, y).
    :param rect: The rectangle (x, y, width, height).
    :return: True if the point is inside the rectangle, False otherwise.
    """
    px, py = point
    rx, ry, rw, rh = rect
    return rx <= px <= rx + rw and ry <= py <= ry + rh


def rects_overlap(rect1: Tuple[float, float, float, float], rect2: Tuple[float, float, float, float]) -> bool:
    """
    Checks if two rectangles overlap.

    :param rect1: First rectangle (x, y, width, height).
    :param rect2: Second rectangle (x, y, width, height).
    :return: True if the rectangles overlap, False otherwise.
    """
    r1x, r1y, r1w, r1h = rect1
    r2x, r2y, r2w, r2h = rect2
    return (r1x < r2x + r2w and r1x + r1w > r2x and r1y < r2y + r2h and r1y + r1h > r2y)


def circles_overlap(c1: Tuple[float, float, float], c2: Tuple[float, float, float]) -> bool:
    """
    Checks if two circles overlap.

    :param c1: First circle (x, y, radius).
    :param c2: Second circle (x, y, radius).
    :return: True if the circles overlap, False otherwise.
    """
    c1x, c1y, c1r = c1
    c2x, c2y, c2r = c2
    return distance((c1x, c1y), (c2x, c2y)) < (c1r + c2r)


def clamp_values(input_value, maximum_value):
    """Clamps the input value within the allowed maximum value."""
    return max(-maximum_value, min(maximum_value, input_value))
