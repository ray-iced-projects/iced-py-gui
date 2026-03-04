# Ipg functions
from .icedpygui import (
    add_button_style, 
    add_button as _add_button, 
    add_checkbox_style,
    add_checkbox as _add_checkbox,
    add_color_picker as _add_color_picker, 
    add_column as _add_column,
    add_divider as _add_divider,
    add_container as _add_container,
    add_container_style,
    add_date_picker as _add_date_picker,
    add_divider_style,
    add_pick_list as _add_pick_list,
    add_pick_list_style,
    add_radio as _add_radio,
    add_radio_style, 
    add_row as _add_row,
    add_scrollable as _add_scrollable,
    add_scrollable_style,
    add_scrollbar,
    add_autoscroll_style,
    add_rail_style,
    add_selectable_text as _add_selectable_text,
    add_separator as _add_separator,
    add_separator_style,
    add_slider as _add_slider,
    add_slider_style, 
    add_space as _add_space,
    add_stack as _add_stack,
    add_table as _add_table,
    add_table_style,
    add_text_input as _add_text_input,
    add_text_input_style,
    add_text as _add_text,
    add_window as _add_window,
    IpgAlignment,
    IpgAlignmentX, 
    IpgArrow,
    IpgButtonParam,
    IpgButtonStyleParam, 
    IpgButtonStyleStandard, 
    IpgCheckboxParam, 
    IpgCheckboxStyleParam,
    IpgColor, 
    IpgColumnParam,
    IpgContainerParam,
    IpgContainerStyleParam,
    IpgDatePickerParam,
    IpgDividerDirection,
    IpgDividerParam,
    IpgDividerStyleParam,            
    IpgIcon, 
    IpgPickListHandle,
    IpgRadioDirection,
    IpgRadioParam,
    IpgRadioStyleParam,
    IpgRowParam,
    IpgScrollableParam,
    IpgScrollableStyleParam,
    IpgScrollbarParam,
    IpgSelectableTextParam,
    IpgSeparatorParam,
    IpgSeparatorType,
    IpgSeparatorStyleParam,
    IpgSliderParam,
    IpgSliderStyleParam,
    IpgStackParam,
    IpgStyleStandard,
    IpgTableParam,
    IpgTableStyleParam,
    IpgTextInputParam,
    IpgTextParam,
    IpgAlignmentY,
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

def _wrap_widget(rust_fn, name):
    """Create a thin wrapper that injects parent_id from the context stack."""
    def wrapper(parent_id=None, **kwargs):
        if parent_id is None:
            parent_id = _current_parent()
        if parent_id is None:
            raise ValueError(f"{name}: parent_id is required (either pass it or use a context manager)")
        return rust_fn(parent_id=parent_id, **kwargs)
    wrapper.__name__ = name
    wrapper.__qualname__ = name
    wrapper.__doc__ = f"Wrapper around the Rust {name}. Falls back to parent stack for parent_id."
    return wrapper

add_button = _wrap_widget(_add_button, "add_button")
add_checkbox = _wrap_widget(_add_checkbox, "add_checkbox")
add_color_picker = _wrap_widget(_add_color_picker, "add_color_picker")
add_date_picker = _wrap_widget(_add_date_picker, "add_date_picker")
add_divider = _wrap_widget(_add_divider, "add_divider")
add_pick_list = _wrap_widget(_add_pick_list, "add_pick_list")
add_radio = _wrap_widget(_add_radio, "add_radio")
add_selectable_text = _wrap_widget(_add_selectable_text, "add_selectable_text")
add_separator = _wrap_widget(_add_separator, "add_separator")
add_slider = _wrap_widget(_add_slider, "add_slider")
add_space = _wrap_widget(_add_space, "add_space")
add_text_input = _wrap_widget(_add_text_input, "add_text_input")
add_text = _wrap_widget(_add_text, "add_text")


def _wrap_container(rust_fn, name):
    """Create a thin wrapper that translates id= to container_id= and
    injects window_id from the window stack."""
    def wrapper(id=None, *, window_id=None, parent_id=None, **kwargs):
        if window_id is None:
            window_id = _current_window()
        if window_id is None:
            raise ValueError(f"{name}: window_id is required (either pass it or use a Window context manager)")
        if id is None:
            id = str(generate_id())
        if parent_id is None:
            parent_id = _current_parent()
        return rust_fn(window_id=window_id, container_id=id, parent_id=parent_id, **kwargs)
    wrapper.__name__ = name
    wrapper.__qualname__ = name
    wrapper.__doc__ = f"Wrapper around the Rust {name}. Accepts id= (maps to container_id=)."
    return wrapper

add_container = _wrap_container(_add_container, "add_container")
add_column = _wrap_container(_add_column, "add_column")
add_row = _wrap_container(_add_row, "add_row")
add_scrollable = _wrap_container(_add_scrollable, "add_scrollable")
add_stack = _wrap_widget(_add_stack, "add_stack")
add_table = _wrap_widget(_add_table, "add_table")

def add_window(id=None, **kwargs):
    """Wrapper around the Rust add_window.

    If *id* is omitted an id is auto-generated.
    """
    if id is None:
        id = str(generate_id())
    return _add_window(window_id=id, **kwargs)


# ---------------------------------------------------------------------------
# Context managers
# ---------------------------------------------------------------------------

class Window:
    """Context manager that calls add_window and tracks the window id.

    Usage::

        with Window(title="My App", center=True) as wnd_id:
            with Container(center=True) as cont_id:
                add_text(content="hello")
    """

    def __init__(self, id=None, **kwargs):
        self.window_id = id if id is not None else str(generate_id())
        self.kwargs = kwargs

    def __enter__(self):
        add_window(id=self.window_id, **self.kwargs)
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

    Usage with explicit id::

        with Container(id="cont", center=True):
            add_text(content="hello")
    """

    def __init__(self, id=None, *, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Container: window_id is required (either pass it or use a Window context manager)")
        self.container_id = id if id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        _add_container(
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


class Column:
    """Context manager wrapper around add_column.

    Usage (reads window_id from Window context)::

        with Window(title="Demo") as wnd_id:
            with Column(id="col", spacing=10.0) as col_id:
                add_text(content="hello")

    Usage with explicit ids::

        with Column(id="col", window_id="main", spacing=10.0):
            add_text(parent_id="col", content="hello")
    """

    def __init__(self, id=None, *, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Column: window_id is required (either pass it or use a Window context manager)")
        self.container_id = id if id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        _add_column(
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


class Row:
    """Context manager wrapper around add_row.

    Usage (reads window_id from Window context)::

        with Window(title="Demo") as wnd_id:
            with Row(id="row", spacing=10.0) as row_id:
                add_text(content="hello")

    Usage with explicit ids::

        with Row(id="row", window_id="main", spacing=10.0):
            add_text(parent_id="row", content="hello")
    """

    def __init__(self, id=None, *, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Row: window_id is required (either pass it or use a Window context manager)")
        self.container_id = id if id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        _add_row(
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

class Stack:
    """Context manager wrapper around add_stack.

    Usage (reads window_id from Window context)::

        with Window(title="Demo") as wnd_id:
            with Stack() as row_id:
                add_text(content="hello")

    Usage with explicit ids::

        with Stack(id="row", window_id="main"):
            add_text(parent_id="row", content="hello")
    """

    def __init__(self, id=None, *, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Row: window_id is required (either pass it or use a Window context manager)")
        self.container_id = id if id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        _add_row(
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
    