#!/usr/bin/env python3
"""
Runtime Python imports for Ptthon interpreter
"""

from typing import Any, Callable, List, Optional

# pylint: disable=no-name-in-module
# pylint: disable=too-many-lines


# Ipg functions
from .icedpygui import (
    clipboard_read,
    clipboard_write,
    add_button as _add_button,
    add_button_style,
    add_card_style,
    add_card as _add_card,
    add_checkbox_style,
    add_checkbox as _add_checkbox,
    add_combobox as _add_combobox,
    add_color_picker as _add_color_picker,
    add_column as _add_column,
    add_container as _add_container,
    add_container_style,
    add_date_picker as _add_date_picker,
    add_draw as _add_draw,
    DrawMode,
    DrawParam,
    DrawWidget,
    update_draw_params,
    delete_draw_widget,
    add_float as _add_float,
    add_grid as _add_grid,
    add_icon,
    add_image as _add_image,
    add_menu as _add_menu,
    add_menu_bar_item as _add_menu_bar_item,
    add_menu_sub_item as _add_menu_sub_item,
    add_mouse_area as _add_mouse_area,
    add_opaque as _add_opaque,
    add_pick_list as _add_pick_list,
    add_pick_list_style,
    add_progress_bar as _add_progress_bar,
    add_progress_bar_style,
    add_radio as _add_radio,
    add_radio_style,
    add_row as _add_row,
    add_rule as _add_rule,
    add_rule_style,
    add_sash as _add_sash,
    add_sash_style,
    add_scrollable as _add_scrollable,
    add_scrollable_style,
    add_scroller,
    add_autoscroll_style,
    add_rail_style,
    add_separator as _add_separator,
    add_separator_style,
    add_slider as _add_slider,
    add_slider_style,
    add_space as _add_space,
    add_stack as _add_stack,
    add_svg as _add_svg,
    add_table_basic as _add_table_basic,
    add_table_style,
    add_table as _add_table,
    add_table_header as _add_table_header,
    add_table_body as _add_table_body,
    add_table_footer as _add_table_footer,
    add_text_input as _add_text_input,
    add_text_input_style,
    add_text as _add_text,
    add_text_editor as _add_text_editor,
    add_rich_text as _add_rich_text,
    add_span as _add_span,
    add_toggler as _add_toggler,
    add_toggler_style,
    add_tool_tip as _add_tool_tip,
    add_window as _add_window,
    add_event_window,
    add_event_keyboard,
    add_event_mouse,
    add_event_timer,
    add_font_style,
    load_font,
    delete_widget,
    hide_widget,
    move_widget,
    show_widget,
    update_timer,
    update_widget,
    update_widget_params,
    start_session,
    generate_id,
    get_rgba_color,
    get_color_palette,
    Arrow,
    arrow_to_str,
    arrow_variants,
    ButtonParam,
    ButtonStyleParam,
    ButtonStyleStd,
    CardParam,
    CardStyleParam,
    CardStyleStd,
    ContainerStyleStd,
    CheckboxParam,
    CheckboxStyleStd,
    CheckboxStyleParam,
    Color,
    ColumnParam,
    ContainerParam,
    ContainerStyleParam,
    ContentFit,
    DatePickerParam,
    FloatParam,
    FontFamily,
    FontWeight,
    FontStretch,
    FontStyle,
    GridParam,
    Icon,
    ImageParam,
    MenuBarItemParam,
    MenuParam,
    MenuStyleParam,
    MenuSubItemParam,
    MousePointer,
    # PickListHandle,
    ProgressBarParam,
    ProgressBarStyleParam,
    ProgressBarStyleStd,
    RadioParam,
    RadioStyleParam,
    RowParam,
    RuleParam,
    RuleStyleParam,
    SashParam,
    SashStyleParam,
    SashStyleStd,
    ScrollableParam,
    ScrollableStyleParam,
    ScrollerParam,
    SeparatorParam,
    SeparatorStyleParam,
    SliderParam,
    SliderStyleParam,
    StackParam,
    StyleStandard,
    SvgParam,
    TableBasicParam,
    TableStyleParam,
    TextInputParam,
    TextParam,
    TextColorStd,
    RichTextParam,
    SpanParam,
    TimerParam,
    TogglerParam,
    TogglerStyleParam,
    ToolTipParam,
    WindowLevel,
    WindowParam,
    WindowTheme,
)

