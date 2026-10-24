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
    add_combobox_input_style as add_combobox_input_style,
    add_combobox_menu_style as add_combobox_menu_style,
    add_container_style as add_container_style,
    add_font_style as add_font_style,
    add_input_int_style as add_input_int_style,
    # add_menu_style as add_menu_style,
    add_menu_button_style as add_menu_button_style,
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
    add_text_style as add_text_style,
    add_text_editor_style as add_text_editor_style,
    add_text_input_style as add_text_input_style,
    add_toggler_style as add_toggler_style,
    Arrow as Arrow,
    arrow_variants as arrow_variants,
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
    FileSystemDialogParam as FileSystemDialogParam,
    FileSystemDialogCallbackType as FileSystemDialogCallbackType,
    get_dialog_filters as get_dialog_filters,
    get_user_data as get_user_data,
    update_user_data as update_user_data,
    get_widget_parameters as get_widget_parameters,
    get_widget_palette_part as get_widget_palette_part,
    get_widget_palette_list as get_widget_palette_list,
    get_widget_default_statuses as get_widget_default_statuses,
    FilterMethod as FilterMethod,
    FloatParam as FloatParam,
    FontFamily as FontFamily,
    FontWeight as FontWeight,
    FontStretch as FontStretch,
    FontStyle as FontStyle,
    GridParam as GridParam,
    Icon as Icon,
    InputFloatParam as InputFloatParam,
    InputFloatStyleParam as InputFloatStyleParam,
    InputIntParam as InputIntParam,
    InputIntStyleParam as InputIntStyleParam,
    # MenuBarItemParam as MenuBarItemParam,
    # MenuParam as MenuParam,
    # MenuStyleParam as MenuStyleParam,
    # MenuSubItemParam as MenuSubItemParam,
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
    TextContrast as TextContrast,
    TextInputParam as TextInputParam,
    TextEditorParam as TextEditorParam,
    TextParam as TextParam,
    TextStyleParam as TextStyleParam,
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
    get_widget_palette_parameters as get_widget_palette_parameters,
    update_widget_palette_parameters as update_widget_palette_parameters,
    load_font as load_font,
    generate_id as generate_id,
    get_rgba_color as get_rgba_color,
    get_color_palette as get_color_palette,
    get_styling_palette as get_styling_palette,
    get_button_palette as get_button_palette,
    StdColorStyle as StdColorStyle,
    custom_palette as custom_palette,
    PaletteKey as PaletteKey,
    WidgetStatus as WidgetId,
    StylePart as StylePart,
    StateVariant as StateVariant,
)


def add_button(
    *,
    parent_id: str | None = None,
    label: str = ...,
    on_press: Callable | None = None,
    width: float | None = None,
    width_fill: bool = False,
    height: float | None = None,
    height_fill: bool = False,
    fill: bool | None = None,
    padding: list[float] | None = None,
    clip: bool | None = None,
    font_id: int | None = None,
    style_id: int | None = None,
    style_std: ButtonStyleStd | None = None,
    style_arrow: Arrow | None = None,
    palette_id: int | None = None,
    user_data: Any | None = None,
    active: bool | None = None,
    hovered: bool | None = None,
    pressed: bool | None = None,
    disabled: bool | None = None,
    show: bool = True,
    gen_id: int | None = None,
) -> int:
    """Adds a button widget.

    A clickable button used for GUI actions. The button must go into a
    container type (Container, Column, Row, etc.).

    Parameters
    ----------
    parent_id : str, optional
        Sets the parent container ID that this button belongs to.
    label : str, optional
        Sets the text label displayed on the button.
    on_press : callable, optional
        Sets the callback method to invoke when the button is pressed.
    width : float, optional
        Sets the fixed width in logical pixels.
    width_fill : bool, default False
        Whether the button fills available width.
    height : float, optional
        Sets the fixed height in logical pixels.
    height_fill : bool, default False
        Whether the button fills available height.
    fill : bool, optional
        Whether the button fills available width and height.
    padding : list of float, optional
        Sets the padding as [all], [vertical, horizontal], or
        [top, right, bottom, left].
    clip : bool, optional
        Whether to clip content that overflows the button.
    font_id : int, optional
        Sets the ID of a custom font created with ``add_font_style``.
    style_id : int, optional
        Sets the ID of a custom style created with ``add_button_style``.
    style_std : ButtonStyleStd, optional
        Sets a predefined standard style variant.
    style_arrow : Arrow, optional
        Sets an arrow icon style for the button.
    palette_id : int, optional
        Sets the ID of a custom color palette, see py_palette_create.py.
    user_data : Any, optional
        Sets arbitrary data forwarded to callbacks.
    show : bool, default True
        Whether the button is visible.
    status_active: bool, Optional
        Whether to set the button status as Active.
    status_disabled: bool, Optional
        Whether to set the button status as Disabled.
    status_hovered: bool, Optional
        Whether to set the button status as Hovered.
    status_pressed: bool, Optional
        Whether to set the button status as Pressed.
    gen_id : int, optional
        Obtains an ID for a widget that has not been created;
        used for the gen_id parameter.

    Returns
    -------
    int
        The numeric widget ID of the newly created button.

    Examples
    --------
    >>> from icedpygui import (
    ...     Window,
    ...     Column,
    ...     Container,
    ...     add_button,
    ...     start_session)
    >>>
    >>> def btn_cb(btn_id: int):
    ...     print(btn_id)
    >>>
    >>> def cb_with_data(btn_id: int, data: any):
    ...     print(btn_id, data)
    >>>
    >>> with Window(title="My App", center=True):
    ...     with Container(align_center=True, fill=True):
    ...         with Column(spacing=20):
    ...             add_button(label="Press Me", on_press=btn_cb)
    ...             add_button(
    ...                 label="Press Me",
    ...                 on_press=cb_with_data,
    ...                 user_data="Some Data"
    ...             )
    >>>
    >>> start_session()

    """
    ...

class ButtonParam:
    """
    Button parameters

    Parameters
    ----------
    Clip: bool
        Whether the label is clipped or not
    Fill : bool
        Whether the button fills available width and height.
    FontId : int
        Sets the ID of a custom font created with ``add_font_style``.
    Height: float
        The height of the button.
    HeightFill: bool
        Whether the button height fills the available space of a container.
    Label: str
        The label of the button.
    Padding: list[float], one or four items
        The padding around the button, [all sides], [top, right, bottom, left].
    PaletteId : int
        Sets the ID of a custom color palette, see py_palette_create.py.
    Show: bool
        Whether to show the button
    StatusActive: bool
        Whether to set the button status as Active.
    StatusDisabled: bool
        Whether to set the button status as Disabled.
    StatusHovered: bool
        Whether to set the button status as Hovered.
    StatusPressed: bool
        Whether to set the button status as Pressed.
    StyleArrow: ButtonArrow
        A button becomes a type of arrow.
    StyleId: int
        The id of the add_button_style() function
    StyleStd: ButtonStyleStd
        The standard styles for a button
    Width: float
        The width of the button
    WidthFill: bool
         Whether the button width fills the available space of a container.
    """
    Clip = ...
    Fill = ...
    FontId = ...
    Height = ...
    HeightFill = ...
    Label = ...
    Padding = ...
    PaletteId = ...
    Show = ...
    StatusActive = ...
    StatusDisabled = ...
    StatusHovered = ...
    StatusPressed = ...
    StyleArrow = ...
    StyleId = ...
    StyleStd = ...
    Width = ...
    WidthFill = ...


