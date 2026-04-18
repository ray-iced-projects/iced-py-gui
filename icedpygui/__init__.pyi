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
from typing import Any, Callable
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
    add_progress_bar_style as add_progress_bar_style,
    add_radio_style as add_radio_style,
    add_scrollable_style as add_scrollable_style,
    add_scroller as add_scroller,
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
    MenuBarItemParam as MenuBarItemParam,
    MenuParam as MenuParam,
    MenuStyleParam as MenuStyleParam,
    MousePointer as MousePointer,
    OpaqueParam as OpaqueParam,
    PickListHandle as PickListHandle,
    ProgressBarParam as ProgressBarParam,
    ProgressBarStyleParam as ProgressBarStyleParam,
    ProgressBarStyleStd as ProgressBarStyleStd,
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
    parent_id: str | None = None,
    label: str = ...,
    gen_id: int | None = None,
    on_press: Callable | None = None,
    width: float | None = None,
    width_fill: bool = False,
    height: float | None = None,
    height_fill: bool = False,
    if_menu_btn: bool = False,
    padding: list[float] | None = None,
    text_top_left: bool | None = None,
    text_top_center: bool | None = None,
    text_top_right: bool | None = None,
    text_center_left: bool | None = None,
    text_center: bool = True,
    text_center_right: bool | None = None,
    text_bottom_left: bool | None = None,
    text_bottom_center: bool | None = None,
    text_bottom_right: bool | None = None,
    text_size: float | None = None,
    clip: bool | None = None,
    style_id: int | None = None,
    style_standard: ButtonStyleStd | None = None,
    style_arrow: Arrow | None = None,
    user_data: Any | None = None,
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
    parent_id: str | None = None,
    head: str | None = None,
    body: str | None = None,
    is_open: bool = True,
    min_max_id: int | None = None,
    foot: str | None = None,
    gen_id: int | None = None,
    close_size: float | None = None,
    on_close: Any | None = None,
    width: float | None = None,
    width_fill: bool = False,
    height: float | None = None,
    height_fill: bool = False,
    max_width: float | None = None,
    max_height: float | None = None,
    padding: list[float] | None = None,
    padding_head: list[float] | None = None,
    padding_body: list[float] | None = None,
    padding_foot: list[float] | None = None,
    style_id: int | None = None,
    style_std: CardStyleStd | None = None,
    style_button: int | None = None,
    show: bool = True,
    user_data: Any | None = None,
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
    parent_id: str | None = None,
    on_toggle: Callable | None = None,
    is_checked: bool = False,
    label: str | None = None,
    width: float | None = None,
    width_fill: bool = False,
    size: float | None = None,
    spacing: float | None = None,
    text_size: float | None = None,
    text_line_height: float | None = None,
    text_shaping: bool | None = None,
    text_wrapping: bool | None = None,
    text_font_id: int | None = None,
    icon_font_id: int | None = None,
    icon: Icon | None = None,
    icon_size: float | None = None,
    icon_line_height: float | None = None,
    icon_shaping: bool | None = None,
    user_data: Any | None = None,
    show: bool = True,
    style_id: int | None = None,
    style_std: int | None = None,
    gen_id: int | None = None,
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
    parent_id: str | None = None,
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
    parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...
