

from .messages import get_error_message


class PyEngineError(Exception):
    def __init__(self, error_code: str):
        error_info = get_error_message(error_code)
        self.error_code = error_code
        self.category = error_info["category"]
        self.message = error_info["message"]
        super().__init__(f"[{self.category.name}] {self.message}")
