#!/usr/bin/env python3
# pylint: disable=useless-import-alias
# pylint: disable=no-name-in-module
# pylint: disable=unused-argument
# pylint: disable=unnecessary_ellipsis
# pylint: disable=too-many-arguments
# pylint: disable=too-many-locals
# pylint: disable=too-many-lines
"""
Re-Imports for Type stubs for the wrapper layer,
Wwhat users see with pylance, mypy, IDE hover, etc
"""
from typing import Any, Callable, Optional
from types import TracebackType

# Re-export everything that is directly imported (not wrapped)
from .icedpygui import (
    add_button_style as add_button_style,
    add_card_style as add_card_style,
    add_checkbox_style as add_checkbox_style,
    add_container_style as add_container_style,
    add_divider_style as add_divider_style,
    add_font_style as add_font_style,
    add_menu_style as add_menu_style,
    add_opaque_style as add_opaque_style,
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
    Arrow as Arrow,
    ButtonParam as ButtonParam,
    ButtonStyleParam as ButtonStyleParam,
    ButtonStyleStd as ButtonStyleStd,
    CardParam as CardParam,
    CardStyleParam as CardStyleParam,
    CardStyleStd as CardStyleStd,
    CheckboxParam as CheckboxParam,
    CheckboxStyleStd as CheckboxStyleStd,
    CheckboxStyleParam as CheckboxStyleParam,
    Color as Color,
    ColumnParam as ColumnParam,
    ContainerParam as ContainerParam,
    ContainerStyleParam as ContainerStyleParam,
    ContainerStyleStd as ContainerStyleStd,
    ContentFit as ContentFit,
    DatePickerParam as DatePickerParam,
    DividerDirection as DividerDirection,
    DividerParam as DividerParam,
    DividerStyleParam as DividerStyleParam,
    FilterMethod,
    FloatParam as FloatParam,
    FontFamily as FontFamily,
    FontWeight as FontWeight,
    FontStretch as FontStretch,
    FontStyle as FontStyle,
    GridParam as GridParam,
    Icon as Icon,
    MenuParam as MenuParam,
    MenuStyleParam as MenuStyleParam,
    MousePointer as MousePointer,
    OpaqueParam as OpaqueParam,
    PickListHandle as PickListHandle,
    RadioDirection as RadioDirection,
    RadioParam as RadioParam,
    RadioStyleParam as RadioStyleParam,
    Rotation,
    RowParam as RowParam,
    ScrollableParam as ScrollableParam,
    ScrollableStyleParam as ScrollableStyleParam,
    ScrollerParam as ScrollerParam,
    SeparatorParam as SeparatorParam,
    SeparatorType as SeparatorType,
    SeparatorStyleParam as SeparatorStyleParam,
    SliderParam as SliderParam,
    SliderStyleParam as SliderStyleParam,
    StackParam as StackParam,
    StyleStandard as StyleStandard,
    TableParam as TableParam,
    TextInputParam as TextInputParam,
    TextParam as TextParam,
    TextColorStd as TextColorStd,
    TinerParam as TimerParam,
    TogglerParam as TogglerParam,
    TogglerStyleParam as TogglerStyleParam,
    ToolTipParam as ToolTipParam,
    ToolTipPosition as ToolTipPosition,
    WindowLevel as WindowLevel,
    WindowMode as WindowMode,
    WindowParam as WindowParam,
    WindowTheme as WindowTheme,
    add_event_window,
    add_event_keyboard,
    add_event_mouse,
    add_event_timer,
    start_session as start_session,
    delete_widget as delete_widget,
    hide_widget as hide_widget,
    move_widget as move_widget,
    show_widget as show_widget,
    update_timer as update_timer,
    update_widget as update_widget,
    update_widget_params as update_widget_params,
    load_font as load_font,
    generate_id as generate_id,
    get_rgba_color,
    get_color_palette,
)


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
    if_menu_btn: bool = False,
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
    style_standard: Optional[ButtonStyleStd] = None,
    style_arrow: Optional[Arrow] = None,
    user_data: Optional[Any] = None,
    show: bool = True,
) -> int:
    """Adds a button widget.
    
    A widget must go into a container type,
    
    i.e. Container, Column, Row, etc.
    
    Usage::

        with Window(title="My App", pos_centered=True) as wnd_id: (if needed)
            with Container(align_center=True):
                add_button(label="Press Me")

    """
    ...

