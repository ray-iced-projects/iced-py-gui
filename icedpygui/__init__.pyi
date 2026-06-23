#!/usr/bin/env python3
# pylint: disable=no-name-in-module
# pylint: disable=unused-argument
# pylint: disable=unnecessary_ellipsis
# pylint: disable=too-many-arguments
# pylint: disable=useless-import-alias
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
    add_combobox_input_style as add_combobox_input_style,
    add_combobox_menu_style as add_combobox_menu_style,
    add_container_style as add_container_style,
    add_font_style as add_font_style,
    # add_menu_style as add_menu_style,
    add_opaque_style as add_opaque_style,
    add_progress_bar_style as add_progress_bar_style,
    add_radio_style as add_radio_style,
    add_sash_style as add_sash_style,
    add_scrollable_style as add_scrollable_style,
    add_scroller as add_scroller,
    add_autoscroll_style as add_autoscroll_style,
    add_rail_style as add_rail_style,
    add_separator_style as add_separator_style,
    add_slider_style as add_slider_style,
    add_text_editor_style as add_text_editor_style,
    add_text_input_style as add_text_input_style,
    add_toggler_style as add_toggler_style,
    Arrow as Arrow,
    arrow_variants as arrow_variants,
    ButtonParam as ButtonParam,
    ButtonStyleParam as ButtonStyleParam,
    ButtonStyleStd as ButtonStyleStd,
    # CardParam as CardParam,
    # CardStyleParam as CardStyleParam,
    # CardStyleStd as CardStyleStd,
    CheckboxParam as CheckboxParam,
    CheckboxStyleStd as CheckboxStyleStd,
    CheckboxStyleParam as CheckboxStyleParam,
    ComboBoxParam as ComboBoxParam,
    ComboBoxMenuStyleParam as ComboBoxMenuStyleParam,
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
    FilterMethod as FilterMethod,
    FloatParam as FloatParam,
    FontFamily as FontFamily,
    FontWeight as FontWeight,
    FontStretch as FontStretch,
    FontStyle as FontStyle,
    GridParam as GridParam,
    Icon as Icon,
    MousePointer as MousePointer,
    OpaqueParam as OpaqueParam,
    PickListParam as PickListParam,
    ProgressBarParam as ProgressBarParam,
    ProgressBarStyleParam as ProgressBarStyleParam,
    ProgressBarStyleStd as ProgressBarStyleStd,
    RadioParam as RadioParam,
    RadioStyleParam as RadioStyleParam,
    Rotation as Rotation,
    RowParam as RowParam,
    SashParam as SashParam,
    ScrollableParam as ScrollableParam,
    ScrollableStyleParam as ScrollableStyleParam,
    ScrollerParam as ScrollerParam,
    SeparatorParam as SeparatorParam,
    SeparatorStyleParam as SeparatorStyleParam,
    SliderParam as SliderParam,
    SliderStyleParam as SliderStyleParam,
    StackParam as StackParam,
    StyleStandard as StyleStandard,
    SvgParam as SvgParam,
    TableParam as TableParam,
    TextInputParam as TextInputParam,
    TextParam as TextParam,
    TextColorStd as TextColorStd,
    TinerParam as TimerParam,
    TogglerParam as TogglerParam,
    TogglerStyleParam as TogglerStyleParam,
    ToolTipParam as ToolTipParam,
    WindowLevel as WindowLevel,
    WindowMode as WindowMode,
    WindowParam as WindowParam,
    WindowTheme as WindowTheme,
    window_theme_names as window_theme_names,
    create_custom_theme as create_custom_theme,
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
    get_rgba_color as get_rgba_color,
    get_color_palette as get_color_palette,
    get_styling_palette as get_styling_palette,
    get_button_palette as get_button_palette,
    StdColorStyle as StdColorStyle,
    custom_palette as custom_palette,
    PaletteKey as PaletteKey,
    WidgetStatus as WidgetStatus,
    StylePart as StylePart,
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
    padding: list[float] | None = None,
    clip: bool | None = None,
    status_active: bool | None = None,
    status_hovered: bool | None = None,
    status_pressed: bool | None = None,
    status_disabled: bool | None = None,
    font_id: int | None = None,
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
        start_session()
    """
    ...

# def add_card(
#     *,
#     parent_id: str | None = None,
#     head: str | None = None,
#     body: str | None = None,
#     is_open: bool = True,
#     min_max_id: int | None = None,
#     foot: str | None = None,
#     gen_id: int | None = None,
#     close_size: float | None = None,
#     on_close: Any | None = None,
#     width: float | None = None,
#     width_fill: bool = False,
#     height: float | None = None,
#     height_fill: bool = False,
#     max_width: float | None = None,
#     max_height: float | None = None,
#     padding: list[float] | None = None,
#     padding_head: list[float] | None = None,
#     padding_body: list[float] | None = None,
#     padding_foot: list[float] | None = None,
#     style_id: int | None = None,
#     style_std: CardStyleStd | None = None,
#     style_button: int | None = None,
#     show: bool = True,
#     user_data: Any | None = None,
# ) -> int:
#     """Adds a button widget.