def add_button_style(
    text_top_left: bool | None = None,
    text_top_center: bool | None = None,
    text_top_right: bool | None = None,
    text_center_left: bool | None = None,
    text_center: bool | None = None,
    text_center_right: bool | None = None,
    text_bottom_left: bool | None = None,
    text_bottom_center: bool | None = None,
    text_bottom_right: bool | None = None,
    text_size: float | None = None,
    wrapping_none: bool | None = None,
    wrapping_glyph: bool | None = None,
    wrapping_word_glyph: bool | None = None,
    gradient_color_stops: list[Color] | None = None,
    gradient_color_alpha_stops: list[float] | None = None,
    gradient_rgba_stops: list[float, 4] | None = None,
    gradient_offset_stops: list[float] | None = None,
    gradient_degrees: float | None = None,
    gradient_radians: float | None = None,
    border_color: Color | None = None,
    border_color_alpha: float | None = None,
    border_rgba: list[float, 4] | None = None,
    border_radius: list[float] | list[float, 4] | None = None,
    border_width: float | None = None,
    shadow_color: Color | None = None,
    shadow_color_alpha: float | None = None,
    shadow_rgba: list[float, 4] | None = None,
    shadow_offset_xy: list[float, 2] | None = None,
    shadow_blur_radius: float | None = None,
    snap: bool | None = None,
    gen_id: int | None = None,
    ) -> int:
    """Adds styling to a button

    There are 4 ways to add styling to a widget.  Widgets vary a bit
    because some have statues like checked, pressed, etc.
    1. Do nothing and the widgets default to one of the standard styles
       which is usually primary but varies some from widget to widget.
    2. You select one of the available standard styles (see below)
    3. You create your own palette (see the palette example which gives a full explanation)
    4. You add styling that is not set by any of the standard styles like gradient,
       shadow, text parameters, border width and color.  These are listed below.

    The standard style colors will vary depending on the theme selected
    for the window. This makes sure that there is enough contrast between the
    colors and the theme background to be seen.

    Standard styles are:
    Background, Danger, Primary, Secondary,
    Subtle, Success, Warning, Text,

    Each widget has a status where the color changes based on the state of the widget.
    The status type will vary among the widgets due of the actions they produce.
    For example, a button has a status of pressed and a checkbox has a status of checked.

    The statuses are:
    Active, Hovered, Pressed, Disabled

    gradient — applied to the background instead of a solid color:
      gradient_color_stops: list[Color, ≤8]
      gradient_color_alpha_stops: list[float]
      gradient_rgba_stops: list[list[float, 4], ≤8]
        Up to 8 stops total (color + rgba combined).
      gradient_offset_stops: list[float]
        Offset positions for each stop (0.0–1.0).
      gradient_degrees: float — angle in degrees
      gradient_radians: float — angle in radians (takes priority over degrees)

    shadow:
      shadow_color, shadow_color_alpha, shadow_rgba
        Shadow color.
      shadow_offset_xy: list[float, 2]
        Shadow offset [x, y].
      shadow_blur_radius: float
        Shadow blur radius.

    text alignment (default: center):
      text_top_left, text_top_center, text_top_right: bool
      text_center_left, text_center, text_center_right: bool
      text_bottom_left, text_bottom_center, text_bottom_right: bool

    text_size: float — font size for the label

    text wrapping (default: Word):
      wrapping_none: bool — no wrapping
      wrapping_glyph: bool — wrap at glyph boundary
      wrapping_word_glyph: bool — wrap at word, fall back to glyph

    border_radius: list[float] — [all] or [top-left, top-right, bottom-right, bottom-left]
    border_width: float
        Sets the border line width.  Unless border width is set, no other parameters take effect.
    border_rounded: float
        Whether to round the button box. Rounded overrides any other border settings.

    snap: bool
        Whether to snap rendering to pixel grid
    gen_id: int, Optional
        Obtain an ID for a widget not yet created; use for the gen_id parameter.

    Examples
    --------
    >>> from icedpygui import (
    ...     Window, Column, Container,
    ...     ButtonStyleStd,
    ...     add_button, add_button_style,
    ...     Color, start_session)
    >>>
    >>> style = add_button_style(
    ...     border_color=Color.ALICE_BLUE,
    ...     border_width=2.0,
    ...     border_rounded=5.0)
    >>>
    >>> with Window(title="Button Styling", center=True):
    ...     with Container(align_center=True, fill=True):
    ...         with Column(spacing=20.0):
    ...             add_button(
    ...                 label="Border Color and Width",
    ...                 padding=[5.0],
    ...                 style_id=style)
    ...
    ...             add_button(
    ...                 label="Style Standard-Danger",
    ...                 padding=[5.0],
    ...                 style_std=ButtonStyleStd.Danger)
    ...
    ...             add_button(
    ...                 label="Style Standard-Text",
    ...                 padding=[5.0],
    ...                 style_std=ButtonStyleStd.Text)
    >>>
    >>> start_session()
    """
    ...