def add_card(
    *,
    parent_id: Optional[str] = None,
    head: Optional[str] = None,
    body: Optional[str] = None,
    is_open: bool = True,
    min_max_id: Optional[int] = None,
    foot: Optional[str] = None,
    gen_id: Optional[int] = None,
    close_size: Optional[float] = None,
    on_close: Optional[Any] = None,
    width: Optional[float] = None,
    width_fill: bool = False,
    height: Optional[float] = None,
    height_fill: bool = False,
    max_width: Optional[float] = None,
    max_height: Optional[float] = None,
    padding: Optional[list[float]] = None,
    padding_head: Optional[list[float]] = None,
    padding_body: Optional[list[float]] = None,
    padding_foot: Optional[list[float]] = None,
    style_id: Optional[int] = None,
    style_std: Optional[CardStyleStd] = None,
    style_button: Optional[int] = None,
    show: bool = True,
    user_data: Optional[Any] = None,
) -> int:
    """Adds a button widget.
    
    A widget must go into a container type,
    
    i.e. Container, Column, Row, etc.
    
    Usage::

        with Window(title="My App", pos_centered=True) as wnd_id: (if needed)
            with Container(align_center=True):
                add_card(head="Card Header", body="Card Body)

    """
    ...

def add_checkbox(
    *,
    parent_id: Optional[str] = None,
    on_toggle: Optional[Callable] = None,
    is_checked: bool = False,
    label: Optional[str] = None,
    width: Optional[float] = None,
    width_fill: bool = False,
    size: Optional[float] = None,
    spacing: Optional[float] = None,
    text_size: Optional[float] = None,
    text_line_height: Optional[float] = None,
    text_shaping: Optional[bool] = None,
    text_wrapping: Optional[bool] = None,
    text_font_id: Optional[int] = None,
    icon_font_id: Optional[int] = None,
    icon: Optional[Icon] = None,
    icon_size: Optional[float] = None,
    icon_line_height: Optional[float] = None,
    icon_shaping: Optional[bool] = None,
    user_data: Optional[Any] = None,
    show: bool = True,
    style_id: Optional[int] = None,
    style_std: Optional[int] = None,
    gen_id: Optional[int] = None,
) -> int:
    """Adds a checkbox widget.
    
    A widget must go into a container type,
    
    i.e. Container, Column, Row, etc.
    
    Usage::
        def checked(chk_id: int, is_checked: bool):
            print(chk_id, is_checked)
        
        with Window(title="My App", pos_centered=True) as wnd_id: (if needed)
            with Container(align_center=True):
                add_checkbox(
                    label="Check Me",
                    on_toggle=checked)

    """
    ...
