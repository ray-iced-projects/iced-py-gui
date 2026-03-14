from typing import Any, Callable, Optional
from types import TracebackType

# Re-export everything that is directly imported (not wrapped)
from .icedpygui import (
    add_button_style as add_button_style,
    add_checkbox_style as add_checkbox_style,
    add_container_style as add_container_style,
    add_divider_style as add_divider_style,
    add_mouse_area as add_mouse_area,
    add_pick_list_style as add_pick_list_style,
    add_radio_style as add_radio_style,
    add_scrollable_style as add_scrollable_style,
    add_scroller_param as add_scroller_param,
    add_autoscroll_style as add_autoscroll_style,
    add_rail_style as add_rail_style,
    add_separator_style as add_separator_style,
    add_slider_style as add_slider_style,
    add_text_input_style as add_text_input_style,
    add_toggler_style as add_toggler_style,
    Align as Align,
    AlignX as AlignX,
    AlignY as AlignY,
    IpgArrow as IpgArrow,
    IpgButtonParam as IpgButtonParam,
    IpgButtonStyleParam as IpgButtonStyleParam,
    IpgButtonStyleStd as IpgButtonStyleStd,
    IpgContainerStyleStd as IpgContainerStyleStd,
    IpgCheckboxParam as IpgCheckboxParam,
    IpgCheckboxStyleStd as IpgCheckboxStyleStd,
    IpgCheckboxStyleParam as IpgCheckboxStyleParam,
    IpgColor as IpgColor,
    IpgColumnParam as IpgColumnParam,
    IpgContainerParam as IpgContainerParam,
    IpgContainerStyleParam as IpgContainerStyleParam,
    IpgDatePickerParam as IpgDatePickerParam,
    IpgDividerDirection as IpgDividerDirection,
    IpgDividerParam as IpgDividerParam,
    IpgDividerStyleParam as IpgDividerStyleParam,
    IpgIcon as IpgIcon,
    IpgMousePointer as IpgMousePointer,
    IpgPickListHandle as IpgPickListHandle,
    IpgRadioDirection as IpgRadioDirection,
    IpgRadioParam as IpgRadioParam,
    IpgRadioStyleParam as IpgRadioStyleParam,
    IpgRowParam as IpgRowParam,
    IpgScrollableParam as IpgScrollableParam,
    IpgScrollableStyleParam as IpgScrollableStyleParam,
    IpgScrollerParam as IpgScrollerParam,
    IpgSelectableTextParam as IpgSelectableTextParam,
    IpgSeparatorParam as IpgSeparatorParam,
    IpgSeparatorType as IpgSeparatorType,
    IpgSeparatorStyleParam as IpgSeparatorStyleParam,
    IpgSliderParam as IpgSliderParam,
    IpgSliderStyleParam as IpgSliderStyleParam,
    IpgStackParam as IpgStackParam,
    IpgStyleStandard as IpgStyleStandard,
    IpgTableParam as IpgTableParam,
    IpgTextInputParam as IpgTextInputParam,
    IpgTextParam as IpgTextParam,
    IpgTogglerParam as IpgTogglerParam,
    IpgTogglerStyleParam as IpgTogglerStyleParam,
    IpgWindowLevel as IpgWindowLevel,
    IpgWindowMode as IpgWindowMode,
    IpgWindowParam as IpgWindowParam,
    IpgWindowTheme as IpgWindowTheme,
    TextShaping as TextShaping,
    TextWrapping as TextWrapping,
    start_session as start_session,
    update_widget as update_widget,
    generate_id as generate_id,
)

# ---------------------------------------------------------------------------
# Wrapped widget functions — full signatures with parent_id made optional
# ---------------------------------------------------------------------------