class ButtonStyleParam:
    """
    Button style parameters

    Parameters
    ----------
    BorderColor: Color
        Border color in Color format
    BorderColorAlpha: float
        Modifies the BorderColor alpha
    BorderRadius: list[float]
        The border radius [float]=all, [float, 4]=each individual one
    BorderRgba :list[float, 4]
        Border color in rgba format
    BorderRounded: bool
        Whether to round the corners of the border.
    BorderWidth: float
        The width of the border line
    GradientColorAlphaStops: list[float]
        List of alpha values for each gradient color stop
    GradientColorStops: list[Color]
        List of color stops for the gradient
    GradientDegrees: float
        Gradient angle in degrees
    GradientRadians: float
        Gradient angle in radians
    GradientRgbaStops: list[list[float, 4]]
        List of RGBA color stops for the gradient
    ShadowBlurRadius: float
        How much to blur the shadow radius
    ShadowColor: Color
        Shadow color in Color format
    ShadowColorAlpha: float
        Modifies the ShadowColor alpha
    ShadowOffsetXy: list[float, 2]
        The offset of the shadow towards the x and y directions
    ShadowRgba: list[float, 4]
        Shadow color in rgba format
    Snap: bool
        Whether to snap the button rendering
    TextAlignBottomCenter: bool
        Whether to align the label bottom center
    TextAlignBottomLeft: bool
        Whether to align the label bottom left
    TextAlignBottomRight: bool
        Whether to align the label bottom right
    TextAlignCenter: bool
        Whether to align the label centered (default)
    TextAlignCenterLeft: bool
        Whether to align the label center left
    TextAlignCenterRight: bool
        Whether to align the label center right
    TextAlignTopCenter: bool
        Whether to align the label top center
    TextAlignTopLeft: bool
        Whether to align the label top left
    TextAlignTopRight: bool
        Whether to align the label top right
    TextSize: float
        The size of the text
    WrappingGlyph: bool
        Enable glyph-based text wrapping
    WrappingNone: bool
        Disable text wrapping
    WrappingWordGlyph: bool
        Enable word or glyph text wrapping
    """
    BorderColor: Color
    BorderColorAlpha: float
    BorderRadius: list[float]
    BorderRgba: list[float]
    BorderRounded: bool
    BorderWidth: float
    GradientColorAlphaStops: list[float]
    GradientColorStops: list[Color]
    GradientDegrees: float
    GradientRadians: float
    GradientRgbaStops: list[list[float, 4]]
    ShadowBlurRadius: float
    ShadowColor: Color
    ShadowColorAlpha: float
    ShadowOffsetXy: list[float, 2]
    ShadowRgba: list[float, 4]
    Snap: bool
    TextAlignBottomCenter: bool
    TextAlignBottomLeft: bool
    TextAlignBottomRight: bool
    TextAlignCenter: bool
    TextAlignCenterLeft: bool
    TextAlignCenterRight: bool
    TextAlignTopCenter: bool
    TextAlignTopLeft: bool
    TextAlignTopRight: bool
    TextSize: float
    WrappingGlyph: bool
    WrappingNone: bool
    WrappingWordGlyph: bool

class ButtonStyleStd:
    """
    Standard styles for Button widget

    Parameters
    ----------
    Background: bool
        Sets the bkg color to that of the background
    Danger:
        Sets the bkg color to taht of a Danger
    Primary: bool
        Sets the bkg color to that of the Primary Theme
    Secondary: bool
        Sets the bkg color to that of the Secondary Theme
    Subtle: bool
        Sets the bkg color to a more sublr color
    Success: bool
        Sets the bkg color to that of the Success Theme
    Warning: bool
        Sets the bkg color to that of the Warning Theme
    Text: bool
        Sets the bkg color to that of the bakg Theme
    """
    Background=...
    Danger=...
    Primary=...
    Secondary=...
    Subtle=...
    Success=...
    Warning=...
    Text=...

def add_checkbox(
    *,
    parent_id: str | None = None,
    gen_id: int | None = None,
    icon_font_id: int | None = None,
    icon_line_height: float | None = None,
    icon_shaping: bool | None = None,
    icon_size: float | None = None,
    icon: Icon | None = None,
    is_checked: bool = False,
    label: str | None = None,
    lock_is_checked: bool | None = None,
    on_toggle: Callable | None = None,
    palette_id: int | None = None,
    show: bool = True,
    size: float | None = None,
    spacing: float | None = None,
    status_active: bool | None = None,
    status_disabled: bool | None = None,
    status_hovered: bool | None = None,
    style_id: int | None = None,
    style_std: int | None = None,
    text_font_id: int | None = None,
    text_line_height: float | None = None,
    text_shaping: bool | None = None,
    text_size: float | None = None,
    text_wrapping: bool | None = None,
    user_data: Any | None = None,
    width_fill: bool = False,
    width: float | None = None,
) -> int:
    """Adds a checkbox widget.

    Add a checkbox widget.

    A checkbox with a text label that can be toggled on and off.

    Parameters
    ----------
    parent_id : str
        Sets the parent container ID that this checkbox belongs to.
    gen_id : int,  Optional
        Obtains an ID of a widget that have not been created, used for the gen_id parameter.
    icon_font_id : int,  Optional
        Sets the Font ID for the checkbox icon.
    icon : Icon,  Optional
        Sets the Icon displayed inside the checkbox when checked.
    icon_size : float,  Optional
        Sets the Size of the checkbox icon.
    icon_line_height : float,  Optional
        Sets the Line height of the checkbox icon.
    is_checked : bool, default False
        Whether the checkbox starts in the checked state.
    label : str,  Optional
        Sets the Text label displayed next to the checkbox.
    lock_is_checked: bool, Optional
        Keeps the check or unchecked value to True or False.
    palette_id: int, Optional
        Sets the ID of a custom color palette, see py_palette_create.py.
    on_toggle : callable,  Optional
        Sets the Callback method to invoke when the checkbox is toggled.
    show : bool, default True
        Whether the checkbox is visible.
    size : float,  Optional
        Sets the size of the checkbox square in logical pixels.
    spacing : float,  Optional
        Sets the Spacing between the checkbox square and the label.
    status_active : bool, Optional
        Sets the checkbox to the Active status.
    status_hovered : bool, Optional
        Sets the checkbox to the Hovered status.
    status_disabled : bool, Optional
        Sets the checkbox to the Disabled status.
    style_id : int,  Optional
        Sets the ID of a custom style created with ``add_checkbox_style``.
    style_std : CheckboxStyleStd,  Optional
        Sets the predefined standard style variant.
    text_font_id : int,  Optional
        Sets the Font ID for the label text.
    text_line_height : float,  Optional
        Sets the Line height for the label text.
    text_size : float,  Optional
        Sets the Font size for the label text.
    text_wrapping_glyph: Optional[bool]
        Sets the wrapping mode of the label to wrap on a glyph.
    text_wrapping_none: Optional[bool]
        Sets the wrapping mode of the label to no wrapping, default is on a word.
    text_wrapping_word_glyph: Optional[bool]
        Sets the wrapping mode of the label to wrap on a glyph or glyph.
    user_data : Any,  Optional
        Sets the Arbitrary data forwarded to callbacks.
    width : float,  Optional
        Sets the Fixed width in logical pixels.
    width_fill : bool, default False
        Whether the checkbox fills available width.

    Returns
    -------
    int
        The numeric widget ID of the newly created button.

    Examples
    --------
    >>> def checked(chk_id: int, is_checked: bool):
    ...     print(chk_id, is_checked)
    >>>
    >>> with Window(title="My App", pos_centered=True):
    ...     with Container(align_center=True):
    ...         add_checkbox(
    ...             label="Check Me",
    ...             on_toggle=checked)
    >>>
    >>> start_session()
    """
    ...

