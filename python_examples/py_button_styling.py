#!/usr/bin/env python3
"""
Button styling demo — shows the colour palette for standard button themes,
with live radio-button switching between colour themes.
"""

from icedpygui import (
    Window,
    WindowParam,
    Container,
    add_container_style,
    Column,
    ColorPicker,
    ContainerParam,
    Row,
    start_session,
    add_button,
    ButtonParam,
    ButtonStyleStd,
    add_pick_list,
    add_radio,
    add_text,
    TextParam,
    update_widget,
    update_widget_params,
    get_color_palette,
    get_styling_palette,
    Color,
    StdColorStyle,
    window_theme_names,
    WindowTheme,
)

def on_press(_btn_id: int):
    """Button test"""
    print()


def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    print()


def color_selected(_cp_id: int, color: list[float]):
    """Color picker callback (reserved for future use)."""
    print(color)

def style_type_selected(_rd_id: int, index: int):
    """Swap text in containers when new style type selected"""
    state["style_type"] = style_types[index]

# pylint: disable=redefined-outer-name
def std_colors_selected(_rd_id: int, index: int):
    """Swap container styles when a standard colour radio button is selected."""
    print()


def get_label(name: str) -> str:
    """Return the display text for a variant tile."""
    match name:
        case "base":
            return "base\nActive/Pressed"
        case "strong":
            return "strong\nHovered"
        case _:
            return name


# ---------------------------------------------------------------------------
# Data
# ---------------------------------------------------------------------------
std_styles = ["Primary", "Secondary", "Success", "Warning", "Danger", "Subtle", "Backgropund"]
cont_ids = {}   # {variant_name: cont_id}
text_ids = {}   # {variant_name: text_id}
bkg_cont_ids = {}   # {variant_name: cont_id}
bkg_text_ids = {}   # {variant_name: text_id}
state = {"style_type": "",
         "current_theme_color": WindowTheme.TokyoNight}

# The bkg keys may be used for styling in some cases
palette_bkg_keys = ("bkg_base", "bkg_weak", "bkg_weaker", "bkg_weakest", "bkg_neutral",
                "bkg_strong", "bkg_stronger", "bkg_strongest")

# The styling keys are used for styling the widget
palette_styling_keys = ("base", "weak", "strong")

# This function get all of the styling and bkg keys for any widget
palettes = get_styling_palette("TokyoNight", StdColorStyle.Primary)

# ---------------------------------------------------------------------------
# GUI — initial display uses PRIMARY
# ---------------------------------------------------------------------------
with Window(title="Button Styling", size=(700, 700), center=True) as wnd_id:

    with Column(spacing=20, padding=[20], wrap=True):

        add_pick_list(options=window_theme_names(), selected="TokyoNight",
                      placeholder="Select Theme", on_select=on_theme_select)

        add_text(content="The radio buttons allow you to pick a standard color (Primary).\n" +
            "These std_styles depend on the selected theme.\n" +
            "if theme=GRUVBOX_LIGHT, primary=light blue, \n" +
            "if theme=GRUVBOX_DARK, primary=dark blue")

        add_radio(
            labels=std_styles,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            radio_wrap=True,
            on_selected=std_colors_selected,
        )

        add_text(content="Button palettes used for the button statuses")

        with Row(spacing=20, wrap=True):
            for key in palette_styling_keys:
                label = get_label(key)
                (bkg_color, text_color) = palettes[key]
                style_id = add_container_style(bkg_rgba=bkg_color)
                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    cont_ids[key] = _cont_id
                    text_ids[key] = add_text(content=label, color_rgba=text_color, size=14)

            # special case for button disable status
            (bkg_color, text_color) = palettes["base"]
            bkg_color[3] *= 0.5
            style_id = add_container_style(bkg_rgba=bkg_color)
            with Container(width=120, height=60,
                            align_center=True, style_id=style_id) as _cont_id:
                CONTENT = "base\nalpha_0.5\nDisabled"
                cont_ids["base_alpha_0.5"] = _cont_id
                text_ids["base_alpha_0.5"] = add_text(content=CONTENT, color_rgba=text_color, size=14)

        add_text(content="Background palettes which may or may not be used for statuses")

        with Row(spacing=20, wrap=True):
            for key in palette_bkg_keys:
                # the palette keys contain the bkg_color and the text_color
                (bkg_color, text_color) = palettes[key]
                style_id = add_container_style(bkg_rgba=bkg_color)
                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    bkg_cont_ids[key] = _cont_id
                    bkg_text_ids[key] = add_text(content=key, color_rgba=text_color, size=14)

        add_button(label="Test Button", on_press=on_press, width=100, height=100)
start_session()
