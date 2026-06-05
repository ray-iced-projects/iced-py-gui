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
    get_styling_palette,
    StdColorStyle,
    window_theme_names,
)

def on_press(_btn_id: int):
    """Button test"""



def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    update_widget(wnd_id, WindowParam.Theme, theme_name)
    state["selected_theme"] = theme_name
    std_colors_selected(0, 0)
    update_background(theme_name)
    update_widget(rd_id, RadioParam.SelectedIndex, 0)


def color_selected(_cp_id: int, color: list[float]):
    """Color picker callback (reserved for future use)."""
    print(color)



# pylint: disable=redefined-outer-name
def std_colors_selected(_rd_id: int, index: int):
    """Swap container styles when a standard style radio button is selected."""

    match index:
        case 0:
            state["selected_style"] = std_colors[0]
            palettes = get_styling_palette(state["selected_theme"], StdColorStyle.Primary)
        case 1:
            state["selected_style"] = std_colors[1]
            palettes = get_styling_palette(state["selected_theme"], StdColorStyle.Secondary)
        case 2:
            state["selected_style"] = std_colors[2]
            palettes = get_styling_palette(state["selected_theme"], StdColorStyle.Success)
        case 3:
            state["selected_style"] = std_colors[3]
            palettes = get_styling_palette(state["selected_theme"], StdColorStyle.Warning)
        case 4:
            state["selected_style"] = std_colors[4]
            palettes = get_styling_palette(state["selected_theme"], StdColorStyle.Danger)

    for (idx, key) in enumerate(palette_styling_keys):
        if key == "base_alpha_0.5":
            # special case for button disable status
            (bkg_rgba, text_rgba) = palettes["base"]
            bkg_rgba[3] *= 0.5
            text_rgba[3] *= 0.5
        else:
            (bkg_rgba, text_rgba) = palettes[key]

        if border_styling[idx]:
            style_id = get_container_border_style(bkg_rgba, palettes["bkg_stronger"][1])
            update_widget(cont_ids[key], ContainerParam.StyleId, style_id)
            update_widget(text_ids[key], TextParam.ColorRgba, text_rgba)
        else:
            style_id = add_container_style(bkg_rgba=bkg_rgba)
            update_widget(cont_ids[key], ContainerParam.StyleId, style_id)
            update_widget(text_ids[key], TextParam.ColorRgba, text_rgba)


def std_styles_selected(_rd_id: int, index: int):
    """Standard styling from radio selection"""
    labels = []
    match index:
        case 0:
            labels = subtle_labels
            border = subtle_border_bkg_styling
        case 1:
            pass

    for (idx, key) in enumerate(palette_bkg_keys):
        # Update the text values
        update_widget(bkg_text_ids[key], TextParam.Content, labels[key])
        if border[idx]:
            style_id = add_container_style()
            update_widget(cont_ids[key], ContainerParam.StyleId, style_id)


def update_background(theme: str):
    """Update the background containers"""
    palettes = get_styling_palette(theme, StdColorStyle.Primary)
    for key in palette_bkg_keys:
        if key == "bkg_weakest_alpha_0.5":
            (bkg_rgba, text_rgba) = palettes["bkg_weakest"]
            bkg_rgba[3] *= 0.5
            text_rgba[3] *=0.5
        else:
            (bkg_rgba, text_rgba) = palettes[key]

        style_id = add_container_style(bkg_rgba=bkg_rgba)
        update_widget(bkg_cont_ids[key], ContainerParam.StyleId, style_id)
        update_widget(bkg_text_ids[key], TextParam.ColorRgba, text_rgba)


def get_container_border_style(bkg_rgba: list, stronger: list) -> int:
    """get the container style with a border"""
    return add_container_style(
            bkg_rgba=bkg_rgba,
            border_rgba=stronger,
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
         "selected_theme": "TokyoNight"}

# The bkg keys are used for styling in some cases
palette_bkg_keys = ("bkg_base", "bkg_weak", "bkg_weaker", "bkg_weakest", "bkg_neutral",
                "bkg_strong", "bkg_stronger", "bkg_strongest", "bkg_weakest_alpha_0.5")

# Button specific styling for Subtle
subtle_border_bkg_styling = (False, False, True, True, False,
                      True, False, False, True)
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

# Button specific styling
palette_styling_keys = ("base", "weak", "strong", "base_alpha_0.5")
border_styling = (True, False, True, True)
palette_labels = ("base\nActive/Pressed", "weak", "strong\nHovered", "base\nalpha_0.5\nDisabled")


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

        rd_id = add_radio(
            labels=std_colors,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            radio_wrap=True,
            on_selected=std_colors_selected,
        )

        rd_id = add_radio(
            labels=std_styles,
            selected_index=None,
            horizontal=True,
            radio_spacing=10,
            radio_wrap=True,
            on_selected=std_styles_selected,
        )

        add_text(content="Button palettes used for the button statuses, border highlighted ones")

        with Row(spacing=20, wrap=True):
            for (index, key) in enumerate(palette_styling_keys):
                label = palette_labels[index]

                if key == "base_alpha_0.5":
                    # special case for button disable status
                    (bkg_rgba, text_rgba) = palettes["base"]
                    bkg_rgba[3] *= 0.5
                    text_rgba[3] *= 0.5
                else:
                    (bkg_rgba, text_rgba) = palettes[key]

                if border_styling[index]:
                    style_id = get_container_border_style(bkg_rgba, palettes["bkg_stronger"][1])
                else:
                    style_id = add_container_style(bkg_rgba=bkg_rgba)

                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    cont_ids[key] = _cont_id
                    text_ids[key] = add_text(content=label, color_rgba=text_rgba, size=14)



        add_text(content="Background palettes which may or may not be used for statuses\n" +
                 "They may be used for additional styling such as Subtle or Background, etc.")

        with Row(spacing=20, wrap=True):
            content = ""
            for (idx, key )in enumerate(palette_bkg_keys):
                # the palette keys contain the bkg_rgba and the text_color

                if key == "bkg_weakest_alpha_0.5":
                    (bkg_rgba, text_rgba) = palettes["bkg_weakest"]
                    bkg_rgba[3] *= 0.5
                    text_rgba[3] *=0.5
                    content = "bkg_weakest\nalpha_0.5"
                else:
                    (bkg_rgba, text_rgba) = palettes[key]
                    content = key
                style_id = add_container_style(bkg_rgba=bkg_rgba)
                with Container(width=120, height=60,
                                align_center=True, style_id=style_id) as _cont_id:
                    bkg_cont_ids[key] = _cont_id
                    bkg_text_ids[key] = add_text(content=content, color_rgba=text_rgba, size=14)

start_session()