def add_button(
    *,
    parent_id: Optional[str] = None,
    label: str = ...,
    gen_id: Optional[int] = None,
    on_press: Optional[Callable] = None,
    width: Optional[float] = None,
    width_fill: bool = False,
    height: Optional[float] = None,
    height_fill: bool = False,
    padding: Optional[list[float]] = None,
    text_top_left: Optional[bool] = None,
    text_top_center: Optional[bool] = None,
    text_top_right: Optional[bool] = None,
    text_center_left: Optional[bool] = None,
    text_center: bool = True,
    text_center_right: Optional[bool] = None,
    text_bottom_left: Optional[bool] = None,
    text_bottom_center: Optional[bool] = None,
    text_bottom_right: Optional[bool] = None,
    text_size: Optional[float] = None,
    clip: Optional[bool] = None,
    style_id: Optional[int] = None,
    style_standard: Optional[IpgButtonStyleStd] = None,
    style_arrow: Optional[IpgArrow] = None,
    user_data: Optional[Any] = None,
    show: bool = True,
) -> int:
    """Adds a button widget.
    
    A widget must go into a container type,
    
    i.e. Container, Column, Row, etc.
    
    Usage::

        with Window(title="My App", pos_centered=True) as wnd_id: (if needed)
            with Container(align_center_center=True):
                add_button(label="Press Me")

    """
    ...

# ---------------------------------------------------------------------------
# Other wrapped widget functions — TODO: add full signatures later
# ---------------------------------------------------------------------------