#     A widget must go into a container type,

#     i.e. Container, Column, Row, etc.

#     Usage::

#         with Window(title="My App", pos_centered=True) as wnd_id: (if needed)
#             with Container(align_center=True):
#                 add_card(head="Card Header", body="Card Body)
#         start_session()
#     """
#     ...

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
        start_session()
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
def add_event_keyboard(
        enabled: bool,
        *,
        on_key_press: Callable | None = None,
        on_key_release: Callable | None = None,
        user_data: any | None = None,
    ) -> int:

    """
    Add a keyboard event handler to process keyboard actions.

    Parameters
    ----------
    enabled: bool
        Enables the event
    on_key_press: Callable
        Calls a function when a key is pressed.
    on_key_release: Callable
        Calls a function when a key is released.
    user_data: any
        Any data that might be needed in the callback function.

    Returns
    -------
    id: int
        The id of the event which can be used to modify the event through update_item.
    """
    ...
def add_event_mouse(
        enabled: bool,
        *,
        on_move: Callable | None = None,
        on_left_press: Callable | None = None,
        on_left_release: Callable | None = None,
        on_middle_press: Callable | None = None,
        on_middle_release: Callable | None = None,
        on_right_press: Callable | None = None,
        on_right_release: Callable | None = None,
        on_middle_scroll_line: Callable | None = None,
        on_middle_scroll_pixel: Callable | None = None,
        user_data: Any | None = None,
    ) ->int:

    """
    Add a mouse button handlers to process mouse actions.

    Parameters
    ----------
    enabled: bool
        Enables the event
    on_move: Callable
        Calls a function when the mouse is moved.
    on_left_press: Callable
        Calls a function when the left mouse button is pressed.
    on_left_release: Callable
        Calls a function when the left mouse button is released.
    on_middle_press: Callable
        Calls a function when the middle mouse button is pressed.
    on_middle_release: Callable
        Calls a function when the middle mouse button is released.
    on_right_press: Callable
        Calls a function when the right mouse button is pressed.
    on_right_release: Callable
        Calls a function when the right mouse button is released.
    on_middle_scroll_line: Callable
        Calls a function when the middle mouse scroll is scrolled, sends line count.
    on_middle_scroll_pixel: Callable
        Calls a function when the middle mouse scroll is scrolled, send pixel count.
    user_data: any
        Any data that might be needed in the callback function.

    Returns
    -------
    id: int
        The id of the event which can be used to modify the event through update_item.
    """
    ...
def add_event_timer(
    enabled: bool,
    *,
    duration_ms: int | None = None,
    on_start: Callable | None = None,
    on_tick: Callable | None = None,
    on_stop: Callable | None = None,
    user_data: Any | None = None,
    gen_id: int | None = None,
    ) -> int:
    """
    Args:
        enabled (bool, optional): _description_. Defaults to False.
        duration_ms (int | None, optional): _description_. Defaults to None.
        on_start (Callable | None, optional): _description_. Defaults to None.
        on_tick (Callable | None, optional): _description_. Defaults to None.
        on_stop (Callable | None, optional): _description_. Defaults to None.
        user_data (Any | None, optional): _description_. Defaults to None.
        gen_id (int | None, optional): _description_. Defaults to None.

    Returns:
        int: _description_
    """
    ...