def add_divider(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...
def add_float(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...
def add_image(
    path: str,
    *,
    parent_id: str | None = None,
    border_radius: list[float, 4] | list[float, 1] | None = None,
    content_fit: ContentFit | None = None,
    crop_height: float | None = None,
    crop_width: float | None = None,
    crop_x: float | None = None,
    crop_y: float | None = None,
    expand: bool | None = None,
    fill: bool | None = None,
    filter_method: FilterMethod | None = None,
    gen_id: int | None = None,
    height_fill: bool | None = None,
    height: float | None = None,
    opacity: float | None = None,
    rotation_degrees: float | None = None,
    rotation_method: Rotation | None = None,
    rotation_radians: float | None = None,
    scale: float | None = None,
    show: bool = True,
    width_fill: bool | None = None,
    width: float | None = None,
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
    parent_id: str | None = None,
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
def add_progress_bar(
    *,
    parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_radio(
    *,
    parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_separator(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_slider(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_space(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_svg(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_text_input(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_text(
    *,
    parent_id: str | None = None,
    content: str,
    width: float | None = None,
    width_fill: bool | None = None,
    height: float | None = None,
    height_fill: bool | None = None,
    fill: bool | None = None,
    align_bottom_center: bool | None = None,
    align_bottom_left: bool | None = None,
    align_bottom_right: bool | None = None,
    align_center_left: bool | None = None,
    align_center_right: bool | None = None,
    align_center: bool | None = None,
    align_top_center: bool | None = None,
    align_top_left: bool | None = None,
    align_top_right: bool | None = None,
    font_id: int | None = None,
    size: float | None = None,
    line_height: float | None = None,
    color: Color | None = None,
    color_alpha: float | None = None,
    color_rgba: list[float, 4] | None = None,
    color_std: TextColorStd | None = None,
    wrapping_none: bool | None = None,
    wrapping_glyph: bool | None = None,
    wrapping_word_glyph: bool | None = None,
    show: bool = True,
    gen_id: int | None = None,
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
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_toggler(
    *, parent_id: str | None = None,
    **kwargs: Any) -> int:
    """
    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...

# ---------------------------------------------------------------------------
# Wrapped container functions — full signatures
# ---------------------------------------------------------------------------

def add_container(
    *,
    container_id: str | None = None,
    window_id: str | None = None,
    parent_id: str | None = None,
    width: float | None = None,
    width_fill: bool = False,
    height: float | None = None,
    height_fill: bool = False,
    clip: bool | None = None,
    max_height: float | None = None,
    max_width: float | None = None,
    align_bottom_center: bool | None = None,
    align_bottom_left: bool | None = None,
    align_bottom_right: bool | None = None,
    align_center: bool | None = None,
    align_center_left: bool | None = None,
    align_center_right: bool | None = None,
    align_top_center: bool | None = None,
    align_top_left: bool | None = None,
    align_top_right: bool | None = None,
    padding: list[float] | None = None,
    show: bool = True,
    style_id: int | None = None,
    style_std: ContainerStyleStd | None = None,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...

def add_column(
    *,
    container_id: str | None = None,
    window_id: str | None = None,
    parent_id: str | None = None,
    align_left: bool | None = None,
    align_center: bool | None = None,
    align_right: bool | None = None,
    width: float | None = None,
    height: float | None = None,
    width_fill: bool = False,
    height_fill: bool = False,
    max_width: float = ...,
    padding: list[float] | None = None,
    spacing: float = 20.0,
    clip: bool = False,
    show: bool = True,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...

def add_menu(
    *,
    window_id: str | None = None,
    container_id: str | None = None,
    bar_items: str | None = None,
    menu_items: list[int],
    parent_id: str | None = None,
    item_offset: list[float] | None = None,
    item_padding: list[float] | None = None,
    item_spacing: list[float] | None = None,
    item_widths: list[float] | None = None,
    bar_height: float | None = None,
    bar_padding: list[float] | None = None,
    bar_spacing: float | None = None,
    bar_width: float | None = None,
    close_on_item_click: bool | None = None,
    close_on_background_click: bool | None = None,
    on_select: Any | None = None,
    style_id: int | None = None,
    style_std_primary: bool | None = None,
    show: bool = True,
    user_data: Any | None = None,
    gen_id: int | None = None,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: widget id
    """
    ...

def add_opaque(
    *,
    window_id: str | None = None,
    container_id: str | None = None,
    parent_id: str | None = None,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_mouse_area(
    *,
    window_id: str,
    container_id: str,
    parent_id: str | None=None,
    gen_id: int | None=None,
    on_press: Callable | None=None,
    on_release: Callable | None=None,
    on_right_press: Callable | None=None,
    on_right_release: Callable | None=None,
    on_middle_press: Callable | None=None,
    on_middle_release: Callable | None=None,
    on_enter: Callable | None=None,
    on_move: Callable | None=None,
    on_exit: Callable | None=None,
    mouse_pointer: MousePointer | None=None,
    show: bool=True,
    user_data: any | None=None,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_row(
    *,
    container_id: str | None = None,
    window_id: str | None = None,
    parent_id: str | None = None,
    align_top: bool | None = None,
    align_center: bool | None = None,
    align_bottom: bool | None = None,
    width: float | None = None,
    height: float | None = None,
    width_fill: bool = False,
    height_fill: bool = False,
    padding: list[float] | None = None,
    spacing: float = 20.0,
    clip: bool = False,
    show: bool = True,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_scrollable(
    *,
    window_id: str | None=None,
    container_id: str | None=None,
    parent_id: str | None=None,
    width: float | None=None,
    width_fill: bool | None=None,
    height: float | None=None,
    height_fill: bool | None=None,
    both_scrollers: bool | None=None,
    scroller_x_id: int | None=None,
    scroller_y_id: int | None=None,
    on_scroll: callable | None=None,
    user_data: callable | None=None,
    style_id: int | None=None,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: wicontainerdget id
    """
    ...
def add_stack(
    *,
    container_id: str | None = None,
    parent_id: str | None = None,
    width: float | None = None,
    height: float | None = None,
    width_fill: bool = False,
    height_fill: bool = False,
    hide_index: int | None = None,
    show: bool = True,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

def add_table(
    *,
    parent_id: str | None = None,
    **kwargs: Any
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...
def add_tool_tip(
    window_id: str | None = None,
    container_id: str | None = None,
    parent_id: str | None = None,
    text: str | None = None,
    position: ToolTipPosition | None = None,
    gap: int | None = None,
    padding: list[float, 4] | list[float] | None = None,
    snap_within_viewport: bool | None = None,
    delay_sec: float | None = None,
    container_style_id = None,
    gen_id: str | None = None,
    style_id: int | None = None,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: container id
    """
    ...

# ---------------------------------------------------------------------------
# add_window wrapper
# ---------------------------------------------------------------------------

def add_window(
    *,
    window_id: str | None = None,
    title: str | None = None,
    width: float | None = None,
    height: float | None = None,
    max_width: float | None = None,
    max_height: float | None = None,
    min_width: float | None = None,
    min_height: float | None = None,
    pos_x: float | None = None,
    pos_y: float | None = None,
    center: bool | None = None,
    resizable: bool | None = None,
    decorations: bool | None = None,
    transparent: bool | None = None,
    level: WindowLevel | None = None,
    theme: WindowTheme | None = None,
    debug: bool | None = None,
    exit_on_close: bool | None = True,
    mode: WindowMode | None = None,
    gen_id: int | None = None,
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
        window_id: str | None = None,
        title: str | None = None,
        size: list | None = None,
        maximized: bool | None = None,
        fullscreen: bool | None = None,
        center: bool | None = None,
        position: list | None = None,
        min_size: list | None = None,
        max_size: list | None = None,
        theme: WindowTheme | None = None,
        visible: bool | None = None,
        resizable: bool | None = None,
        minimizable: bool | None = None,
        closeable: bool | None = None,
        decorations: bool | None = None,
        transparent: bool | None = None,
        blur: bool | None = None,
        level: WindowLevel | None = None,
        icon_rgba: list | None = None,
        icon_width_height: list | None = None,
        exit_on_close_request: bool | None = None,
        scale_factor: float | None = None,
        debug: bool | None = None,
        on_resize: Any | None = None,
        user_data: Any | None = None,
        gen_id: int | None = None,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        is_open: bool | None = True,
        close_icon: bool | None = None,
        close_icon_size: float | None = None,
        on_close: Callable | None = None,
        width: float | None = None,
        width_fill: bool | None = None,
        height: float | None = None,
        height_fill: bool | None = None,
        fill: bool | None = None,
        max_width: float | None = None,
        max_height: float | None = None,
        padding: list[float] | None = None,
        padding_body: list[float] | None = None,
        padding_foot: list[float] | None = None,
        style_id: int | None = None,
        style_std: CardStyleStd | None = None,
        show: bool = True,
        user_data: Any | None = None,
        gen_id: int | None = None,
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
        container_id: str | None = None,
        window_id: str | None = None,
        parent_id: str | None = None,
        width: float | None = None,
        width_fill: bool | None = False,
        height: float | None = None,
        height_fill: bool | None = False,
        fill: bool | None = None,
        clip: bool | None = None,
        max_height: float | None = None,
        max_width: float | None = None,
        align_bottom_center: bool | None = None,
        align_bottom_left: bool | None = None,
        align_bottom_right: bool | None = None,
        align_center: bool | None = None,
        align_center_left: bool | None = None,
        align_center_right: bool | None = None,
        align_top_center: bool | None = None,
        align_top_left: bool | None = None,
        align_top_right: bool | None = None,
        padding: list[float] | None = None,
        show: bool = True,
        style_id: int | None = None,
        styl_std:ContainerStyleStd | None = None,
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
        container_id: str | None = None,
        window_id: str | None = None,
        parent_id: str | None = None,
        align_left: bool | None = None,
        align_center: bool | None = None,
        align_right: bool | None = None,
        width: float | None = None,
        width_fill: bool | None = False,
        height: float | None = None,
        height_fill: bool | None = False,
        fill: bool | None = None,
        max_width: float | None = ...,
        padding: list[float] | None = None,
        spacing: float | None = None,
        clip: bool | None = False,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        height: float | None = None,
        padding: list[float] | None = None,
        spacing: float | None = None,
        width: float | None = None,
        close_on_item_click: bool | None = None,
        close_on_background_click: bool | None = None,
        on_select: Any | None = None,
        style_id: int | None = None,
        style_primary: bool | None = None,
        show: bool = True,
        user_data: Any | None = None,
        gen_id: int | None = None,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        gen_id: int | None = None,
        on_press: Callable | None = None,
        on_release: Callable | None = None,
        on_right_press: Callable | None = None,
        on_right_release: Callable | None = None,
        on_middle_press: Callable | None = None,
        on_middle_release: Callable | None = None,
        on_enter: Callable | None = None,
        on_move: Callable | None = None,
        on_exit: Callable | None = None,
        mouse_pointer: MousePointer | None = None,
        show: bool | None = True,
        user_data: any | None = None,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        width: float | None = None,
        width_fill: bool | None = None,
        height: float | None = None,
        height_fill: bool | None = None,
        align_bottom: bool | None = None,
        align_center: bool | None = None,
        align_top: bool | None = None,
        padding: list | None = None,
        spacing: float | None = None,
        clip: bool | None = None,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        width: float | None = None,
        height: float | None = None,
        width_fill: bool = False,
        height_fill: bool = False,
        hide_index: int | None = None,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        width: float | None = None,
        width_fill: bool | None = None,
        height: float | None = None,
        height_fill: bool | None = None,
        scroller_x_id: int | None = None,
        scroller_y_id: int | None = None,
        on_scroll: callable | None = None,
        user_data: callable | None = None,
        style_id: int | None = None,
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
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        position: ToolTipPosition | None = None,
        gap: int | None = None,
        padding: list[float, 4] | list[float] | None = None,
        snap_within_viewport: bool | None = None,
        delay_sec: float | None = None,
        container_style_id = None,
        gen_id: str | None = None,
        style_id: int | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...
