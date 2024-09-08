import inspect
from typing import Optional


def _trace_headerline(filename: str, lineno: int, function_name: str) -> str:
    return f"File \"{filename}\", line {lineno}, in {function_name}\n"


def _gather_frame_info(frame_info: inspect.FrameInfo) -> str:
    return getattr(frame_info, 'filename', 'Unknown file'), getattr(frame_info, 'lineno', -1), getattr(frame_info, 'function', 'Unknown function')


def _trace_sourceline(*args) -> str:
    try:
        source_lines = inspect.getsourcelines(args)
        return ''.join(source_lines) if source_lines and isinstance(source_lines, str) else ""
    except Exception:
        return 'No context available'


def _process_stack_frames(frames: list[inspect.FrameInfo], sourcelines_enabled: bool = False):
    if not frames or not isinstance(frames, list):
        return ['Frames are not defined or is not iterable']
    elif len(frames) <= 1:
        return ['No frames to process']
    else:
        return [f"{_trace_headerline(*_gather_frame_info(f))}{'No sourcelines collected.' if not sourcelines_enabled else _trace_sourceline(f)}" for f in frames[1:]]


def _trace(sourcelines=True) -> str:
    try:
        return "\n".join(_process_stack_frames(
            inspect.stack(), sourcelines))
    except Exception as e:
        return f"Error capturing stack trace: {str(e)}"


class Trace():
    source_lines_default = True

    def __init__(self, sourcelines: Optional[bool] = True):
        self.source_lines = _trace(sourcelines if isinstance(
            sourcelines, bool) else Trace.source_lines_default) or 'Failed to initialize trace'

    def __str__(self) -> str:
        return self.source_lines
