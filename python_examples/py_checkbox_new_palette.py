#!/usr/bin/env python3
"""
Checkbox palette demo — shows the colour palette for checkbox themes.

Checkbox statuses: Active {is_checked}, Hovered {is_checked}, Disabled {is_checked}
Default palette mapping:
  Base      → Active   (unchecked background)
  Weak      → Hovered  (unchecked hovered background)
  Weaker    → Disabled (unchecked disabled background)
  BaseAlpha → IsChecked (checked accent colour, alpha-dimmed for disabled)
"""

from icedpygui import (
    Window,
    Container,
    add_container_style,
    Column,
    Row,
    start_session,
    add_checkbox,
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
PALETTE_KEY_TO_STR = {
    PaletteKey.Base:      "base",
    PaletteKey.Weak:      "weak",
    PaletteKey.Weaker:    "weaker",
    PaletteKey.Weakest:   "weakest",
    PaletteKey.Neutral:   "neutral",
    PaletteKey.Strong:    "strong",
    PaletteKey.Stronger:  "stronger",
    PaletteKey.Strongest: "strongest",
    PaletteKey.BaseAlpha:     "base",
    PaletteKey.WeakAlpha:     "weak",
    PaletteKey.WeakerAlpha:   "weaker",
    PaletteKey.WeakestAlpha:  "weakest",
    PaletteKey.NeutralAlpha:  "neutral",
    PaletteKey.StrongAlpha:   "strong",
    PaletteKey.StrongerAlpha: "stronger",
    PaletteKey.StrongestAlpha:"strongest",
}

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


def label_for(_pal_key: PaletteKey, _widget_status) -> str:
    """Build a tile label from its PaletteKey and optional WidgetStatus."""
    key_name = PALETTE_KEY_TO_STR[_pal_key]
    suffix = f"\n{str(_widget_status).rsplit('.', maxsplit=1)[-1]}" \
        if _widget_status is not None else ""
    return key_name + suffix


# ---------------------------------------------------------------------------
# Standard statuses: default checkbox palette mapping
#   (PaletteKey, alpha, WidgetStatus or None, is_status)
# Checkbox has no Pressed status. IsChecked drives the checked accent colour.
# ---------------------------------------------------------------------------
std_tiles = [
    (PaletteKey.Base,      1.0, WidgetStatus.Active,    True),
    (PaletteKey.Weak,      1.0, WidgetStatus.Hovered,   True),
    (PaletteKey.Weaker,    1.0, WidgetStatus.Disabled,  True),
    (PaletteKey.BaseAlpha, 0.5, WidgetStatus.IsChecked, True),
    (PaletteKey.Neutral,   1.0, None,                   False),
    (PaletteKey.Strong,    1.0, None,                   False),
    (PaletteKey.Stronger,  1.0, None,                   False),
    (PaletteKey.Weakest,   1.0, None,                   False),
    (PaletteKey.Strongest, 1.0, None,                   False),
]

# ---------------------------------------------------------------------------
# Custom statuses: mirrors the `statuses` list passed to custom_palette()
# ---------------------------------------------------------------------------
statuses = [
    (PaletteKey.Weakest,  WidgetStatus.Active),
    (PaletteKey.Neutral,  WidgetStatus.Hovered),
    (PaletteKey.Strongest,WidgetStatus.Disabled),
    (PaletteKey.Strong,   WidgetStatus.IsChecked),
]

custom_tiles = [
    (PaletteKey.Weakest,  1.0, WidgetStatus.Active,    True),
    (PaletteKey.Neutral,  1.0, WidgetStatus.Hovered,   True),
    (PaletteKey.Strongest,1.0, WidgetStatus.Disabled,  True),
    (PaletteKey.Strong,   1.0, WidgetStatus.IsChecked, True),
    (PaletteKey.Base,     1.0, None,                   False),
    (PaletteKey.Weak,     1.0, None,                   False),
    (PaletteKey.Weaker,   1.0, None,                   False),
    (PaletteKey.Stronger, 1.0, None,                   False),
    (PaletteKey.Weakest,  1.0, None,                   False),
]

selected_color = [0.32, 0.2, 0.13, 1.0]
pal_id        = custom_palette(rgba=selected_color)
custom_pal_id = custom_palette(rgba=selected_color, statuses=statuses)

palette = get_color_palette(rgba=selected_color)
border_color = palette["strongest_color"]

# ---------------------------------------------------------------------------
# GUI
# ---------------------------------------------------------------------------
with Window(title="Checkbox Palette", size=(700, 800), center=True) as wnd_id:

    with Column(spacing=20, padding=[20], wrap=True):

        add_text(content=(
            "Standard checkbox palette statuses — border-highlighted tiles" +
            " indicate a checkbox state.\n"
            "Default: Base→Active, Weak→Hovered, Weaker→Disabled, BaseAlpha→IsChecked" +
            " (checked accent)"
        ))

        with Row(spacing=20, wrap=True):
            for (pal_key, alpha, widget_status, is_status) in std_tiles:
                bkg_rgba, text_rgba = get_tile_colors(palette, pal_key, alpha)
                style_id = (get_container_border_style(bkg_rgba, border_color)
                            if is_status else add_container_style(bkg_rgba=bkg_rgba))
                with Container(width=120, height=40, align_center=True, style_id=style_id):
                    add_text(content=label_for(pal_key, widget_status),
                             color_rgba=text_rgba, size=14)

        add_text(content="Checkboxes with standard palette (unchecked / checked):")
        with Row(spacing=20, wrap=True):
            add_checkbox(label="unchecked", is_checked=False, palette_id=pal_id)
            add_checkbox(label="checked",   is_checked=True,  palette_id=pal_id)

        add_text(content=(
            "Custom palette statuses — the same PaletteKey/WidgetStatus pairs\n"
            "passed to custom_palette(statuses=[...]) are shown as tiles."
        ))

        with Row(spacing=20, wrap=True):
            for (pal_key, alpha, widget_status, is_status) in custom_tiles:
                bkg_rgba, text_rgba = get_tile_colors(palette, pal_key, alpha)
                style_id = (get_container_border_style(bkg_rgba, border_color)
                            if is_status else add_container_style(bkg_rgba=bkg_rgba))
                with Container(width=120, height=40, align_center=True, style_id=style_id):
                    add_text(content=label_for(pal_key, widget_status),
                             color_rgba=text_rgba, size=14)

        add_text(content="Checkboxes with custom palette (unchecked / checked):")
        with Row(spacing=20, wrap=True):
            add_checkbox(label="unchecked", is_checked=False, palette_id=custom_pal_id)
            add_checkbox(label="checked",   is_checked=True,  palette_id=custom_pal_id)

        add_text(content="Checkboxes with no palette (theme default):")
        with Row(spacing=20, wrap=True):
            add_checkbox(label="unchecked", is_checked=False)
            add_checkbox(label="checked",   is_checked=True)

start_session()
