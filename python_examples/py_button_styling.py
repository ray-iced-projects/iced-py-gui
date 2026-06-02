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
    get_rgba_color,
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
    update_widget(wnd_id, WindowParam.Theme, theme_name)
    state["current_theme_color"] = get_rgba_color(name=theme_name)


def color_selected(_cp_id: int, color: list[float]):
    """Color picker callback (reserved for future use)."""
    print(color)

def style_type_selected(_rd_id: int, index: int):
    """Swap text in containers when new style type selected"""
    state["style_type"] = style_types[index]

# pylint: disable=redefined-outer-name
def std_colors_selected(_rd_id: int, index: int):
    """Swap container styles when a standard colour radio button is selected."""
    color_name = std_colors[index]
    for name, (style_id, text_rgba, label) in all_styles[color_name].items():
        update_widget_params(cont_ids[name], {ContainerParam.StyleId: style_id})
        update_widget_params(text_ids[name], {
            TextParam.ColorRgba: list(text_rgba),
            TextParam.Content: label,
        })
    match std_colors[index]:
        case "Primary":
            style_std = ButtonStyleStd.Primary
        case "Secondary":
            style_std = ButtonStyleStd.Secondary
        case "Success":
            style_std = ButtonStyleStd.Success
        case "Warning":
            style_std = ButtonStyleStd.Warning
        case "Danger":
            style_std = ButtonStyleStd.Danger
        case "Subtle":
            style_std = ButtonStyleStd.Subtle

    update_widget(color_btn_id, ButtonParam.StyleStd, style_std)


def get_variants(color: Color) -> list[tuple]:
    """Return sorted (name, color_rgba, text_rgba) tuples for a colour palette."""
    palette = get_color_palette(color)
    sorted_palette = dict(sorted(palette.items()))
    return [
        (k[:-6], list(v), list(sorted_palette[k[:-6] + "_text"]))
        for k, v in sorted_palette.items()
        if k.endswith("_color")
    ]


def get_label(name: str, color_name: str = "") -> str:
    """Return the display text for a variant tile."""
    if color_name == "Subtle":
        match name:
            case "base":
                return "base\nActive"
            case "strong":
                return "strong\nPressed"
            case "weaker":
                return "weaker\nHovered"
            case "base alpha":
                return "base alpha 0.5\nDisabled"
            case _:
                return name
    match name:
        case "base":
            return "base\nActive/Pressed"
        case "strong":
            return "strong\nHovered"
        case "base alpha":
            return "base alpha 0.5\nDisabled"
        case _:
            return name


# ---------------------------------------------------------------------------
# Data
# ---------------------------------------------------------------------------
std_colors = ["Primary", "Secondary", "Success", "Warning", "Danger"]
style_types = ["Background", "Subtle"]
cont_ids = {}   # {variant_name: cont_id}
text_ids = {}   # {variant_name: text_id}
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
with Window(title="Button Styling", size=(800, 600), center=True) as wnd_id:

    with Column(spacing=20, padding=[20], wrap=True):

        add_pick_list(options=window_theme_names(), selected="TokyoNight",
                      placeholder="Select Theme", on_select=on_theme_select)

        add_text(content="Button palettes used for the button statuses")

        with Row(spacing=20, wrap=True):
            for key in ("base", "weak", "strong"):
                label = get_label(key)
                palette = palettes[key]
                style_id = add_container_style(bkg_rgba=palette[0])
                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    cont_ids[key] = _cont_id
                    text_ids[key] = add_text(content=label, color_rgba=palette[1], size=14)

        add_text(content="Background palettes which may or may not be used for statuses")

        with Row(spacing=20, wrap=True):
            for key in palette_bkg_keys:
                # the palette keys contain the bkg_color and the text_color
                (bkg_color, text_color) = palettes[key]
                style_id = add_container_style(bkg_rgba=bkg_color)
                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    cont_ids[key] = _cont_id
                    text_ids[key] = add_text(content=key, color_rgba=text_color, size=14)

        add_radio(
            labels=std_colors,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            on_selected=std_colors_selected,
        )

        with ColorPicker(on_submit=color_selected):
            # A button is required to open the color picker.
            color_btn_id = add_button(label="Color Picker", padding=[3.0],
                                      style_std=ButtonStyleStd.Primary)
        add_button(label="Test", on_press=on_press)
start_session()