class CheckboxParam:
    """
    The Checkbox parameters.

    Parameters
    ----------
    Fill: bool
        Whether to fill the containing width.
    Icon: Icon
        Sets the override Icon to use instead of the checkmark, i.e. a X icon.
    IconFontId: int
        The id of the get_font() method
    IconLineHeight: float
        Sets the line height of the icon, defaults 1.3
    IconSize: float
        Sets the size of the icon.
    IsChecked: bool
        Whether the checkbox is checked or not.
    Label: str
        Sets the label of the checkbox.
    LockIsChecked: bool
        Whether to lock the checkmark so it can't be changed.
    PaletteId : int
        Sets the ID of a custom color palette, see py_palette_create.py.
    Show: bool
        Whether to show the checkbox.
    Size: float
        The size of the square.
    Spacing: float
        The spacing between the square and label.
    StatusActive: bool
        Sets and holds the active status of the checkbox.
    StatusDisabled: bool
        Sets and holds the disabled status of the checkbox.
    StatusHovered: bool
        Sets and holds the hovered status of the checkbox.
    StyleId: int
        The id of the add_checkbox_style() function.
    StyleStd: StyleStd
        One of the standard styles of CheckboxStyleStd enum.
    TextFontId: int
        Sets the ID of the label font from get_font().
    TextLineHeight: float
        The height of the text box holding the label.
    TextSize: float
        The size of the label text.
    TextWrappingGlyph: bool
        Whether the label wraps on a glyph.
    TextWrappingNone: bool
        Sets for no wrapping.
    TextWrappingWordGlyph: bool
        Whether to wrap on a word or glyph.
    Width: float
        The width of the entire checkbox.

    Examples
        --------
        Given a widget id for a checkbox = wid
        >>>
        >>> update_widget(wid, CheckboxParam.Width, 100.0)
        >>>
    """
    Fill: bool
    Icon: Icon
    IconFontId: int
    IconLineHeight: float
    IconSize: float
    IsChecked: bool
    Label: str
    LockIsChecked: bool
    PaletteId: int
    Show: bool
    Size: float
    Spacing: float
    StatusActive: bool
    StatusDisabled: bool
    StatusHovered: bool
    StyleId: int
    StyleStd: CheckboxStyleStd
    TextFontId: int
    TextLineHeight: float
    TextSize: float
    TextWrappingGlyph: bool
    TextWrappingNone: bool
    TextWrappingWordGlyph: bool
    Width: float
    ...

def add_checkbox_style(
    bkg_color_alpha: float | None = None,
    bkg_color: Color | None = None,
    bkg_rgba: list[float, 4] | None = None,
    border_color_alpha: float | None = None,
    border_color: Color | None = None,
    border_radius_bottom_left: float | None = None,
    border_radius_bottom_right: float | None = None,
    border_radius_bottom: float | None = None,
    border_radius_left: float | None = None,
    border_radius_right: float | None = None,
    border_radius_top_left: float | None = None,
    border_radius_top_right: float | None = None,
    border_radius_top: float | None = None,
    border_radius: list[float, 4] | list[float, 4] | None = None,
    border_rgba: list[float, 4] | None = None,
    border_rounded: float | None = None,
    border_width: float | None = None,
    gen_id: int | None = None,
    icon_color_alpha: float | None = None,
    icon_color: Color | None = None,
    icon_rgba: list[float, 4] | None = None,
    text_color_alpha: float | None = None,
    text_color: Color | None = None,
    text_rgba: list[float, 4] | None = None,
    ) -> int:
    """Adds styling to a checkbox

    Notes
    --------
    Two styles can be defined:

    custom - defined by using an add_style method

    First, define the style, this can be placed anywhere as long as the
    add_checkbox can access the style value, type int.

    Then, add your checkbox(es) as usual and use the style_id=your_style parameter.

    standard - using the style_std parameter and the CheckboxStyleStd class

    Just use the style_std=CheckboxStyleStd.Primary parameter in the add_checkbox method.

    Additional style can be done by using the custom_palette.

    Parameters
    ----------
    bkg_color: Color
        Background color in Color format.
    bkg_color_alpha: float
        Sets the alpha value of the background Color.
    bkg_rgba: list[float, 4]
        Background color in rgba format.
    icon_color: Color, Optional
        Sets the icon color.
    icon_color_alpha: float, Optional
        Sets the alpha value of the icon Color.
    icon_rgba: list[float, 4], Optional
        The icon color in rgba format.
    border_color: Color
        Border color in Color format.
    border_color_alpha: float
        Sets the alpha of the border Color.
    border_rgba: list[float, 4]
        Border color in rgba format.
    border_radius: list[float, 4], Optional
        Sets the radius for all four corners as [top_left, top_right, bottom_right, bottom_left].
    border_rounded: float, Optional
        Sets the radius for all four corners to the same value.
    border_radius_top: float, Optional
        Sets the radius for top-left and top-right corners.
    border_radius_top_left: float, Optional
        Sets the radius for the top-left corner.
    border_radius_top_right: float, Optional
        Sets the radius for the top-right corner.
    border_radius_bottom: float, Optional
        Sets the radius for bottom-left and bottom-right corners.
    border_radius_bottom_left: float, Optional
        Sets the radius for the bottom-left corner.
    border_radius_bottom_right: float, Optional
        Sets the radius for the bottom-right corner.
    border_radius_left: float, Optional
        Sets the radius for top-left and bottom-left corners.
    border_radius_right: float, Optional
        Sets the radius for top-right and bottom-right corners.
    border_width: float
        The width of the border line.
    text_color: Color
        The label text color in Color format.
    text_rgba: list[float, 4]
        The label text color in rgba format.

    Examples
    --------
    >>> from icedpygui import (Window, Column, Container, CheckboxStyleStd,
    ...     add_checkbox, add_checkbox_style, Color, start_session)
    >>>
    >>> style = add_checkbox_style(
    ...     border_color=Color.RED,
    ...     border_width=3.0)
    >>>
    >>> with Window(title="Checkbox Styling", center=True):
    ...     with Container(align_center=True, width_fill=True, height_fill=True):
    ...         with Column(spacing=20.0):
    ...             add_checkbox(
    ...                 label="Border Color and Width",
    ...                 style_id=style)
    ...
    ...             add_checkbox(
    ...                 label="Secondary",
    ...                 style_std=CheckboxStyleStd.Secondary)
    >>>
    >>> start_session()
    """