def add_event_window(
    enabled: bool,
    *,
    on_closed: Callable | None = None,
    on_moved: Callable | None = None,
    on_resized: Callable | None = None,
    on_redraw_requested: Callable | None = None,
    on_close_requested: Callable | None = None,
    on_focused: Callable | None = None,
    on_unfocused: Callable | None = None,
    on_file_hovered: Callable | None = None,
    on_file_dropped: Callable | None = None,
    on_files_hovered_left: Callable | None = None,
    user_data: Any | None = None,
    ) -> int:
    """
    Adds event to the window other than those in the add_window method.

    Parameters
    ----------
    enabled: bool
        Enables the event
    on_closed: Callable | None=None
        Calls a function when the window is closed.
    on_moved: Callable | None=None
        Calls a function when the window is moved.
    on_resized: Callable | None=None
        Calls a function when the window id resized.
    on_redraw_requested: Callable | None=None
        When a redraw command is requested.
    on_close_requested: Callable | None=None
        When a window close is requested, the window setting on_close_request must be set to False.
    on_focused: Callable | None=None
        When an unfocused window is brought into focus.
    on_unfocused: Callable | None=None
        When another window is focused or unfocused.
    on_file_hovered: Callable | None=None
        When a file is dragged over the window. The file path is delivered.
    on_file_dropped: Callable | None=None
        When a file is dropped onto the window. The file path is delivered.
    on_files_hovered_left: Callable | None=None
        When the file leaves the window without being dropped.
    user_data: Any | None=None
        Any data that might be needed in the callback function.

    Returns
    -------
    id: int
        The id of the event which can be used to modify the event through update_item.
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
def add_icon(
    *,
    arrow: Arrow | None = None,
    icon: Icon | None = None,
    code_point: int | None = None,
    font_id: int | None = None,
    parent_id: str | None = None,
    size: float | None = None,
    line_height: float | None = None,
    gen_id: int | None = None,
    ) -> int:
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
    *,
    parent_id: str | None = None,
    placeholder: str | None = None,
    on_input: Any | None = None,
    on_submit: Any | None = None,
    on_paste: Any | None = None,
    width: float | None = None,
    width_fill: bool | None = None,
    padding: list[float, 4 | list[float]] | None = None,
    size: float | None = None,
    line_height: float | None = None,
    align_left: bool | None = None,
    align_center: bool | None = None,
    align_right: bool | None = None,
    user_data: Any | None = None,
    is_secure: bool | None = None,
    font_id: int | None = None,
    style_id: int | None = None,
    show: bool = True,
    gen_id: int | None = None,
    ) -> int:
    """
    Add a text input widget.

    A single-line text input field with placeholder text.

    Parameters
    ----------
    parent_id : str
        Sets the parent container ID that this text input belongs to.
    placeholder : str
        Sets the placeholder text shown when the input is empty.
    gen_id : int, Optional
        Obtains an ID of a widget that have not been created, used for the gen_id parameter.
    on_input : callable, Optional
        Sets the Callback method to invoke when the input text changes.
    on_submit : callable, Optional
        Sets the Callback method to invoke when the user presses enter.
    on_paste : callable, Optional
        Sets the Callback method to invoke when text is pasted.
    width : float, Optional
        Sets the Fixed width in logical pixels.
    width_fill : bool, default False
        Whether the text input fills available width.
    padding : list of float, Optional
        Sets the Padding as [all], [vertical, horizontal], or
        [top, right, bottom, left].
    size : float, Optional
        Sets the font size for the input text.
    line_height : float, Optional
        Sets the line height for the input text.
    align_left : bool, Optional
        Whether to set the horizontal alignment left (default).
    align_center : bool, Optional
        Whether to set the horizontal alignment center.
    align_right : bool, Optional
        Whether to set the horizontal alignment right.
    user_data : Any, Optional
        Sets the Arbitrary data forwarded to callbacks.
    is_secure : bool, Optional
        Whether the input text is obscured (password mode).
    font_id : int, Optional
        Sets the Font ID for the input text.
    style_id : int, Optional
        Sets the ID of a custom style created with ``add_text_input_style``.
    show : bool, default True
        Whether the text input is visible.

    Returns
    -------
    int
        The numeric widget ID of the newly created text input.
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
        start_session()

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
def add_sash(
    *,
    window_id: str | None = None,
    container_id: str | None = None,
    initial_sizes: list[float],
    size: float,
    sash_size: float,
    sync_sashes: bool | None = None,
    sync_cross_sashes: bool | None = None,
    parent_id: str | None = None,
    outer_handle_size: float | None = None,
    cross_handle_size: float | None = None,
    resize_mode_last_only: bool | None = None,
    resize_mode_uniform: bool | None = None,
    resize_mode_proportional: bool | None = None,
    on_resize: Callable | None = None,
    on_resize_outer: Callable | None = None,
    on_release: Callable | None = None,
    vertical_direction: bool | None = None,
    min_size: float | None = None,
    max_size: float | None = None,
    min_cross_size: float | None = None,
    max_cross_size: float | None = None,
    clip: bool | None = None,
    style_id: int | None = None,
    user_data: Any | None = None,
    show: bool = True,
) -> int:
    """
    Add a sash (resizable panels) container.

    A sash divides its children into resizable panels separated by draggable
    handle bars. By default it is horizontal (panels arranged left-to-right).
    Set ``vertical_direction=True`` to arrange panels top-to-bottom instead.

    Parameters
    ----------
    window_id : str
        Sets the window ID that this sash belongs to.
    container_id : str
        Sets the parent container ID that this sash belongs to.
    initial_sizes : list of float
        Sets the starting pixel size of each panel in the main axis direction.
        One entry per child widget. The total should equal ``size``.
    size : float
        Sets the total pixel size of the sash in the main axis direction.
    sash_size : float
        Sets the width of each draggable handle bar in logical pixels.
    sync_sashes : bool, Optional
        When True, this sash joins the global sync group. All sashes with
        ``sync_sashes=True`` share their panel sizes — resizing one updates
        all others. Useful for syncing a header sash with a body sash.
    sync_cross_sashes : bool, Optional
        When True, syncs sashes that run perpendicular to the main axis.
    parent_id : str, Optional
        Sets an explicit parent container ID, overriding ``window_id``.
    outer_handle_size : float, Optional
        Sets the pixel size of the outer (edge) handle bars.
    cross_handle_size : float, Optional
        Sets the pixel size of the cross-axis handle bar. Setting this enables
        cross-axis resizing (dragging the sash taller or shorter).
    resize_mode_last_only : bool, Optional
        When True, only the last panel absorbs resize overflow.
    resize_mode_uniform : bool, Optional
        When True, all panels resize by equal amounts.
    resize_mode_proportional : bool, Optional
        When True, panels resize proportionally to their current sizes.
    on_resize : callable, Optional
        Callback invoked while a handle is being dragged.
        Signature: ``def cb(wid: int, data: tuple[int, float])`` where
        ``data`` is ``(panel_index, new_size)``.
    on_resize_outer : callable, Optional
        Callback invoked when an outer handle is dragged.
        Signature: ``def cb(wid: int, size: float)``.
    on_release : callable, Optional
        Callback invoked when the mouse button is released after a drag.
        Signature: ``def cb(wid: int)``.
    vertical_direction : bool, Optional
        When True, panels are stacked top-to-bottom (vertical sash).
        Default is False (horizontal sash, panels left-to-right).
    min_size : float, Optional
        Sets the minimum pixel size any panel may be resized to.
    max_size : float, Optional
        Sets the maximum total pixel size of the sash.
    min_cross_size : float, Optional
        Sets the minimum pixel size of the sash in the cross-axis direction.
    max_cross_size : float, Optional
        Sets the maximum pixel size of the sash in the cross-axis direction.
    clip : bool, Optional
        Sets whether to clip the content when resized samller than content.
    style_id : int, Optional
        Sets the ID of a custom style created with ``add_sash_style``.
    user_data : Any, Optional
        Sets arbitrary data forwarded as a third argument to all callbacks.
    show : bool, default True
        Whether the sash is visible.

    Returns
    -------
    int
        The numeric container ID of the newly created sash.
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
    fill: bool | None=None,
    auto_scroll: bool | None=None,
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

def add_table_basic(
    *,
    window_id: str,
    container_id: str,
    headers: list[str],
    body: list[list[float]],
    footers: list[str],
    column_widths: list[float],
    height: float,
    parent_id: str | None = None,
    width: float | None = None,
    resizer_width: float | None = None,
    header_enabled: bool = True,
    header_row_height: float | None = None,
    header_scrollbar_height: float | None = None,
    header_scrollbar_margin: float | None = None,
    header_scroller_height: float | None = None,
    header_scrollbar_spacing: float | None = None,
    header_row_spacing: float | None = None,
    footer_height: float | None = None,
    footer_scrollbar_height: float | None = None,
    footer_scrollbar_margin: float | None = None,
    footer_scroller_height: float | None = None,
    footer_scrollbar_spacing: float | None = None,
    footer_spacing: float | None = None,
    body_scrollbar_width: float | None = None,
    body_scrollbar_margin: float | None = None,
    body_scroller_width: float | None = None,
    body_scrollbar_spacing: float | None = None,
    body_row_highlight: bool = True,
    custom_header_rows: int | None = None,
    custom_footer_rows: int | None = None,
    control_columns: list[int] | None = None,
    column_proportional_resize: bool = True,
    row_spacing: float | None = None,
    row_height: float | None = None,
    header_body_spacing: float | None = None,
    body_footer_spacing: float | None = None,
    resize_columns_enabled: bool = True,
    min_column_width: float | None = None,
    text_size: float | None = None,
    table_width_fixed: bool = True,
    gen_id: int | None = None,
    style_id: int | None = None,
    scrollable_style_id: int | None = None,
    show: bool,
    on_column_resize: Any | None = None,
    on_column_resize_release: Any | None = None,
    user_data: Any | None = None,
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
    position_follow_cursor: bool | None = None,
    position_bottom: bool | None = None,
    position_left: bool | None = None,
    position_top: bool | None = None,
    position_right: bool | None = None,
    gap: int | None = None,
    padding: list[float, 4] | list[float] | None = None,
    snap_within_viewport: bool | None = None,
    delay_sec: float | None = None,
    container_style_id = None,
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


# class Card:
#     """Context manager wrapper around add_card.

