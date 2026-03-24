#!/usr/bin/env python3
"""
Runtime Python imports for Ptthon interpreter
"""

from typing import Any, Callable, List, Optional

# pylint: disable=no-name-in-module

# Ipg functions
from .icedpygui import (
    add_button_style,
    add_button as _add_button,
    add_card_style,
    add_card as _add_card,
    add_checkbox_style,
    add_checkbox as _add_checkbox,
    add_color_picker as _add_color_picker,
    add_column as _add_column,
    add_divider as _add_divider,
    add_container as _add_container,
    add_container_style,
    add_date_picker as _add_date_picker,
    add_mouse_area as _add_mouse_area,
    add_divider_style,
    add_pick_list as _add_pick_list,
    add_pick_list_style,
    add_radio as _add_radio,
    add_radio_style,
    add_row as _add_row,
    add_scrollable as _add_scrollable,
    add_scrollable_style,
    add_scroller_param,
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
    add_text_input as _add_text_input,
    add_text_input_style,
    add_text as _add_text,
    add_text_editor as _add_text_editor,
    add_rich_text as _add_rich_text,
    add_span,
    add_toggler as _add_toggler,
    add_toggler_style,
    add_tool_tip as _add_tool_tip,
    add_window as _add_window,
    add_event_window,
    add_event_timer,
    delete_widget,
    hide_widget,
    move_widget,
    show_widget,
    update_timer,
    update_widget,
    start_session,
    generate_id,
    Align,
    AlignX,
    AlignY,
    IpgArrow,
    IpgButtonParam,
    IpgButtonStyleParam,
    IpgButtonStyleStd,
    IpgCardParam,
    IpgCardStyleParam,
    IpgCardStyleStd,
    IpgContainerStyleStd,
    IpgCheckboxParam,
    IpgCheckboxStyleStd,
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
    IpgScrollerParam,
    IpgSelectableTextParam,
    IpgSeparatorParam,
    IpgSeparatorType,
    IpgSeparatorStyleParam,
    IpgSliderParam,
    IpgSliderStyleParam,
    IpgStackParam,
    IpgStyleStandard,
    IpgTableParam,
    IpgTextInputParam,
    IpgTextParam,
    IpgRichTextParam,
    IpgSpanParam,
    IpgTimerParam,
    IpgTogglerParam,
    IpgTogglerStyleParam,
    IpgToolTipParam,
    IpgToolTipPosition,
    IpgWindowLevel,
    IpgWindowParam,
    IpgWindowTheme,
    TextShaping,
    TextWrapping,
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
    def wrapper(*, parent_id=None, **kwargs):
        if parent_id is None:
            parent_id = _current_parent()
        if parent_id is None:
            raise ValueError(f"{name}: parent_id is required (either pass it \
                or use a context manager)")
        return rust_fn(parent_id=parent_id, **kwargs)
    wrapper.__name__ = name
    wrapper.__qualname__ = name
    return wrapper

add_button = _wrap_widget(_add_button, "add_button")
add_button.__doc__ = _add_button.__doc__
add_card = _wrap_widget(_add_card, "add_card")
add_card.__doc__ = _add_card.__doc__
add_checkbox = _wrap_widget(_add_checkbox, "add_checkbox")
add_checkbox.__doc__ = _add_checkbox.__doc__
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
add_text_editor = _wrap_widget(_add_text_editor, "add_text_editor")
add_rich_text = _wrap_widget(_add_rich_text, "add_rich_text")
add_rich_text.__doc__ = _add_rich_text.__doc__
add_toggler = _wrap_widget(_add_toggler, "add_toggler")


def _wrap_container(rust_fn, name):
    """Create a thin wrapper that injects window_id and container_id
    from the context stacks."""
    def wrapper(*, container_id=None, window_id=None, parent_id=None, **kwargs):
        if window_id is None:
            window_id = _current_window()
        if window_id is None:
            raise ValueError(f"{name}: window_id is required (either pass it\
                or use a Window context manager)")
        if container_id is None:
            container_id = str(generate_id())
        if parent_id is None:
            parent_id = _current_parent()
        return rust_fn(window_id=window_id, container_id=container_id, parent_id=parent_id, **kwargs)
    wrapper.__name__ = name
    wrapper.__qualname__ = name
    return wrapper

add_container = _wrap_container(_add_container, "add_container")
add_container.__doc__ = _add_container.__doc__
add_column = _wrap_container(_add_column, "add_column")
add_column.__doc__ = _add_column.__doc__
add_row = _wrap_container(_add_row, "add_row")
add_row.__doc__ = _add_row.__doc__
add_scrollable = _wrap_container(_add_scrollable, "add_scrollable")
add_scrollable.__doc__ = _add_scrollable.__doc__
add_mouse_area = _wrap_container(_add_mouse_area, "add_mouse_area")
add_mouse_area.__doc__ = _add_mouse_area.__doc__
add_stack = _wrap_container(_add_stack, "add_stack")
add_stack.__doc__ = _add_stack.__doc__
add_table = _wrap_container(_add_table, "add_table")
add_table.__doc__ = _add_table.__doc__
add_tool_tip = _wrap_container(_add_tool_tip, "_add_tool_tip")
add_tool_tip.__doc__ = _add_tool_tip.__doc__

def add_window(*, window_id=None, **kwargs):
    """Wrapper for add_window"""
    if window_id is None:
        window_id = str(generate_id())
    return _add_window(window_id=window_id, **kwargs)

add_window.__doc__ = _add_window.__doc__


# ---------------------------------------------------------------------------
# Context managers
# ---------------------------------------------------------------------------

class Window:
    """Wrapper for add_window"""
    def __init__(self, *, window_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else str(generate_id())
        self.kwargs = kwargs

    def __enter__(self):
        self.numeric_id = add_window(window_id=self.window_id, **self.kwargs)
        _window_stack.append(self.window_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _window_stack.pop()
        return False


class Container:
    """Wrapper for add_container"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Container: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = container_id if container_id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        self.numeric_id = _add_container(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=self.parent_id or _current_parent(),
            **self.kwargs,
        )
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False


class Column:
    """Wrapper for add_column"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Column: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = container_id if container_id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        self.numeric_id = _add_column(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=self.parent_id or _current_parent(),
            **self.kwargs,
        )
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False


class Row:
    """Wrapper for add_row"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Row: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = container_id if container_id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        self.numeric_id = _add_row(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=self.parent_id or _current_parent(),
            **self.kwargs,
        )
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Stack:
    """Wrapper for add_stack"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Stack: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = container_id if container_id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        self.numeric_id = _add_stack(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=self.parent_id or _current_parent(),
            **self.kwargs,
        )
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False


class Scrollable:
    """Wrapper for add_scrollable"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("Scrollable: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = container_id if container_id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        self.numeric_id = _add_scrollable(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=self.parent_id or _current_parent(),
            **self.kwargs,
        )
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class ToolTip:
    """Wrapper for add_tool_tip"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = window_id if window_id is not None else _current_window()
        if self.window_id is None:
            raise ValueError("ToolTip: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = container_id if container_id is not None else str(generate_id())
        self.parent_id = parent_id
        self.kwargs = kwargs

    def __enter__(self):
        self.numeric_id = _add_tool_tip(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=self.parent_id or _current_parent(),
            **self.kwargs,
        )
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False
    