class CheckboxStyleParam:
    """
    Checkbox style parameters

    Parameters
    ----------
    BkgColor: Color
        Background color in Color format.
    BkgColorAlpha: float
        Sets the alpha value of the background Color.
    BkgRgba: list[float, 4]
        Background color in rgba format.
    BorderColor: Color
        Border color in Color format.
    BorderColorAlpha: float
        Sets the alpha of the border Color.
    BorderRadius: list[float, 4], Optional
        Sets the radius for all four corners as [top_left, top_right, bottom_right, bottom_left].
    BorderRadiusBottom: float, Optional
        Sets the radius for bottom-left and bottom-right corners.
    BorderRadiusBottomLeft: float, Optional
        Sets the radius for the bottom-left corner.
    BorderRadiusBottomRight: float, Optional
        Sets the radius for the bottom-right corner.
    BorderRadiusLeft: float, Optional
        Sets the radius for top-left and bottom-left corners.
    BorderRadiusRight: float, Optional
        Sets the radius for top-right and bottom-right corners.
    BorderRadiusTop: float, Optional
        Sets the radius for top-left and top-right corners.
    BorderRadiusTopLeft: float, Optional
        Sets the radius for the top-left corner.
    BorderRadiusTopRight: float, Optional
        Sets the radius for the top-right corner.
    BorderRgba: list[float, 4]
        Border color in rgba format.
    BorderRounded: float, Optional
        Sets the radius for all four corners to the same value.
    BorderWidth: float
        The width of the border line.
    IconColor: Color, Optional
        Sets the icon color.
    IconColorAlpha: float, Optional
        Sets the alpha value of the icon Color.
    IconRgba: list[float, 4], Optional
        The icon color in rgba format.
    TextColor: Color
        The label text color in Color format.
    TextColorAlpha: float, Optional
        Sets the alpha value of the text_color.
    TextRgba: list[float, 4]
        The label text color in rgba format.

    Examples
        --------
        Given a checkbox style id = wid
        >>> update_widget(wid, CheckboxStyleParam.TextColor, Color.WHITE)
    """
    BkgColor: Color
    BkgColorAlpha: float
    BkgRgba: list[float, 4]
    BorderColor: Color
    BorderColorAlpha: float
    BorderRadius: list[float, 4]
    BorderRadiusBottom: float
    BorderRadiusBottomLeft: float
    BorderRadiusBottomRight: float
    BorderRadiusLeft: float
    BorderRadiusRight: float
    BorderRadiusTop: float
    BorderRadiusTopLeft: float
    BorderRadiusTopRight: float
    BorderRgba: list[float, 4]
    BorderRounded: float
    BorderWidth: float
    IconColor: Color
    IconColorAlpha: float
    IconRgba: list[float, 4]
    TextColor: Color
    TextColorAlpha: float
    TextRgba: list[float, 4]

class CheckboxStyleStd:
    """
    Standard styles for Checkbox widget

    Parameters
    ----------
    Danger:
        Sets the bkg color to taht of a Danger
    Primary: bool
        Sets the bkg color to that of the Primary Theme
    Secondary: bool
        Sets the bkg color to that of the Secondary Theme
    Success: bool
        Sets the bkg color to that of the Success Theme
    """
    Danger=...
    Primary=...
    Secondary=...
    Success=...


def add_combobox(
    *,
    options: list[any],
    parent_id: int,
    on_select: Callable | None = None,
    on_open: Callable | None = None,
    on_close: Callable | None = None,
    on_input: Callable | None = None,
    width: float | None = None,
    width_fill: int | None = None,
    menu_height: float | None = None,
    menu_height_fill: int | None = None,
    padding: list[float, 1] | list[float, 4] | None = None,
    placeholder: str | None = None,
    selected: str | None = None,
    text_size: float | None = None,
    text_line_height: float | None = None,
    text_ellipsis_start: bool | None = None,
    text_ellipsis_middle: bool | None = None,
    text_ellipsis_end: bool | None = True,
    disabled: bool | None = None,
    font_id: int | None = None,
    input_style_id: int | None = None,
    menu_style_id: int | None = None,
    user_data: any | None = None,
    show: bool = True,
    gen_id: int | None = None,
) -> int:
    """
    Add a combobox widget.
    A dropdown list that lets the user select one option
    from a list of choices.  The selctions are filter based
    on what is typed.  The combobox is good for long lists.

    Parameters
    ----------
    parent_id : str
        Sets the parent container ID that this pick list belongs to.
        Not used when parent is using with Widget(): ...
    options : list of any
        Sets the list of selectable options.
    gen_id : int, Optional
        Obtains an ID of a widget that have not been created,
        used for the gen_id parameter.
    on_select : callable, Optional
        Sets the Callback method to invoke when an option is selected.
    on_open : callable, Optional
        Sets the Callback method to invoke when the list is opened.
    on_close : callable, Optional
        Sets the Callback method to invoke when the list is closed.
    on_input : callable, Optional
        Sets the Callback method to invoke when the letters are typed in.
    width : float, Optional
        Sets the Fixed width in logical pixels.  This is required
        if the container has no defined width.
    width_fill : bool, default False
        Whether the pick list fills available width.
    menu_height : float, Optional
        Sets the Fixed height of the dropdown menu in logical pixels.
    menu_height_fill : bool, default False
        Whether the dropdown menu fills available height.
    padding : list of float, Optional
        Sets the Padding as [all], [vertical, horizontal], or
        [top, right, bottom, left].
    placeholder : str, Optional
        Sets the placeholder text shown when no option is selected.
    selected : str, Optional
        Sets the currently selected option.
    text_size : float, Optional
        Sets the Font size for the text.
    text_line_height : float, Optional
        Sets the Line height for the text.
    text_ellipsis_start: bool, Optional
        Ellipsize the start of the last visual line in the text.
    text_ellipsis_middle: bool, Optional
        Ellipsize the middle of the last visual line in the text.
    text_ellipsis_end: bool, Optional
        Ellipsize the end of the last visual line in the text.
    user_data : Any, Optional
        Sets the Arbitrary data forwarded to callbacks.
    show : bool, default True
        Whether the pick list is visible.

    Returns
    -------
    int
        The numeric widget ID of the newly created pick list.
    """
    ...