#     Wraps the iced_aw Card — a widget that aligns its contents inside
#     of its boundaries.\n
#     A Card take 1, 2, or 3 widgets.\n
#     if 1, assumed only body of card.\n
#     if 2, assumes head and body of card, respectively.\n
#     if 3, uses head, body, foot, respectively.

#     Usage::

#         with Window(title="Demo"):
#             with Card(
#                 width=300.0,
#                 height=200.0,
#                 padding=[5],
#                 on_close=minimize_card
#                 ):

#                 add_text(content="Card")
#                 add_text(content="This is the body of the card.")
#                 with Column(width_fill=True, height=30):
#                     add_separator(line_length=300)
#                     add_text(content="Foot content")

#         start_session()
#     """
#     def __init__(
#         self,
#         *,
#         window_id: str | None = None,
#         container_id: str | None = None,
#         parent_id: str | None = None,
#         is_open: bool | None = True,
#         close_icon: bool | None = None,
#         close_icon_size: float | None = None,
#         on_close: Callable | None = None,
#         width: float | None = None,
#         width_fill: bool | None = None,
#         height: float | None = None,
#         height_fill: bool | None = None,
#         fill: bool | None = None,
#         max_width: float | None = None,
#         max_height: float | None = None,
#         padding: list[float] | None = None,
#         padding_body: list[float] | None = None,
#         padding_foot: list[float] | None = None,
#         style_id: int | None = None,
#         style_std: CardStyleStd | None = None,
#         show: bool = True,
#         user_data: Any | None = None,
#         gen_id: int | None = None,
#     ) -> None: ...
#     def __enter__(self) -> int: ...
#     def __exit__(self, exc_type: type[BaseException] | None, \
#         exc_val: BaseException | None, \
#             exc_tb: TracebackType | None) -> bool: ...

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
        wrap: bool | None = False,
        wrap_horizontal_spacing: bool | None = False,
        wrap_align_top: bool | None = False,
        wrap_align_center: bool | None = False,
        wrap_align_bottom: bool | None = False,
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
        start_session()
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