# ---------------------------------------------------------------------------
# Context stacks — used by Window / Container context managers and
# auto-injecting wrappers so that window_id and parent_id can be inferred.
# ---------------------------------------------------------------------------
_window_stack = []
_parent_stack = []
_id_to_container_str = {}  # maps numeric widget id -> container_id string
_container_to_window_str = {}  # maps container_id string -> window_id string
_id_to_window_str = {}  # maps numeric window id -> window_id string


def _current_window():
    """Return the current window_id from the stack, or None."""
    return _window_stack[-1] if _window_stack else None


def _current_parent():
    """Return the current parent_id from the stack, or None."""
    return _parent_stack[-1] if _parent_stack else None


def _register_container(numeric_id, container_id, window_id):
    """Register id mappings for container instances."""
    _id_to_container_str[numeric_id] = container_id
    _container_to_window_str[container_id] = window_id


# ---------------------------------------------------------------------------
# Thin wrappers that fall back to the context stacks
# ---------------------------------------------------------------------------

def _resolve_parent_id(parent_id):
    """Convert parent_id to a string. If it's an int, look up the container string."""
    if isinstance(parent_id, int):
        try:
            return _id_to_container_str[parent_id]
        except KeyError as exc:
            raise ValueError(
                f"parent_id={parent_id} is not a known container id. "
                "Use the value returned by a container context manager "
                "(e.g. 'with Column() as col')."
            ) from exc
    return parent_id


def _resolve_window_id(window_id):
    """Convert window_id to a string.

    If it is an int, map numeric window id to the window string id.
    """
    if isinstance(window_id, int):
        return _id_to_window_str.get(window_id, str(window_id))
    return window_id


def _current_window_or_parent(parent_id=None):
    """Resolve window_id from window stack, or infer from parent container."""
    window_id = _current_window()
    if window_id is not None:
        return window_id
    if parent_id is None:
        parent_id = _current_parent()
    if parent_id is None:
        return None
    parent_id = _resolve_parent_id(parent_id)
    return _container_to_window_str.get(parent_id)


def _wrap_widget(rust_fn, name):
    """Create a thin wrapper that injects parent_id from the context stack."""
    def wrapper(*, parent_id=None, **kwargs):
        if parent_id is None:
            parent_id = _current_parent()
        if parent_id is None:
            raise ValueError(f"{name}: parent_id is required (either pass it \
                or use a context manager)")
        parent_id = _resolve_parent_id(parent_id)
        return rust_fn(parent_id=parent_id, **kwargs)
    wrapper.__name__ = name
    wrapper.__qualname__ = name
    return wrapper

add_button = _wrap_widget(_add_button, "add_button")
add_button.__doc__ = _add_button.__doc__
add_checkbox = _wrap_widget(_add_checkbox, "add_checkbox")
add_checkbox.__doc__ = _add_checkbox.__doc__
add_combobox = _wrap_widget(_add_combobox, "add_combobox")
add_combobox.__doc__ = _add_combobox.__doc__
add_date_picker = _wrap_widget(_add_date_picker, "add_date_picker")
add_date_picker.__doc__ = _add_date_picker.__doc__
add_image = _wrap_widget(_add_image, "add_image")
add_image.__doc__ = _add_image.__doc__
add_pick_list = _wrap_widget(_add_pick_list, "add_pick_list")
add_pick_list.__doc__ = _add_pick_list.__doc__
add_progress_bar = _wrap_widget(_add_progress_bar, "add_progress_bar")
add_progress_bar.__doc__ = _add_progress_bar.__doc__
add_radio = _wrap_widget(_add_radio, "add_radio")
add_radio.__doc__ = _add_radio.__doc__
add_rule = _wrap_widget(_add_rule, "add_rule")
add_rule.__doc__ = _add_rule.__doc__
add_separator = _wrap_widget(_add_separator, "add_separator")
add_separator.__doc__ = _add_separator.__doc__
add_slider = _wrap_widget(_add_slider, "add_slider")
add_slider.__doc__ = _add_slider.__doc__
add_space = _wrap_widget(_add_space, "add_space")
add_space.__doc__ = _add_space.__doc__
add_span = _wrap_widget(_add_span, "add_span")
add_span.__doc__ = _add_span.__doc__
add_svg = _wrap_widget(_add_svg, "add_svg")
add_svg.__doc__ = _add_svg.__doc__
add_text_input = _wrap_widget(_add_text_input, "add_text_input")
add_text_input.__doc__ = _add_text_input.__doc__
add_text = _wrap_widget(_add_text, "add_text")
add_text.__doc__ = _add_text.__doc__
add_text_editor = _wrap_widget(_add_text_editor, "add_text_editor")
add_text_editor.__doc__ = _add_text_editor.__doc__
add_toggler = _wrap_widget(_add_toggler, "add_toggler")
add_toggler.__doc__ = _add_toggler.__doc__


