import inspect
from typing import Optional


def _trace_headerline(filename: str, lineno: int, function_name: str) -> str:
    return f'File "{filename}", line {lineno}, in {function_name}\n'


def _gather_frame_info(frame_info: inspect.FrameInfo) -> tuple:
    filename = getattr(frame_info, 'filename', 'Unknown file')
    lineno = getattr(frame_info, 'lineno', -1)
    function_name = getattr(frame_info, 'function', 'Unknown function')
    return filename, lineno, function_name


def _trace_sourceline(frame_info: inspect.FrameInfo) -> str:
    try:
        source_lines, _ = inspect.getsourcelines(frame_info.frame)
        return ''.join(source_lines) if source_lines else 'No source lines available.\n'
    except (OSError, TypeError):
        return 'No context available.\n'


def _process_stack_frames(frames: list[inspect.FrameInfo], sourcelines_enabled: bool = False) -> list[str]:
    if not frames or not isinstance(frames, list):
        return ['Frames are not defined or are not iterable.\n']
    if len(frames) <= 1:
        return ['No frames to process.\n']

    trace_lines = []
    for frame in frames[1:]:
        header = _trace_headerline(*_gather_frame_info(frame))
        sourceline = _trace_sourceline(
            frame) if sourcelines_enabled else 'No source lines collected.\n'
        trace_lines.append(f"{header}{sourceline}")
    return trace_lines


def _trace(sourcelines=True) -> str:
    try:
        frames = inspect.stack()
        return ''.join(_process_stack_frames(frames, sourcelines))
    except Exception as e:
        return f"Error capturing stack trace: {str(e)}\n"


class Trace:
    def __init__(self, sourcelines: Optional[bool] = None):
        sourcelines = sourcelines if isinstance(
            sourcelines, bool) else T
        self.source_lines = _trace(
            sourcelines) or 'Failed to initialize trace\n'

    def __str__(self) -> str:
        return self.source_lines
