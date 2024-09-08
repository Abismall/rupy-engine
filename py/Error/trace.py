import inspect
from typing import Optional


def trace_headerline(filename: str, lineno: int, function_name: str) -> str:
    return f"File \"{filename}\", line {lineno}, in {function_name}\n"


def gather_frame_info(frame_info: inspect.FrameInfo) -> str:
    return getattr(frame_info, 'filename', 'Unknown file'), getattr(frame_info, 'lineno', -1), getattr(frame_info, 'function', 'Unknown function')


def trace_sourceline(*args) -> str:
    try:
        source_lines = inspect.getsourcelines(args)
        return ''.join(source_lines) if source_lines and isinstance(source_lines, str) else ""
    except Exception:
        return 'No context available'


def process_stack_frames(frames: list[inspect.FrameInfo], sourcelines_enabled: bool = False):
    if not frames or not isinstance(frames, list):
        return ['Frames are not defined or is not iterable']
    elif len(frames) <= 1:
        return ['No frames to process']
    else:
        return [f"{trace_headerline(*gather_frame_info(f))}{'No sourcelines collected.' if not sourcelines_enabled else trace_sourceline(f)}" for f in frames[1:]]


def trace(sourcelines=True) -> str:
    try:
        return "\n".join(process_stack_frames(
            inspect.stack(), sourcelines))
    except Exception as e:
        return f"Error capturing stack trace: {str(e)}"


class Trace():
    source_lines_default = True

    def __init__(self, sourcelines: Optional[bool] = True):
        self.source_lines = trace(sourcelines if isinstance(
            sourcelines, bool) else Trace.source_lines_default) or 'Failed to initialize trace'

    def __str__(self) -> str:
        return self.source_lines