def add_date_picker(
    window_id: str,
    container_id: str,
    *,
    parent_id: str | None = None,
    on_open: bool | None = None,
    on_submit: bool | None = None,
    on_cancel: bool | None = None,
    opened: bool = False,
    size_factor: float | None = None,
    gap: float | None = None,
    snap_within_viewport: bool | None = None,
    position_bottom: bool | None = None,
    position_left: bool | None = None,
    position_top: bool | None = None,
    position_right: bool | None = None,
    user_data: any | None = None,
    gen_id: bool | None = None,
    ) -> int:
    """
    Add a date picker container widget.

    A date picker that opens a calendar from a button or other widget,
    allowing the user to select a date.

    Parameters
    ----------
    window_id: str
        Sets the window id for the date picker.
        When the parent is using a with ... construction then not required.
    container_id: str,
        Sets the container id of the date picker.
        When the parent is using a with ... construction then not required.
    parent_id : str
        Sets the parent container ID that this date picker belongs to.
        When the parent is using a with ... construction then not required.
    label : str, Optional
        Sets the Text label displayed on the button.
    gen_id : int, Optional
        Obtains an ID of a widget that have not been created, used for the gen_id parameter.
    size_factor : float, Optional
        Sets the size scaling factor for the calendar.
    padding : list of float, Optional
        Sets the Padding as [all], [vertical, horizontal], or
        [top, right, bottom, left].
    on_submit : callable, Optional
        Sets the Callback method to invoke when a date is submitted.
    user_data : Any, Optional
        Sets the Arbitrary data forwarded to callbacks.
    show : bool, default True
        Whether the date picker is visible.
    show_calendar : bool, Optional
        Whether the calendar popup is shown.

    Returns
    -------
    int
        The numeric widget ID of the newly created date picker.
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

def add_file_system_dialog(
    select_file: bool | None = None,
    select_files: bool | None = None,
    select_folder: bool | None = None,
    select_folders: bool | None = None,
    load_file: bool | None = None,
    load_file_for_editor: bool | None = None,
    save_file: bool | None = None,
    file_name: str | None = None,
    file_content: str | None = None,
    filters: list[str] | None = None,
    default_directory: str | None = None,
    title: str | None = None,
    show_hidden_files: bool | None = None,
    remember_last_directory: bool | None = None,
    update_json_file: bool | None = None,
    results_callback: callable | None = None,
    ) -> int:
    """Create a file system dialog for selecting folders or files.

    Multiple modes are supported: selecting single/multiple files or folders,
    loading file content, or saving files. Results are returned via callback.

    Parameters
    ----------
    select_file : bool, optional
        Whether to select a single file name
    select_files : bool, optional
        Whether to select multiple file names
    select_folder : bool, optional
        Whether to select a single folder name
    select_folders : bool, optional
        Whether to select multiple folder names
    load_file : bool, optional
        Whether to load a single file's content
    load_file_for_editor : bool, optional
        Whether to load a file for editing in a text editor
    save_file : bool, optional
        Whether to save/create a new file
    file_name : str, optional
        Default file name for save operations
    file_content : str, optional
        Initial file content or loaded file content
    filters : list[str], optional
        List of file type filters (e.g., ["*.txt", "*.yml", "*.py"]).
        Default: ["*"] (all files)
    default_directory : str, optional
        Starting directory path for the dialog
    title : str, optional
        Custom title for the dialog window
    show_hidden_files : bool, optional
        Whether to show hidden files and folders
    remember_last_directory : bool, optional
        Whether to remember the last opened directory
    update_json_file : bool, optional
        Whether to update a JSON configuration file with the result
    results_callback : callable, optional
        Callback function to invoke when file/folder selection completes

    Returns
    -------
    int
        The numeric widget ID of the newly created file system dialog
    """
def add_float(
    window_id: str ,
    container_id: str,
    *,
    parent_id: str | None = None,
    scale: float | None = None,
    scale_clamped: float | None = None,
    clamped_padding: list[float, 1] | list[float, 4] | None = None,
    translate: list[float, 2] | None = None,
    show: bool = True,
    ) -> int:
    """
    Add an float container widget.

    Allows a widget to foat over others

    Parameters
    ----------
    window_id : str
        Sets the window this float container belongs to.
        If parent uses the construction with ... then not required.
    container_id : str
        Sets the Unique string identifier for the float container.
        If parent uses the construction with ... then not required.
    parent_id : str, Optional
        Sets the parent container ID.  Defaults to the window itself.
    scale: float, Optional
        Sets the scale factor for the contents
    scale_clamped: bool, Optional
        Whether to clamp the scale to the content container
    translate List[float], Optional
        Sets for translation vector [x, y]
    clamped_padding: list[float, 1] | list[float, 4], Optional
        Whether to clamp the padding to the content container

    Returns
    -------
    int
        The numeric widget ID of the newly created opaque container.
    """
    ...
def add_grid(
    window_id: str,
    container_id: str,
    width: float,
    *,
    parent_id: str | None = None,
    spacing: float | None = None,
    columns_max_width: float | None = None,
    columns_amount: int | None = None,
    height_aspect_ratio: float | None = None,
    height_evenly_distribute: float | None = None,
    height_evenly_distribute_fill: bool | None = None,
    show: bool = True,
) -> int:
    """
    Add an grid container widget.

    Allows a widget to foat over others

    Parameters
    ----------
    window_id : str
        Sets the window this grid container belongs to.
    container_id : str
        Sets the Unique string identifier for the grid container.
    parent_id : str, Optional
        Sets the parent container ID.  Defaults to the window itself.
    width: float
        Sets the width of the grid
    spacing: float, Optional
        Sets the spacing between grid items
    columns_max_width float, Optional
        Makes the amount of columns dynamic, never
        exceeding the provided max_width
    columns_amount: int, Optional
        Sets the number of columns in the grid
    height_aspect_ratio: float, Optional
        Sets the aspection ratio for a grid.
    height_evenly_distribute: float, Optional
        Sets how the cels of the grid as distributed
    height_evenly_distribute_fill bool
        Whether to distribute the cells based on size.

    Returns
    -------
    int
        The numeric widget ID of the newly created opaque container.
    Returns:
        int: _description_
    """
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
def add_input_float(
    *,
    parent_id: str | None = None,
    placeholder: str | None = None,
    on_input: Any | None = None,
    on_submit: Any | None = None,
    on_paste: Any | None = None,
    left_side: bool | None = None,
    width: float | None = None,
    width_fill: bool | None = None,
    padding: list[float, 4 | list[float]] | None = None,
    size: float | None = None,
    line_height: float | None = None,
    align_left: bool | None = None,
    align_center: bool | None = None,
    align_right: bool | None = None,
    user_data: Any | None = None,
    font_id: int | None = None,
    style_id: int | None = None,
    show: bool = True,
    gen_id: int | None = None,
) -> int:
    """