def _wrap_container(rust_fn, name):
    """Create a thin wrapper that injects window_id and container_id
    from the context stacks."""
    def wrapper(*, container_id=None, window_id=None, parent_id=None, **kwargs):
        if parent_id is None:
            parent_id = _current_parent()
        if parent_id is not None:
            parent_id = _resolve_parent_id(parent_id)
        if window_id is None:
            window_id = _current_window_or_parent(parent_id)
        if window_id is None:
            raise ValueError(f"{name}: window_id is required (either pass it\
                or use a Window context manager)")
        window_id = _resolve_window_id(window_id)
        if container_id is None:
            container_id = str(generate_id())
        numeric_id = rust_fn(window_id=window_id, container_id=container_id,
                             parent_id=parent_id, **kwargs)
        _register_container(numeric_id, container_id, window_id)
        return numeric_id
    wrapper.__name__ = name
    wrapper.__qualname__ = name
    return wrapper


add_card = _wrap_container(_add_card, "add_card")
add_card.__doc__ = _add_card.__doc__
add_color_picker = _wrap_container(_add_color_picker, "add_color_picker")
add_color_picker.__doc__ = _add_color_picker.__doc__
add_column = _wrap_container(_add_column, "add_column")
add_column.__doc__ = _add_column.__doc__
add_container = _wrap_container(_add_container, "add_container")
add_container.__doc__ = _add_container.__doc__
add_draw = _wrap_container(_add_draw, "add_draw")
add_draw.__doc__ = _add_draw.__doc__
add_float = _wrap_container(_add_float, "add_float")
add_float.__doc__ = _add_float.__doc__
add_grid = _wrap_container(_add_grid, "add_grid")
add_grid.__doc__ = _add_grid.__doc__
add_menu = _wrap_container(_add_menu, "add_menu")
add_menu.__doc__ = _add_menu.__doc__
add_menu_bar_item = _wrap_container(_add_menu_bar_item, "add_menu_bar_item")
add_menu_bar_item.__doc__ = _add_menu_bar_item.__doc__
add_menu_sub_item = _wrap_container(_add_menu_sub_item, "add_menu_sub_item")
add_menu_sub_item.__doc__ = _add_menu_sub_item.__doc__
add_mouse_area = _wrap_container(_add_mouse_area, "add_mouse_area")
add_mouse_area.__doc__ = _add_mouse_area.__doc__
add_opaque = _wrap_container(_add_opaque, "add_opaque")
add_opaque.__doc__ = _add_opaque.__doc__
add_rich_text = _wrap_container(_add_rich_text, "add_rich_text")
add_rich_text.__doc__ = _add_rich_text.__doc__
add_row = _wrap_container(_add_row, "add_row")
add_row.__doc__ = _add_row.__doc__
add_sash = _wrap_container(_add_sash, "add_sash")
add_sash.__doc__ = _add_sash.__doc__
add_scrollable = _wrap_container(_add_scrollable, "add_scrollable")
add_scrollable.__doc__ = _add_scrollable.__doc__
add_stack = _wrap_container(_add_stack, "add_stack")
add_stack.__doc__ = _add_stack.__doc__
add_table_basic = _wrap_container(_add_table_basic, "add_table_basic")
add_table_basic.__doc__ = _add_table_basic.__doc__
add_table = _wrap_container(_add_table, "add_table")
add_table.__doc__ = _add_table.__doc__
add_table_header = _wrap_container(_add_table_header, "_add_table_header")
add_table_header.__doc__ = _add_table_header.__doc__
add_table_body = _wrap_container(_add_table_body, "_add_table_body")
add_table_body.__doc__ = _add_table_body.__doc__
add_table_footer = _wrap_container(_add_table_footer, "_add_table_footer")
add_table_footer.__doc__ = _add_table_footer.__doc__
add_tool_tip = _wrap_container(_add_tool_tip, "_add_tool_tip")
add_tool_tip.__doc__ = _add_tool_tip.__doc__

