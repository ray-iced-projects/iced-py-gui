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

import os
from icedpygui import (
    Window,
    WindowTheme,
    add_container,
    add_container_style,
    Column,
    Scrollable,
    Row,
    start_session,
    add_checkbox,
    add_text,
    add_font_style,
    FontWeight,
    custom_palette,
    get_color_palette,
    PaletteKey,
    WidgetStatus,
    StylePart,
    add_text_editor,
    add_text_editor_style,
)


def make_tiles(status: tuple[WidgetStatus, list[list[StylePart, PaletteKey, float]]],
               col_pal: dict):
    """Make the tiles"""
    # (WidgetStatus, (StylePart, PaletteKey, alpha))

    for( part, pal_key, alpha) in status[1]:
        (rgba, text_rgba)= col_pal.get(pal_key)
        c = "Color" if alpha == 1.0 else f"Color Alpha {alpha}"
        t = "Text" if alpha == 1.0 else f"Text Alpha {alpha}"
        rgba[3] *= alpha
        text_rgba[3] *= alpha
        match part:
            case StylePart.Border:
                content = f"Border\n{c}"
                add_text(content=content, width=150)
                style_id = add_container_style(bkg_rgba=rgba)
                add_container(width=120, height=20, style_id=style_id)
            case StylePart.Base:
                content = f"Base\n{c}"
                add_text(content=content, width=150)
                style_id = add_container_style(bkg_rgba=rgba)
                add_container(width=120, height=20, style_id=style_id)
            case StylePart.Icon:
                content = f"Icon\n{c}"
                add_text(content=content, width=150)
                style_id = add_container_style(bkg_rgba=rgba)
                add_container(width=120, height=20, style_id=style_id)
            case StylePart.Accent:
                content = f"Accent\n{c}"
                add_text(content=content, width=150)
                style_id = add_container_style(bkg_rgba=rgba)
                add_container(width=120, height=20, style_id=style_id)
            case StylePart.Text:
                add_text(content=t)
                style_id = add_container_style(bkg_rgba=text_rgba)
                add_container(width=120, height=20, style_id=style_id)


# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------

# When creating a palette for the checkbox, you can only use
# StyleParts listed below.  The other StyleParts seen in the hint
# belong to other widgets with overlap.
# You don't need to use all parts but it's advisable to use
# all statuses for each part.
# Border will ony show when the border width > 0 but it's default
# for the Checkbox is 1.0, so generally no issue.
pal = [
    (WidgetStatus.Active, (
                            (StylePart.Border, PaletteKey.Strong, 1.0),
                            (StylePart.Base,   PaletteKey.Base,   1.0),
                            (StylePart.Icon,   PaletteKey.Base,   1.0),
                            (StylePart.Accent, PaletteKey.Base,   1.0),
                            (StylePart.Text,   PaletteKey.Base,   1.0),
                            ),
    ),
    (WidgetStatus.Hovered, (
                            (StylePart.Border, PaletteKey.Strong, 1.0),
                            (StylePart.Base,   PaletteKey.Weak,   1.0),
                            (StylePart.Icon,   PaletteKey.Base,   1.0),
                            (StylePart.Accent, PaletteKey.Strong, 1.0),
                            (StylePart.Text,   PaletteKey.Base,   1.0),
                            ),
    ),
    (WidgetStatus.Disabled, (
                            (StylePart.Border, PaletteKey.Weak,   1.0),
                            (StylePart.Base,   PaletteKey.Weaker, 1.0),
                            (StylePart.Icon,   PaletteKey.Base,   1.0),
                            (StylePart.Accent, PaletteKey.Strong, 1.0),
                            (StylePart.Text,   PaletteKey.Base,   1.0),
                            ),
    ),
]

new_color = [0.32, 0.2, 0.13, 1.0]
pal_id = custom_palette(rgba=new_color, statuses=pal)

# This demo will need the colors for the containers, normally
# only the above custom_palette() would be used.
color_pal = get_color_palette(rgba=new_color)

font_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)


cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/py_checkbox/py_checkbox_text.txt"
state = {"file": ""}

try:
    with open(FILE_PATH, "r", encoding='utf-8') as file:
        state["file"] = file.read()
except FileNotFoundError:
    print(f"*********The file does not exist using {FILE_PATH}.*******")

# ---------------------------------------------------------------------------
# GUI — Initial display with a TokyoNight background (Selected by PickList)
# ---------------------------------------------------------------------------
with Window(title="Button Custom Palette",
            size=(1100, 850), center=True, theme=WindowTheme.TokyoNight) as wnd_id:
    with Scrollable(height=800):
        with Column(spacing=10, padding=[20], width_fill=True):
            with Scrollable(height=300):
                txt_ed_style_id = add_text_editor_style(border_width=0)
                add_text_editor(content=state["file"],
                                width_fill=True,
                                style_id=txt_ed_style_id)

            with Column(spacing=10, width_fill=True):
                add_text(content=(
                    ""))

                add_text(content="The Checkbox statuses: Active, Hovered, and Disabled")

                with Column(spacing=20, width_fill=True, height_fill=True):
                    add_text(content="******Custom Status Styling******")

                    with Row(spacing=20, width_fill=True):
                        with Column(spacing=5):
                            add_text(content="Status: Active",
                                    size=20, font_id=font_id)
                            make_tiles(pal[0], color_pal)

                        with Column(spacing=5):
                            add_text(content="Status: Hovered",
                                        size=20, font_id=font_id)
                            make_tiles(pal[1], color_pal)

                        with Column(spacing=5):
                            add_text(content="Status: Disabled",
                                        size=20, font_id=font_id)
                            make_tiles(pal[2], color_pal)

                    with Row(spacing=20):
                        with Column(spacing=5):
                            add_checkbox(label="Custom Palette", palette_id=pal_id)
                            add_checkbox(label="Custom Palette Disabled",
                                        palette_id=pal_id, disabled=True)
                        with Column(spacing=5):
                            add_checkbox(label="Default Palette")
                            add_checkbox(label="Default Palette Disabled",
                                        disabled=True)
start_session()