Add a text input widget.

    A integer input field with placeholder text.

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
    left_side: bool, Optional
        Whether to place the buttons on the left, default is right.
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
def add_input_int(
    *,
    parent_id: str | None = None,
    placeholder: str | None = None,
    on_input: Any | None = None,
    on_submit: Any | None = None,
    on_paste: Any | None = None,
    arrows: Arrow | None = None,
    width: float | None = None,
    width_fill: bool | None = None,
    padding: list[float, 4 | list[float]] | None = None,
    size: float | None = None,
    line_height: float | None = None,
    align_left: bool | None = None,
    align_center: bool | None = None,
    align_right: bool | None = None,
    user_data: Any | None = None,
    font_id: int | None = None,
    style_id: int | None = None,
    show: bool = True,
    gen_id: int | None = None,
) -> int:
    """
Add a text input widget.

    A integer input field with placeholder text.

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
def add_pick_list(
    options: list[str],
    *,
    parent_id: str | None = None,
    on_select: callable | None = None,
    on_open: callable | None = None,
    on_close: callable | None = None,
    width: float | None = None,
    width_fill: bool | None = None,
    menu_height: float | None = None,
    menu_height_fill: bool | None = None,
    padding: list[float] | None = None,
    placeholder: str | None = None,
    selected: str | None = None,
    text_size: float | None = None,
    text_line_height: float | None = None,
    text_ellipsis_start: bool | None = None,
    text_ellipsis_middle: bool | None = None,
    text_ellipsis_end: bool = True,
    handle_size: float | None = None,
    handle_static_icon_id: int | None = None,
    handle_dynamic_closed_icon_id: int | None = None,
    handle_dynamic_open_icon_id: int | None = None,
    style_id: int | None = None,
    user_data: any | None = None,
    show: bool = True,
    gen_id: int | None = None,
    ) -> int:
    """
    Adds a Pick List widget

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

    Parameters
    ----------
    parent_id : str
        Sets the parent container ID that this pick list belongs to.
    options : list of str
        Sets the list of selectable options.
    gen_id : int, Optional
        Obtains an ID of a widget that have not been created, used for the gen_id parameter.
    on_select : callable, Optional
        Sets the Callback method to invoke when an option is selected.
    width : float, Optional
        Sets the Fixed width in logical pixels.
    width_fill : bool, default False
        Whether the pick list fills available width.
    menu_height : float, Optional
        Sets the Fixed height of the dropdown menu in logical pixels.
    menu_height_fill : bool, default False
        Whether the dropdown menu fills available height.
    padding : list of float, Optional
        Sets the Padding as [all], [vertical, horizontal], or
        [top, right, bottom, left].
    placeholder : str, Optional
        Sets the placeholder text shown when no option is selected.
    selected : str, Optional
        Sets the currently selected option.
    text_size : float, Optional
        Sets the Font size for the text.
    text_line_height : float, Optional
        Sets the Line height for the text.
    handle_size : float, Optional
        Sets the size of the icon.
    handle_static_icon_id : int, Optional
        Sets the id using the add_icon id.
    handle_dynamic_closed_icon_id : int, Optional
        Sets the id using the add_icon id.
    handle_dynamic_open_icon_id : int, Optional
        Sets the id using the add_icon id.
    style_id : int, Optional
        Sets the ID of a custom style created with ``add_pick_list_style``.
    user_data : Any, Optional
        Sets the Arbitrary data forwarded to callbacks.
    show : bool, default True
        Whether the pick list is visible.

    Returns
    -------
    int
        The numeric widget ID of the newly created pick list.
    """
    ...
def add_progress_bar(
    *,
    parent_id: str | None = None,
    **kwargs: Any) -> int:
    """_summary_

    Keyword Arguments:
        parent_id -- _description_ (default: {None})

    Returns:
        _description_
    """
    ...
def add_radio(
    *,
    parent_id: str | None = None,
    labels: list[str],
    horizontal: bool = False,
    spacing: float | None = None,
    radio_spacing: float | None = None,
    radio_wrap: bool | None = None,
    radio_wrap_spacing: float | None = None,
    radio_wrap_align_start: bool | None = None,
    radio_wrap_align_center: bool | None = None,
    radio_wrap_align_end: bool | None = None,
    padding: list[float] | None = None,
    width: float | None = None,
    width_fill: bool | None = None,
    height: float | None = None,
    height_fill: bool | None = None,
    fill: bool | None = None,
    on_selected: Callable | None = None,
    selected_index: int | None = None,
    size: float | None = None,
    style_id: int | None = None,
    font_id: int | None = None,
    text_size: float | None = None,
    line_height: float | None = None,
    text_wrapping_none: bool | None = None,
    text_wrapping_glyph: bool | None = None,
    text_wrapping_word_glyph: bool | None = None,
    user_data: Callable | None = None,
    show: bool = True,
    gen_id: int | None = None,
    group_id: int | None = None,
) -> int:
    """
    Add a radio button group widget.
    A group of radio buttons where the user can select one option
    from a list of labels.

    Parameters
    ----------
    parent_id : str
        Sets the parent container ID that this radio group belongs to.
    labels : list of str
        Sets the list of labels for each radio button.
    horizontal : bool, default false
        Whether the layout direction is horizontal (default vertical).
    spacing : float, Optional
        Sets the spacing between the radio circle and its label.
    radio_spacing: float, Optional
        Sets the spacing between radio buttons.
    padding : list of float, Optional
        Sets the Padding as [all], [vertical, horizontal], or
        [top, right, bottom, left].
    width : float, Optional
        Sets the Fixed width in logical pixels.
    width_fill : bool, default False
        Whether the radio group fills available width.
    height : float, Optional
        Sets the Fixed height in logical pixels.
    height_fill : bool, default False
        Whether the radio group fills available height.
    on_selected : callable, Optional
        Sets the Callback method to invoke when a radio button is selected.
    selected_index : int, Optional
        Sets the index of the initially selected radio button.
    size : float, Optional
        Sets the size of the radio circle in logical pixels.
    style_id : int, Optional
        Sets the ID of a custom style created with ``add_radio_style``.
    font_id : int, Optional
        Sets the Font ID for the label text.
    text_spacing : float, Optional
        Sets the spacing between the radio circle and text.
    text_size : float, Optional
        Sets the Font size for the label text.
    line_height : float, Optional
        Sets the Line height for the label text.
    wrapping : TextWrapping, Optional
        Sets the Text wrapping strategy for the labels.
    user_data : Any, Optional
        Sets the Arbitrary data forwarded to callbacks.
    show : bool, default True
        Whether the radio group is visible.
    gen_id : int, Optional
        Obtains an ID of a widget that have not been created, used for the gen_id parameter.
    group_id : int, Optional
        Sets the Group ID for radio button grouping. When multiple radios share the same
        group_id, selecting one will automatically unselect all others in that group.

    Returns:
        int: The numeric widget ID of the newly created radio group.
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
    bar_items: list[str],
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
#                 # If only one item added then body
#                 add_text(content="Card") # 1st item header
#                 add_text(content="This is the body of the card.") # 2nd, if added, item body
#                 with Column(width_fill=True, height=30): # 3rd, if added, item footer
#                     add_separator(line_length=300)
#                     add_text(content="Foot content")

#         start_session()
#     """
#     def __init__(
#         self,
#         *,
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