def add_color_picker(
    *,
    parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """Adds a Color Picker widget.
    
    Usage::

        def color_selected(cp_id: int, color: list):
            print(cp_id, color)
            
        def cp_opened(_cp_id: int, _user_data: any):
            print("color picker opened")
            

        def cp_canceled(_cp_id: int, _user_data: any):
            print("color picker canceled")

        add_color_picker(
            on_press=cp_opened, # Button to open color picker
            on_select=color_selected, # the color selection selected
            on_cancel=cp_canceled,  # color selection was canceled
            )
    
    Returns:
        int: widget id
    """
    ...
def add_date_picker(
    *,
    parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...
def add_divider(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...
def add_float(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...
def add_image(
    path: str,
    *,
    parent_id: Optional[str] = None,
    border_radius: Optional[list[float, 4] | list[float, 1]] = None,
    content_fit: Optional[ContentFit] = None,
    crop_height: Optional[float] = None,
    crop_width: Optional[float] = None,
    crop_x: Optional[float] = None,
    crop_y: Optional[float] = None,
    expand: Optional[bool] = None,
    fill: Optional[bool] = None,
    filter_method: Optional[FilterMethod] = None,
    gen_id: Optional[int] = None,
    height_fill: Optional[bool] = None,
    height: Optional[float] = None,
    opacity: Optional[float] = None,
    rotation_degrees: Optional[float] = None,
    rotation_method: Optional[Rotation] = None,
    rotation_radians: Optional[float] = None,
    scale: Optional[float] = None,
    show: bool = True,
    width_fill: Optional[bool] = None,
    width: Optional[float] = None,
    ) -> int:
    """
        add_image parameters
    Usage::
        with Window(title="My App", center=True):
            with Container(align_center=True):
                add_image(path="my path to image)
                
        start_session()
        
    Args::
        path: str
            Sets the path to where the image is located.
        parent_id: str, Optional
            Set the parent_id, if needed.
        border_radius: list[float, 4] | list[float], Optional
            Sets the border radius of the image either all corner same 
            value [float] or independent [float,4].
        content_fit: ContentFit, Optional
            Set how the image contents fits see ContentFit class.
        crop_height: float, Optional
            Sets the height of the crop rectangle.
        crop_width: float, Optional
            Sets the crop width of the crop rectangle.
        crop_x: float, Optional
            Sets the origin x of the crop rectangle.
        crop_y: float, Optional
            Sets the origin y of the crop rectangle.
        expand: float, Optional
            Sets whether the image should try to fill as much space
            available as possible while keeping aspect ratio and without
            allocating extra space in any axis.
        filter_method: FilterMethod, Optional
            Sets the filter method, see FilterMethod
        fill: bool, Optional
            Sets both width_fill and length_fill.
        height: float, Optional
            Sets the height of the widget. 
        height_fill: bool, Optional
            Sets the height to fill the available space, overrides height.
        opacity: float, Optional
            Sets the opacity of the image.
        padding: list[float], Optional
            Sets the padding around the image.
        rotation_degrees: float, Optional
            Sets the roation of the image in degrees format.
        rotation_radians: float, Optional
            Sets the rotate the image in radians.
        rotation_method: Rotation, Optional
            Set the rotation method, see Rotation.
        scale: float, Optional
            Sets the scale factor of the Image.
            The region of the Image drawn will be scaled from the center by the given scale factor.
        show: bool, Optional
            Whether to show or hide the image.
        width: float, Optional
            Sets the width of the image.
        width_fill: bool, Optional
            Whether to fill the width to the available container size.

    Returns:
        int: widget id
    """
    ...
def add_pick_list(
    *,
    parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """
    Adds a Pick List widget
    A widget must go into a container type.
    
    i.e. Container, Column, Row, etc.
    
    Usage::
        def picked_item(pl_id: int, data: str):
            print(f"pl_id = {pl_id} data = {data}")
            
        options = ["One", "Two", "Three"]
        
        with Window(title="My App", pos_centered=True) as wnd_id: (if needed)
            with Container(align_center=True):
                add_pick_list(
                    options=options,
                    placeholder="Choose a Number...",
                    on_select=picked_item)

    Returns:
        int: widget id
    """
    ...
def add_radio(
    *,
    parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_separator(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_slider(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_space(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_svg(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_text_input(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_text(
    *,
    parent_id: Optional[str] = None,
    content: str,
    width: Optional[float] = None,
    width_fill: Optional[bool] = None,
    height: Optional[float] = None,
    height_fill: Optional[bool] = None,
    fill: Optional[bool] = None,
    align_bottom_center: Optional[bool] = None,
    align_bottom_left: Optional[bool] = None,
    align_bottom_right: Optional[bool] = None,
    align_center_left: Optional[bool] = None,
    align_center_right: Optional[bool] = None,
    align_center: Optional[bool] = None,
    align_top_center: Optional[bool] = None,
    align_top_left: Optional[bool] = None,
    align_top_right: Optional[bool] = None,
    font_id: Optional[int] = None,
    size: Optional[float] = None,
    line_height: Optional[float] = None,
    color: Optional[Color] = None,
    color_alpha: Optional[float] = None,
    color_rgba: Optional[list[float, 4]] = None,
    color_std: Optional[TextColorStd] = None,
    wrapping_none: Optional[bool] = None,
    wrapping_glyph: Optional[bool] = None,
    wrapping_word_glyph: Optional[bool] = None,
    show: bool = True,
    gen_id: Optional[int] = None,
    ) -> int:
    """Adds a text widget.
    
    Usage::

        with Window(title="My App", pos_centered=True):
            with Container(align_center=True):
                add_text(content="Some Text")

    Run the doc_help.py to see arguement descriptions.
    """
    ...
def add_text_editor(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_toggler(
    *, parent_id: Optional[str] = None,
    **kwargs: Any) -> int:
    """
    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...

# ---------------------------------------------------------------------------
# Wrapped container functions — full signatures
# ---------------------------------------------------------------------------

def add_container(
    *,
    container_id: Optional[str] = None,
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
    style_std: Optional[ContainerStyleStd] = None,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...

def add_column(
    *,
    container_id: Optional[str] = None,
    window_id: Optional[str] = None,
    parent_id: Optional[str] = None,
    align_left: Optional[bool] = None,
    align_center: Optional[bool] = None,
    align_right: Optional[bool] = None,
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
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...

def add_menu(
    *,
    window_id: Optional[str] = None,
    container_id: Optional[str] = None,
    bar_items: Optional[str] = None,
    menu_items: list[int],
    parent_id: Optional[str] = None,
    item_offset: Optional[list[float]] = None,
    item_padding: Optional[list[float]] = None,
    item_spacing: Optional[list[float]] = None,
    item_widths: Optional[list[float]] = None,
    bar_height: Optional[float] = None,
    bar_padding: Optional[list[float]] = None,
    bar_spacing: Optional[float] = None,
    bar_width: Optional[float] = None,
    close_on_item_click: Optional[bool] = None,
    close_on_background_click: Optional[bool] = None,
    on_select: Optional[Any] = None,
    style_id: Optional[int] = None,
    style_std_primary: Optional[bool] = None,
    show: bool = True,
    user_data: Optional[Any] = None,
    gen_id: Optional[int] = None,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...

def add_opaque(
    *,
    window_id: Optional[str] = None,
    container_id: Optional[str] = None,
    parent_id: Optional[str] = None,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_mouse_area(
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
    mouse_pointer: Optional[MousePointer]=None,
    show: bool=True,
    user_data: Optional[any]=None,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_row(
    *,
    container_id: Optional[str] = None,
    window_id: Optional[str] = None,
    parent_id: Optional[str] = None,
    align_top: Optional[bool] = None,
    align_center: Optional[bool] = None,
    align_bottom: Optional[bool] = None,
    width: Optional[float] = None,
    height: Optional[float] = None,
    width_fill: bool = False,
    height_fill: bool = False,
    padding: Optional[list[float]] = None,
    spacing: float = 20.0,
    clip: bool = False,
    show: bool = True,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_scrollable(
    *,
    window_id: Optional[str]=None,
    container_id: Optional[str]=None,
    parent_id: Optional[str]=None,
    width: Optional[float]=None,
    width_fill: Optional[bool]=None,
    height: Optional[float]=None,
    height_fill: Optional[bool]=None,
    both_scrollers: Optional[bool]=None,
    scroller_x_id: Optional[int]=None,
    scroller_y_id: Optional[int]=None,
    on_scroll: Optional[callable]=None,
    user_data: Optional[callable]=None,
    style_id: Optional[int]=None,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: wicontainerdget id
    """
    ...

def add_stack(
    *,
    container_id: Optional[str] = None,
    parent_id: Optional[str] = None,
    width: Optional[float] = None,
    height: Optional[float] = None,
    width_fill: bool = False,
    height_fill: bool = False,
    hide_index: Optional[int] = None,
    show: bool = True,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_table(
    *,
    parent_id: Optional[str] = None,
    **kwargs: Any
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...
def add_tool_tip(
    window_id: Optional[str] | None = None,
    container_id: Optional[str] | None = None,
    parent_id: Optional[str] | None = None,
    text: Optional[str] | None = None,
    position: Optional[ToolTipPosition] | None = None,
    gap: Optional[int] | None = None,
    padding: Optional[list[float, 4] | list[float]] | None = None,
    snap_within_viewport: Optional[bool] | None = None,
    delay_sec: Optional[float] | None = None,
    container_style_id = None,
    gen_id: Optional[str] | None = None,
    style_id: Optional[int] = None,
) -> int:
    """_summary_

    Args:
        parent_id (Optional[str], optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

# ---------------------------------------------------------------------------
# add_window wrapper
# ---------------------------------------------------------------------------

def add_window(
    *,
    window_id: Optional[str] | None = None,
    title: Optional[str] | None = None,
    width: Optional[float] | None = None,
    height: Optional[float] | None = None,
    max_width: Optional[float] | None = None,
    max_height: Optional[float] | None = None,
    min_width: Optional[float] | None = None,
    min_height: Optional[float] | None = None,
    pos_x: Optional[float] | None = None,
    pos_y: Optional[float] | None = None,
    center: Optional[bool] | None = None,
    resizable: Optional[bool] | None = None,
    decorations: Optional[bool] | None = None,
    transparent: Optional[bool] | None = None,
    level: Optional[WindowLevel] | None = None,
    theme: Optional[WindowTheme] | None = None,
    debug: Optional[bool] | None = None,
    exit_on_close: Optional[bool] | None = True,
    mode: Optional[WindowMode] | None = None,
    gen_id: Optional[int] | None = None,
)-> int:
    """Context manager that calls add_window and tracks the window id.
    
    A window is not a container for widgets, only other containers.
    
    Usage::

        add_window(window_id="main", title="My App", center=True)
        add_container(window_id="main", container_id="cont", align_center=True):
        add_text(parent="cont", content="hello")
        start_session
    """
    ...

# ---------------------------------------------------------------------------
# Context managers
# ---------------------------------------------------------------------------

class Window:
    """Context manager that calls add_window and tracks the window id.
    
    A window is not a container for widgets, only other containers.

    Usage::

        with Window(title="My App", center=True) as wnd_id: (if needed)
            with Container(align_center_center=True):
                add_text(content="hello")
        start_session
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] | None = None,
        title: Optional[str] | None = None,
        size: Optional[list] | None = None,
        maximized: Optional[bool] | None = None,
        fullscreen: Optional[bool] | None = None,
        center: Optional[bool] | None = None,
        position: Optional[list] | None = None,
        min_size: Optional[list] | None = None,
        max_size: Optional[list] | None = None,
        theme: Optional[WindowTheme] | None = None,
        visible: Optional[bool] | None = None,
        resizable: Optional[bool] | None = None,
        minimizable: Optional[bool] | None = None,
        closeable: Optional[bool] | None = None,
        decorations: Optional[bool] | None = None,
        transparent: Optional[bool] | None = None,
        blur: Optional[bool] | None = None,
        level: Optional[WindowLevel] | None = None,
        icon_rgba: Optional[list] | None = None,
        icon_width_height: Optional[list] | None = None,
        exit_on_close_request: Optional[bool] | None = None,
        scale_factor: Optional[float] | None = None,
        debug: Optional[bool] | None = None,
        on_resize: Optional[Any] | None = None,
        user_data: Optional[Any] | None = None,
        gen_id: Optional[int] | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...


class Card:
    """Context manager wrapper around add_card.

    Wraps the iced_aw Card — a widget that aligns its contents inside
    of its boundaries.  A Card take 1, 2, or 3 widgets.
    if 1, assumed only body of card.
    if 2, assumes head and body of card, respectively.
    if 3, uses head, body, foot, respectively.
    see demo file py_card.py
    
    Usage::

        with Window(title="Demo"):
            with Card(
                width=300.0,
                height=200.0,
                padding=[5],
                on_close=minimize_card
                ):

                add_text(content="Card")
                add_text(content="This is the body of the card.")
                with Column(width_fill=True, height=30):
                    add_separator(line_length=300)
                    add_text(content="Foot content")

        start_session()
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] | None = None,
        container_id: Optional[str] | None = None,
        parent_id: Optional[str] | None = None,
        is_open: bool | None = True,
        close_icon: Optional[bool] | None = None,
        close_icon_size: Optional[float] | None = None,
        on_close: Optional[Callable] | None = None,
        width: Optional[float] | None = None,
        width_fill: Optional[bool] | None = None,
        height: Optional[float] | None = None,
        height_fill: Optional[bool] | None = None,
        fill: Optional[bool] | None = None,
        max_width: Optional[float] | None = None,
        max_height: Optional[float] | None = None,
        padding: Optional[list[float]] | None = None,
        padding_body: Optional[list[float]] | None = None,
        padding_foot: Optional[list[float]] | None = None,
        style_id: Optional[int] | None = None,
        style_std: Optional[CardStyleStd] | None = None,
        show: bool = True,
        user_data: Optional[Any] | None = None,
        gen_id: Optional[int] | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...

class Container:
    """Context manager wrapper around add_container.

    Wraps the iced Container — a widget that aligns its contents inside
    of its boundaries.  Unlike a row or column a container can only have
    one child.

    Usage::

        with Window(title="Demo"):
            with Container(align_center=True, fill=True):
                add_text(content="hello")
        start_session
    """
    def __init__(
        self,
        *,
        container_id: Optional[str] | None = None,
        window_id: Optional[str] | None = None,
        parent_id: Optional[str] | None = None,
        width: Optional[float] | None = None,
        width_fill: bool | None = False,
        height: Optional[float] | None = None,
        height_fill: bool | None = False,
        fill: Optional[bool] | None = None,
        clip: Optional[bool] | None = None,
        max_height: Optional[float] | None = None,
        max_width: Optional[float] | None = None,
        align_bottom_center: Optional[bool] | None = None,
        align_bottom_left: Optional[bool] | None = None,
        align_bottom_right: Optional[bool] | None = None,
        align_center: Optional[bool] | None = None,
        align_center_left: Optional[bool] | None = None,
        align_center_right: Optional[bool] | None = None,
        align_top_center: Optional[bool] | None = None,
        align_top_left: Optional[bool] | None = None,
        align_top_right: Optional[bool] | None = None,
        padding: Optional[list[float]] | None = None,
        show: bool = True,
        style_id: Optional[int] | None = None,
        styl_std:Optional[ContainerStyleStd] | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...


class Column:
    """Context manager wrapper around add_column.

    A container that distributes its contents vertically.

    Usage::

        with Window(title="Demo"):
            with Container(fill=True, align_center=True)
                with Column(spacing=10.0):
                    add_text(content="hello")
                    add_text(content="hello")
        start_session
    """
    def __init__(
        self,
        *,
        container_id: Optional[str] | None = None,
        window_id: Optional[str] | None = None,
        parent_id: Optional[str] | None = None,
        align_left: Optional[bool] | None = None,
        align_center: Optional[bool] | None = None,
        align_right: Optional[bool] | None = None,
        width: Optional[float] | None = None,
        width_fill: bool | None = False,
        height: Optional[float] | None = None,
        height_fill: bool | None = False,
        fill: Optional[bool] | None = None,
        max_width: float | None = ...,
        padding: Optional[list[float]] | None = None,
        spacing: Optional[float] | None = None,
        clip: Optional[bool] | None = False,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...

class Menu:
    """Context manager wrapper around add_menu.

    A container create a menu.

    Usage::

        with Window(title="Demo"):
            with Menu(bar_items=[], menu_items=[])
        start_session
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] | None = None,
        container_id: Optional[str] | None = None,
        bar_items: Optional[str] | None = None,
        menu_items: list[int],
        parent_id: Optional[str] | None = None,
        item_offset: Optional[list[float]] | None = None,
        item_padding: Optional[list[float]] | None = None,
        item_spacing: Optional[list[float]] | None = None,
        item_widths: Optional[list[float]] | None = None,
        bar_height: Optional[float] | None = None,
        bar_padding: Optional[list[float]] | None = None,
        bar_spacing: Optional[float] | None = None,
        bar_width: Optional[float] | None = None,
        close_on_item_click: Optional[bool] | None = None,
        close_on_background_click: Optional[bool] | None = None,
        on_select: Optional[Any] | None = None,
        style_id: Optional[int] | None = None,
        style_std_primary: Optional[bool] | None = None,
        show: bool = True,
        user_data: Optional[Any] | None = None,
        gen_id: Optional[int] | None = None,
    )  -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...

class MouseArea:
    """Context Manager wrapper for add_mousearea
    
    A container that is like Container but allows
    mouse interactions.
    
    Usage::
    
        with Window(title="Demo"):
            with MouseArea():
                add_svg() # Your svg will have mouse interaction    
        start_session
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] | None = None,
        container_id: Optional[str] | None = None,
        parent_id: Optional[str] | None = None,
        gen_id: Optional[int] | None = None,
        on_press: Optional[Callable] | None = None,
        on_release: Optional[Callable] | None = None,
        on_right_press: Optional[Callable] | None = None,
        on_right_release: Optional[Callable] | None = None,
        on_middle_press: Optional[Callable] | None = None,
        on_middle_release: Optional[Callable] | None = None,
        on_enter: Optional[Callable] | None = None,
        on_move: Optional[Callable] | None = None,
        on_exit: Optional[Callable] | None = None,
        mouse_pointer: Optional[MousePointer] | None = None,
        show: bool | None = True,
        user_data: Optional[any] | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...

class Opague:
    """Context manager wrapper around add_row.

    A container that prevents mouse actions for passing throught.

    Usage::

        with Window(title="Demo"):
            with Row(spacing=10.0):
                add_text(content="hello")
                add_text(content="hello")
        start_session
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] = None,
        container_id: Optional[str] = None,
        parent_id: Optional[str] = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None,\
            exc_tb: TracebackType | None) -> bool: ...

class Row:
    """Context manager wrapper around add_row.

    A container that distributes its contents horizontally.

    Usage::

        with Window(title="Demo"):
            with Row(spacing=10.0):
                add_text(content="hello")
                add_text(content="hello")
        start_session
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] = None,
        container_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        width: Optional[float] = None,
        width_fill: Optional[bool] = None,
        height: Optional[float] = None,
        height_fill: Optional[bool] = None,
        align_bottom: Optional[bool] = None,
        align_center: Optional[bool] = None,
        align_top: Optional[bool] = None,
        padding: Optional[list] = None,
        spacing: Optional[float] = None,
        clip: Optional[bool] = None,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None,\
            exc_tb: TracebackType | None) -> bool: ...


class Stack:
    """Context manager wrapper around add_stack.

    A stack of containers.

    Usage::

        with Window(title="Demo"):
            with Stack():
                add_text(content="hello")
        start_session
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] = None,
        container_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        width: Optional[float] = None,
        height: Optional[float] = None,
        width_fill: bool = False,
        height_fill: bool = False,
        hide_index: Optional[int] = None,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...


class Scrollable:
    """Context manager wrapper around add_scrollable.

    Usage::

        with Window(title="Demo"):
            with Scrollable(width=200.0, height=100.0):
                for i in range(0, 20):
                    add_text(content=f"Some Text {i}")
        start_session
    """
    def __init__(
        self,
        *,
        window_id: Optional[str] = None,
        container_id: Optional[str] = None,
        parent_id: Optional[str] = None,
        width: Optional[float] = None,
        width_fill: Optional[bool] = None,
        height: Optional[float] = None,
        height_fill: Optional[bool] = None,
        both_scrollers: Optional[float] = None,
        scroller_x_id: Optional[int] = None,
        scroller_y_id: Optional[int] = None,
        on_scroll: Optional[callable] = None,
        user_data: Optional[callable] = None,
        style_id: Optional[int] = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...

class ToolTip:
    """Context manager wrapper around add_tool_tip.

    Usage::

        with Window(title="Demo"):
            with Container(fill=True, align_center-True):
                with ToolTip(text="Tool Tip text"):
                    add_text(content="Place mouse over me to see tooltip)
        start_session
    """
    def __init__(
        self,
        text,
        *,
        window_id: Optional[str] | None = None,
        container_id: Optional[str] | None = None,
        parent_id: Optional[str] | None = None,
        position: Optional[ToolTipPosition] | None = None,
        gap: Optional[int] | None = None,
        padding: Optional[list[float, 4] | list[float]] | None = None,
        snap_within_viewport: Optional[bool] | None = None,
        delay_sec: Optional[float] | None = None,
        container_style_id = None,
        gen_id: Optional[str] | None = None,
        style_id: Optional[int] = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...