def add_checkbox(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_color_picker(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_date_picker(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_divider(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_pick_list(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_radio(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_selectable_text(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_separator(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_slider(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_space(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_text_input(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_text(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...
def add_toggler(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...

# ---------------------------------------------------------------------------
# Wrapped container functions — full signatures
# ---------------------------------------------------------------------------

def add_container(
    *,
    id: Optional[str] = None,
    window_id: Optional[str] = None,
    parent_id: Optional[str] = None,
    width: Optional[float] = None,
    width_fill: bool = False,
    height: Optional[float] = None,
    height_fill: bool = False,
    clip: Optional[bool] = None,
    max_height: Optional[float] = None,
    max_width: Optional[float] = None,
    align_bottom_center: Optional[bool] = None,
    align_bottom_left: Optional[bool] = None,
    align_bottom_right: Optional[bool] = None,
    align_center: Optional[bool] = None,
    align_center_left: Optional[bool] = None,
    align_center_right: Optional[bool] = None,
    align_top_center: Optional[bool] = None,
    align_top_left: Optional[bool] = None,
    align_top_right: Optional[bool] = None,
    padding: Optional[list[float]] = None,
    show: bool = True,
    style_id: Optional[int] = None,
) -> int:
    ...

def add_column(
    *,
    id: Optional[str] = None,
    window_id: Optional[str] = None,
    parent_id: Optional[str] = None,
    align: Align = ...,
    width: Optional[float] = None,
    height: Optional[float] = None,
    width_fill: bool = False,
    height_fill: bool = False,
    max_width: float = ...,
    padding: Optional[list[float]] = None,
    spacing: float = 20.0,
    clip: bool = False,
    show: bool = True,
) -> int:
    ...

def add_mouse_area(
    window_id: str,
    container_id: str,
    *,
    parent_id: Optional[str]=None,
    gen_id: Optional[int]=None,
    on_press: Optional[Callable]=None, 
    on_release: Optional[Callable]=None, 
    on_right_press: Optional[Callable]=None, 
    on_right_release: Optional[Callable]=None, 
    on_middle_press: Optional[Callable]=None, 
    on_middle_release: Optional[Callable]=None,
    on_enter: Optional[Callable]=None,
    on_move: Optional[Callable]=None,
    on_exit: Optional[Callable]=None,
    mouse_pointer: Optional[IpgMousePointer]=None,
    show: bool=True,  
    user_data: Optional[any]=None,
) -> int:
    ...
    
def add_row(
    *,
    id: Optional[str] = None,
    window_id: Optional[str] = None,
    parent_id: Optional[str] = None,
    align: Align = ...,
    width: Optional[float] = None,
    height: Optional[float] = None,
    width_fill: bool = False,
    height_fill: bool = False,
    padding: Optional[list[float]] = None,
    spacing: float = 20.0,
    clip: bool = False,
    show: bool = True,
) -> int:
    ...

def add_scrollable(
    *,
    id: Optional[str] = None,
    window_id: Optional[str] = None,
    parent_id: Optional[str] = None,
    width: Optional[float] = None,
    height: Optional[float] = None,
    width_fill: bool = False,
    height_fill: bool = False,
    direction: Any = None,
    h_bar_width: float = 10.0,
    h_bar_margin: float = 0.0,
    h_scroller_width: float = 10.0,
    h_spacing: float = 0.0,
    h_bar_alignment: Align = ...,
    v_bar_width: float = 10.0,
    v_bar_margin: float = 0.0,
    v_scroller_width: float = 10.0,
    v_spacing: float = 0.0,
    v_bar_alignment: Align = ...,
    on_scroll: Optional[Callable] = None,
    style_id: Optional[str] = None,
    user_data: Optional[Any] = None,
) -> int:    
    ...

def add_stack(
    *,
    parent_id: Optional[str] = None,
    width: Optional[float] = None,
    height: Optional[float] = None,
    width_fill: bool = False,
    height_fill: bool = False,
    hide_index: Optional[int] = None,
    show: bool = True,
) -> int:
    ...

def add_table(*, parent_id: Optional[str] = None, **kwargs: Any) -> int: ...

# ---------------------------------------------------------------------------
# add_window wrapper
# ---------------------------------------------------------------------------

def add_window(
    *,
    window_id: Optional[str] = None,
    title: str = ...,
    width: float = ...,
    height: float = ...,
    max_width: Optional[float] = None,
    max_height: Optional[float] = None,
    min_width: Optional[float] = None,
    min_height: Optional[float] = None,
    pos_x: Optional[float] = None,
    pos_y: Optional[float] = None,
    center: bool = False,
    resizable: bool = True,
    decorations: bool = True,
    transparent: bool = False,
    level: IpgWindowLevel = ...,
    theme: IpgWindowTheme = ...,
    debug: bool = False,
    exit_on_close: bool = True,
    mode: IpgWindowMode = ...,
    gen_id: Optional[int] = None,
) -> int:
    """Context manager that calls add_window and tracks the window id.
    
    A window is not a container for widgets, only other containers.
    
    Usage::

        add_window(window_id="main", title="My App", center=True)
            with Container(align_center_center=True):
                add_text(content="hello")

    """
    ...

# ---------------------------------------------------------------------------
# Context managers
# ---------------------------------------------------------------------------

class Window:
    """Context manager that calls add_window and tracks the window id.
    
    A window is not a container for widgets, only other containers.

    Usage::

        with Window(title="My App", pos_centered=True) as wnd_id: (if needed)
            with Container(align_center_center=True):
                add_text(content="hello")

    """
    def __init__(
        self,
        *,
        id: Optional[str] = None,
        title: str = ...,
        width: float = ...,
        height: float = ...,
        max_width: Optional[float] = None,
        max_height: Optional[float] = None,
        min_width: Optional[float] = None,
        min_height: Optional[float] = None,
        pos_x: Optional[float] = None,
        pos_y: Optional[float] = None,
        center: bool = False,
        resizable: bool = True,
        decorations: bool = True,
        transparent: bool = False,
        level: IpgWindowLevel = ...,
        theme: IpgWindowTheme = ...,
        debug: bool = False,
        exit_on_close: bool = True,
        mode: IpgWindowMode = ...,
        gen_id: Optional[int] = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None) -> bool: ...


class Container:
    """Context manager wrapper around add_container.

    Wraps the iced Container — a widget that aligns its contents inside
    of its boundaries.  Unlike a row or column a container can only have
    one child.

    Usage::

        with Window(title="Demo"):
            with Container(align_center=True, width_fill=True):
                add_text(content="hello")

    """
    def __init__(
        self,
        *,
        id: Optional[str] = None,
        window_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        width: Optional[float] = None,
        width_fill: bool = False,
        height: Optional[float] = None,
        height_fill: bool = False,
        clip: Optional[bool] = None,
        max_height: Optional[float] = None,
        max_width: Optional[float] = None,
        align_bottom_center: Optional[bool] = None,
        align_bottom_left: Optional[bool] = None,
        align_bottom_right: Optional[bool] = None,
        align_center: Optional[bool] = None,
        align_center_left: Optional[bool] = None,
        align_center_right: Optional[bool] = None,
        align_top_center: Optional[bool] = None,
        align_top_left: Optional[bool] = None,
        align_top_right: Optional[bool] = None,
        padding: Optional[list[float]] = None,
        show: bool = True,
        style_id: Optional[int] = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None) -> bool: ...


class Column:
    """Context manager wrapper around add_column.

    A container that distributes its contents vertically.

    Usage::

        with Window(title="Demo"):
            with Column(spacing=10.0):
                add_text(content="hello")
                add_text(content="hello")

    """
    def __init__(
        self,
        *,
        id: Optional[str] = None,
        window_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        align: Align = ...,
        width: Optional[float] = None,
        height: Optional[float] = None,
        width_fill: bool = False,
        height_fill: bool = False,
        max_width: float = ...,
        padding: Optional[list[float]] = None,
        spacing: float = 20.0,
        clip: bool = False,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None) -> bool: ...


class MouseArea:
    """Context Manager wrapper for add_mousearea
    
    A container that is like Container but allows
    mouse interactions.
    
    Usage::
    
        with Window(title="Demo"):
            with MouseArea():
                add_svg() # Your svg will have mouse interaction    
    
    """
    def __init__(
        self,
        *,
        window_id: str,
        container_id: str,
        parent_id: Optional[str]=None,
        gen_id: Optional[int]=None,
        on_press: Optional[Callable]=None, 
        on_release: Optional[Callable]=None, 
        on_right_press: Optional[Callable]=None, 
        on_right_release: Optional[Callable]=None, 
        on_middle_press: Optional[Callable]=None, 
        on_middle_release: Optional[Callable]=None,
        on_enter: Optional[Callable]=None,
        on_move: Optional[Callable]=None,
        on_exit: Optional[Callable]=None,
        mouse_pointer: Optional[IpgMousePointer]=None,
        show: bool=True,  
        user_data: Optional[any]=None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None) -> bool: ...


class Row:
    """Context manager wrapper around add_row.

    A container that distributes its contents horizontally.

    Usage::

        with Window(title="Demo"):
            with Row(spacing=10.0):
                add_text(content="hello")
                add_text(content="hello")

    """
    def __init__(
        self,
        *,
        id: Optional[str] = None,
        window_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        align: Align = ...,
        width: Optional[float] = None,
        height: Optional[float] = None,
        width_fill: bool = False,
        height_fill: bool = False,
        padding: Optional[list[float]] = None,
        spacing: float = 20.0,
        clip: bool = False,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None) -> bool: ...


class Stack:
    """Context manager wrapper around add_stack.

    A stack of containers.

    Usage::

        with Window(title="Demo"):
            with Stack():
                add_text(content="hello")

    """
    def __init__(
        self,
        *,
        id: Optional[str] = None,
        window_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        width: Optional[float] = None,
        height: Optional[float] = None,
        width_fill: bool = False,
        height_fill: bool = False,
        hide_index: Optional[int] = None,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None) -> bool: ...


class Scrollable:
    """Context manager wrapper around add_scrollable.

    Usage::

        with Window(title="Demo"):
            with Scrollable(width=200.0, height=100.0):
                add_text(content="long text...")

    """
    def __init__(
        self,
        *,
        id: Optional[str] = None,
        window_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        width: Optional[float] = None,
        height: Optional[float] = None,
        width_fill: bool = False,
        height_fill: bool = False,
        direction: Any = None,
        h_bar_width: float = 10.0,
        h_bar_margin: float = 0.0,
        h_scroller_width: float = 10.0,
        h_spacing: float = 0.0,
        h_bar_alignment: Align = ...,
        v_bar_width: float = 10.0,
        v_bar_margin: float = 0.0,
        v_scroller_width: float = 10.0,
        v_spacing: float = 0.0,
        v_bar_alignment: Align = ...,
        on_scroll: Optional[Callable] = None,
        style_id: Optional[str] = None,
        user_data: Optional[Any] = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: TracebackType | None) -> bool: ...