class MenuBarItem:
    """Context manager wrapper around add_menu_bar_item.

    A container which hold the dropdown menu item.

    Usage::

        with Menu(spacing=10.0) as state["bar_testing_id"]:
                # First item of the MenuBarItem is the bar item followed by the dropdown items
                with MenuBarItem(width=75.0, spacing=5.0):
                    add_text(content="File") # bar item
                    # dropdown items
                    add_button(label="New",
                            if_menu_btn=True,
                            on_press=on_press)
                    add_button(label="Open",
                            if_menu_btn=True,
                            on_press=on_press)
        start_session()
    """
    def __init__(
        self,
        *,
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        width: float | None = None,
        spacing: float | None = None,
        offset: float | None = None,
        padding: list[float] | None = None,
        close_on_item_click: bool | None = None,
        close_on_background_click: bool | None = None,
        show: bool = True,
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
        start_session()
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

class Opaque:
    """Context manager wrapper around add_opaque.

    A container that prevents mouse actions from passing throught.

    Usage::

        with Window(center=True):
            with Container(fill=True, align_center=True):
                with Stack():
                    with Container(width=200, height=100, align_top_center=True,
                                    style_std=ContainerStyleStd.BorderedBox):
                        add_button(
                            label="I'm on the bottom, so you can't press me",
                            on_press=no_press)
                    with Opaque():
                        with Container(width=200, height=100, align_bottom_center=True):
                            add_button(
                                label="I'm on the top so my call back works",
                                on_press=on_press)
        start_session()
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

class RichText:
    """Context manager wrapper around add_rich_text.

    A container that holds text spans.

    Usage::

        with Window(title="Demo"):
            with RichText():
                add_span(
                    text="I am Light Blue and have an underline!",
                    color=Color.LIGHT_BLUE,
                    underline=True)
        start_session()
    """
    def __init__(
        self,
        *,
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        size: float | None = None,
        line_height: float | None = None,
        width: float,
        width_fill: bool | None = None,
        height: float,
        height_fill: bool | None = None,
        fill: bool | None = None,
        font_id: int | None = None,
        color: Color | None = None,
        color_alpha: float | None = None,
        rgba: list[float, 4] | None = None,
        align_bottom_center: bool | None = None,
        align_bottom_left: bool | None = None,
        align_bottom_right: bool | None = None,
        align_center_left: bool | None = None,
        align_center_right: bool | None = None,
        align_center: bool | None = None,
        align_top_center: bool | None = None,
        align_top_left: bool | None = None,
        align_top_right: bool | None = None,
        wrapping_none: bool | None = None,
        wrapping_glyph: bool | None = None,
        wrapping_word_glyph: bool | None = None,
        on_link_click: Any | None = None,
        user_data: Any | None = None,
        show: bool = True,
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
        start_session()
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
        wrap: bool | None = None,
        wrap_vertical_spacing: bool | None = None,
        wrap_align_left: bool | None = None,
        wrap_align_center: bool | None = None,
        wrap_align_right: bool | None = None,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None,\
            exc_tb: TracebackType | None) -> bool: ...

class Sash:
    """Context manager wrapper around add_sash.

    Add a sash (resizable panels) container.

    A sash divides its children into resizable panels separated by draggable
    handle bars. By default it is horizontal (panels arranged left-to-right).
    Set ``vertical_direction=True`` to arrange panels top-to-bottom instead.

    Usage::

        with Window(title="Demo"):
            with Sash(
                initial_sizes=sizes,
                size=200.0,
                sash_size=4.0,
                outer_handle_size=4.0,
                on_resize=on_resize_sash_h,
                on_resize_outer=on_resize_outer) as sash_id:

                state["sash_ids"].append(sash_id)

                # Add containers to the Sash
                with Container(
                    fill=True,
                    align_center=True,
                    style_id=left_style):

                    add_text(content="Left")

                with Container(
                    fill=True,
                    align_center=True,
                    style_id=middle_style):

                    add_text(content="Center")

                with Container(
                    fill=True,
                    align_center=True,
                    style_id=right_style):

                    add_text(content="Right")

        start_session()
    """
    def __init__(
        self,
        initial_sizes: list[float],
        size: float,
        sash_size: float,
        *,
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        sync_sashes: bool | None = None,
        sync_cross_sashes: bool | None = None,
        outer_handle_size: float | None = None,
        resize_mode_last_only: bool | None = None,
        resize_mode_uniform: bool | None = None,
        resize_mode_proportional: bool | None = None,
        on_resize: Callable | None = None,
        on_resize_outer: Callable | None = None,
        on_release: Callable | None = None,
        vertical_direction: bool | None = None,
        min_size: float | None = None,
        max_size: float | None = None,
        style_id: int | None = None,
        user_data: any | None = None,
        show: bool | None = True,
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
        start_session()
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
        start_session()
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
        start_session()
    """
    def __init__(
        self,
        text,
        *,
        window_id: str | None = None,
        container_id: str | None = None,
        parent_id: str | None = None,
        position_follow_cursor: bool | None = None,
        position_bottom: bool | None = None,
        position_left: bool | None = None,
        position_top: bool | None = None,
        position_right: bool | None = None,
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
