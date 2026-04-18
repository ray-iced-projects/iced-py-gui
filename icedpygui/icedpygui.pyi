#!/usr/bin/env python3
"""
IcedPyGui functions
Type stubs for the Rust native module
"""

from typing import Any, Callable, List

# pylint: disable=unused-argument
# pylint: disable=too-many-arguments
# pylint: disable=too-many-locals
# pylint: disable=too-many-positional-arguments
# pylint: disable=too-few-public-methods
# pylint: disable=too-many-lines

# ---------------------------------------------------------------------------
# Standalone functions (migrated from IPG class methods)
# ---------------------------------------------------------------------------

class ButtonStyleStd:
    """
    Standard styles for Button widget
    """
    Background=''
    Danger=''
    Primary=''
    Secondary=''
    Subtle=''
    Success=''
    Warning=''
    Text=''


def add_button_style(
        background_color: Color | None = None,
        background_color_alpha: float | None = None,
        background_rgba: list[float, 4] | None = None,
        background_gradient_color_stop: Color | None = None,
        background_gradient_color_stop_alpha: float | None = None,
        background_gradient_rgba_stop: float | None = None,
        background_gradient_degrees: float | None = None,
        background_gradient_radians: float | None = None,
        border_color: Color | None = None,
        border_color_alpha: float | None = None,
        border_rgba: list[float, 4] | None = None,
        border_radius: list[float | float, 4] | None = None,
        border_width: float | None = None,
        shadow_color: Color | None = None,
        shadow_color_alpha: float | None = None,
        shadow_rgba: list | None = None,
        shadow_offset_xy: list[float, 2] | None = None,
        shadow_blur_radius: float | None = None,
        text_color: Color | None = None,
        text_color_alpha: float | None = None,
        text_rgba: list[float, 4] | None = None
    ) -> int:
    """Adds styling to a button

    Run the doc_helper.py to see the individual parameter definitions.

    Notes
    --------
    Two styles can be defined:
    custom - defined by using an add_style method
    First, define the style, this can be placed anywhere as long as the
    add_button can access the style value, type int.
    Then, add your button(s) as usual and use the style_id=your_style parameter.
    standard - using the style_std parameter and the ButtonStyleStd class
    Just use the style_std=ButtonStyleStd.Primary parameter in the add_button method.

    Examples
    --------
    >>> from icedpygui import Window, Column, Container, ButtonStyleStd, \
    >>>       add_button, add_button_style, Color, start_session
    >>>
    >>> style = add_button_style(
    >>>             background_color=Color.LIGHT_BLUE,
    >>>             border_color=Color.ALICE_BLUE,
    >>>             border_width=2.0,
    >>>             border_radius=[5.0])
    >>>
    >>> with Window(title="Button Styling", center=True):
    >>>     with Container(align_center=True, width_fill=True, height_fill=True):
    >>>         with Column(spacing=20.0):
    >>>             add_button(
    >>>                 label="Border Color and Width",
    >>>                     padding=[5.0],
    >>>                     style_id=style)
    >>>
    >>>             add_button(
    >>>                 label="Style Standard-Danger",
    >>>                     padding=[5.0],
    >>>                     style_std=ButtonStyleStd.Danger)
    >>>
    >>>             add_button(
    >>>                 label="Style Standard-Text",
    >>>                     padding=[5.0],
    >>>                     style_std=ButtonStyleStd.Text)
    >>>
    >>> start_session()
    >>>
    """

