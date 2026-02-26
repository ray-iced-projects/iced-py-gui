# Ipg functions
from .icedpygui import (
    add_button_style, 
    add_button, 
    add_checkbox_style,
    add_checkbox,
    add_color_picker, 
    add_column, 
    add_container,
    add_container_style,
    add_date_picker,
    add_pick_list,
    add_pick_list_style,
    add_radio,
    add_radio_style, 
    add_row,
    add_scrollable,
    add_scrollbar,
    add_autoscroll_style,
    add_rail_style,
    add_selectable_text,
    add_separator,
    add_separator_style,
    add_space,
    add_slider,
    add_slider_style,
    add_text as _add_text,
    add_window as _add_window,
    generate_id, 
    IpgAlignment,
    IpgButtonParam,
    IpgButtonStyleParam, 
    IpgButtonStyleStandard, 
    IpgCheckboxParam, 
    IpgCheckboxStyleParam,
    IpgColor, 
    IpgColumnParam,
    IpgContainerStyleParam,
    IpgDatePickerParam,            
    IpgHorizontalAlignment, 
    IpgIcon, 
    IpgPickListHandle,
    IpgRadioDirection,
    IpgRadioParam,
    IpgRadioStyleParam,
    IpgScrollableParam,
    IpgScrollbarParam,
    IpgSelectableTextParam,
    IpgSeparatorParam,
    IpgSeparatorType,
    IpgSeparatorStyleParam,
    IpgSliderParam,
    IpgSliderStyleParam,
    IpgStyleStandard,
    IpgTextParam,
    IpgVerticalAlignment,
    IpgWindowLevel, 
    IpgWindowMode, 
    IpgWindowTheme,
    start_session, 
    update_widget,
    generate_id,
)


# ---------------------------------------------------------------------------
# Context stacks — used by Window / Container context managers and
# auto-injecting wrappers so that window_id and parent_id can be inferred.
# ---------------------------------------------------------------------------
_window_stack = []
_parent_stack = []


def _current_window():
    """Return the current window_id from the stack, or None."""
    return _window_stack[-1] if _window_stack else None


def _current_parent():
    """Return the current parent_id from the stack, or None."""
    return _parent_stack[-1] if _parent_stack else None


# ---------------------------------------------------------------------------
# Thin wrappers that fall back to the context stacks
# ---------------------------------------------------------------------------

def add_window(window_id=None, **kwargs):
    """Wrapper around the Rust add_window.

    If *window_id* is omitted an id is auto-generated.
    """
    if window_id is None:
        window_id = str(generate_id())
    return _add_window(window_id=window_id, **kwargs)


def add_text(parent_id=None, **kwargs):
    """Wrapper around the Rust add_text.

    If *parent_id* is omitted it is read from the parent stack
    (set by the innermost Container / Column / Row / … context manager).
    """
    if parent_id is None:
        parent_id = _current_parent()
    if parent_id is None:
        raise ValueError("add_text: parent_id is required (either pass it or use a context manager)")
    return _add_text(parent_id=parent_id, **kwargs)


# ---------------------------------------------------------------------------
# Context managers
# ---------------------------------------------------------------------------

class Window:
    """Context manager that calls add_window and tracks the window_id.

    Usage::

        with Window(title="My App", center=True) as wnd_id:
            with Container(center=True) as cont_id:
                add_text(content="hello")
    """

    def __init__(self, window_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else str(generate_id())
        self.kwargs = kwargs

    def __enter__(self):
        add_window(window_id=self.window_id, **self.kwargs)
        _window_stack.append(self.window_id)
        return self.window_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _window_stack.pop()
        return False


class Container:
    """Context manager wrapper around add_container.

    Usage with auto-generated ids (reads window_id from Window context)::

        with Window(title="Demo") as wnd_id:
            with Container(center=True, width_fill=True) as cont_id:
                add_text(content="hello")

    Usage with explicit ids::

        with Container("main", "cont", center=True):
            add_text(parent_id="cont", content="hello")
    """

    def __init__(self, window_id=None, container_id=None, *, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Container: window_id is required (either pass it or use a Window context manager)")
        self.container_id = container_id if container_id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        add_container(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=self.parent_id or _current_parent(),
            **self.kwargs,
        )
        _parent_stack.append(self.container_id)
        return self.container_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False