class ColorPicker:
    """Context manager wrapper around add_color_picker.

    A container for selecting colors and holds an activating widget, i.e. button.

    Usage::

        with Window(title="Demo"):
            with Container(fill=True, align_center=True)
                with ColorPicker(
                    on_open=cp_opened, # Callback when button pressed
                    on_submit=color_selected, # Callback with the selected color
                    on_cancel=cp_canceled, # Callback when canceled
                    ):

                    add_button(label="Color Picker")

        start_session
    """
    def __init__(
        self,
        *,
        on_open: bool | None = None,
        on_submit: bool | None = None,
        on_cancel: bool | None = None,
        opened: bool = False,
        color_format_int: bool | None = None,
        color_format_rgba: bool | None = None,
        color_format_hex: bool | None = None,
        color_format_percent: bool | None = None,
        gap: bool | None = None,
        snap_within_viewport: bool | None = None,
        position_bottom: bool | None = None,
        position_left: bool | None = None,
        position_top: bool | None = None,
        position_right: bool | None = None,
        user_data: bool | None = None,
        gen_id: bool | None = None,
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

class Container:
    """Context manager wrapper around add_container.

    A widget that aligns its contents inside
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
        style_std: ContainerStyleStd | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...

class DatePicker:
    """Context manager wrapper around add_date_picker.

    A container for selecting dates and holds an activating widget, i.e. button.

    Usage::

        with Window(title="Demo"):
            with Container(fill=True, align_center=True)
                with DatePicker(
                    on_open=cp_opened, # Callback when button pressed
                    on_submit=date_selected, # Callback with the selected date
                    on_cancel=cp_canceled, # Callback when canceled
                    ):

                    add_button(label="Date Picker")

        start_session
    """
    def __init__(
        self,
        *,
        on_open: bool | None = None,
        on_submit: bool | None = None,
        on_cancel: bool | None = None,
        opened: bool = False,
        size_factor: float | None = None,
        gap: float | None = None,
        snap_within_viewport: bool | None = None,
        position_bottom: bool | None = None,
        position_left: bool | None = None,
        position_top: bool | None = None,
        position_right: bool | None = None,
        user_data: any | None = None,
        gen_id: bool | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...

class Float:
    """Context manager wrapper around add_float.

    A container that can overlayed other widgets.

    Usage::

        with Window(title="Demo"):
            with Container(fill=True, align_center=True)
                with Float(scale=1.0, translate=[0.0, 0.0], clamped_padding=[10]):
                    with Container(
                        width=200,
                        height=100,
                        style_std=ContainerStyleStd.Primary,
                        align_center=True,
                    ):
                        add_text(content="I'm a Float!", size=20.0)

        start_session
    """
    def __init__(
        self,
        *,
        scale: float | None = None,
        scale_clamped: float | None = None,
        clamped_padding: list[float] | list[float, 4] | None= None,
        translate: list[float, 2] | None = None,
        show: bool = True,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
            exc_tb: TracebackType | None) -> bool: ...


class Grid:
    """Context manager wrapper around add_grid.

    A container that contains widgets in a grid format.

    Usage::

        with Window(title="Demo"):
            with Container(fill=True, align_center=True):
                with Grid(width=400, columns_amount=4, spacing=3.0):
                    # typical row/column iteration
                    for row in range(2):
                        for col in range(4):
                            with Container(height=40, align_center=True,
                                        style_std=ContainerStyleStd.BorderedBox):
                                add_text(content=f"Grid {row} {col}")

        start_session
    """
    def __init__(
        self,
        width: float,
        *,
        spacing: float | None = None,
        columns_max_width: float | None = None,
        columns_amount: int | None = None,
        height_aspect_ratio: float | None = None,
        height_evenly_distribute: bool | None = None,
        height_evenly_distribute_fill: bool | None = None,
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
        bar_items,
        *,
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
        is_open,
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
        gen_id: int | None = None,
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
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None,\
            exc_tb: TracebackType | None) -> bool: ...

class Popover:
    """Context manager wrapper around add_container.

    A container that opens when opened by a callback or
    by setting the opened = True.

    Usage::
        def open_popover(_btn_id: int):
            '''Called to open Popover'''
            update_widget(popover_id, PopoverParam.Opened, True)

        with Window(title="Demo"):
            with Container(align_center=True):
                with Popover() as popover_id:
                    add_button(label="Press Me", on_press=open_popover)
                    with Container(style_std=ContainerStyleStd.BorderedBox):
                        add_text(content="I'm a Popover Container")

        start_session()
    """
    def __init__(
        self,
        *,
        opened: bool = False,
        position_bottom: bool | None = None,
        position_center: bool | None = True,
        position_left: bool | None = None,
        position_top: bool | None = None,
        position_right: bool | None = None,
        gap: bool | None = 0.0,
        padding: bool | None = None,
        snap_within_viewport: bool | None = None,
        focus_trap: bool | None = False,
        on_open: Callable | None = None,
        on_close: Callable | None = None,
        on_click_outside: Callable | None = None,
        user_data: Any | None = None,
    ) -> None: ...
    def __enter__(self) -> int: ...
    def __exit__(self, exc_type: type[BaseException] | None, \
        exc_val: BaseException | None, \
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


class Table:
    """Content manager wrapper for add_table.

    Usage::

        # See example - py_table_advanced.py
        with Window(
                title="Table Demo",
                size=(1000, 600),
                center=True):

            # Add the container for centering the table
            with Container(fill=True, align_center=True):

                with Table(
                    row_height=30.0,
                    col_widths=column_widths,
                    sash_size=6,):

                    with TableHeader():
                        for h1 in header:
                            add_text(content=h1, align_center=True, fill=True, size=14)

                        for h2 in header:
                            pl_id = add_pick_list(
                                options=pick_list_options,
                                placeholder="Sort",
                                handle_dynamic_closed_icon_id=icon_closed,
                                handle_dynamic_open_icon_id=icon_open,
                                on_select=sort_books)

                            sort_dict[h2] = pl_id

                    with TableHeader():
                        for index in range(len(header)):
                            if index == len(header)-1:
                                add_pick_list(
                                    options=genres,
                                    placeholder="Filter",
                                    on_select=filter_genre)
                            else:
                                add_space(width=0)


                    with TableBody():
                        for row in book_lines:
                            for cell in row:
                                body_text_ids.append(
                                    add_text(content=cell, align_center=True, fill=True, size=12))

                    with TableFooter():
                        for f in footers:
                            add_text(content=f, align_center=True, fill=True, size=12)

        start_session()
    """
    def __init__(
        self,
        col_widths: list[float],
        row_height: float,
        *,
        header_height: list[float] | None = None,
        footer_height: list[float] | None = None,
        parent_id: list[str] | None = None,
        sash_size: float = 4.0,
        min_size: float = 10.0,
        file_path: list[str] | None = None,
        style_id: int | None = None,
        sash_style_id: int | None = None,
        on_column_resize: callable | None = None,
        on_column_resize_release: callable | None = None,
        user_data: callable | None = None,
        show: bool = True,
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