def add_card_style(
        background_color: Color | None = None,
        background_color_alpha: float | None = None,
        background_rgba: list[float, 4] | None = None,
        border_radius: float | None = None,
        border_width: float | None = None,
        border_color: Color | None = None,
        border_color_alpha: float | None = None,
        border_rgba: list[float] | list[float, 4] | None = None,
        head_background_color: Color | None = None,
        head_background_color_alpha: float | None = None,
        head_background_rgba: list[float, 4] | None = None,
        body_background_color: Color | None = None,
        body_background_color_alpha: float | None = None,
        body_background_rgba: list[float, 4] | None = None,
        foot_background_color: Color | None = None,
        foot_background_color_alpha: float | None = None,
        foot_background_rgba: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:
    """
    Adds styling to the Card

    Run the doc_helper.py to see the individual parameter definitions.

    Notes
    --------
    Two styles can be defined:

    custom - defined by using an add_style method

    First, define the style, this can be placed anywhere as long as the
    add_card can access the style value, type int.

    Then, add your card(s) as usual and use the style_id=your_style parameter.

    standard - using the style_std parameter and the CardStyleStd class

    Just use the style_std=CardStyleStd.Primary parameter in the add_card method.

    Examples
    --------
    >>> from icedpygui import Window, Column, Container, CardStyleStd, \
    >>>     add_card, add_card_style, Color, start_session
    >>>
    >>> style = add_card_style(
    >>>             background_color=Color.LIGHT_BLUE,
    >>>             border_color=Color.ALICE_BLUE,
    >>>             border_width=2.0,
    >>>             border_radius=[5.0])
    >>>
    >>> with Window(title="Card Styling", center=True):
    >>>     with Container(align_center=True, width_fill=True, height_fill=True):
    >>>         with Column(spacing=20.0):
    >>>             add_card(
    >>>                 label="Border Color and Width",
    >>>                     padding=[5.0],
    >>>                     style_id=style)
    >>>
    >>>             add_card(
    >>>                 label="Style Standard-Danger",
    >>>                     padding=[5.0],
    >>>                     style_std=ButtonStyleStd.Danger)
    >>>
    >>> start_session()
    >>>
    """

def add_checkbox_style(
        background_color: Color | None = None,
        background_color_alpha: float | None = None,
        background_rgba: list[float, 4] | None = None,
        border_color: Color | None = None,
        border_color_alpha: float | None = None,
        border_rgba: list[float, 4] | None = None,
        border_radius: list[float] | list[float, 4] | None = None,
        border_width: float | None = None,
        icon_color: Color | None = None,
        icon_color_alpha: float | None = None,
        icon_rgba: list[float, 4] | None = None,
        text_color: Color | None = None,
        text_color_alpha: float | None = None,
        text_rgba: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:
    """Adds styling to a checkbox

    Run the doc_helper.py to see the individual parameter definitions.

    Notes
    --------
    Two styles can be defined:

    custom - defined by using an add_style method

    First, define the style, this can be placed anywhere as long as the
    add_checkbox can access the style value, type int.

    Then, add your checkbox(es) as usual and use the style_id=your_style parameter.

    standard - using the style_std parameter and the CheckboxStyleStd class

    Just use the style_std=CheckboxStyleStd.Primary parameter in the add_checkbox method.

    Examples
    --------
    >>> from icedpygui import Window, Column, Container, CheckboxStyleStd, \
    >>>     add_checkbox, add_checkbox_style, Color, start_session
    >>>
    >>> style = add_checkbox_style(
    >>>         border_color=Color.RED,
    >>>         border_width=3.0)
    >>>
    >>> with Window(title="Checkbox Styling", center=True):
    >>>     with Container(align_center=True, width_fill=True, height_fill=True):
    >>>         with Column(spacing=20.0):
    >>>             add_checkbox(
    >>>                 label="Border Color and Width",
    >>>                     style_id=style)
    >>>
    >>>             add_checkbox(
    >>>                 label="Secondary",
    >>>                 style_std=CheckboxStyleStd.Secondary)
    >>>
    >>> start_session()
    >>>
    """

def add_color_picker_style(
        background_color: Color | None = None,
        background_rgba: list[float, 4] | None = None,
        background_color_hovered: Color | None = None,
        background_rgba_hovered: list[float, 4] | None = None,
        border_color: Color | None = None,
        border_rgba: list[float, 4] | None = None,
        border_radius: list[float] | list[float, 4] | None = None,
        border_width: float | None = None,
        shadow_color: Color | None = None,
        shadow_rgba: list[float, 4] | None = None,
        shadow_offset_x: float | None = None,
        shadow_offset_y: float | None = None,
        shadow_blur_radius: float | None = None,
        text_color: Color | None = None,
        text_rgba: list[float, 4] | None = None,
    ) -> int:

    """
    Adds styling to color picker button

    """

class ContainerStyleStd:
    """Container standard styles"""
    BorderedBox=''
    Danger=''
    Dark=''
    Primary=''
    RoundedBox=''
    Secondary=''
    Success=''
    Transparent=''
    Warning=''

def add_container_style(
        background_color: Color | None = None,
        background_color_alpha: list[float, 4] | None = None,
        background_rgba: list[float, 4] | None = None,
        background_gradient_color_stop: Color | None = None,
        background_gradient_color_stop_alpha: float | None = None,
        background_gradient_rgba_stop: list[float, 4] | None = None,
        background_gradient_degrees: float | None = None,
        background_gradient_radians: float | None = None,
        background_gradient_alpha: float | None = None,
        border_color: Color | None = None,
        border_color_alpha: float | None = None,
        border_rgba: list[float, 4] | None = None,
        border_radius: list[float] | None = None,
        border_width: float | None = None,
        shadow_color: Color | None = None,
        shadow_color_alpha: float | None = None,
        shadow_rgba: list[float, 4] | None = None,
        shadow_offset_xy: list[float, 2] | None = None,
        shadow_blur_radius: float | None = None,
        text_color: Color | None = None,
        text_color_alpha: float | None = None,
        text_rgba: list[float, 4] | None = None,
        snap: bool | None = None,
        gen_id: int | None = None,
    ) -> int:
    """Adds styling to a container

    Run the doc_helper.py to see the individual parameter definitions.

    Notes
    --------
    Two styles can be defined:

    custom - defined by using an add_style method

    First, define the style, this can be placed anywhere as long as the
    add_container or Container can access the style value, type int.

    Then, add your container(s) as usual and use the style_id=your_style parameter.

    standard - using the style_std parameter and the ContainerStyleStd class

    Just use the style_std=ContainerStyleStd.Primary parameter in the
    add_container or Container method.

    Examples
    --------
    >>> from icedpygui import Window, Column, Container, ContainerStyleStd, \
    >>>     add_container_style, add_text, Color, start_session
    >>>
    >>> style = add_container_style(
    >>>             background_color=Color.AQUA,
    >>>             border_color=Color.BLUE,
    >>>             border_radius=[10.0],
    >>>             border_width=5.0,
    >>>             shadow_color=Color.YELLOW)
    >>>
    >>> with Window(title="Container Styling", center=True):
    >>>     with Column(spacing=20.0, padding=[20.0], align_center=True, width_fill=True):
    >>>
    >>>         with Container(align_center=True, width=400.0, height=200.0, style_id=style):
    >>>             add_text(content="Some Container Custom Styling")
    >>>
    >>>         with Container(align_center=True, width=400.0, height=200.0,
    >>>                        style_std=ContainerStyleStd.BorderedBox):
    >>>             add_text(content="Some Container Standard Styling\n BorderedBox")
    >>>
    >>> start_session()
    >>>
    """

def add_font_style(
    family: FontFamily | None = None,
    family_name: str | None = None,
    weight: FontWeight | None = None,
    stretch: FontStretch | None = None,
    style: FontStyle | None = None,
    gen_id: int | None = None,
    ) -> int:

    """
    Adds font style

    """

def load_font(path: str) -> None:
    """
    Load a font from a .ttf or .otf file path.
    Must be called before start_session().

    """

class FontFamily:
    """Font fakmily parameters"""
    Name=''
    Cursive=''
    Fantasy=''
    Monospace=''
    SansSerif=''
    Serif=''


class FontStretch:
    """Font stretch parameters"""
    Condensed=''
    Expanded=''
    ExtraCondensed=''
    ExtraExpanded=''
    Normal=''
    SemiCondensed=''
    SemiExpanded=''
    UltraCondensed=''
    UltraExpanded=''


class FontStyle:
    """Font styling"""
    Normal=''
    Italic=''
    Oblique=''


class FontWeight:
    """Adds a font weighting

    """
    Black=''
    Bold=''
    ExtraBold=''
    ExtraLight=''
    Light=''
    Medium=''
    Normal=''
    Semibold=''
    Thin=''

def add_menu_bar_style(
        base_color: Color | None = None,
        base_rgba: list[float, 4] | None = None,
        border_color: Color | None = None,
        border_rgba: list[float, 4] | None = None,
        border_radius: list[float] | list[float, 4] | None = None,
        border_width: float | None = None,
        shadow_color: Color | None = None,
        shadow_rgba: list[float, 4] | None = None,
        shadow_offset_xy: list[float, 2] | None = None,
        shadow_blur_radius: float | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Adds style in the menu itself, not the menu items or bar items

    """

def add_menu_style(
        base_color: Color | None=None,
        base_rgba: list[float, 4] | None=None,
        border_color: Color | None=None,
        border_rgba: list[float, 4] | None=None,
        border_radius: list[float] | list[float, 4] | None=None,
        border_width: float | None=None,
        shadow_color: Color | None=None,
        shadow_rgba: list[float, 4] | None=None,
        shadow_offset_xy: list[float, 2] | None=None,
        shadow_blur_radius: float | None=None,
        path_base_color: Color | None=None,
        path_base_rgba: list[float, 4] | None=None,
        path_border_color: Color | None=None,
        path_border_rgba: list[float, 4] | None=None,
        path_border_radius: list[float] | list[float, 4] | None=None,
        path_border_width: float | None=None,
        gen_id: int | None=None,
    ) -> int:

    """
    Adds style in the menu itself, not the menu items or bar items

    """

class MenuSeparatorType:
    """
    The separator types id used.
    """
    Circle=''
    Dot=''
    Label=''
    Line=''


def add_menu_separator_style(
        style_id: str,
        separator_type: MenuSeparatorType,
        *,
        height: float=20.0,
        height_fill: bool=False,
        width: float | None=None,
        width_fill: bool=True,
        quad_ratios: list[float] | None=None,
        separator_color: Color | None=None,
        separator_rgba: list[float] | None=None,
        separator_border_color: Color | None=None,
        separator_border_rgba: list[float] | None=None,
        separator_border_width: float | None=None,
        separator_border_radius: list[float] | None=None,
        separator_shadow_color: Color | None=None,
        separator_shadow_rgba: list[float] | None=None,
        separator_shadow_offset: list[float] | None=None,
        separator_shadow_blur_radius: float | None=None,
        background_color: Color | None=None,
        background_rgba: list[float] | None=None,
        background_border_color: Color | None=None,
        background_border_rgba: list[float] | None=None,
        background_border_width: float | None=None,
        background_border_radius: list[float] | None=None,
        background_shadow_color: Color | None=None,
        background_shadow_rgba: list[float] | None=None,
        background_shadow_offset: list[float] | None=None,
        background_shadow_blur_radius: float | None=None,
        gen_id: int | None=None,
    ) -> int:

    """
    The styling of the separators, if used, in the menu.

    """

def add_pick_list_style(
        background_color: Color | None=None,
        background_rgba: list[float, 4] | None=None,
        text_color: Color | None=None,
        text_rgba: list[float, 4] | None=None,
        handle_color: Color | None=None,
        handle_rgba: list[float, 4] | None=None,
        placeholder_color: Color | None=None,
        placeholder_rgba: list[float, 4] | None=None,
        border_color: Color | None=None,
        border_rgba: list[float, 4] | None=None,
        border_color_hovered: Color | None=None,
        border_rgba_hovered: list[float, 4] | None=None,
        border_radius: list[float] | None=None,
        border_width: float | None=None,
        gen_id: int | None=None,
    ) -> int:

    """
    Add PickList styling.

    """

def add_progress_bar_style(
        background_color: Color | None=None,
        background_rgba: list[float, 4] | None=None,
        bar_color: Color | None=None,
        bar_rgba: list[float, 4] | None=None,
        border_color: Color | None=None,
        border_rgba: list[float, 4] | None=None,
        border_radius: list[float, 4] | None=None,
        border_width: float | None=None,
        gen_id: int | None=None,
    ) -> int:

    """
    Add ProgressBar style.
    """

def add_radio_style(
        background_color: Color | None=None,
        background_rgba: list[float, 4] | None=None,
        background_color_hover: Color | None=None,
        background_color_hovered: list[float, 4] | None=None,
        border_color: Color | None=None,
        border_rgba: list[float, 4] | None=None,
        border_width: float | None=None,
        dot_color: Color | None=None,
        dot_rgba: list[float, 4] | None=None,
        dot_color_hovered: Color | None=None,
        dot_rgba_hovered: list[float, 4] | None=None,
        text_color: Color | None=None,
        text_rgba: list[float, 4] | None=None,
        gen_id: int | None=None,
    ) -> int:

    """
    Add Radio style.

    """

def add_rule_style(
        color: Color | None,
        color_rgba: list[float, 4] | None,
        border_radius: list[float, 4] | None,
        fillmode_percent: float | None,
        fillmode_padded: int | None,
        fillmode_asymmetric_padding: list[int, 2] | None,
        gen_id: int | None,
    ) -> int:

    """
    Add Rule styling.

    """

def add_scrollable_style(
        vertical_rail_style_id: int | None = None,
        horizontal_rail_style_id: int | None = None,
        auto_scroll_style_id: int | None = None,
        gap_color: Color | None = None,
        gap_color_alpha: float | None = None,
        gap_rgba: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:
    """_summary_


    """

def add_separator_style(
        ipg_color: Color | None = None,
        rgba_color: list[float] | None = None,
        border_ipg_color: Color | None = None,
        border_rgba: list[float] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    The styling of the separators.

    """
def add_scroller(
    *,
    width: float | None = None,
    margin: float | None = None,
    scroller_width: float | None = None,
    spacing: float | None = None,
    anchor_start: bool | None = None,
    anchor_end: bool | None = None,
    hidden: bool | None = None,
    gen_id: int | None = None,
) -> int:
    """_summary_

    Args:
        parent_id (str | None, optional): _description_. Defaults to None.

    Returns:
        int: wicontainerdget id
    """
def add_slider_style(
        style_id: str,
        rail_color: Color | None = None,
        rail_rgba: list[float, 4] | None = None,
        rail_color_hovered: Color | None = None,
        rail_rgba_hovered: list[float, 4] | None = None,
        rail_width: float | None = None,
        rail_border_radius: list[float] | None = None,
        handle_circle_radius: float | None = None,
        handle_rectangle_width: int | None = None,
        handle_rectangle_border_radius: list[float] | None = None,
        handle_color: Color | None = None,
        handle_rgba: list[float, 4] | None = None,
        handle_border_width: float | None = None,
        handle_border_color: Color | None = None,
        handle_border_rgba: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Add styling to the Slider.

    """

def add_text_input_style(
        style_id: str,
        background_color: Color | None = None,
        background_rgba: list[float, 4] | None = None,
        border_color: Color | None = None,
        border_rgba: list[float, 4] | None = None,
        border_color_hovered: Color | None = None,
        border_rgba_hovered: list[float, 4] | None = None,
        border_color_focused: Color | None = None,
        border_rgba_focused: list[float, 4] | None = None,
        border_width: float | None = None,
        border_radius: list[float] | None = None,
        placeholder_color: Color | None = None,
        placeholder_rgba: list[float, 4] | None = None,
        value_color: Color | None = None,
        value_rgba: list[float, 4] | None = None,
        selection_color: Color | None = None,
        selection_rgba: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Add textInput styling.

    """

def add_tooltip_style(
        background_color: Color | None = None,
        background_rgba: list[float, 4] | None = None,
        border_color: Color | None = None,
        border_rgba: list[float, 4] | None = None,
        border_radius: list[float] | None = None,
        border_width: float | None = None,
        shadow_color: Color | None = None,
        shadow_rgba: list | None = None,
        shadow_offset_xy: list[float, 2] | None = None,
        shadow_blur_radius: float | None = None,
        text_color: Color | None = None,
        text_rgba: list[float, 4] | None = None,
    ) -> int:
    """
    Adds styling to the tool tip


    """

def add_canvas_timer_style(
        background_color: Color | None = None,
        background_rgba: list[float, 4] | None = None,
        background_color_hovered: Color | None = None,
        background_rgba_hovered: list[float, 4] | None = None,
        border_color: Color | None = None,
        border_rgba: list[float, 4] | None = None,
        border_radius: list[float | float, 4] | None = None,
        border_width: float | None = None,
        shadow_color: Color | None = None,
        shadow_rgba: list | None = None,
        shadow_offset_x: float | None = None,
        shadow_offset_y: float | None = None,
        shadow_blur_radius: float | None = None,
        text_color: Color | None = None,
        text_rgba: list[float, 4] | None = None
    ) -> int:

    """
    Adds styling to canvas timer button

    """

def add_toggler_style(
        background_color: Color | None = None,
        background_rgba: list[float, 4] | None = None,
        background_color_toggled: Color | None = None,
        background_rgba_toggled: list[float, 4] | None = None,
        background_border_color: Color | None = None,
        background_border_rgba: list[float, 4] | None = None,
        background_border_width: float | None = None,
        foreground_color: Color | None = None,
        foreground_rgba: list[float, 4] | None = None,
        foreground_color_toggled: Color | None = None,
        foreground_rgba_toggled: list[float, 4] | None = None,
        foreground_border_color: Color | None = None,
        foreground_border_rgba: list[float, 4] | None = None,
        foreground_border_width: Color | None = None,
    ) -> int:

    """
    Adds a toggler to the gui

    """

    # ***************canvas widgets**********************

def add_arc(
        canvas_id: str,
        center_xy: tuple[float, float],
        radius: float,
        start_angle: float,
        end_angle: float,
        *,
        stroke_width: float=2.0,
        stroke_ipg_color: Color | None=None,
        stroke_rgba_color: list[float, 4] | None=None,
        fill_ipg_color: Color | None=None,
        fill_rgba_color: list[float, 4] | None=None,
        gen_id: int | None=None,
    ) -> int:

    """
    Adds an Arc to the canvas.

    Parameters
    ----------
        canvas_id: str
            The canvas to be used for the widget
        center_xy: tuple[float, float]
            center of the arc
        radius: float
            radius of the arc
        start_angle: float
            start angle in radians
        end_angle: float
            end angle in radians
        stroke_width: float
            line width
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        fill_ipg_color: Color | None
            Whether to use the Color for the fill or None
        fill_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the fill or None
        gen_id: int | None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_bezier(
        canvas_id: str,
        points: list[tuple[float, float], 3],
        *,
        stroke_width: float=2.0,
        stroke_ipg_color: Color | None=None,
        stroke_rgba_color: list[float, 4] | None=None,
        fill_ipg_color: Color | None=None,
        fill_rgba_color: list[float, 4] | None=None,
        degrees: float=0.0,
        gen_id: int | None=None,
    ) -> int:

    """
    Adds an Bezier to the canvas.

    Parameters
    ----------
        canvas_id: str
        points: tuple[tuple[float, float], tuple[float, float], tuple[float, float]]
            3 points are needed to define the bezier
        stroke_width: float
            Line width
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        fill_ipg_color: Color | None
            Whether to use the Color for the fill or None
        fill_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the fill or None
        degrees: float
            Rotation of widget using degrees
        gen_id: int | None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_circle(
        canvas_id: str,
        position_xy: tuple[float, float],
        radius: float,
        *,
        stroke_width: float=2.0,
        stroke_dash_offset: int | None = None,
        stroke_dash_segments: list[float] | None = None,
        stroke_ipg_color: Color | None = None,
        stroke_rgba_color: list[float, 4] | None = None,
        stroke_color_alpha: float | None = None,
        fill_ipg_color: Color | None = None,
        fill_rgba_color: list[float, 4] | None = None,
        fill_color_alpha: float | None = 1.0,
        gen_id: int | None = None,
    ) -> int:

    """
    Add Circle to a canvas

    Parameters
    ----------
        canvas_id: str
            String id of the add_canvas().
        position_xy: tuple[float, float]
            Position of the center point of the circle.
        radius: float
            Radius of the circle
        stroke_width: float
            Width of the stoke.
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        stroke_color_alpha: float
            The alpha of the color
        fill_ipg_color: Color | None
            Whether to use the Color for the fill or None
        fill_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the fill or None
        fill_color_alpha: float
            The alpha of the color
        gen_id: int | None=None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_ellipse(
        canvas_id: str,
        position_xy: tuple[float, float],
        radius_x: float,
        radius_y: float,
        *,
        stroke_width: float=2.0,
        degrees: float=0.0,
        stroke_ipg_color: Color | None = None,
        stroke_rgba_color: list[float, 4] | None = None,
        fill_ipg_color: Color | None = None,
        fill_rgba_color: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Adds an ellipse to the canvas.

    Parameters
    ----------
        canvas_id: str
            String id of the add_canvas().
        cposition_xy: tuple[float, float]
            Position of the center point of the ellipse
        radius_x: float
            The radius in the X direction
        radius_y: float
            The radius in the Y direction
        degrees: float
            The rotation in degrees
        stroke_width: float
            Width of the stoke.
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        fill_ipg_color: Color | None
            Whether to use the Color for the fill or None
        fill_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the fill or None
        gen_id: int | None=None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_line(
        canvas_id: str,
        start: tuple[float, float],
        end: tuple[float, float],
        *,
        degrees: float=0.0,
        stroke_width: float=2.0,
        stroke_ipg_color: Color | None = None,
        stroke_rgba_color: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Add a line to the canvas.

    Parameters
    ----------
        canvas_id: str
            String id of the add_canvas().
        start: tuple[float, float]
            Tuple of starting position
        end: tuple[float, float]
            Tuple of starting position
        degrees: float
            The rotation of the line in degrees
        stroke_width: float
            Width of the stoke.
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        gen_id: int | None=None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_polygon(
        canvas_id: str,
        position_xy: tuple[float, float],
        radius: float,
        sides: int,
        *,
        degrees: float=0.0,
        stroke_width: float=2.0,
        stroke_ipg_color: Color | None = None,
        stroke_rgba_color: list[float, 4] | None = None,
        fill_ipg_color: Color | None = None,
        fill_rgba_color: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Add a polygon to the canvas.

    Parameters
    ----------
        canvas_id: str
            String id of the add_canvas().
        position_xy: tuple[float, float]
            Position of the center point of the polygon.
        radius: float
            Defines the radius
        sides: int
            The number of side for the polygon
        degrees: float
            The rotation of the polygon in degrees
        stroke_width: float
            Width of the stoke.
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        fill_ipg_color: Color | None
            Whether to use the Color for the fill or None
        fill_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the fill or None
        gen_id: int | None=None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_poly_line(
        canvas_id: str,
        points: list[(float, float)],
        *,
        stroke_width: float=2.0,
        stroke_ipg_color: Color | None = None,
        stroke_rgba_color: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Add a polygon to the canvas.

    Parameters
    ----------
        canvas_id: str
            String id of the add_canvas().
        points: list[tuple[float, float]]
            defines each point in the continous line
        stroke_width: float
            Width of the stoke.
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        gen_id: int | None=None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_rectangle(
        canvas_id: str,
        top_left_xy: tuple[float, float],
        width: float,
        height: float,
        *,
        degrees: float=0.0,
        stroke_width: float=2.0,
        stroke_ipg_color: Color | None = None,
        stroke_rgba_color: list[float, 4] | None = None,
        fill_ipg_color: Color | None = None,
        fill_rgba_color: list[float, 4] | None = None,
        gen_id: int | None = None,
    ) -> int:

    """
    Add a rectangle to the canvas.

    Parameters
    ----------
        canvas_id: str
            String id of the add_canvas().
        top_left_xy: tuple[float, float]
            Position of the top left point of the rectangle.
        widths: float
            Defines the width
        height: float
            Defines the height
        degrees: float
            The rotation of the rectangle in degrees
        stroke_width: float
            Width of the stroke.
        stroke_ipg_color: Color | None
            Whether to use the Color for the stroke
        stroke_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the stroke
        fill_ipg_color: Color | None
            Whether to use the Color for the fill or None
        fill_rgba_color: list[float, 4] | None
            Whether to use the rgba color for the fill or None
        gen_id: int | None=None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """

def add_canvas_image(
        canvas_id: str,
        image_path: str,
        width: float,
        height: float,
        position_xy: tuple[float, float],
        *,
        align_center: bool=True,
        align_top_left_xy: tuple[float, float] | None=None,
        gen_id: int | None=None,
    ) -> int:

    """
    Add an image to the canvas.

    Parameters
    ----------
        canvas_id: str
            String id of the add_canvas().
        image_path: str,
            Path to the image file
        top_left_xy: tuple[float, float]
            Location of the top left corner
        width: float
            Defines the width
        height: float
            Defines the height
        gen_id: int | None=None
            The only allowable entry for this id is that generated by ipg.generate_id().

    Returns
    -------
        id: int
        The id of the event which can be used to modify the event through update_item.
    """


    # *********************events***********************
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

def add_event_timer (
    enabled: bool=False,
    duration_ms: int | None = None,
    on_start: Callable | None = None,
    on_tick: Callable | None = None,
    on_stop: Callable | None = None,
    user_data: Any | None = None,
    gen_id: int | None = None,
    ) -> int:
    """
    Add a timer event.

    A timer event which can be controlled via update_timer.

    Parameters
    ----------
    enabled : bool
        Whether the timer starts immediately.
    duration_ms : int, Optional
        The time between ticks in milliseconds (default 10).
    on_start : Callable, Optional
        Callback fired on the first tick after enabling.
        Receives (timer_id, tick_count, elapsed_ms).
    on_tick : Callable, Optional
        Callback fired on each tick.
        Receives (timer_id, tick_count, elapsed_ms).
    on_stop : Callable, Optional
        Callback fired when the timer is disabled.
        Receives (timer_id, tick_count, elapsed_ms).
    user_data : Any, Optional
        Any user data passed to the callback functions.
    gen_id : int, Optional
        Obtains an ID of a widget that has not been created.

    Returns
    -------
    int
        The numeric ID of the newly created timer.
    """

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

def add_chart(
        self,
        window_id: str,
        chart_id: str,
        series: list[tuple[str, float]],
        x_axis_labels: list[str],
        width: float,
        height: float,
        *,
        position_xy: list[float, 2] | None = None,
        theme: ChartTheme | None = None,
        margin: list[float, 4] | None = None,
        font_family: str = "",
        background_ipgcolor: Color | None = None,
        background_rgba: list[float, 4] | None = None,
        is_light: bool = False,
        grid_stroke_ipgcolor: Color | None = None,
        grid_stroke_rgba: list[float, 4] | None = None,
        grid_stroke_width: float = 0,
        radius: float | None = None,
        parent_id: str | None = None,
        gen_id: int | None = None,
    ) -> int:

    """_summary_

    Args:
        window_id (str): _description_
        chart_id (str): _description_
        series (list[tuple[str, float]]): _description_
        x_axis_labels (list[str]): _description_
        width (float): _description_
        height (float): _description_
        position_xy (list[float, 2] | None): _description_
        theme (ChartTheme | None): _description_
        margin (list[float, 4] | None): _description_
        font_family (str): _description_
        background_ipgcolor (Color | None): _description_
        background_rgba (list[float, 4] | None): _description_
        is_light (bool): _description_
        grid_stroke_ipgcolor (Color | None): _description_
        grid_stroke_rgba (list[float, 4] | None): _description_
        grid_stroke_width (float): _description_
        radius (float | None): _description_
        parent_id (str | None): _description_
        gen_id (int | None): _description_

    Returns:
        int: _description_
    """

def construct_chart(
        self,
        chart_ids: list[str]
    ):

    """_summary_

    Args:
        chart_ids (list[str]):
            List of string chart ids.
    """

def add_chart_title(
        chart_id: str,
        title_text: str | None,
        title_font_size: float,
        *,
        title_font_ipgcolor: Color | None = None,
        title_font_rgba: list[float, 4] | None = None,
        title_font_weight: str | None = None,
        title_margin: list[float, 4] | None = None,
        title_align: bool | None = None,
        title_height: float,
        sub_title_text: str | None = None,
        sub_title_font_size: float | None = None,
        sub_title_font_ipgcolor: Color | None = None,
        sub_title_font_rgba: list[float, 4] | None = None,
        sub_title_font_weight: str | None = None,
        sub_title_margin: list[float, 4] | None = None,
        sub_title_align: bool | None = None,
        sub_title_height: float,
        gen_id: int | None = None,
    ) -> int:

    """_summary_

    Args:
        chart_id (str): _description_
        title_text (str | None): _description_
        title_font_size (float): _description_
        title_font_ipgcolor (Color | None): _description_
        title_font_rgba (list[float, 4] | None): _description_
        title_font_weight (str | None): _description_
        title_margin (list[float, 4] | None): _description_
        title_align (HorizontalAlignment): _description_
        title_height (float): _description_
        sub_title_text (str | None): _description_
        sub_title_font_size (float | None): _description_
        sub_title_font_ipgcolor (Color | None): _description_
        sub_title_font_rgba (list[float, 4] | None): _description_
        sub_title_font_weight (str | None): _description_
        sub_title_margin (list[float, 4] | None): _description_
        sub_title_align (HorizontalAlignment): _description_
        sub_title_height (float): _description_
        gen_id (int | None): _description_

    Returns:
        The id of the event which can be used to modify the event through update_item.
    """


class ChartLegendCategory:
    """Chart Legend Type"""
    Normal=''
    RoundRect=''
    Circle=''
    Rect=''


def add_chart_legend(
        chart_id: str,
        legend_font_size: float,
        legend_font_ipgcolor: Color | None = None,
        legend_font_rgba: list[float, 4] | None = None,
        legend_font_weight: str | None = None,
        legend_align: bool | None = None,
        legend_margin: list[float, 4] | None = None,
        legend_category: ChartLegendCategory | None = None,
        legend_show: bool = True,
        gen_id: int | None = None,
    ) -> int:

    """_summary_

    Args:
        chart_id (str): _description_
        legend_font_size (float): _description_
        legend_font_ipgcolor (Color | None): _description_
        legend_font_rgba (list[float, 4] | None): _description_
        legend_font_weight (str | None): _description_
        legend_align (HorizontalAlignment): _description_
        legend_margin (list[float, 4] | None): _description_
        legend_category (LegendCategory): _description_
        legend_show (bool): _description_
        gen_id (int | None): _description_

    Returns:
        int: _description_
    """

def add_chart_x_axis(
        chart_id: str,
        x_axis_data: list[str],
        x_axis_height: float,
        x_axis_stroke_ipgcolor: Color | None = None,
        x_axis_stroke_rgba: list[float, 4] | None = None,
        x_axis_font_size: float | None = None,
        x_axis_font_ipgcolor: Color | None = None,
        x_axis_font_rgba: list[float, 4] | None = None,
        x_axis_font_weight: str | None = None,
        x_axis_name_gap: float | None = None,
        x_axis_name_rotate: float | None = None,
        x_axis_margin: list[float, 4] | None = None,
        x_axis_hidden: bool | None = None,
        x_boundary_gap: bool | None = None,
        gen_id: int | None = None,
    ) -> int:

    """_summary_

    Args:
        chart_id (str):
            The unique identifier for the chart.
        x_axis_data (list[str]):
            The data points or labels to be displayed along the x-axis.
        x_axis_height (float):
            The height of the x-axis.
        x_axis_stroke_ipgcolor (Color | None):
            The color of the x-axis stroke using the Color class.
        x_axis_stroke_rgba (list[float, 4] | None):
            The color of the x-axis stroke in RGBA format.
        x_axis_font_size (float): The font size of the x-axis labels.
        x_axis_font_ipgcolor (Color | None):
            The color of the x-axis font using the Color class.
        x_axis_font_rgba (list[float, 4] | None):
            The color of the x-axis font in RGBA format.
        x_axis_font_weight (str | None):
            The weight of the x-axis font (e.g., "bold", "normal").
        x_axis_name_gap (float):
            The gap between the x-axis name and the axis line.
        x_axis_name_rotate (float):
            The rotation angle of the x-axis name in degrees.
        x_axis_margin (list[float, 4] | None):
            The margin around the x-axis in the format [top, right, bottom, left].
        x_axis_hidden (bool):
            Whether the x-axis is hidden or visible.
        x_boundary_gap (bool | None):
            Whether to leave a gap between the x-axis boundary and the data points.
        gen_id (int | None):
            The unique identifier generated by `ipg.generate_id()` for the widget.

    Returns:
        int: _description_
    """

def add_chart_y_axis(
        chart_id: str,
        y_axis_hidden: bool,
        y_axis_font_size: float,
        y_axis_font_ipgcolor: Color | None = None,
        y_axis_font_rgba: list[float, 4] | None = None,
        y_axis_font_weight: str | None = None,
        y_axis_stroke_ipgcolor: Color | None = None,
        y_axis_stroke_rgba: list[float, 4] | None = None,
        y_axis_width: float | None = None,
        y_axis_split_number: int = 0,
        y_axis_name_gap: float = 0,
        y_axis_name_align: bool | None = None,
        y_axis_margin: list[float, 4] | None = None,
        y_axis_formatter: str | None = None,
        y_axis_min: float | None = None,
        y_axis_max: float | None = None,
        gen_id: int | None = None,
    ) -> int:

    """_summary_

    Args:
        chart_id (str):
            The unique identifier for the chart.
        y_axis_hidden (bool):
            Whether the y-axis is hidden or visible.
        y_axis_font_size (float):
            The font size of the y-axis labels.
        y_axis_font_ipgcolor (Color | None):
            The color of the y-axis font using the Color class.
        y_axis_font_rgba (list[float, 4] | None):
            The color of the y-axis font in RGBA format.
        y_axis_font_weight (str | None):
            The weight of the y-axis font (e.g., "bold", "normal").
        y_axis_stroke_ipgcolor (Color | None):
            The color of the y-axis stroke using the Color class.
        y_axis_stroke_rgba (list[float, 4] | None):
            The color of the y-axis stroke in RGBA format.
        y_axis_width (float | None):
            The width of the y-axis stroke.
        y_axis_split_number (int):
            The number of splits or intervals on the y-axis.
        y_axis_name_gap (float):
            The gap between the y-axis name and the axis line.
        y_axis_name_align (HorizontalAlignment | None):
            The alignment of the y-axis name (e.g., "Left", "Center", "Right").
        y_axis_margin (list[float, 4] | None):
            The margin around the y-axis in the format [top, right, bottom, left].
            y_axis_formatter (str | None): A formatter string for customizing the y-axis labels.
        y_axis_min (float | None):
            The minimum value of the y-axis.
        y_axis_max (float | None):
            The maximum value of the y-axis.
        gen_id (int | None):
            The unique identifier generated by `ipg.generate_id()` for the widget.

    Returns:
        int: _description_
    """

def add_chart_series(
        chart_id: str,
        series_stroke_width: float,
        series_label_font_ipgcolor: Color | None = None,
        series_label_font_rgba: list[float, 4] | None = None,
        series_label_font_size: float = 16.0,
        series_label_font_weight: str | None = None,
        series_label_formatter: str | None = None,
        series_ipgcolors: list[Color] | None = None,
        series_rgbas: list[float, 4] | None = None,
        series_symbol: bool = False,
        symbol_ipgcolor: Color | None = None,
        symbol_rgba: list[float, 4] | None = None,
        symbol_radius: float | None = None,
        series_smooth: bool | None = None,
        series_fill: bool | None = None,
        gen_id: int | None = None,
    ) -> int:
    """_summary_

    Args:
        chart_id (str): The unique identifier for the chart.
        series_stroke_width (float):
            The width of the stroke for the series lines.
        series_label_font_ipgcolor (Color | None):
            The color of the series label font using the Color class.
        series_label_font_rgba (list[float, 4] | None):
            The color of the series label font in RGBA format.
        series_label_font_size (float):
            The size of the series label font.
        series_label_font_weight (str | None):
            The weight of the series label font (e.g., "bold", "normal").
        series_label_formatter (str | None):
            A formatter string for customizing the series label.
        series_ipgcolors (list[Color]):
            A list of colors for the series using the Color class.
        series_rgbas (list[float, 4]):
            A list of colors for the series in RGBA format.
        series_symbol (bool):
            Whether to display symbols for the series points.
        symbol_ipgcolor (Color | None):
            The color of the series symbols using the Color class.
        symbol_rgba (list[float, 4] | None):
            The color of the series symbols in RGBA format.
        symbol_radius (float | None):
            The radius of the series symbols.
        series_smooth (bool):
            Whether to smooth the series lines.
        series_fill (bool):
        Whether to fill the area under the series lines.
        gen_id (int | None):
            The only allowable entry for this id is the value generated by ipg.generate_id().

    Returns:
        id: (int):
            Internal id of widget and can be used by user if equated.
    """

def get_color_palette(
        base_color: Color | None = None,
        base_rgba: list[float, 4] | None = None,
    ) -> list[list[float], list[float, 4], list[float, 4]]:

    """
    Gets the colors strong, weak, and text in rgba format

    Parameters:
    -----------
        base_color: Color | None,
            The color with class Color
        base_rgba: list[float, 4] | None,
            The color in rgba format

    Returns:
    -------
        list of 3 lists of rgba colors (strong, weak, text)
    """

# **************all item ops***********
def start_session(self) -> None:
    """
    Starts the gui session.  Must be the last called.

    Returns
    -------
    None
    """

def generate_id(self) -> int:
    """
    Generates an id for some future widget

    Returns
    -------
    id: int
        Pre-generated id to use for a widget with parameter gen_id.
    """

def delete_item(window_id: str, wid: int):
    """
    Deletes an item using the widgets id.
    Example: btn_id = add_button("Button")
                delete_item(btn_id)

    Parameters
    ----------
    window_id: str
        Window id in string form.
    wid: int
        The widget id of the widget to be updated.

    Returns
    -------
    None
    """

def show_items(
        window_id: str,
        ids: List[tuple[int, bool]]
    ):

    """
    Shows or hides items

    Args:
        window_id (str):
            The window id that the widget is in.
        ids (List[tuple[int, bool]]):
            A list of the ids and they bool value to indicate either sgoe(True) or Hide(False).
    """

def update_widget(
        wid: int,
        param: str,
        value: any,
    ):

    """
    Update a widget by supplying the widget id, wid, the parameter to update,
    a class property value, and a value based on the type of value used by the widget.

    Parameters
    ----------
    wid: int
        The widget id of the widget to be updated.
    param: class property
        Example: a button has a parameter ButtonParams.Width and value=float
    value: any
        Any value which matches that used by the widget.  For example, to set a checkbox to true,
        param=CheckboxParams.IsChecked, value=True

    Returns
    -------
    None
    """

def update_widget_params(
        wid: int,
        updates: dict,
    ):

    """
    Update a widget by supplying the widget id, wid, the parameter to update,
    a class property value, and a value based on the type of value used by the widget.

    Parameters
    ----------
    wid: int
        The widget id of the widget to be updated.
    updates: dict, {parameter: value}
        Example: update_widget_params(checkbox_id: {isChecked: True, label: "Checked"})
    param: class property
        Example: a button has a parameter ButtonParams.Width and value=float
    value: any
        Any value which matches that used by the widget.  For example, to set a checkbox to true,
        param=CheckboxParams.IsChecked, value=True

    Returns
    -------
    None
    """

def update_canvas_item(
        wid: int,
        param: str,
        value: any,
    ):

    """
    Update a canvas widget by supplying the widget id, wid, the parameter to update,
    a class property value, and a value based on the type of value used by the widget.

    Parameters
    ----------
    wid: int
        The widget id of the widget to be updated.
    param: class property
        Example: a canvas has a class CanvasParams with properties of Position
        Therefore, a circle's position could be changed by using the circle's is
        and the CanvasParam.Position where the value would be a new point.
    value: any
        Any value which matches that used by the widget.  For example, to set a circle position,
        param=CanvasParams.Position, value=[100.0, 100.0]

    Returns
    -------
    None
    """

def move_widget(
        wid: int,
        move_after: int | None = None,
        move_before: int | None = None,
        target_parent_id: int | None = None,
    ) -> None:

    """
    Moves a widget to another container.

    The target container is derived from the sibling when move_after or
    move_before is given.  Use target_parent_id only when appending to
    the end of a container with no sibling reference.

    Parameters
    ----------
        wid: int
            Widget id to move.
        move_after: int | None
            Insert after this widget id.
        move_before: int | None
            Insert before this widget id.
        target_parent_id: int | None
            Parent container widget id (only needed when neither
            move_after nor move_before is supplied).

    Returns
    -------
    None
    """

class Arrow:
    """
    The various arrows for the widgets
    """
    ArrowBarLeft=''
    ArrowBarRight=''
    ArrowBarUp=''
    ArrowClockwise=''
    ArrowCounterclockwise=''
    ArrowDown=''
    ArrowDownCircle=''
    ArrowDownCircleFill=''
    ArrowDownLeft=''
    ArrowDownLeftCircle=''
    ArrowDownLeftCircleFill=''
    ArrowDownLeftSquare=''
    ArrowDownLeftSquareFill=''
    ArrowDownRight=''
    ArrowDownRightCircle=''
    ArrowDownRightCircleFill=''
    ArrowDownRightSquare=''
    ArrowDownRightSquareFill=''
    ArrowDownShort=''
    ArrowDownSquare=''
    ArrowDownSquareFill=''
    ArrowDownUp=''
    ArrowLeft=''
    ArrowLeftCircle=''
    ArrowLeftCircleFill=''
    ArrowLeftRight=''
    ArrowLeftShort=''
    ArrowLeftSquare=''
    ArrowLeftSquareFill=''
    ArrowNinezerodegDown=''
    ArrowNinezerodegLeft=''
    ArrowNinezerodegRight=''
    ArrowNinezerodegUp=''
    ArrowRepeat=''
    ArrowReturnLeft=''
    ArrowReturnRight=''
    ArrowRight=''
    ArrowRightCircle=''
    ArrowRightCircleFill=''
    ArrowRightShort=''
    ArrowRightSquare=''
    ArrowRightSquareFill=''
    ArrowThroughHeart=''
    ArrowThroughHeartFill=''
    ArrowUp=''
    ArrowUpCircle=''
    ArrowUpCircleFill=''
    ArrowUpLeft=''
    ArrowUpLeftCircle=''
    ArrowUpLeftCircleFill=''
    ArrowUpLeftSquare=''
    ArrowUpLeftSquareFill=''
    ArrowUpRight=''
    ArrowUpRightCircle=''
    ArrowUpRightCircleFill=''
    ArrowUpRightSquare=''
    ArrowUpRightSquareFill=''
    ArrowUpShort=''
    ArrowUpSquare=''
    ArrowUpSquareFill=''
    Arrows=''
    ArrowsAngleContract=''
    ArrowsAngleExpand=''
    ArrowsCollapse=''
    ArrowsCollapseVertical=''
    ArrowsExpand=''
    ArrowsExpandVertical=''
    ArrowsFullscreen=''
    ArrowsMove=''
    ArrowsVertical=''


class ButtonParam:
    """
    Button parameters

    Parameters
    ----------
    Clip: bool
        Whether the label is clipped or not
    Height: float
        The height of the button.
    HeightFill: bool
        Whether the button height fills the available space of a container.
    IfMenuBtn: bool
        Whether the button is used in a mnau widget, effects alignment
    Label: str
        The label of the button.
    Padding: list[float], one or four items
        The padding around the button, [all sides], [top, right, bottom, left].
    Show: bool
        Whether to show the button
    StyleArrow: ButtonArrow
        A button becomes a type of arrow.
    StyleId: int
        The id of the add_button_style() function
    StyleStd: ButtonStyleStd
        The standard styles for a button
    TextAlignBottomCenter: bool
        Whether to align label botton center.
    TextAlignBottomLeft: bool
        Whether to align the label bottom left.
    TextAlignBottomRight: bool
        Whether to align the label bottom right.
    TextAlignCenter: bool
        Whether to align the label centered (default).
    TextAlignCenterLeft: bool
        Whether to align the label center left.
    TextAlignCenterRight: bool
        Whether to align the label center right
    TextAlignTopCenter: bool
        Whether to align the label top center.
    TextAlignTopLeft: bool
        Whether to align the label top left.
    TextAlignTopRight: bool
        Whether to align the label top right.
    TextSize: float
        The size of the text.
    Width: float
        The width of the button
    WidthFill: bool
         Whether the button width fills the available space of a container.
    """
    Clip: bool
    Height: float
    HeightFill: bool
    IfMenuBtn: bool
    Label: str
    Padding: list[float]
    Show: bool
    StyleArrow: Arrow
    StyleId: int
    StyleStd: ButtonStyleStd
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
    Width: float
    WidthFill: bool


class ButtonStyleParam:
    """
    Button style parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbga :list[float, 4]
        Background color in rgba format
    BorderColor: Color
        Border color in Color format
    BorderRgba: list[float, 4]
        Border color in rgba format
    BorderRadius: list
        The border radius [float]=all, [float, 4]=each individual one
    BorderWidth: float
        The width of the border line
    ShadowColor: Color
        Shadow color in Color format
    ShadowRgba: list[float, 4]
        Shadow color in rgba format
    ShadowOffsetXY: [float, 2]
        The offfset of the show towards the x and y directions
    ShadowBlurRadius: float
        How much to blur the shadow radius
    TextColor: Color
        The label text color in Color format
    TextRgba: list[float, 4]
        The label text color in rgba format
    """
    BackgroundColor:Color
    BackgroundRbga:list[float, 4]
    BorderColor:Color
    BorderRgba:list[float, 4]
    BorderRadius:list
    BorderWidth:float
    ShadowColor:Color
    ShadowRgba:list[float, 4]
    ShadowOffsetXY:list[float, 2]
    ShadowBlurRadius:float
    TextColor:Color
    TextRgba:list[float, 4]


class DrawMode:
    """Canvas draw modes"""
    Display: str
    New: str
    Edit: str
    Rotate: str

class CanvasWidget:
    """List of canvas widget available"""
    Arc=''
    Bezier=''
    Circle=''
    Ellipse=''
    Image=''
    Line=''
    Polygon=''
    PolyLine=''
    RightTriangle=''
    FreeHand=''
    Text=''


class CanvasParam:
    """
    Canvas Parameters

    Parameters
    ----------
    Clear bool
        Whether to clear the drawing cache.
    DrawColor list
        The list of rgba values.
    FillColor list
        The list of rgba values.
    CanvasColor list
        The list of rgba values.
    FilePath str
        Path to file.
    Mode str
        The CanvasDrawMode selected.
    PolyPoints int
        The number of points to use for polygon and polyline.
    Widget CanvasWidget
        One of the CanvasWidgets to use.

    """
    Clear: bool
    DrawColor: list
    FillColor: list
    CanvasColor: list
    FilePath: str
    Mode: str
    PolyPoints: int
    Widget: CanvasWidget
    Load: None
    Save: None


class CanvasImageParam:
    """Canvas image parameters"""
    Position: tuple[float, float]
    Rotation: float


class CardStyleStd:
    """
    The standard styles for the card widget
    """
    Danger=''
    Dark=''
    Info=''
    Light=''
    Primary=''
    Secondary=''
    Success=''
    Warning=''
    White=''


class ChartTheme:
    """Cahrt themes"""
    DarkTheme=''
    AntTheme=''
    VintageTheme=''
    ShineTheme=''
    WaldenTheme=''
    WesterosTheme=''
    ChalkTheme=''
    GrafanaTheme=''
    ShadcnTheme=''


class CardParam:
    """
    The card parameters

    Parameters
    ----------
    Body: str
        The body string
    Foot: str
        The footer string
    Head: str
        The header string
    IsOpen: bool
        Whether the card is open or minimized
    Style: CardStyle
        The string id of the add_card_style()
    """
    Body: str
    Foot: str
    Head: str
    IsOpen: bool
    Style: str


class CheckboxParam:
    """
    The Checkbox parameters

    Parameters
    ----------
    IconSize: float
        Size of the icon.
    IconX: bool
        Whether to use and x or the default check.
    IsChecked: bool
        Whether the checkbox is checked or not.
    Label: str
        THe label of the checkbox.
    Show: bool
        Whether to show the checkbox.
    Size: float
        The size of the square.
    Spacing: float
        The spacing between the square and label.
    StyleId: int
        The id of the add_checkbox_style() function.
    StyleStandard: StyleStandard
        One of the standard styles.
    TextLineHeight: float
        The height of the text box holding the label.
    TextSize: float
        The size of the label text.
    Width: float
        The width of the entire checkbox.
    WidthFill: bool
        Whether the checkbox width fills the available space of the container.
    """
    IconSize: float
    IconX: bool
    IsChecked: bool
    Label: str
    Show: bool
    Size: float
    Spacing: float
    StyleId: int
    StyleStandard: ButtonStyleStd
    TextLineHeight: float
    TextSize: float
    Width: float
    WidthFill: bool


class CheckboxStyleParam:
    """
    Checkbox style parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbgaColor :list[float, 4]
        Background color in rgba format
    BackgroundColorHovered: Color
        When hovered, the Background color in Color format
    BackgroundRgbaHovered: list[float, 4]
        When hovered, the background color in rgba format
    AccentColor: Color
    AccentRgbaColor: list[float, 4]
    AccentColorHovered: Color
    AccentRgbaColorHovered: list[float, 4]
    BorderColor: Color
        Border color in Color format
    BorderRgbaColor: list[float, 4]
        Border color in rgba format
    BorderRadius: list
        The border radius [float]=all, [float, 4]=each individual one
    BorderWidth: float
        The width of the border line
    TextColor: Color
        The label text color in Color format
    TextRgbaColor: list[float, 4]
        The label text color in rgba format
    """
    BackgroundColor:Color
    BackgroundRgbaColor:list[float, 4]
    BackgroundColorHovered:Color
    BackgroundRgbaColorHovered:list[float, 4]
    AccentColor:Color
    AccentRgbaColor:list[float, 4]
    AccentColorHovered:Color
    AccentRgbaColorHovered:list[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderRadius:list[float]
    BorderWidth:float
    IconColor:Color
    IconRgbaColor:list[float, 4]
    TextColor:Color
    TextRgbaColor:list[float, 4]


class ColorPickerParam:
    """
    Button parameters

    Parameters
    ----------
    ArrowStyle: ButtonArrow
        A button becomes a type of arrow.
    Height: float
        The height of the button.
    HeightFill: bool
        Whether the button height fills the available space of a container.
    Label: str
        The label of the button.
    Padding: list
        The padding around the button.
    Clip: bool
        Whether the label is clipped or not
    Show: bool
        Whether to show the button
    StyleId: int
        The id of the add_button_style()
    StyleStandard: StyleStandard
        One of the standard styles
    Width: float
        The width of the button
    WidthFill: bool
         Whether the button width fills the available space of a container.
    """
    ArrowStyle: Arrow
    CanvasColor: list
    DrawColor: list
    Height: float
    HeightFill: bool
    Label: str
    Padding: list
    Clip: bool
    Show: bool
    StyleId: int
    StyleStandard: ButtonStyleStd
    Width: float
    WidthFill: bool


class ColorPickerStyleParam:
    """
    Color Picker Style parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbgaColor :list[float, 4]
        Background color in rgba format
    BackgroundColorHovered: Color
        When hovered, the Background color in Color format
    BackgroundRgbaHovered: list[float, 4]
        When hovered, the background color in rgba format
    BorderColor: Color
        Border color in Color format
    BorderRgbaColor: list[float, 4]
        Border color in rgba format
    BorderRadius: list
        The border radius [float]=all, [float, 4]=each individual one
    BorderWidth: float
        The width of the border line
    ShadowColor: Color
        Shadow color in Color format
    ShadowRgbaColor: list[float, 4]
        Shadow color in rgba format
    ShadowOffsetX: float
        The offfset of the show towards the x direction
    ShadowOffsetY: float
        The offfset of the show towards the y direction
    ShadowBlurRadius: float
        How much to blur the shadow radius
    TextColor: Color
        The label text color in Color format
    TextRgbaColor: list[float, 4]
        The label text color in rgba format
    """
    Background:Color
    BackgroundRbga:list[float, 4]
    BackgroundColorHovered:Color
    BackgroundRgbaHovered:list[float, 4]
    BorderColor:Color
    BorderRgba:list[float, 4]
    BorderRadius:list
    BorderWidth:float
    ShadowColor:Color
    ShadowRgba:list[float, 4]
    ShadowOffsetX:float
    ShadowOffsetY:float
    ShadowBlurRadius:float
    TextColor:Color
    TextRgbaColor:list[float, 4]


class ColumnParam:
    """Column Parameters"""
    AlignLeft: bool
    AlignCenter: bool
    AlignRight: bool
    Clip: bool
    Fill: bool
    Height: float
    HeightFill: bool
    Padding: list[float]
    Spacing: float
    Width: float
    WidthFill: bool


class ContainerParam:
    """Container parameters"""
    AlignBottomCenter: bool
    AlignBottomLeft: bool
    AlignBottomRight: bool
    AlignCenter: bool
    AlignCenterLeft: bool
    AlignCenterRight: bool
    AlignTopCenter: bool
    AlignTopLeft: bool
    AlignTopRight: bool
    Clip: bool
    Fill: bool
    Height: float
    HeightFill: bool
    MaxHeight: float
    MaxWidth: float
    Padding: list[float]
    Width: float
    WidthFill: bool
    Show: bool
    StyleId: int
    StyleStd: ContainerStyleStd


class ContainerStyleParam:
    """Container style parameters"""
    BackgroundColor:Color
    BackgroundRgbaColor:list[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderRadius:float
    BorderWidth:float
    ShadowColor:Color
    ShadowRgbaColor:list[float, 4]
    ShadowOffsetXY:list[float, 4]
    ShadowBlurRadius:float
    TextColor:Color
    TextRgbaColor:list[float, 4]


class DatePickerParam:
    """
    Date Picker parameters

    Parameters
    ----------
    Label: str
        Label for the initial button
    Padding: list[float]
        Padding around the button
    SizeFactor: float
        Size of the calendar
    Show: bool
        Whether to show or not.
    """
    Label: str
    Padding: list[float]
    SizeFactor: float
    Show: bool


class DividerDirection:
    """
    Direction of the divider.

    Parameters
    ----------
    Horizontal:
        Horizontal resizing.
    Vertical:
        Vertical resizing.
    """
    Horizontal: ...
    Vertical: ...


class DividerParam:
    """
    Divider parameters

    Parameters
    ----------
    HandleWidth: float
        Width of handle
    HandleHeight: float
        Height of the handle
    Sizes: list[float],
        The sizes for the divider (widths for horizontal, heights for vertical)
    StyleId: int,
        The id of the add_divider_style()
    Show: bool,
        Whether to show the widget or not.
    """
    HandleWidth:float
    HandleHeight:float
    Sizes:list[float]
    Style:int
    Show:bool


class DividerStyleParam:
    """_
    Divider style parameters

    Parameters
    ----------
    BackgroundColor: Color,
        Background color
    BackgroundRgbaColor: list[float, 4],
        Background color in rgba format
    BorderColor: Color,
        Border color
    BorderRgbaColor: list[float, 4],
        Border color in rgba format
    BorderWidth: float,
        Border width
    BorderRadius: float,
        Radius of the 4 border corners
    Transparent: bool,
        Whether to use the transparent style
    """
    BackgroundColor: Color
    BackgroundRgbaColor: list[float, 4]
    BorderColor: Color
    BorderRgbaColor: list[float, 4]
    BorderWidth: float
    BorderRadius: float
    Transparent: bool

class FloatParam:
    """Float parameters"""
    Scale: float
    Translate: list[float]
    ScaleClamped: bool
    """
    Parameters for the Float container

    Parameters
    ---------
    Scale: FloatParam.Scale
        Sets the scale of the float content
    Translate: FloatParam.Translate
        Sets the translate offset for the float content [x, y]
    ScaleClamped: FloatParam.ScaleClamped
        Whether the scaled content exceeds the container size
    """

class ContentFit:
    """
    Content fit for a image or svg widget

    Parameters
    ---------
    Contain: ImageContentFit.Contain
        The image will be scaled (preserving aspect ratio) so that it just fits within the window.
    Cover: ImageContentFit.Cover
        Scale the image to cover all of the bounding box, cropping if needed.
    Fill: ImageContentFit.Fill
        Distort the image so the widget is 100% covered without cropping.
    ScaleDown: ImageContentFit.ScaleDown
        Scale the image down if it's too big for the space, but never scale it up.
    """
    Contain: str
    Cover: str
    Fill: str
    ScaleDown: str


class FilterMethod:
    '''
    How the image is filtered

    Parameters
    ----------
    Linear: ImageMethodFilter.Linear
        Bi-linear interpolation image filtering strategy.
    Nearest: ImageMethodFilter.Nearest
        Nearest neighbor image filtering strategy.
    '''
    Linear: str
    Nearest: str


class Rotation:
    """
    What happens to the container when image is rotated.

    Parameters
    ----------
    Floating: ImageRotation.Floating
        When image is rotated, it floats above the container, not distorting it.
    Solid: ImageRotation.Solid
        When the image is rotated, the container resizes to fit.
    """
    Floating: str
    Solid: str


class ImageParam:
    """
    Image parameters

    Parameters
    ----------
    BorderRadius: list[float, 4] | list[float]
        Sets the border radius of the image either all corner
        same value [float] or independent [float,4].
    ContentFit: ContentFit
        Set how the image contents fits see ContentFit class.
    CropHeight: float
        sets the height of the crop rectangle.
    CropWidth: float
        Sets the crop width of the crop rectangle.
    CropX: float
        Sets the origin x of the crop rectangle.
    CropY: float
        Sets the origin y of the crop rectangle.
    Expand: float
        Sets whether the image should try to fill as much space
        available as possible while keeping aspect ratio and without
        allocating extra space in any axis.
    FilterMethod: FilterMethod
        Sets the filter method, see FilterMethod
    Fill: bool
        Sets both width_fill and length_fill.
    Height: float
        Sets the height of the widget.
    HeightFill: bool
        Sets the height to fill the available space, overrides height.
    ImagePath: str
        Sets the path to where the image is located.
    Opacity: float
        How much opacity, 1=opaque, 0=transparent
    Opacity: float
        Sets the opacity of the image.
    Padding: list[float]
        The padding around the image.
    RotationDegrees: float
        Sets the roation of the image in degrees format.
    RotationRadians: float
        How much to rotate the image in radians.
    RotationMethod: Rotation
        Set the roation method, see Rotation.
    Scale: float
        Sets the scale factor of the Image.
        The region of the Image drawn will be scaled from the center by the given scale factor.
    Show: bool
        Whether to show or hide the image.
    Width: float
        Width of the image.
    WidthFill: bool
        Whether to fill the width to the available container size.
    """
    BorderRadius: list[float, 4] | list[float]
    ContentFit: ContentFit
    CropHeight: float
    CropWidth: float
    CropX: float
    CropY: float
    Expand: float
    Fill: bool
    FilterMethod: FilterMethod
    HeightFill: bool
    Height: float
    ImagePath: str
    Opacity: float
    RotationDegrees: float
    RotationMethod: Rotation
    RotationRadians: float
    Scale: float
    Show: bool
    WidthFill: bool
    Width: float


class MenuParam:
    """
    Menu Bar parameters

    Parameters
    ----------
    CursorBoundsMargin: float
        Close menu items if cursor exceed the items bounds
    Height: float
        The height od the bar
    Height: bool | None
        The height of the bar
    Padding: list[float]
        The padding around the bar.
    Spacing: float
        The space between the bar and the menu items.
    Widths: list[float]
        The width of each menu item in the bar.

    Show: bool
        Whether to show or hide the widget
    """
    CursorBoundsMargin: float | None
    Height: float | None
    Padding: list[float] | list[float, 4] | None
    ScrollSpeedLine: float | None
    ScrollSpeedPixel: float | None
    Show: bool | None
    Spacing: float | None
    Width: float | None
    WidthFill: bool | None


class MenuBarItemParam:
    """
    Menu Bar Item Parameters

    Parameters
    ----------
    CloseOnBackgroundClick: bool
        Whether to close items on background click
    CloseOnItemClick: bool
        Whether to close on items click
    Offset: float
        Set the offset of the items column
    Padding: list[float, 4] | list[float]
        Set the padding of the items,
        [float] = all sides, [float, 4]=[top, right, bottom, left]
    Show: bool
        Whether to show the items
    Spacing: float
        Sets the spacing between the items
    Width: float
        Sets the width of the items container
    """
    CloseOnBackgroundClick: bool | None
    CloseOnItemClick: bool | None
    Offset: float | None
    Padding: list[float, 4] | list[float] | None
    Show: bool | None
    Spacing: float | None
    Width: float | None


class MenuStyleParam:
    """Menu style parameters"""
    BaseColor:Color
    BaseRgbaColor:list[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderRadius:list[float]
    BorderWidth:float
    ShadowColor:Color
    ShadowRgbaColor:list[float, 4]
    ShadowOffsetXY:list[float, 2]
    ShadowBlurRadius:float
    PathBaseColor:Color
    PathBaseRgbaColor:list[float, 4]
    PathBorderColor:Color
    PathBorderRgbaColor:list[float, 4]
    PathBorderRadius:list[float]
    PathBorderWidth:float

class MenuBarStyleParam:
    """Menu bar style parameters"""
    BaseColor:Color
    BaseRgbaColor:list[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderRadius:list[float]
    BorderWidth:float
    ShadowColor:Color
    ShadowRgbaColor:list[float, 4]
    ShadowOffsetXY:list[float, 2]
    ShadowBlurRadius:float


class MenuSeparatorStyleParam:
    """Menu Separator style parameters"""
    SeparatorType:MenuSeparatorType
    Width:float
    WidthFill:bool
    Height:float
    HeightFill:bool
    QuadRatios:list[float, 2]
    SeparatorColor:Color
    SeparatorRgbaColor:list[float, 4]
    SeparatorBorderColor:Color
    SeparatorBorderRgbaColor:list[float, 4]
    SeparatorBorderRadius:list[float]
    SeparatorBorderWidth:float
    SeparatorShadowColor:Color
    SeparatorShadowRgbaColor:list[float, 4]
    SeparatorShadowOffset:float
    SeparatorShadowBlurRadius:float
    BackgroundColor:Color
    BackgroundRgbaColor:list[float, 4]
    BackgroundBorderColor:Color
    BackgroundBorderRbgaColor:list[float, 4]
    BackgroundBorderWidth:float
    BackgroundBorderRadius:list[float]
    BackgroundShadowColor:Color
    BackgroundShadowRbgaColor:list[float, 4]
    BackgroundShadowOffset:float
    BackgroundShadowBlurRadius:float


class MenuType:
    """
    The type of widgets a menu item can be.  Used in item_styles parameter.
    """
    Button=''
    Checkbox=''
    Dot=''
    Label=''
    Line=''
    Text=''
    Toggler=''

class MousePointer:
    """
    Pointer types for the mouse interactions
    """
    Idle=''
    Pointer=''
    Grab=''
    Text=''
    Crosshair=''
    Working=''
    Grabbing=''
    ResizingHorizontally=''
    ResizingVertically=''
    NotAllowed=''
    ZoomIn=''

class OpaqueParam:
    """
    Opaque update parameters.
    """
    Show: bool


class PickListParam:
    """
    PickList update parameters

    Parameters
    ----------
    Options: list[str]
        Items in the picklist.
    Selected: str
        Item selected.
    Placeholder: str
        A placeholder in the picklist box.
    Padding: list[float]
        Padding around the picklist.
    Show: bool
        Whether to show or hide the widget.
    StyleId: int
        The id of the add_picklist_style()
    TextSize: float
        Size of the text.
    TextLineHeight: float
        Size od the text box.
    Width: float
        Width of the picklist.
    WidthFill: bool
        Whether the picklist expands the available width of the container.
    """
    Options: list[str]
    Selected: str
    Placeholder: str
    Padding: list[float]
    Show: bool
    StyleId: int
    TextSize: float
    TextLineHeight: float
    Width: float
    WidthFill: bool


class PickListHandle:
    """
    The type of handle for the picklist.
    """
    Arrow=''
    Dynamic=''
    HandleNone=''
    Static=''


class PickListStyleParam:
    """
    PickList style parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbgaColor :list[float, 4]
        Background color in rgba format
    BorderColor: Color
        Border color in Color format
    BorderRgbaColor: list[float, 4]
        Border color in rgba format
    BorderRadius: list
        The border radius [float]=all, [float, 4]=each individual one
    BorderWidth: float
        The width of the border line
    HandleColor: Color
        Handle color in Color format
    HandleRgbaColor: list[float, 4]
        Handle color in rgba format
    PlaceholderColor: Color
        Placeholder color in Color format
    PlaceholderRgbaColor: list[float, 4]
        Placeholder color in rgba format
    TextColor: Color
        The label text color in Color format
    TextRgbaColor: list[float, 4]
        The label text color in rgba format
    """
    BackgroundColor:Color
    BackgroundRbgaColor:List[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderRadius:list
    BorderWidth:float
    HandleColor:Color
    HandleRgbaColor:list[float, 4]
    PlaceholderColor:Color
    PlaceholderRgbaColor:list[float, 4]
    TextColor:Color
    TextRgbaColor:list[float, 4]

class ProgressBarParam:
    """
    The ProgressBar update parameters.

    Parameters
    ----------
    Height: float
        Height of the bar.
    HeightFill: bool
        Whether the bar fills the height of the container.
    Min: float
        Minimum value of the bar.
    Max: float
        Maximum value of the bar.
    Show: bool
        Whether to show or hide the bar.
    StyleId: int
        The id of the add_progress_bar_style()
    StyleStandard: str
        The standard style of the StandardStyle class
    Value: float
        The value of the bar.
    Width: float
        The width of the bar.
    WidthFill: bool
        Whether the bar fills the width of the container.
    """
    Height: float
    HeightFill: bool
    Min: float
    Max: float
    Show: bool
    StyleId: int
    StyleStandard: str
    Value: float
    Width: float
    WidthFill: bool


class ProgressBarStyleParam:
    """
    Progress Bar style parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbgaColor :list[float, 4]
        Background color in rgba format
    BarColor: Color
        Bar color in Color format
    BarRgbaColor: list[float, 4]
        Bar color in rgba format
    BorderColor: Color
        Border color in Color format
    BorderRgbaColor: list[float, 4]
        Border color in rgba format
    BorderRadius: list
        The border radius [float]=all, [float, 4]=each individual one
    BorderWidth: float
        The width of the border line
    """
    BackgroundColor:Color
    BackgroundRbgaColor:List[float, 4]
    BarColor:Color
    BarRgbaColor:list[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderRadius:list
    BorderWidth:float


class ProgressBarStyleStd:
    """Standard styles for the progress bar"""
    Danger=''
    Primary=''
    Secondary=''
    Success=''
    Warning=''


class RadioDirection:
    """
    Direction the radio button are aligned.
    """
    Horizontal=''
    Vertical=''


class RadioParam:
    """
    Radio button parameters

    Parameters
    ----------
    Direction: str
        The RadioDirection class
    Labels: list[str]
        The labels for each radio button
    Padding: list[float]
        The padding around the radio
    SelectedIndex: int
        The index of the radio selected
    Show: bool
        Whether to show or hide the radio group
    Size: float
        The size of the radio circle.
    Spacing: float
        The spacing between the radio buttons in the group.
    StyleId: int
        The id of the add_radio_style().
    TextSpacing: float
        The spacing between the radio  and the label.
    TextSize=''
        The size of the label.
    LineHeightPixels: int
        The label box height in pixels.
    LineHeightRelative: float
        The label box height relative to the default.
    UserData: any
        Any user data need of any format.
    Width: float
        The width of the widget group.
    WidthFill: bool
        Whether the widget group fill the container width.
    Height: float
        The height of the widget group.
    HeightFill: bool
        Whether the widget group fill the container height.
    """
    Direction: str
    Labels: list[str]
    Padding: list[float]
    SelectedIndex: int
    Show: bool
    Size: float
    Spacing: float
    StyleId: int
    TextSpacing: float
    TextSize:float
    LineHeightPixels: int
    LineHeightRelative: float
    UserData: any
    Width: float
    WidthFill: bool
    Height: float
    HeightFill: bool


class RadioStyleParam:
    """
    Radio style parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbgaColor :list[float, 4]
        Background color in rgba format
    DotColor: Color
        Dot color in Color format
    DotRgbaColor: list[float, 4]
        Dot color in rgba format
    DotColorHovered: Color
        Dot hover color in Color format
    DotRgbaColorHovered: list[float, 4]
        Dot hover color in rgba format
    BorderColor: Color
        Border color in Color format
    BorderRgbaColor: list[float, 4]
        Border color in rgba format
    BorderWidth: float
        The width of the border line
    TextColor: Color
        The label text color in Color format
    TextRgbaColor: list[float, 4]
        The label text color in rgba format
    """
    BackgroundColor:Color
    BackgroundRbgaColor:List[float, 4]
    DotColor:Color
    DotRgbaColor:list[float, 4]
    DotColorHovered:Color
    DotRgbaColorHovered:list[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderWidth:float
    TextColor:Color
    TextRgbaColor:list[float, 4]


class RowParam:
    """Row parameters"""
    AlignBottom: bool
    AlignCenter: bool
    AlignTop: bool
    Clip: bool
    Fill: bool
    Height: float
    HeightFill: bool
    Padding: list[float]
    Spacing: float
    Width: float
    WidthFill: bool


class RuleStyleParam:
    """Rule style parameters"""
    Color:Color
    RbgaColor:List[float, 4]
    BorderRadius:float
    FillModePercent:float
    FillModePadded:int
    FillModeAsymmetricPadding:list[int]


class ScrollableDirection:
    """
    The scroll direction of the Scrollable
    """
    Vertical=''
    Horizontal=''
    Both=''


class ScrollableParam:
    """
    The Scrollable parameters

    Parameters
    ----------
    Fill: bool
        Whether the scrollable fills both width and height of a holding container
    Width: float
        The width of the scrollable.
    WidthFill: bool
        Whether the scrollable fills the width container.
    Height: float
        The height of the scrollable.
    HeightFill: bool
        Whether the scrollable fills the height container.

    Examples
    --------
    >>> ipg.update_item(wid=0, param=ScrollableParam.Width , 300.0)
    >>> sipg.update_item(wid=0, param=ScrollableParam.HeightFill, True)
    """
    Fill: bool
    Height: float
    ScrollerXId: int
    ScrollerYId: int
    StyleId: int
    Width: float


class ScrollableStyleParam:
    """
    The Scrollable style parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbgaColor: list[float, 4]
        Background color in rgba format
    BorderColor: Color
        Border color in Color format
    BorderRgbaColor: list[float, 4]
        Border color in rgba format
    BorderRadius: list[float]
        Border radius as a list of 1 or 4
    BorderWidth: float
        Border widt as a float
    ShadowColor: Color
        Shadow color in Color format
    ShadowRgbaColor: list[float, 4]
        Shadow color in rgba format
    ShadowOffsetX: float
        Amount of offset shadow in x irection
    ShadowOffsetY: float
        Amount of offset shadow in y direction
    ShadowBlurRadius: list[float]
        How much to blur the radius
    TextColor: Color
        Text color in Color format
    TextRgbaColor: list[float, 4]
        Text color in rgba format
    ScrollbarColor: Color
        Scrollbar color in Color format
    ScrollbarRgbaColor: list[float, 4]
        Scrollbar color in rgba format
    ScrollbarBorderRadius: list[float]
        Border radius as a list of 1 or 4
    ScrollbarBorderWidth: float
        Width of the border
    ScrollbarBorderColor: Color
        Scrollbar border color in Color format
    ScrollbarBorderRgbaColor: list[float, 4]
        Scrollbar border color in rgba format
    ScrollerColor: Color
        Scroller color in Color format
    ScrollerRgbaColor: list[float, 4]
        Scroller color in rgba format
    ScrollerColorHovered: Color
        Scroller hover color in Color format
    ScrollerRgbaColorHovered: list[float, 4]
        Scroller hover color in rgba format
    ScrollerColorDragged: Color
        Scroller drag color in Color format
    ScrollerRgbaColorDragged: list[float, 4]
        Scroller drag color in rgba format
    """
    BackgroundColor:Color
    BackgroundRbgaColor:list[float, 4]
    BorderColor:Color
    BorderRgbaColor:list[float, 4]
    BorderRadius:list[float]
    BorderWidth:float
    ShadowColor:Color
    ShadowRgbaColor:list[float, 4]
    ShadowOffsetX:float
    ShadowOffsetY:float
    ShadowBlurRadius:list[float]
    TextColor:Color
    TextRgbaColor:list[float, 4]
    ScrollbarColor:Color
    ScrollbarRgbaColor:list[float, 4]
    ScrollbarBorderRadius:list[float]
    ScrollbarBorderWidth:float
    ScrollbarBorderColor:Color
    ScrollbarBorderRgbaColor:list[float, 4]
    ScrollerColor:Color
    ScrollerRgbaColor:list[float, 4]
    ScrollerColorHovered:Color
    ScrollerRgbaColorHovered:list[float, 4]
    ScrollerColorDragged:Color
    ScrollerRgbaColorDragged:list[float, 4]


class SeparatorParam:
    """Separator parameters"""
    DotCount: int
    DotFill: bool
    DotBorderWidth: float
    DotRadius: float
    Height: float
    HeightFill: bool
    Label: str
    Spacing: float
    Show: bool
    StyleId: int
    Width: float
    WidthFill: bool

class SliderParam:
    """
    Slider update parameters

    Parameters
    ----------
    Min: float
        The minimum range value.
    Max: float
        The maximum range value.
    Step: float
        The step of the range value.
    Value: float
        The current value.
    Width: float
        The width of the widget.
    WidthFill: bool
        Whether the width of the widget fills the container.
    Height: float
        The height of the widget.
    StyleId: int
        The id of the add_slider_style() function.
    Show: bool
        Whether to show or hide the widget.
    """
    Min: float
    Max: float
    Step: float
    Value: float
    Width: float
    WidthFill: bool
    Height: float
    StyleId: int
    Show: bool


class SliderStyleParam:
    """Slider style parameters"""
    RailColor:Color
    RailRbgaColor:list[float, 4]
    RailColorHovered:Color
    RailRgbaHovered:list[float, 4]
    RailBorderRadius:list[float]
    RailWidth:float

    HandleColor:Color
    HandleRgbaColor:list[float, 4]
    HandleBorderColor:Color
    HandleBorderRgbaColor:list[float, 4]
    HandleBorderWidth:float
    HandleCircleRadius:float
    HandleRectangleWidth:int
    HandleRectangleBorderRadius:list[float]


class StackParam:
    """
    Stack update parameters

    Parameters
    ----------
    ShowStack: bool
        Show the top widget of the stack.
    """
    ShowStack: bool


class SvgParam:
    """
    SVG image parameters

    Parameters
    ----------
    Height: float
        The height of the widget.
    HeightFill: bool
        Whether the height of the widget fills the container.
    ImagePath: str
        The path to the image.
    Show: bool
        Whether to show or hide the widget.
    Width: float
        The width of the widget.
    WidthFill: bool
        Whether the width of the widget fills the container.
    RotationRadians: float
        How much to rotate the svg image in radians.
    Opacity: float
        The opacity of the widget.
    """
    Height: float
    HeightFill: bool
    ImagePath: str
    Show: bool
    Width: float
    WidthFill: bool
    RotationRadians: float
    Opacity: float



class TableParam:
    """
    Table parameters

    Parameters
    ----------
    Headers: list[str]
        Column header strings.
    Body: list[list[float]]
        Table data as a list of rows, each row a list of floats.
    Footers: list[str]
        Column footer strings.
    Width: float
        Width of the table.
    Height: float
        Height of the table.
    ColumnWidths: list[float]
        List of column widths.
    ColumnSpacing: float
        Spacing between columns
    RowSpacing: float
        Spacing between rows
    RowMaxHeight: float
        Max height of the rows
    RowHighlight: TableRowHighLight
        Sets the color type for highlighting alternate rows.
    HighlightAmount: float
        The highlighting amount.
    ResizeColumnsEnabled: bool
        Enables the ability to resize the columns
    MinColumnWidth: float
        If resizing, the min column width
    CellPadding: list[float]
        Padding around the cells
    TableWidthFixed: bool
        If the table width is fixed, doesn't change with resizing
    TableWidth: float
        Changes the table width which initially is the sum of column widths
    ScrollerWidth: float
        Scroller width
    ScrollerBarWidth: float
        Scroller bar width
    ScrollerMargin: float
        Scroller margin
    Show: bool
        Whether to show or hide the widget.
    """
    Headers:list[str]
    Body:list[list[float]]
    Footers:list[str]
    Width:float
    Height:float
    HeaderEnabled:bool
    HeaderCustomEnabled:bool
    Footer:bool
    ColumnWidths:list[float]
    ColumnSpacing:float
    RowSpacing:float
    RowMaxHeight:float
    DividerWidth:float
    ResizeColumnsEnabled:bool
    ColumnPorportionalResize:bool
    MinColumnWidth:float
    CellPadding:list[float]
    TableWidthFixed:bool
    ScrollerWidth:float
    ScrollerBarWidth:float
    ScrollerMargin:float
    Show:bool


class TableStyleParam:
    """Table style parameters"""
    HeaderBackgroundColor:Color
    HeaderBackgroundRgbaColor:list[float, 4]
    HeaderBorderColor:Color
    HeaderBorderRgbaColor:list[float, 4]
    HeaderBorderRadius:float
    HeaderBorderWidth:float
    HeaderTextColor:Color
    HeaderTextRgbaColor:list[float, 4]

    BodyBackgroundColor:Color
    BodyBackgroundRgbaColor:list[float, 4]
    BodyBorderColor:Color
    BodyBorderRgbaColor:list[float, 4]
    BodyBorderRadius:float
    BodyBorderWidth:float
    BodyTextColor:Color
    BodyTextRgbaColor:list[float, 4]
    BodyRowHighlighColor:Color
    BodyRowHighlightRgba:list[float, 4]

    FooterBackgroundColor:Color
    FooterBackgroundRgbaColor:list[float, 4]
    FooterBorderColor:Color
    FooterBorderRgbaColor:list[float, 4]
    FooterBorderRadius:float
    FooterBorderWidth:float
    FooterTextColor:Color
    FooterTextRgbaColor:list[float, 4]

    DividerBackgroundColor:Color
    DividerBackgroundRgbaColor:list[float, 4]
    DividerHoverColor:Color
    DividerHoverRgbaColor:list[float, 4]

    ScrollerBackgroundColor:Color
    ScrollerBackgroundRgbaColor:list[float, 4]
    ScrollerHoverColor:Color
    ScrollerHoverRgbaColor:list[float, 4]
    ScrollerRailColor:Color
    ScrollerRailRgbaColor:list[float, 4]


class TextInputParam:
    """
    TextInput parameters

    Parameters
    ----------
    Placeholder: str
        The text in the box for instructions.
    Value: str
        The value typed in or pasted in the box
    IsSecure: bool
        Whether to display the text in a readable format or not.
    Width: float
        The width of the widget.
    Padding: list[float]
        The padding around the widget.
    Size: float
        The text ize.
    LineHeightPixels: float
        The height of the text box in pixels.
    LineHeightRelative: float
        The height of the text box relative to the default size.
    StyleId: int
        The id of add_textinput_style() function.
    """
    Placeholder:str
    Value:str
    IsSecure:bool
    Width:float
    Padding:list[float]
    Size:float
    LineHeight:float
    StyleId:int


class TextColorStd:
    """
    The standard colors for the text
    """
    Base=''
    Danger=''
    Primary=''
    Secondary=''
    Success=''
    Warning=''


class TextParam:
    """
    Text parameters

    Parameters
    ----------
    Content: str
        The text.
    AlignBottomCenter: bool | None
        Whether to align the text to bottom center.
    AlignBottomLeft: bool | None
        Whether to align the text bottom left,
    AlignBottomRight: bool | None
        Whether to align the text bottom right.
    AlignCenter: bool | None
        Whether to align the text centered.
    AlignCenterLeft: bool | None
        Whether to align the text center left.
    AlignCenterRight: bool | None
        Whether to align the text center right.
    AlignTopCenter: bool | None
        Whether to align the text top center.
    AlignTopLeft: bool | None
        Whether to align the text top left.
    AlignTopRight: bool | None
        Whether to align the text top right.
    Height: float | None
        The height of the widget, default height shrinks to text height.
    HeightFill: bool | None
        Whether to fill the height of the container with the widget.
    LineHeight: float | None
        The height of the text box.
    Size: float | None
        The size of the text.
    Color: Color | None
        The color of the text, default depends on theme.
    ColorAlpha: float | None
        The alpha value of the Color.
    ColorRgba: list[float, 4] | None
        The color of the text in rgba format.
    ColorStd: TextColorStd | None
        Some standard colors for the text.
    Width: float | None
        The width of the widget, default width shrinks to text width.
    WidthFill: bool
        Whether to fill the width of the container with the widget.
    WrappingGlyph: bool | None
        Whether to wrap at a glyph.
    WrappingNone: bool | None
        Whether to have no wrapping, default is word.
    WrappingWordGlyph: bool | None
        Whether to wrap on a glyph or word.
    Show: bool=True
        Whether to show or hide the widget.
    """
    AlignBottomCenter: bool
    AlignBottomLeft: bool
    AlignBottomRight: bool
    AlignCenter: bool
    AlignCenterLeft: bool
    AlignCenterRight: bool
    AlignTopCenter: bool
    AlignTopLeft: bool
    AlignTopRight: bool
    Content: str
    Height: float
    HeightFill: bool
    LineHeight: float
    Show: bool
    Size: float
    Color: Color
    ColorAlpha: bool
    ColorRgba: list[float, 4]
    ColorStd: TextColorStd
    WrappingGlyph: bool
    WrappingNone: bool
    WrappingWordGlyph: bool
    Width: float
    WidthFill: bool


class RichTextParam:
    """
    Rich text parameters for updating a rich text widget.

    Parameters
    ----------
    LineHeight : float | None
        The line height.
    Show : bool
        Whether to show or hide the widget.
    Size : float | None
        The text size.
    TextColor : Color | None
        The text color.
    TextRgba : list[float, 4] | None
        The text color in rgba format.
    """
    LineHeight: float
    Show: bool
    Size: float
    TextColor: Color
    TextRgba: list[float]


class SpanParam:
    """
    Span parameters for updating a span within a rich text widget.

    Parameters
    ----------
    Text : str
        The text content.
    Bold : bool
        Whether the text is bold.
    Italic : bool
        Whether the text is italic.
    Color : Color | None
        The text color.
    ColorRgba : list[float, 4] | None
        The text color in rgba format.
    LineHeight : float | None
        The line height.
    Padding : list | None
        The padding around the span.
    Size : float | None
        The text size.
    Strikethrough : bool
        Whether to apply strikethrough.
    Underline : bool
        Whether to underline the text.
    """
    Text: str
    Bold: bool
    Italic: bool
    Color: Color
    ColorRgba: list[float]
    LineHeight: float
    Padding: list
    Size: float
    Strikethrough: bool
    Underline: bool


class TextInputStyleParam:
    """Text input style parameters"""
    BackgroundColor: Color
    BackgroundRgbaColor: list[float, 4]
    BorderColor: Color
    BorderRadius: list[float]
    BorderRgbaColor: list[float, 4]
    BorderWidth: float
    PlaceholderColor: Color
    PlaceholderRgbaColor: list[float, 4]
    SelectionColor: Color
    SelectionRgbaColor: list[float, 4]
    ValueColor: Color
    ValueRgbaColor: list[float, 4]


class TimerParam:
    """Parameter for updating the timer"""
    DurationMs : int
    ArrowStyle : str
    Counter : int
    Height : float
    HeightFill : bool
    Label : str
    Padding : list
    Clip : bool
    Show : bool
    StyleId : int
    StyleStandard : str
    Width : float
    WidthFill : bool


class TimerStyleParam:
    """
    Timer button parameters

    Parameters
    ----------
    BackgroundColor: Color
        Background color in Color format
    BackgroundRbgaColor :list[float, 4]
        Background color in rgba format
    BackgroundColorHovered: Color
        When hovered, the Background color in Color format
    BackgroundRgbaHovered: list[float, 4]
        When hovered, the background color in rgba format
    BorderColor: Color
        Border color in Color format
    BorderRgbaColor: list[float, 4]
        Border color in rgba format
    BorderRadius: list
        The border radius [float]=all, [float, 4]=each individual one
    BorderWidth: float
        The width of the border line
    ShadowColor: Color
        Shadow color in Color format
    ShadowRgbaColor: list[float, 4]
        Shadow color in rgba format
    ShadowOffsetX: float
        The offfset of the show towards the x direction
    ShadowOffsetY: float
        The offfset of the show towards the y direction
    ShadowBlurRadius: float
        How much to blur the shadow radius
    TextColor: Color
        The label text color in Color format
    TextRgbaColor: list[float, 4]
        The label text color in rgba format
    """
    BackgroundColor:Color
    BackgroundRbgaColor:list[float]
    BackgroundColorHovered:Color
    BackgroundRgbaHovered:list[float]
    BorderColor:Color
    BorderRgbaColor:list[float]
    BorderRadius:list
    BorderWidth:float
    ShadowColor:Color
    ShadowRgbaColor:list[float]
    ShadowOffsetX:float
    ShadowOffsetY:float
    ShadowBlurRadius:float
    TextColor:Color
    TextRgbaColor:list[float]


class CanvasTimerParam:
    """Parametes for updating the canvas timer"""
    DurationMs : int
    ArrowStyle : str
    Counter : int
    Height : float
    HeightFill : bool
    Label : str
    Padding : list
    Clip : bool
    Show : bool
    StyleId : int
    StyleStandard : str
    Width : float
    WidthFill : bool


class TogglerParam:
    """Parameters for updating the Toggler"""
    FontId: int
    Label: str
    Show: bool
    Size: float
    Spacing: float
    StyleId: int
    TextCenter: bool
    TextLeft: bool
    TextRight: bool
    TextLineHeight: float
    TextSize: float
    Width: float
    WidthFill: bool


class ToolTipPosition:
    """Parameters for tooltip positioning"""
    Bottom=""
    FollowCursor=""
    Left=""
    Right=""
    Top=""


class ToolTipParam:
    """Params for updaing the tooltip"""
    ContainerStyleId=''
    ContentId=''
    DelaySec=''
    Gap=''
    Padding=''
    Position=''
    SnapWithinViewport=''
    Text=''

class WindowLevel:
    """The stacking order of the windows"""
    Normal=''
    AlwaysOnBottom=''
    AlwaysOnTop=''


class WindowTheme:
    """Window themes"""
    Dark=''
    Light=''
    CatppuccinLatte=''
    CatppuccinFrappe=''
    CatppuccinMacchiato=''
    CatppuccinMocha=''
    Dracula=''
    Ferra=''
    GruvboxLight=''
    GruvboxDark=''
    KanagawaWave=''
    KanagawaDragon=''
    KanagawaLotus=''
    Moonfly=''
    Nightfly=''
    Nord=''
    Oxocarbon=''
    SolarizedLight=''
    SolarizedDark=''
    TokyoNight=''
    TokyoNightStorm=''
    TokyoNightLight=''


class WindowParam:
    '''
    Parameters
    ----------
    Center: bool
        Whether to center the window on the screen.
    Closeable: bool
        Whether to allow window to be closed.
    Debug: bool
        Whether to turn on the debug mode which outlines the widgets.
    Decorations: bool
        Whether the window should have a border, a title bar, etc.
    ExitOnCloseRequest: bool
        Whether the window will close on request.
    Fullscreen: bool
        Whether to display fullscreen.
    Hidden: bool
        Whether to hide window.
    IconRgba: list[float]
        Setting for the icon color in rgba format.
    IconWidthHeight: list[float]
        Setting for the icon wide and height
    Level: WindowLevel
        Setting for how the window is stacked (see WindowLevel)
    MaxSize: float
        Setting for maximum size.
    Maximized: float
        Whetehr the window is maximized.
    MinSize: float
        Setting for the minimum window size.
    Minimizable: bool
        Whether the window can be minimized.
    Position: list[float]
        Setting for window position.
    Resizable: bool
        Whether the window can be resized.
    ScaleFactor: float
        Setting for the window scale factor.
    Size: list[float]
        Setting for the window size.
    Theme: WindowTheme
        Setting for the window theme(background color) using Color
    Title: str
        Setting for the window title.
    Transparent: bool
        Whetehr the window background is transparent.
    '''
    Center : bool
    Closeable : bool
    Debug : bool
    Decorations : bool
    ExitOnCloseRequest : bool
    Fullscreen : bool
    Hidden : bool
    IconRgba : list[float]
    IconWidthHeight : list[float]
    Level : WindowLevel
    MaxSize : float
    Maximized : float
    MinSize : float
    Minimizable : bool
    Position : list[float]
    Resizable : bool
    ScaleFactor : float
    Size : list[float]
    Theme : WindowTheme
    Title : str
    Transparent : bool


class Color:
    """The standard colors available"""
    PRIMARY=''
    SUCCESS=''
    DANGER=''
    WARNING=''
    INFO=''
    LIGHT=''
    DARK=''
    BACKGROUND_THEME=''
    ALICE_BLUE=''
    ANTIQUE_WHITE=''
    AQUA=''
    AQUAMARINE=''
    AZURE=''
    BEIGE=''
    BISQUE=''
    BLACK=''
    BLANCHED_ALMOND=''
    BLUE=''
    BLUE_VIOLET=''
    BROWN=''
    BURLY_WOOD=''
    CADET_BLUE=''
    CHARTREUSE=''
    CHOCOLATE=''
    CORAL=''
    CORNFLOWER_BLUE=''
    CORNSILK=''
    CRIMSON=''
    CYAN=''
    DARK_BLUE=''
    DARK_CYAN=''
    DARK_GOLDEN_ROD=''
    DARK_GRAY=''
    DARK_GREY=''
    DARK_GREEN=''
    DARK_KHAKI=''
    DARK_MAGENTA=''
    DARK_OLIVE_GREEN=''
    DARK_ORANGE=''
    DARK_ORCHID=''
    DARK_RED=''
    DARK_SALMON=''
    DARK_SEA_GREEN=''
    DARK_SLATE_BLUE=''
    DARK_SLATE_GRAY=''
    DARK_SLATE_GREY=''
    DARK_TURQUOISE=''
    DARK_VIOLET=''
    DEEP_PINK=''
    DEEP_SKY_BLUE=''
    DIM_GRAY=''
    DIM_GREY=''
    DODGER_BLUE=''
    FIRE_BRICK=''
    FLORAL_WHITE=''
    FOREST_GREEN=''
    FUCHSIA=''
    GAINSBORO=''
    GHOST_WHITE=''
    GOLD=''
    GOLDEN_ROD=''
    GRAY=''
    GREY=''
    GREEN=''
    GREEN_YELLOW=''
    HONEY_DEW=''
    HOT_PINK=''
    INDIAN_RED=''
    INDIGO=''
    IVORY=''
    KHAKI=''
    LAVENDER=''
    LAVENDER_BLUSH=''
    LAWN_GREEN=''
    LEMON_CHIFFON=''
    LIGHT_BLUE=''
    LIGHT_CORAL=''
    LIGHT_CYAN=''
    LIGHT_GOLDEN_ROD_YELLOW=''
    LIGHT_GRAY=''
    LIGHT_GREY=''
    LIGHT_GREEN=''
    LIGHT_PINK=''
    LIGHT_SALMON=''
    LIGHT_SEA_GREEN=''
    LIGHT_SKY_BLUE=''
    LIGHT_SLATE_GRAY=''
    LIGHT_SLATE_GREY=''
    LIGHT_STEEL_BLUE=''
    LIGHT_YELLOW=''
    LIME=''
    LIME_GREEN=''
    LINEN=''
    MAGENTA=''
    MAROON=''
    MEDIUM_AQUA_MARINE=''
    MEDIUM_BLUE=''
    MEDIUM_ORCHID=''
    MEDIUM_PURPLE=''
    MEDIUM_SEA_GREEN=''
    MEDIUM_SLATE_BLUE=''
    MEDIUM_SPRING_GREEN=''
    MEDIUM_TURQUOISE=''
    MEDIUM_VIOLET_RED=''
    MIDNIGHT_BLUE=''
    MINT_CREAM=''
    MISTY_ROSE=''
    MOCCASIN=''
    NAVAJO_WHITE=''
    NAVY=''
    OLD_LACE=''
    OLIVE=''
    OLIVE_DRAB=''
    ORANGE=''
    ORANGE_RED=''
    ORCHID=''
    PALE_GOLDEN_ROD=''
    PALE_GREEN=''
    PALE_TURQUOISE=''
    PALE_VIOLET_RED=''
    PAPAYA_WHIP=''
    PEACH_PUFF=''
    PERU=''
    PINK=''
    PLUM=''
    POWDER_BLUE=''
    PURPLE=''
    REBECCA_PURPLE=''
    RED=''
    ROSY_BROWN=''
    ROYAL_BLUE=''
    SADDLE_BROWN=''
    SALMON=''
    SANDY_BROWN=''
    SEA_GREEN=''
    SEA_SHELL=''
    SIENNA=''
    SILVER=''
    SKY_BLUE=''
    SLATE_BLUE=''
    SLATE_GRAY=''
    SLATE_GREY=''
    SNOW=''
    SPRING_GREEN=''
    STEEL_BLUE=''
    TAN=''
    TEAL=''
    THISTLE=''
    TOMATO=''
    TRANSPARENT=''
    TURQUOISE=''
    VIOLET=''
    WHEAT=''
    WHITE=''
    WHITE_SMOKE=''
    YELLOW=''
    YELLOW_GREEN=''

class Icon:
    """Icon types"""
    Alarm = ...
    AlarmFill = ...
    AlignBottom = ...
    AlignCenter = ...
    AlignEnd = ...
    AlignMiddle = ...
    AlignStart = ...
    AlignTop = ...
    Alt = ...
    App = ...
    AppIndicator = ...
    Archive = ...
    ArchiveFill = ...
    Arrow90DegDown = ...
    Arrow90DegLeft = ...
    Arrow90DegRight = ...
    Arrow90DegUp = ...
    ArrowBarDown = ...
    ArrowBarLeft = ...
    ArrowBarRight = ...
    ArrowBarUp = ...
    ArrowClockwise = ...
    ArrowCounterclockwise = ...
    ArrowDown = ...
    ArrowDownCircle = ...
    ArrowDownCircleFill = ...
    ArrowDownLeft = ...
    ArrowDownLeftCircle = ...
    ArrowDownLeftCircleFill = ...
    ArrowDownLeftSquare = ...
    ArrowDownLeftSquareFill = ...
    ArrowDownRight = ...
    ArrowDownRightCircle = ...
    ArrowDownRightCircleFill = ...
    ArrowDownRightSquare = ...
    ArrowDownRightSquareFill = ...
    ArrowDownShort = ...
    ArrowDownSquare = ...
    ArrowDownSquareFill = ...
    ArrowDownUp = ...
    ArrowLeft = ...
    ArrowLeftCircle = ...
    ArrowLeftCircleFill = ...
    ArrowLeftRight = ...
    ArrowLeftShort = ...
    ArrowLeftSquare = ...
    ArrowLeftSquareFill = ...
    ArrowRepeat = ...
    ArrowReturnLeft = ...
    ArrowReturnRight = ...
    ArrowRight = ...
    ArrowRightCircle = ...
    ArrowRightCircleFill = ...
    ArrowRightShort = ...
    ArrowRightSquare = ...
    ArrowRightSquareFill = ...
    ArrowUp = ...
    ArrowUpCircle = ...
    ArrowUpCircleFill = ...
    ArrowUpLeft = ...
    ArrowUpLeftCircle = ...
    ArrowUpLeftCircleFill = ...
    ArrowUpLeftSquare = ...
    ArrowUpLeftSquareFill = ...
    ArrowUpRight = ...
    ArrowUpRightCircle = ...
    ArrowUpRightCircleFill = ...
    ArrowUpRightSquare = ...
    ArrowUpRightSquareFill = ...
    ArrowUpShort = ...
    ArrowUpSquare = ...
    ArrowUpSquareFill = ...
    ArrowsAngleContract = ...
    ArrowsAngleExpand = ...
    ArrowsCollapse = ...
    ArrowsExpand = ...
    ArrowsFullscreen = ...
    ArrowsMove = ...
    AspectRatio = ...
    AspectRatioFill = ...
    Asterisk = ...
    At = ...
    Award = ...
    AwardFill = ...
    Back = ...
    Backspace = ...
    BackspaceFill = ...
    BackspaceReverse = ...
    BackspaceReverseFill = ...
    Badge4K = ...
    Badge4KFill = ...
    Badge8K = ...
    Badge8KFill = ...
    BadgeAd = ...
    BadgeAdFill = ...
    BadgeCc = ...
    BadgeCcFill = ...
    BadgeHd = ...
    BadgeHdFill = ...
    BadgeTm = ...
    BadgeTmFill = ...
    BadgeVo = ...
    BadgeVoFill = ...
    Bag = ...
    BagCheck = ...
    BagCheckFill = ...
    BagDash = ...
    BagDashFill = ...
    BagFill = ...
    BagPlus = ...
    BagPlusFill = ...
    BagX = ...
    BagXFill = ...
    BarChart = ...
    BarChartFill = ...
    BarChartLine = ...
    BarChartLineFill = ...
    BarChartSteps = ...
    Basket = ...
    Basket2 = ...
    Basket2Fill = ...
    Basket3 = ...
    Basket3Fill = ...
    BasketFill = ...
    Battery = ...
    BatteryCharging = ...
    BatteryFull = ...
    BatteryHalf = ...
    Bell = ...
    BellFill = ...
    Bezier = ...
    Bezier2 = ...
    Bicycle = ...
    Binoculars = ...
    BinocularsFill = ...
    BlockquoteLeft = ...
    BlockquoteRight = ...
    Book = ...
    BookFill = ...
    BookHalf = ...
    Bookmark = ...
    BookmarkCheck = ...
    BookmarkCheckFill = ...
    BookmarkDash = ...
    BookmarkDashFill = ...
    BookmarkFill = ...
    BookmarkHeart = ...
    BookmarkHeartFill = ...
    BookmarkPlus = ...
    BookmarkPlusFill = ...
    BookmarkStar = ...
    BookmarkStarFill = ...
    BookmarkX = ...
    BookmarkXFill = ...
    Bookmarks = ...
    BookmarksFill = ...
    Bookshelf = ...
    Bootstrap = ...
    BootstrapFill = ...
    BootstrapReboot = ...
    BorderStyle = ...
    BorderWidth = ...
    BoundingBox = ...
    BoundingBoxCircles = ...
    Box = ...
    BoxArrowDown = ...
    BoxArrowDownLeft = ...
    BoxArrowDownRight = ...
    BoxArrowInDown = ...
    BoxArrowInDownLeft = ...
    BoxArrowInDownRight = ...
    BoxArrowInLeft = ...
    BoxArrowInRight = ...
    BoxArrowInUp = ...
    BoxArrowInUpLeft = ...
    BoxArrowInUpRight = ...
    BoxArrowLeft = ...
    BoxArrowRight = ...
    BoxArrowUp = ...
    BoxArrowUpLeft = ...
    BoxArrowUpRight = ...
    BoxSeam = ...
    Braces = ...
    Bricks = ...
    Briefcase = ...
    BriefcaseFill = ...
    BrightnessAltHigh = ...
    BrightnessAltHighFill = ...
    BrightnessAltLow = ...
    BrightnessAltLowFill = ...
    BrightnessHigh = ...
    BrightnessHighFill = ...
    BrightnessLow = ...
    BrightnessLowFill = ...
    Broadcast = ...
    BroadcastPin = ...
    Brush = ...
    BrushFill = ...
    Bucket = ...
    BucketFill = ...
    Bug = ...
    BugFill = ...
    Building = ...
    Bullseye = ...
    Calculator = ...
    CalculatorFill = ...
    Calendar = ...
    Calendar2 = ...
    Calendar2Check = ...
    Calendar2CheckFill = ...
    Calendar2Date = ...
    Calendar2DateFill = ...
    Calendar2Day = ...
    Calendar2DayFill = ...
    Calendar2Event = ...
    Calendar2EventFill = ...
    Calendar2Fill = ...
    Calendar2Minus = ...
    Calendar2MinusFill = ...
    Calendar2Month = ...
    Calendar2MonthFill = ...
    Calendar2Plus = ...
    Calendar2PlusFill = ...
    Calendar2Range = ...
    Calendar2RangeFill = ...
    Calendar2Week = ...
    Calendar2WeekFill = ...
    Calendar2X = ...
    Calendar2XFill = ...
    Calendar3 = ...
    Calendar3Event = ...
    Calendar3EventFill = ...
    Calendar3Fill = ...
    Calendar3Range = ...
    Calendar3RangeFill = ...
    Calendar3Week = ...
    Calendar3WeekFill = ...
    Calendar4 = ...
    Calendar4Event = ...
    Calendar4Range = ...
    Calendar4Week = ...
    CalendarCheck = ...
    CalendarCheckFill = ...
    CalendarDate = ...
    CalendarDateFill = ...
    CalendarDay = ...
    CalendarDayFill = ...
    CalendarEvent = ...
    CalendarEventFill = ...
    CalendarFill = ...
    CalendarMinus = ...
    CalendarMinusFill = ...
    CalendarMonth = ...
    CalendarMonthFill = ...
    CalendarPlus = ...
    CalendarPlusFill = ...
    CalendarRange = ...
    CalendarRangeFill = ...
    CalendarWeek = ...
    CalendarWeekFill = ...
    CalendarX = ...
    CalendarXFill = ...
    Camera = ...
    Camera2 = ...
    CameraFill = ...
    CameraReels = ...
    CameraReelsFill = ...
    CameraVideo = ...
    CameraVideoFill = ...
    CameraVideoOff = ...
    CameraVideoOffFill = ...
    Capslock = ...
    CapslockFill = ...
    CardChecklist = ...
    CardHeading = ...
    CardImage = ...
    CardList = ...
    CardText = ...
    CaretDown = ...
    CaretDownFill = ...
    CaretDownSquare = ...
    CaretDownSquareFill = ...
    CaretLeft = ...
    CaretLeftFill = ...
    CaretLeftSquare = ...
    CaretLeftSquareFill = ...
    CaretRight = ...
    CaretRightFill = ...
    CaretRightSquare = ...
    CaretRightSquareFill = ...
    CaretUp = ...
    CaretUpFill = ...
    CaretUpSquare = ...
    CaretUpSquareFill = ...
    Cart = ...
    Cart2 = ...
    Cart3 = ...
    Cart4 = ...
    CartCheck = ...
    CartCheckFill = ...
    CartDash = ...
    CartDashFill = ...
    CartFill = ...
    CartPlus = ...
    CartPlusFill = ...
    CartX = ...
    CartXFill = ...
    Cash = ...
    CashStack = ...
    Cast = ...
    Chat = ...
    ChatDots = ...
    ChatDotsFill = ...
    ChatFill = ...
    ChatLeft = ...
    ChatLeftDots = ...
    ChatLeftDotsFill = ...
    ChatLeftFill = ...
    ChatLeftQuote = ...
    ChatLeftQuoteFill = ...
    ChatLeftText = ...
    ChatLeftTextFill = ...
    ChatQuote = ...
    ChatQuoteFill = ...
    ChatRight = ...
    ChatRightDots = ...
    ChatRightDotsFill = ...
    ChatRightFill = ...
    ChatRightQuote = ...
    ChatRightQuoteFill = ...
    ChatRightText = ...
    ChatRightTextFill = ...
    ChatSquare = ...
    ChatSquareDots = ...
    ChatSquareDotsFill = ...
    ChatSquareFill = ...
    ChatSquareQuote = ...
    ChatSquareQuoteFill = ...
    ChatSquareText = ...
    ChatSquareTextFill = ...
    ChatText = ...
    ChatTextFill = ...
    Check = ...
    Check2 = ...
    Check2All = ...
    Check2Circle = ...
    Check2Square = ...
    CheckAll = ...
    CheckCircle = ...
    CheckCircleFill = ...
    CheckSquare = ...
    CheckSquareFill = ...
    ChevronBarContract = ...
    ChevronBarDown = ...
    ChevronBarExpand = ...
    ChevronBarLeft = ...
    ChevronBarRight = ...
    ChevronBarUp = ...
    ChevronCompactDown = ...
    ChevronCompactLeft = ...
    ChevronCompactRight = ...
    ChevronCompactUp = ...
    ChevronContract = ...
    ChevronDoubleDown = ...
    ChevronDoubleLeft = ...
    ChevronDoubleRight = ...
    ChevronDoubleUp = ...
    ChevronDown = ...
    ChevronExpand = ...
    ChevronLeft = ...
    ChevronRight = ...
    ChevronUp = ...
    Circle = ...
    CircleFill = ...
    CircleHalf = ...
    CircleSquare = ...
    Clipboard = ...
    ClipboardCheck = ...
    ClipboardData = ...
    ClipboardMinus = ...
    ClipboardPlus = ...
    ClipboardX = ...
    Clock = ...
    ClockFill = ...
    ClockHistory = ...
    Cloud = ...
    CloudArrowDown = ...
    CloudArrowDownFill = ...
    CloudArrowUp = ...
    CloudArrowUpFill = ...
    CloudCheck = ...
    CloudCheckFill = ...
    CloudDownload = ...
    CloudDownloadFill = ...
    CloudFill = ...
    CloudMinus = ...
    CloudMinusFill = ...
    CloudPlus = ...
    CloudPlusFill = ...
    CloudSlash = ...
    CloudSlashFill = ...
    CloudUpload = ...
    CloudUploadFill = ...
    Code = ...
    CodeSlash = ...
    CodeSquare = ...
    Collection = ...
    CollectionFill = ...
    CollectionPlay = ...
    CollectionPlayFill = ...
    Columns = ...
    ColumnsGap = ...
    Command = ...
    Compass = ...
    CompassFill = ...
    Cone = ...
    ConeStriped = ...
    Controller = ...
    Cpu = ...
    CpuFill = ...
    CreditCard = ...
    CreditCard2Back = ...
    CreditCard2BackFill = ...
    CreditCard2Front = ...
    CreditCard2FrontFill = ...
    CreditCardFill = ...
    Crop = ...
    Cup = ...
    CupFill = ...
    CupStraw = ...
    Cursor = ...
    CursorFill = ...
    CursorText = ...
    Dash = ...
    DashCircle = ...
    DashCircleFill = ...
    DashSquare = ...
    DashSquareFill = ...
    Diagram2 = ...
    Diagram2Fill = ...
    Diagram3 = ...
    Diagram3Fill = ...
    Diamond = ...
    DiamondFill = ...
    DiamondHalf = ...
    Dice1 = ...
    Dice1Fill = ...
    Dice2 = ...
    Dice2Fill = ...
    Dice3 = ...
    Dice3Fill = ...
    Dice4 = ...
    Dice4Fill = ...
    Dice5 = ...
    Dice5Fill = ...
    Dice6 = ...
    Dice6Fill = ...
    Disc = ...
    DiscFill = ...
    Discord = ...
    Display = ...
    DisplayFill = ...
    DistributeHorizontal = ...
    DistributeVertical = ...
    DoorClosed = ...
    DoorClosedFill = ...
    DoorOpen = ...
    DoorOpenFill = ...
    Dot = ...
    Download = ...
    Droplet = ...
    DropletFill = ...
    DropletHalf = ...
    Earbuds = ...
    Easel = ...
    EaselFill = ...
    Egg = ...
    EggFill = ...
    EggFried = ...
    Eject = ...
    EjectFill = ...
    EmojiAngry = ...
    EmojiAngryFill = ...
    EmojiDizzy = ...
    EmojiDizzyFill = ...
    EmojiExpressionless = ...
    EmojiExpressionlessFill = ...
    EmojiFrown = ...
    EmojiFrownFill = ...
    EmojiHeartEyes = ...
    EmojiHeartEyesFill = ...
    EmojiLaughing = ...
    EmojiLaughingFill = ...
    EmojiNeutral = ...
    EmojiNeutralFill = ...
    EmojiSmile = ...
    EmojiSmileFill = ...
    EmojiSmileUpsideDown = ...
    EmojiSmileUpsideDownFill = ...
    EmojiSunglasses = ...
    EmojiSunglassesFill = ...
    EmojiWink = ...
    EmojiWinkFill = ...
    Envelope = ...
    EnvelopeFill = ...
    EnvelopeOpen = ...
    EnvelopeOpenFill = ...
    Exclamation = ...
    ExclamationCircle = ...
    ExclamationCircleFill = ...
    ExclamationDiamond = ...
    ExclamationDiamondFill = ...
    ExclamationOctagon = ...
    ExclamationOctagonFill = ...
    ExclamationSquare = ...
    ExclamationSquareFill = ...
    ExclamationTriangle = ...
    ExclamationTriangleFill = ...
    Exclude = ...
    Eye = ...
    EyeFill = ...
    EyeSlash = ...
    EyeSlashFill = ...
    Eyeglasses = ...
    Facebook = ...
    File = ...
    FileArrowDown = ...
    FileArrowDownFill = ...
    FileArrowUp = ...
    FileArrowUpFill = ...
    FileBarGraph = ...
    FileBarGraphFill = ...
    FileBinary = ...
    FileBinaryFill = ...
    FileBreak = ...
    FileBreakFill = ...
    FileCheck = ...
    FileCheckFill = ...
    FileCode = ...
    FileCodeFill = ...
    FileDiff = ...
    FileDiffFill = ...
    FileEarmark = ...
    FileEarmarkArrowDown = ...
    FileEarmarkArrowDownFill = ...
    FileEarmarkArrowUp = ...
    FileEarmarkArrowUpFill = ...
    FileEarmarkBarGraph = ...
    FileEarmarkBarGraphFill = ...
    FileEarmarkBinary = ...
    FileEarmarkBinaryFill = ...
    FileEarmarkBreak = ...
    FileEarmarkBreakFill = ...
    FileEarmarkCheck = ...
    FileEarmarkCheckFill = ...
    FileEarmarkCode = ...
    FileEarmarkCodeFill = ...
    FileEarmarkDiff = ...
    FileEarmarkDiffFill = ...
    FileEarmarkEasel = ...
    FileEarmarkEaselFill = ...
    FileEarmarkExcel = ...
    FileEarmarkExcelFill = ...
    FileEarmarkFill = ...
    FileEarmarkFont = ...
    FileEarmarkFontFill = ...
    FileEarmarkImage = ...
    FileEarmarkImageFill = ...
    FileEarmarkLock = ...
    FileEarmarkLock2 = ...
    FileEarmarkLock2Fill = ...
    FileEarmarkLockFill = ...
    FileEarmarkMedical = ...
    FileEarmarkMedicalFill = ...
    FileEarmarkMinus = ...
    FileEarmarkMinusFill = ...
    FileEarmarkMusic = ...
    FileEarmarkMusicFill = ...
    FileEarmarkPerson = ...
    FileEarmarkPersonFill = ...
    FileEarmarkPlay = ...
    FileEarmarkPlayFill = ...
    FileEarmarkPlus = ...
    FileEarmarkPlusFill = ...
    FileEarmarkPost = ...
    FileEarmarkPostFill = ...
    FileEarmarkPpt = ...
    FileEarmarkPptFill = ...
    FileEarmarkRichtext = ...
    FileEarmarkRichtextFill = ...
    FileEarmarkRuled = ...
    FileEarmarkRuledFill = ...
    FileEarmarkSlides = ...
    FileEarmarkSlidesFill = ...
    FileEarmarkSpreadsheet = ...
    FileEarmarkSpreadsheetFill = ...
    FileEarmarkText = ...
    FileEarmarkTextFill = ...
    FileEarmarkWord = ...
    FileEarmarkWordFill = ...
    FileEarmarkX = ...
    FileEarmarkXFill = ...
    FileEarmarkZip = ...
    FileEarmarkZipFill = ...
    FileEasel = ...
    FileEaselFill = ...
    FileExcel = ...
    FileExcelFill = ...
    FileFill = ...
    FileFont = ...
    FileFontFill = ...
    FileImage = ...
    FileImageFill = ...
    FileLock = ...
    FileLock2 = ...
    FileLock2Fill = ...
    FileLockFill = ...
    FileMedical = ...
    FileMedicalFill = ...
    FileMinus = ...
    FileMinusFill = ...
    FileMusic = ...
    FileMusicFill = ...
    FilePerson = ...
    FilePersonFill = ...
    FilePlay = ...
    FilePlayFill = ...
    FilePlus = ...
    FilePlusFill = ...
    FilePost = ...
    FilePostFill = ...
    FilePpt = ...
    FilePptFill = ...
    FileRichtext = ...
    FileRichtextFill = ...
    FileRuled = ...
    FileRuledFill = ...
    FileSlides = ...
    FileSlidesFill = ...
    FileSpreadsheet = ...
    FileSpreadsheetFill = ...
    FileText = ...
    FileTextFill = ...
    FileWord = ...
    FileWordFill = ...
    FileX = ...
    FileXFill = ...
    FileZip = ...
    FileZipFill = ...
    Files = ...
    FilesAlt = ...
    Film = ...
    Filter = ...
    FilterCircle = ...
    FilterCircleFill = ...
    FilterLeft = ...
    FilterRight = ...
    FilterSquare = ...
    FilterSquareFill = ...
    Flag = ...
    FlagFill = ...
    Flower1 = ...
    Flower2 = ...
    Flower3 = ...
    Folder = ...
    Folder2 = ...
    Folder2Open = ...
    FolderCheck = ...
    FolderFill = ...
    FolderMinus = ...
    FolderPlus = ...
    FolderSymlink = ...
    FolderSymlinkFill = ...
    FolderX = ...
    Fonts = ...
    Forward = ...
    ForwardFill = ...
    Front = ...
    Fullscreen = ...
    FullscreenExit = ...
    Funnel = ...
    FunnelFill = ...
    Gear = ...
    GearFill = ...
    GearWide = ...
    GearWideConnected = ...
    Gem = ...
    Geo = ...
    GeoAlt = ...
    GeoAltFill = ...
    GeoFill = ...
    Gift = ...
    GiftFill = ...
    Github = ...
    Globe = ...
    Globe2 = ...
    Google = ...
    GraphDown = ...
    GraphUp = ...
    Grid = ...
    Grid1X2 = ...
    Grid1X2Fill = ...
    Grid3X2 = ...
    Grid3X2Gap = ...
    Grid3X2GapFill = ...
    Grid3X3 = ...
    Grid3X3Gap = ...
    Grid3X3GapFill = ...
    GridFill = ...
    GripHorizontal = ...
    GripVertical = ...
    Hammer = ...
    HandIndex = ...
    HandIndexThumb = ...
    HandThumbsDown = ...
    HandThumbsUp = ...
    Handbag = ...
    HandbagFill = ...
    Hash = ...
    Hdd = ...
    HddFill = ...
    HddNetwork = ...
    HddNetworkFill = ...
    HddRack = ...
    HddRackFill = ...
    HddStack = ...
    HddStackFill = ...
    Headphones = ...
    Headset = ...
    Heart = ...
    HeartFill = ...
    HeartHalf = ...
    Heptagon = ...
    HeptagonFill = ...
    HeptagonHalf = ...
    Hexagon = ...
    HexagonFill = ...
    HexagonHalf = ...
    Hourglass = ...
    HourglassBottom = ...
    HourglassSplit = ...
    HourglassTop = ...
    House = ...
    HouseDoor = ...
    HouseDoorFill = ...
    HouseFill = ...
    Hr = ...
    Image = ...
    ImageAlt = ...
    ImageFill = ...
    Images = ...
    Inbox = ...
    InboxFill = ...
    Inboxes = ...
    InboxesFill = ...
    Info = ...
    InfoCircle = ...
    InfoCircleFill = ...
    InfoSquare = ...
    InfoSquareFill = ...
    InputCursor = ...
    InputCursorText = ...
    Instagram = ...
    Intersect = ...
    Journal = ...
    JournalAlbum = ...
    JournalArrowDown = ...
    JournalArrowUp = ...
    JournalBookmark = ...
    JournalBookmarkFill = ...
    JournalCheck = ...
    JournalCode = ...
    JournalMedical = ...
    JournalMinus = ...
    JournalPlus = ...
    JournalRichtext = ...
    JournalText = ...
    JournalX = ...
    Journals = ...
    Joystick = ...
    Justify = ...
    JustifyLeft = ...
    JustifyRight = ...
    Kanban = ...
    KanbanFill = ...
    Key = ...
    KeyFill = ...
    Keyboard = ...
    KeyboardFill = ...
    Ladder = ...
    Lamp = ...
    LampFill = ...
    Laptop = ...
    LaptopFill = ...
    Layers = ...
    LayersFill = ...
    LayersHalf = ...
    LayoutSidebar = ...
    LayoutSidebarInset = ...
    LayoutSidebarInsetReverse = ...
    LayoutSidebarReverse = ...
    LayoutSplit = ...
    LayoutTextSidebar = ...
    LayoutTextSidebarReverse = ...
    LayoutTextWindow = ...
    LayoutTextWindowReverse = ...
    LayoutThreeColumns = ...
    LayoutWtf = ...
    LifePreserver = ...
    Lightning = ...
    LightningFill = ...
    Link = ...
    Link45Deg = ...
    Linkedin = ...
    List = ...
    ListCheck = ...
    ListNested = ...
    ListOl = ...
    ListStars = ...
    ListTask = ...
    ListUl = ...
    Lock = ...
    LockFill = ...
    Mailbox = ...
    Mailbox2 = ...
    Map = ...
    MapFill = ...
    Markdown = ...
    MarkdownFill = ...
    MenuApp = ...
    MenuAppFill = ...
    MenuButton = ...
    MenuButtonFill = ...
    MenuButtonWide = ...
    MenuButtonWideFill = ...
    MenuDown = ...
    MenuUp = ...
    Mic = ...
    MicFill = ...
    MicMute = ...
    MicMuteFill = ...
    Minecart = ...
    MinecartLoaded = ...
    Moon = ...
    Mouse = ...
    Mouse2 = ...
    Mouse3 = ...
    MusicNote = ...
    MusicNoteBeamed = ...
    MusicNoteList = ...
    MusicPlayer = ...
    MusicPlayerFill = ...
    Newspaper = ...
    NodeMinus = ...
    NodeMinusFill = ...
    NodePlus = ...
    NodePlusFill = ...
    Nut = ...
    NutFill = ...
    Octagon = ...
    OctagonFill = ...
    OctagonHalf = ...
    Option = ...
    Outlet = ...
    Paperclip = ...
    Paragraph = ...
    PatchCheck = ...
    PatchCheckFll = ...
    PatchExclamation = ...
    PatchExclamationFll = ...
    PatchMinus = ...
    PatchMinusFll = ...
    PatchPlus = ...
    PatchPlusFll = ...
    PatchQuestion = ...
    PatchQuestionFll = ...
    Pause = ...
    PauseBtn = ...
    PauseBtnFill = ...
    PauseCircle = ...
    PauseCircleFill = ...
    PauseFill = ...
    Peace = ...
    PeaceFill = ...
    Pen = ...
    PenFill = ...
    Pencil = ...
    PencilFill = ...
    PencilSquare = ...
    Pentagon = ...
    PentagonFill = ...
    PentagonHalf = ...
    People = ...
    PeopleFill = ...
    Percent = ...
    Person = ...
    PersonBadge = ...
    PersonBadgeFill = ...
    PersonBoundingBox = ...
    PersonCheck = ...
    PersonCheckFill = ...
    PersonCircle = ...
    PersonDash = ...
    PersonDashFill = ...
    PersonFill = ...
    PersonLinesFill = ...
    PersonPlus = ...
    PersonPlusFill = ...
    PersonSquare = ...
    PersonX = ...
    PersonXFill = ...
    Phone = ...
    PhoneFill = ...
    PhoneLandscape = ...
    PhoneLandscapeFill = ...
    PhoneVibrate = ...
    PieChart = ...
    PieChartFill = ...
    Pip = ...
    PipFill = ...
    Play = ...
    PlayBtn = ...
    PlayBtnFill = ...
    PlayCircle = ...
    PlayCircleFill = ...
    PlayFill = ...
    Plug = ...
    PlugFill = ...
    Plus = ...
    PlusCircle = ...
    PlusCircleFill = ...
    PlusSquare = ...
    PlusSquareFill = ...
    Power = ...
    Printer = ...
    PrinterFill = ...
    Puzzle = ...
    PuzzleFill = ...
    Question = ...
    QuestionCircle = ...
    QuestionCircleFill = ...
    QuestionDiamond = ...
    QuestionDiamondFill = ...
    QuestionOctagon = ...
    QuestionOctagonFill = ...
    QuestionSquare = ...
    QuestionSquareFill = ...
    Receipt = ...
    ReceiptCutoff = ...
    Reception0 = ...
    Reception1 = ...
    Reception2 = ...
    Reception3 = ...
    Reception4 = ...
    Record = ...
    Record2 = ...
    Record2Fill = ...
    RecordBtn = ...
    RecordBtnFill = ...
    RecordCircle = ...
    RecordCircleFill = ...
    RecordFill = ...
    Reply = ...
    ReplyAll = ...
    ReplyAllFill = ...
    ReplyFill = ...
    Rss = ...
    RssFill = ...
    Scissors = ...
    Screwdriver = ...
    Search = ...
    SegmentedNav = ...
    Server = ...
    Share = ...
    ShareFill = ...
    Shield = ...
    ShieldCheck = ...
    ShieldExclamation = ...
    ShieldFill = ...
    ShieldFillCheck = ...
    ShieldFillExclamation = ...
    ShieldFillMinus = ...
    ShieldFillPlus = ...
    ShieldFillX = ...
    ShieldLock = ...
    ShieldLockFill = ...
    ShieldMinus = ...
    ShieldPlus = ...
    ShieldShaded = ...
    ShieldSlash = ...
    ShieldSlashFill = ...
    ShieldX = ...
    Shift = ...
    ShiftFill = ...
    Shop = ...
    ShopWindow = ...
    Shuffle = ...
    Signpost = ...
    Signpost2 = ...
    Signpost2Fill = ...
    SignpostFill = ...
    SignpostSplit = ...
    SignpostSplitFill = ...
    Sim = ...
    SimFill = ...
    SkipBackward = ...
    SkipBackwardBtn = ...
    SkipBackwardBtnFill = ...
    SkipBackwardCircle = ...
    SkipBackwardCircleFill = ...
    SkipBackwardFill = ...
    SkipEnd = ...
    SkipEndBtn = ...
    SkipEndBtnFill = ...
    SkipEndCircle = ...
    SkipEndCircleFill = ...
    SkipEndFill = ...
    SkipForward = ...
    SkipForwardBtn = ...
    SkipForwardBtnFill = ...
    SkipForwardCircle = ...
    SkipForwardCircleFill = ...
    SkipForwardFill = ...
    SkipStart = ...
    SkipStartBtn = ...
    SkipStartBtnFill = ...
    SkipStartCircle = ...
    SkipStartCircleFill = ...
    SkipStartFill = ...
    Slack = ...
    Slash = ...
    SlashCircle = ...
    SlashCircleFill = ...
    SlashSquare = ...
    SlashSquareFill = ...
    Sliders = ...
    Smartwatch = ...
    SortAlphaDown = ...
    SortAlphaDownAlt = ...
    SortAlphaUp = ...
    SortAlphaUpAlt = ...
    SortDown = ...
    SortDownAlt = ...
    SortNumericDown = ...
    SortNumericDownAlt = ...
    SortNumericUp = ...
    SortNumericUpAlt = ...
    SortUp = ...
    SortUpAlt = ...
    Soundwave = ...
    Speaker = ...
    SpeakerFill = ...
    Spellcheck = ...
    Square = ...
    SquareFill = ...
    SquareHalf = ...
    Star = ...
    StarFill = ...
    StarHalf = ...
    Stickies = ...
    StickiesFill = ...
    Sticky = ...
    StickyFill = ...
    Stop = ...
    StopBtn = ...
    StopBtnFill = ...
    StopCircle = ...
    StopCircleFill = ...
    StopFill = ...
    Stoplights = ...
    StoplightsFill = ...
    Stopwatch = ...
    StopwatchFill = ...
    Subtract = ...
    SuitClub = ...
    SuitClubFill = ...
    SuitDiamond = ...
    SuitDiamondFill = ...
    SuitHeart = ...
    SuitHeartFill = ...
    SuitSpade = ...
    SuitSpadeFill = ...
    Sun = ...
    Sunglasses = ...
    Table = ...
    Tablet = ...
    TabletFill = ...
    TabletLandscape = ...
    TabletLandscapeFill = ...
    Tag = ...
    TagFill = ...
    Tags = ...
    TagsFill = ...
    Telephone = ...
    TelephoneFill = ...
    TelephoneForward = ...
    TelephoneForwardFill = ...
    TelephoneInbound = ...
    TelephoneInboundFill = ...
    TelephoneMinus = ...
    TelephoneMinusFill = ...
    TelephoneOutbound = ...
    TelephoneOutboundFill = ...
    TelephonePlus = ...
    TelephonePlusFill = ...
    TelephoneX = ...
    TelephoneXFill = ...
    Terminal = ...
    TerminalFill = ...
    TextCenter = ...
    TextIndentLeft = ...
    TextIndentRight = ...
    TextLeft = ...
    TextParagraph = ...
    TextRight = ...
    Textarea = ...
    TextareaResize = ...
    TextareaT = ...
    Thermometer = ...
    ThermometerHalf = ...
    ThreeDots = ...
    ThreeDotsVertical = ...
    Toggle2Off = ...
    Toggle2On = ...
    ToggleOff = ...
    ToggleOn = ...
    Toggles = ...
    Toggles2 = ...
    Tools = ...
    Trash = ...
    Trash2 = ...
    Trash2Fill = ...
    TrashFill = ...
    Tree = ...
    TreeFill = ...
    Triangle = ...
    TriangleFill = ...
    TriangleHalf = ...
    Trophy = ...
    TrophyFill = ...
    Truck = ...
    TruckFlatbed = ...
    Tv = ...
    TvFill = ...
    Twitch = ...
    Twitter = ...
    Type = ...
    TypeBold = ...
    TypeH1 = ...
    TypeH2 = ...
    TypeH3 = ...
    TypeItalic = ...
    TypeStrikethrough = ...
    TypeUnderline = ...
    UiChecks = ...
    UiChecksGrid = ...
    UiRadios = ...
    UiRadiosGrid = ...
    Union = ...
    Unlock = ...
    UnlockFill = ...
    Upc = ...
    UpcScan = ...
    Upload = ...
    VectorPen = ...
    ViewList = ...
    ViewStacked = ...
    Vinyl = ...
    VinylFill = ...
    Voicemail = ...
    VolumeDown = ...
    VolumeDownFill = ...
    VolumeMute = ...
    VolumeMuteFill = ...
    VolumeOff = ...
    VolumeOffFill = ...
    VolumeUp = ...
    VolumeUpFill = ...
    Vr = ...
    Wallet = ...
    Wallet2 = ...
    WalletFill = ...
    Watch = ...
    Wifi = ...
    Wifi1 = ...
    Wifi2 = ...
    WifiOff = ...
    Window = ...
    Wrench = ...
    X = ...
    XCircle = ...
    XCircleFill = ...
    XDiamond = ...
    XDiamondFill = ...
    XOctagon = ...
    XOctagonFill = ...
    XSquare = ...
    XSquareFill = ...
    Youtube = ...
    ZoomIn = ...
    ZoomOut = ...
