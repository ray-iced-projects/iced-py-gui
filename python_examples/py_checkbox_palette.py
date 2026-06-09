#!/usr/bin/env python3
"""
Checkbox palette demo — shows the colour palette for standard checkbox themes,
with live radio-button switching between colour themes.

Palette key → checkbox status mapping (checkbox::primary):
  Swatch (changes with color selection):
    base   → Checked Active background / icon
    strong → Checked Hovered background / icon
  Background (common to all themes):
    bkg_base   → Unchecked Active background
    bkg_weak   → Unchecked Hovered background
    bkg_strong → Border (Active & Hovered) / Checked Disabled background
    bkg_weaker → Unchecked Disabled background
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
    add_checkbox,
    CheckboxStyleStd,
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
    std_colors_selected(0, 0)
    update_widget(rd_id_colors, RadioParam.SelectedIndex, 0)
    std_styles_selected(0, 0)
    update_widget(rd_id_styles, RadioParam.SelectedIndex, 0)


# pylint: disable=redefined-outer-name
def std_colors_selected(_rd_id: int, index: int):
    """Swap container styles when a standard colour radio button is selected."""
    state["selected_style"] = std_colors[index]
    state["palette"] = get_styling_palette(state["selected_theme"], std_color_enums[index])
    apply_tiles(state["palette"], palette_tiles, cont_ids, text_ids, "bkg_stronger")


def std_styles_selected(_rd_id: int, index: int):
    """Background palette style from radio selection"""
    apply_tiles(state["palette"], std_style_tiles[index],
                bkg_cont_ids, bkg_text_ids, "bkg_strongest")


def get_container_border_style(bkg_rgba: list, color_rgba: list) -> int:
    """Get the container style with a border"""
    return add_container_style(
            bkg_rgba=bkg_rgba,
            border_rgba=color_rgba,
            border_width=2.0)


# Index into (bkg_rgba, text_rgba) palette tuples
TEXT = 1


def get_tile_colors(palette: dict, key: str, alpha: float) -> tuple:
    """Return copies of (bkg_rgba, text_rgba) with optional alpha scaling."""
    bkg_rgba = list(palette[key][0])
    text_rgba = list(palette[key][TEXT])
    if alpha != 1.0:
        bkg_rgba[3] *= alpha
        text_rgba[3] *= alpha
    return bkg_rgba, text_rgba


def apply_tiles(palette: dict, tiles: list, cont_ids: dict, text_ids: dict, border_key: str):
    """Update existing tile containers with new palette colors and labels."""
    for i, (pal_key, alpha, label, is_status) in enumerate(tiles):
        bkg_rgba, text_rgba = get_tile_colors(palette, pal_key, alpha)
        if is_status:
            style_id = get_container_border_style(bkg_rgba, palette[border_key][TEXT])
        else:
            style_id = add_container_style(bkg_rgba=bkg_rgba)
        update_widget(cont_ids[i], ContainerParam.StyleId, style_id)
        update_widget_params(text_ids[i], {
            TextParam.ColorRgba: text_rgba,
            TextParam.Content: label,
        })


def build_tiles(palette: dict, tiles: list, cont_ids: dict, text_ids: dict, border_key: str):
    """Create tile containers inside a Row context (call only during GUI construction)."""
    for i, (pal_key, alpha, label, is_status) in enumerate(tiles):
        bkg_rgba, text_rgba = get_tile_colors(palette, pal_key, alpha)
        if is_status:
            style_id = get_container_border_style(bkg_rgba, palette[border_key][TEXT])
        else:
            style_id = add_container_style(bkg_rgba=bkg_rgba)
        with Container(width=120, height=60, align_center=True, style_id=style_id) as _cont_id:
            cont_ids[i] = _cont_id
            text_ids[i] = add_text(content=label, color_rgba=text_rgba, size=14)


# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------
# Checkbox has no Warning variant
std_colors = ["Primary", "Secondary", "Success", "Danger"]
std_styles = ["Checkbox States", "Background"]

# Maps std_colors radio index → StdColorStyle enum (must match std_colors order)
std_color_enums = [
    StdColorStyle.Primary,
    StdColorStyle.Secondary,
    StdColorStyle.Success,
    StdColorStyle.Danger,
]

cont_ids = {}       # {index: cont_id}
text_ids = {}       # {index: text_id}
bkg_cont_ids = {}   # {index: cont_id}
bkg_text_ids = {}   # {index: text_id}
state = {"selected_style": "",
         "selected_theme": "TokyoNight",
         "palette": get_styling_palette("TokyoNight", StdColorStyle.Primary)}

# ---------------------------------------------------------------------------
# Tile definitions — change only these three lists when porting to a new widget.
# Format: (palette_key, alpha_mult, display_label, is_status)
# is_status=True adds a border indicating this palette slot drives a widget state.
# ---------------------------------------------------------------------------

# Standard colour swatch tiles (Primary / Secondary / Success / Danger).
# checkbox::primary uses base (checked active) and strong (checked hovered).
palette_tiles = [
    ("base",   1.0, "base\nChecked Active",      True),
    ("weak",   1.0, "weak",                      False),
    ("strong", 1.0, "strong\nChecked Hovered",   True),
    ("base",   0.5, "base\nalpha_0.5\nDisabled", True),
]

# Background palette tiles annotated with checkbox status usage.
checkbox_state_tiles = [
    ("bkg_base",     1.0, "bkg_base\nUnchecked Active",          True),
    ("bkg_weak",     1.0, "bkg_weak\nUnchecked Hovered",         True),
    ("bkg_weaker",   1.0, "bkg_weaker\nUnchecked Disabled",      True),
    ("bkg_weakest",  1.0, "bkg_weakest",                         False),
    ("bkg_neutral",  1.0, "bkg_neutral",                         False),
    ("bkg_strong",   1.0, "bkg_strong\nBorder/Checked Disabled", True),
    ("bkg_stronger", 1.0, "bkg_stronger",                        False),
    ("bkg_strongest",1.0, "bkg_strongest",                       False),
    ("bkg_weakest",  0.5, "bkg_weakest\nalpha_0.5",              False),
]

# Full background palette tiles (unannotated — same across all widgets)
bkg_tiles = [
    ("bkg_base",     1.0, "bkg_base",               False),
    ("bkg_weak",     1.0, "bkg_weak",               False),
    ("bkg_weaker",   1.0, "bkg_weaker",             False),
    ("bkg_weakest",  1.0, "bkg_weakest",            False),
    ("bkg_neutral",  1.0, "bkg_neutral",            False),
    ("bkg_strong",   1.0, "bkg_strong",             False),
    ("bkg_stronger", 1.0, "bkg_stronger",           False),
    ("bkg_strongest",1.0, "bkg_strongest",          False),
    ("bkg_weakest",  0.5, "bkg_weakest\nalpha_0.5", False),
]

# Indexed by std_styles radio selection order
std_style_tiles = [checkbox_state_tiles, bkg_tiles]


# ---------------------------------------------------------------------------
# GUI — initial display uses PRIMARY
# ---------------------------------------------------------------------------
with Window(title="Checkbox Palette", size=(700, 900), center=True) as wnd_id:

    with Column(spacing=20, padding=[20], wrap=True):

        add_pick_list(options=window_theme_names(), selected="TokyoNight",
                      placeholder="Select Theme", on_select=on_theme_select)

        add_text(content="Select a standard colour to see the swatch palette it uses.\n"
                 "Checkbox has Primary, Secondary, Success and Danger (no Warning).\n"
                 "Border-highlighted tiles are the ones that drive a checkbox status.")

        rd_id_colors = add_radio(
            labels=std_colors,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            radio_wrap=True,
            on_selected=std_colors_selected,
        )

        add_text(content="Swatch palette — drives checked states.\n"
                 "This palette is common across widgets.")

        with Row(spacing=20, wrap=True):
            build_tiles(state["palette"], palette_tiles, cont_ids, text_ids, "bkg_stronger")

        add_text(content="Background palette — drives unchecked states and borders.\n"
                 "'Checkbox States' labels which bkg keys map to a status.\n"
                 "'Background' shows the full palette without annotations.")

        rd_id_styles = add_radio(
            labels=std_styles,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            radio_wrap=True,
            on_selected=std_styles_selected,
        )

        with Row(spacing=20, wrap=True):
            build_tiles(state["palette"], checkbox_state_tiles,
                        bkg_cont_ids, bkg_text_ids, "bkg_strongest")

        add_text(content="Checkboxes using the standard colour styles:")

        with Row(spacing=10, wrap=True):
            for (label, style) in [
                ("Primary",   CheckboxStyleStd.Primary),
                ("Secondary", CheckboxStyleStd.Secondary),
                ("Success",   CheckboxStyleStd.Success),
                ("Danger",    CheckboxStyleStd.Danger),
            ]:
                add_checkbox(label=label, is_checked=False, style_std=style)

        with Row(spacing=10, wrap=True):
            for (label, style) in [
                ("Primary",   CheckboxStyleStd.Primary),
                ("Secondary", CheckboxStyleStd.Secondary),
                ("Success",   CheckboxStyleStd.Success),
                ("Danger",    CheckboxStyleStd.Danger),
            ]:
                add_checkbox(label=label, is_checked=True, style_std=style)

start_session()