def add_window(*, window_id=None, **kwargs):
    """Wrapper for add_window"""
    if window_id is None:
        window_id = str(generate_id())
    window_id = _resolve_window_id(window_id)
    numeric_id = _add_window(window_id=window_id, **kwargs)
    _id_to_window_str[numeric_id] = window_id
    return numeric_id

add_window.__doc__ = _add_window.__doc__


# ---------------------------------------------------------------------------
# Context managers
# ---------------------------------------------------------------------------

class Window:
    """Wrapper for add_window"""
    def __init__(self, *, window_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else str(generate_id())
        )
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        self.numeric_id = add_window(window_id=self.window_id, **self.kwargs)
        _id_to_window_str[self.numeric_id] = self.window_id
        _window_stack.append(self.window_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _window_stack.pop()
        return False

class Card:
    """Wrapper for add_card"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Container: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_card(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Container:
    """Wrapper for add_container"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Container: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_container(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class ColorPicker:
    """Wrapper for add_column"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("ColorPicker: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_color_picker(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Column:
    """Wrapper for add_column"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Column: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_column(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Draw:
    """Wrapper for add_draw"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Draw: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_draw(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Float:
    """Wrapper for add_float"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Float: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_float(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Grid:
    """Wrapper for add_grid"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Grid: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_grid(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Menu:
    """Wrapper for add_menu"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Menu: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_menu(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class MenuBarItem:
    """Wrapper for add_menu_bar_item"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("MenuBarItem: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_menu_bar_item(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class MenuSubItem:
    """Wrapper for add_menu_sub_item.

    Use inside a ``MenuBarItem`` (or another ``MenuSubItem``) to create a
    nested sub-menu.  The first child added inside this context manager is
    the trigger widget shown in the parent dropdown; all subsequent children
    become the items of the child menu that opens on hover.
    """
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("MenuSubItem: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_menu_sub_item(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class MouseArea:
    """Wrapper for add_mouse_area"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("MouseArea: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_mouse_area(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Opaque:
    """Wrapper for add_opaque"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Opaque: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_opaque(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False


class RichText:
    """Wrapper for add_rich_text"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("RichText: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_rich_text(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Row:
    """Wrapper for add_row"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Row: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_row(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False


class Sash:
    """Wrapper for add_sash"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Sash: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_sash(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False


class Stack:
    """Wrapper for add_stack"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Stack: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_stack(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False


class Scrollable:
    """Wrapper for add_scrollable"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Scrollable: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_scrollable(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class TableBasic:
    """Wrapper for add_table_basic"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("TableBasic: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_table_basic(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class Table:
    """Wrapper for add_table"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("Table: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_table(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class TableHeader:
    """Wrapper for add_table_header"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("TableHeader: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_table_header(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class TableBody:
    """Wrapper for add_table_body"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("TableBody: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_table_body(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class TableFooter:
    """Wrapper for add_table_footer"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("TableFooter: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_table_footer(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False

class ToolTip:
    """Wrapper for add_tool_tip"""
    def __init__(self, *, container_id=None, window_id=None, parent_id=None, **kwargs):
        self.window_id = (
            _resolve_window_id(window_id)
            if window_id is not None
            else _current_window_or_parent(parent_id)
        )
        if self.window_id is None:
            raise ValueError("ToolTip: window_id is required (either pass it\
                or use a Window context manager)")
        self.container_id = (
            container_id
            if container_id is not None
            else str(generate_id())
        )
        self.parent_id = parent_id
        self.kwargs = kwargs
        self.numeric_id = 0

    def __enter__(self):
        pid = self.parent_id or _current_parent()
        if pid is not None:
            pid = _resolve_parent_id(pid)
        self.numeric_id = _add_tool_tip(
            window_id=self.window_id,
            container_id=self.container_id,
            parent_id=pid,
            **self.kwargs,
        )
        _register_container(self.numeric_id, self.container_id, self.window_id)
        _parent_stack.append(self.container_id)
        return self.numeric_id

    def __exit__(self, exc_type, exc_val, exc_tb):
        _parent_stack.pop()
        return False
