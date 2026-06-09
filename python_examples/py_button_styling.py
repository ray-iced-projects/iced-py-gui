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
    ContainerParam,
    Row,
    start_session,
    add_pick_list,
    add_radio,
    RadioParam,
    add_text,
    TextParam,
    update_widget,
    update_widget_params,
    get_styling_palette,
    StdColorStyle,
    window_theme_names,
)


def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    update_widget(wnd_id, WindowParam.Theme, theme_name)
    state["selected_theme"] = theme_name
    state["palette"] = get_styling_palette(state["selected_theme"], StdColorStyle.Primary)
    std_colors_selected(0, 0)
    update_widget(rd_id_colors, RadioParam.SelectedIndex, 0)
    std_styles_selected(0,0)
    update_widget(rd_id_styles, RadioParam.SelectedIndex, 0)


def color_selected(_cp_id: int, color: list[float]):
    """Color picker callback (reserved for future use)."""
    print(color)



# pylint: disable=redefined-outer-name
def std_colors_selected(_rd_id: int, index: int):
    """Swap container styles when a standard style radio button is selected."""

    match index:
        case 0:
            state["selected_style"] = std_colors[0]
            state["palette"] = get_styling_palette(state["selected_theme"], StdColorStyle.Primary)
        case 1:
            state["selected_style"] = std_colors[1]
            state["palette"] = get_styling_palette(state["selected_theme"], StdColorStyle.Secondary)
        case 2:
            state["selected_style"] = std_colors[2]
            state["palette"] = get_styling_palette(state["selected_theme"], StdColorStyle.Success)
        case 3:
            state["selected_style"] = std_colors[3]
            state["palette"] = get_styling_palette(state["selected_theme"], StdColorStyle.Warning)
        case 4:
            state["selected_style"] = std_colors[4]
            state["palette"] = get_styling_palette(state["selected_theme"], StdColorStyle.Danger)

    for (key, label) in palette_labels.items():
        if key == "base_alpha_0.5":
            # special case for button disable status
            (bkg_rgba, text_rgba) = state["palette"]["base"]
            bkg_rgba = bkg_rgba.copy()
            text_rgba = text_rgba.copy()
            bkg_rgba[3] *= 0.5
            text_rgba[3] *= 0.5
        else:
            (bkg_rgba, text_rgba) = state["palette"][key]

        if key != label:
            style_id = get_container_border_style(bkg_rgba, state["palette"]["bkg_stronger"][1])
        else:
            style_id = add_container_style(bkg_rgba=bkg_rgba)

        update_widget(cont_ids[key], ContainerParam.StyleId, style_id)
        update_widget(text_ids[key], TextParam.ColorRgba, text_rgba)


def std_styles_selected(_rd_id: int, index: int):
    """Standard styling from radio selection"""
    match index:
        case 0:
            labels = subtle_labels
        case 1:
            labels = bkg_labels

    # add border if palette used by style
    for (key, label )in labels.items():
        # the palette keys contain the bkg_rgba and the text_color
        if key == "bkg_weakest_alpha_0.5":
            (bkg_rgba, text_rgba) = state["palette"]["bkg_weakest"]
            bkg_rgba = bkg_rgba.copy()
            text_rgba = text_rgba.copy()
            bkg_rgba[3] *= 0.5
            text_rgba[3] *=0.5
        else:
            (bkg_rgba, text_rgba) = state["palette"][key]

        if key != label:
            style_id = get_container_border_style(
                            bkg_rgba,
                            state["palette"]["bkg_strongest"][1]) # border color
        else:
            style_id = add_container_style(bkg_rgba=bkg_rgba) # no border

        update_widget(bkg_cont_ids[key], ContainerParam.StyleId, style_id)
        update_widget_params(
            bkg_text_ids[key], {
                TextParam.ColorRgba: text_rgba,
                TextParam.Content: label,
                })


def get_container_border_style(bkg_rgba: list, color_rgba: list) -> int:
    """get the container style with a border"""
    return add_container_style(
            bkg_rgba=bkg_rgba,
            border_rgba=color_rgba,
            border_width=2.0)


# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------
std_colors = ["Primary", "Secondary", "Success", "Warning", "Danger"]
std_styles = ["Subtle", "Background"]
cont_ids = {}   # {variant_name: cont_id}
text_ids = {}   # {variant_name: text_id}
bkg_cont_ids = {}   # {variant_name: cont_id}
bkg_text_ids = {}   # {variant_name: text_id}
state = {"selected_style": "",
         "selected_theme": "TokyoNight",
         "palette": get_styling_palette("TokyoNight", StdColorStyle.Primary)}

