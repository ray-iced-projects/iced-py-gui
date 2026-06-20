#!/usr/bin/env python3
"""
Button styling demo — shows the color palette for standard button themes.
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
    PaletteKey,
    WidgetStatus,
)


def get_container_border_style(_bkg_rgba: list, color_rgba: list) -> int:
    """Get the container style with a border"""
    return add_container_style(
            bkg_rgba=_bkg_rgba,
            border_rgba=color_rgba,
            border_width=2.0)


# Maps PaletteKey enum → get_color_palette dict key prefix.
# PaletteKey.BaseAlpha is a special case: uses "base" with alpha value.
PALETTE_KEY_TO_STR = {
    PaletteKey.Base:      "base",
    PaletteKey.Weak:      "weak",
    PaletteKey.Weaker:    "weaker",
    PaletteKey.Weakest:   "weakest",
    PaletteKey.Neutral:   "neutral",
    PaletteKey.Strong:    "strong",
    PaletteKey.Stronger:  "stronger",
    PaletteKey.Strongest: "strongest",
    # Alpha variants use same base colour with scaled alpha
    PaletteKey.BaseAlpha:     "base",
    PaletteKey.WeakAlpha:     "weak",
    PaletteKey.WeakerAlpha:   "weaker",
    PaletteKey.WeakestAlpha:  "weakest",
    PaletteKey.NeutralAlpha:  "neutral",
    PaletteKey.StrongAlpha:   "strong",
    PaletteKey.StrongerAlpha: "stronger",
    PaletteKey.StrongestAlpha:"strongest",
}

# Alpha variants — these keys get their alpha scaled by the tile's alpha value
ALPHA_KEYS = {
    PaletteKey.BaseAlpha, PaletteKey.WeakAlpha, PaletteKey.WeakerAlpha,
    PaletteKey.WeakestAlpha, PaletteKey.NeutralAlpha, PaletteKey.StrongAlpha,
    PaletteKey.StrongerAlpha, PaletteKey.StrongestAlpha,
}


def get_tile_colors(_palette: dict, key: PaletteKey, _alpha: float) -> tuple:
    """Return (bkg_rgba, text_rgba) for a PaletteKey with optional alpha scaling."""
    prefix = PALETTE_KEY_TO_STR[key]
    _bkg_rgba  = list(_palette[prefix + "_color"])
    _text_rgba = list(_palette[prefix + "_text"])
    if _alpha != 1.0 or key in ALPHA_KEYS:
        _bkg_rgba[3]  *= _alpha
        _text_rgba[3] *= _alpha
    return _bkg_rgba, _text_rgba


def label_for(_pal_key: PaletteKey, _widget_status: WidgetStatus) -> str:
    """Build a tile label from its PaletteKey and optional WidgetStatus."""
    key_name = PALETTE_KEY_TO_STR[_pal_key]
    if key_name != PALETTE_KEY_TO_STR.get(_pal_key, key_name):
        key_name = str(_pal_key).rsplit('.', maxsplit=1)[-1]  # fallback
    suffix = f"\n{str(_widget_status).rsplit('.', maxsplit=1)[-1]}" \
        if _widget_status is not None else ""
    return key_name + suffix


# ---------------------------------------------------------------------------
# Standard statuses: default button palette mapping
#   (PaletteKey, alpha, WidgetStatus or None, is_status)
# ---------------------------------------------------------------------------
std_tiles = [
    (PaletteKey.Base,      1.0, WidgetStatus.Active,   True),
    (PaletteKey.Base,      1.0, WidgetStatus.Pressed,  True),
    (PaletteKey.Strong,    1.0, WidgetStatus.Hovered,  True),
    (PaletteKey.BaseAlpha, 0.5, WidgetStatus.Disabled, True),
    (PaletteKey.Weak,      1.0, None,                  False),
    (PaletteKey.Weaker,    1.0, None,                  False),
    (PaletteKey.Weakest,   1.0, None,                  False),
    (PaletteKey.Neutral,   1.0, None,                  False),
    (PaletteKey.Stronger,  1.0, None,                  False),
    (PaletteKey.Strongest, 1.0, None,                  False),
]

# ---------------------------------------------------------------------------
# Custom statuses: mirrors the `statuses` list passed to custom_palette()
# ---------------------------------------------------------------------------
custom_tiles = [
    (PaletteKey.Weakest,  1.0, WidgetStatus.Active,   True),
    (PaletteKey.Neutral,  1.0, WidgetStatus.Pressed,  True),
    (PaletteKey.Strongest,1.0, WidgetStatus.Hovered,  True),
    (PaletteKey.BaseAlpha,0.5, WidgetStatus.Disabled, True),
    (PaletteKey.Base,     1.0, None,                  False),
    (PaletteKey.Weak,     1.0, None,                  False),
    (PaletteKey.Weaker,   1.0, None,                  False),
    (PaletteKey.Strong,   1.0, None,                  False),
    (PaletteKey.Stronger, 1.0, None,                  False),
]

selected_color = [0.32, 0.2, 0.13, 1.0]
# Just uses a new background color for the button, no status changes
pal_id = custom_palette(rgba=selected_color)

# The statuses below are what's needed for the real Button
# The ones above are similar but are used just for styling the
# containers since they are really only background colors.
statuses = [
    (PaletteKey.Weakest,  WidgetStatus.Active),
    (PaletteKey.Neutral,  WidgetStatus.Pressed),
    (PaletteKey.Strongest,WidgetStatus.Hovered),
    (PaletteKey.BaseAlpha,WidgetStatus.Disabled),
]

custom_pal_id = custom_palette(rgba=selected_color, statuses=statuses)

palette = get_color_palette(rgba=selected_color)
border_color = palette["strongest_color"]

# ---------------------------------------------------------------------------
# GUI
# ---------------------------------------------------------------------------
with Window(title="Button Styling", size=(700, 650), center=True) as wnd_id:

    with Column(spacing=10, padding=[20], wrap=True):

        add_text(content=(
            "Standard palette statuses — border-highlighted tiles indicate a button state.\n"
            "The button only changes it's background for each status."
        ))

        with Row(spacing=20, wrap=True):
            for (pal_key, alpha, widget_status, is_status) in std_tiles:
                bkg_rgba, text_rgba = get_tile_colors(palette, pal_key, alpha)
                style_id = (get_container_border_style(bkg_rgba, border_color)
                            if is_status else add_container_style(bkg_rgba=bkg_rgba))

                with Container(width=120, height=40, align_center=True, style_id=style_id):
                    add_text(content=label_for(pal_key, widget_status),
                             color_rgba=text_rgba, size=14)

        add_button(label="Button with standard palette", padding=[10], palette_id=pal_id)

        add_text(content=(
            "Custom palette statuses — the same as bove with statuses changed."
        ))

        with Row(spacing=20, wrap=True):
            for (pal_key, alpha, widget_status, is_status) in custom_tiles:
                bkg_rgba, text_rgba = get_tile_colors(palette, pal_key, alpha)
                style_id = (get_container_border_style(bkg_rgba, border_color)
                            if is_status else add_container_style(bkg_rgba=bkg_rgba))

                with Container(width=120, height=40, align_center=True, style_id=style_id):
                    add_text(content=label_for(pal_key, widget_status),
                             color_rgba=text_rgba, size=14)

        with Row(spacing=10):
            add_button(label="Button with custom statuses", padding=[10], palette_id=custom_pal_id)
            add_button(label="Button default (no palette mods)", padding=[10])

        with Row(spacing=10):
            add_button(label="Button with disabled custom statuses",
                       padding=[10], palette_id=custom_pal_id, disabled=True)
            add_button(label="Button disabled (no palette mods)", padding=[10], disabled=True)

start_session()
