#!/usr/bin/env python3
"""
Button styling demo — shows the colour palette for standard button themes,
with live radio-button switching between colour themes.
"""

from icedpygui import (
    Window,
    Container,
    add_container_style,
    Column,
    Row,
    start_session,
    add_button,
    add_text,
    get_color_palette,
    custom_palette,
)


def get_container_border_style(_bkg_rgba: list, color_rgba: list) -> int:
    """Get the container style with a border"""
    return add_container_style(
            bkg_rgba=_bkg_rgba,
            border_rgba=color_rgba,
            border_width=2.0)


# Index into (bkg_rgba, text_rgba) palette tuples
TEXT = 1

def get_tile_colors(palette: dict, key: str, _alpha: float) -> tuple:
    """Return copies of (bkg_rgba, text_rgba) with optional alpha scaling."""
    if key == "base_disabled":
        bkg = "base_color"
        text = "base_text"
    else:
        bkg = key + "_color"
        text = key + "_text"

    _bkg_rgba = palette[bkg]
    _text_rgba = palette[text]

    if alpha != 1.0:
        _bkg_rgba[3] *= _alpha
        _text_rgba[3] *= _alpha
    return _bkg_rgba, _text_rgba


# ---------------------------------------------------------------------------
# Create data
# ---------------------------------------------------------------------------

# Background palette tiles for Subtle style
tiles = [
    ("base",            1.0, "base\nActive/Pressed",        True),
    ("base_disabled",    0.5, "base\nalpha_0.5\nDisabled",   True),
    ("weak",            1.0, "weak",                        False),
    ("weaker",          1.0, "weaker",                      False),
    ("weakest",         1.0, "weakest",                     False),
    ("neutral",         1.0, "neutral",                     False),
    ("strong",          1.0, "strong\nHovered",             True),
    ("stronger",        1.0, "stronger",                    False),
    ("strongest",       1.0, "strongest",                   False),
]

selected_color = [0.32, 0.2, 0.13, 1.0]
pal_id = custom_palette(rgba=selected_color)

state = {"palette": get_color_palette(rgba=selected_color),
         "tiles": tiles}

# ---------------------------------------------------------------------------
# GUI — initial display uses PRIMARY palette with a TokyoNight background
# ---------------------------------------------------------------------------
with Window(title="Button Styling", size=(700, 800), center=True) as wnd_id:

    with Column(spacing=20, padding=[20], wrap=True):

        add_text(content="This is the default Button palette and statuses, border highlighted ones\n" +
                 "You may change the status palette color by defining them, see further below")

        with Row(spacing=20, wrap=True):
            for i, (pal_key, alpha, label, is_status) in enumerate(state["tiles"]):
                bkg_rgba, text_rgba = get_tile_colors(state["palette"], pal_key, alpha)
                if is_status:
                    style_id = get_container_border_style(bkg_rgba, state["palette"]["strongest_text"])
                else:
                    style_id = add_container_style(bkg_rgba=bkg_rgba)
                with Container(width=120, height=60, align_center=True, style_id=style_id):
                    add_text(content=label, color_rgba=text_rgba, size=14)


        add_button(label="Button with custom palette", padding=[20], palette_id=pal_id)

start_session()