# Button specific standard styling colors
# These may be fairly common across widgets
palette_labels = {
    "base": "base\nActive/Pressed",
    "weak": "weak",
    "strong": "strong\nHovered",
    "base_alpha_0.5": "base\nalpha_0.5\nDisabled"
    }

# Specific button standard styles
# The keys are common across widhets, the labels will vary
subtle_labels = {
    "bkg_base": "bkg_base",
    "bkg_weak": "bkg_weak",
    "bkg_weaker": "bkg_weaker\nHovered",
    "bkg_weakest": "bkg_weakest\nActive",
    "bkg_neutral": "bkg_neutral",
    "bkg_strong": "bkg_strong\nPressed",
    "bkg_stronger": "bkg_stronger",
    "bkg_strongest": "bkg_strongest",
    "bkg_weakest_alpha_0.5": "bkg_weakest\nalpha_0.5\nDisabled"}

bkg_labels = {
    "bkg_base": "bkg_base\nActive",
    "bkg_weak": "bkg_weak\nHovered",
    "bkg_weaker": "bkg_weaker",
    "bkg_weakest": "bkg_weakest",
    "bkg_neutral": "bkg_neutral",
    "bkg_strong": "bkg_strong\nPressed",
    "bkg_stronger": "bkg_stronger",
    "bkg_strongest": "bkg_strongest",
    "bkg_weakest_alpha_0.5": "bkg_weakest\nalpha_0.5\nDisabled"}


# ---------------------------------------------------------------------------
# GUI — initial display uses PRIMARY
# ---------------------------------------------------------------------------
with Window(title="Button Styling", size=(700, 800), center=True) as wnd_id:
    # This function get all of the styling and bkg keys for any widget (default startup)
    palettes = get_styling_palette("TokyoNight", StdColorStyle.Primary)

    with Column(spacing=20, padding=[20], wrap=True):

        add_pick_list(options=window_theme_names(), selected="TokyoNight",
                      placeholder="Select Theme", on_select=on_theme_select)

        add_text(content="The radio buttons allow you to pick a standard color (Primary).\n" +
            "These std_styles depend on the selected theme.\n" +
            "if theme=GRUVBOX_LIGHT, primary=light blue, \n" +
            "if theme=GRUVBOX_DARK, primary=dark blue")

        rd_id_colors = add_radio(
            labels=std_colors,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            radio_wrap=True,
            on_selected=std_colors_selected,
        )

        add_text(content="Button palette used for the button statuses, border highlighted ones\n" +
                 "This palette is common across widgets")

        with Row(spacing=20, wrap=True):
            for (key, label) in palette_labels.items():
                if key == "base_alpha_0.5":
                    # special case for button disable status
                    (bkg_rgba, text_rgba) = palettes["base"]
                    bkg_rgba[3] *= 0.5
                    text_rgba[3] *= 0.5
                else:
                    (bkg_rgba, text_rgba) = palettes[key]

                if key != label:
                    style_id = get_container_border_style(
                                    bkg_rgba,
                                    palettes["bkg_stronger"][1]) # border color
                else:
                    style_id = add_container_style(bkg_rgba=bkg_rgba) # no border

                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    cont_ids[key] = _cont_id
                    text_ids[key] = add_text(content=label, color_rgba=text_rgba, size=14)

        add_text(content="Background palette may or may not be used for statuses\n" +
                 "They may be used for additional styling such as Subtle or Background, etc.\n" +
                 "This bkg palette is common to all widgets")

        rd_id_styles = add_radio(
            labels=std_styles,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            radio_wrap=True,
            on_selected=std_styles_selected,
        )

        with Row(spacing=20, wrap=True):
            for (key, label )in subtle_labels.items():
                # the palette keys contain the bkg_rgba and the text_color
                if key == "bkg_weakest_alpha_0.5":
                    (bkg_rgba, text_rgba) = palettes["bkg_weakest"]
                    bkg_rgba[3] *= 0.5
                    text_rgba[3] *=0.5
                    style_id = add_container_style(bkg_rgba=bkg_rgba)
                else:
                    (bkg_rgba, text_rgba) = palettes[key]

                if key != label:
                    style_id = get_container_border_style(
                                    bkg_rgba,
                                    palettes["bkg_strongest"][1]) # border color
                else:
                    style_id = add_container_style(bkg_rgba=bkg_rgba) # no border

                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    bkg_cont_ids[key] = _cont_id
                    bkg_text_ids[key] = add_text(content=label, color_rgba=text_rgba, size=14)

start_session()
