#!/usr/bin/env python3
"""
Button crating a new palette.
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
    add_button,
    add_button_style,
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
    # (WidgetStatus.Active, (PaletteKey.Base, 1.0))

    for( part, pal_key, alpha) in status[1]:
        (rgba, text_rgba)= col_pal.get(pal_key)
        c = "Color" if alpha == 1.0 else f"Color Alpha {alpha}"
        t = "Text" if alpha == 1.0 else f"Text Alpha {alpha}"
        rgba[3] *= alpha
        text_rgba[3] *= alpha
        match part:
            case StylePart.Background:
                content = f"Background\n{c}"
                add_text(content=content, width=150)
                style_id = add_container_style(bkg_rgba=rgba)
                add_container(width=120, height=20, style_id=style_id)
            case StylePart.Text:
                add_text(content=t)
                style_id = add_container_style(bkg_rgba=text_rgba)
                add_container(width=120, height=20, style_id=style_id)
            case StylePart.Border:
                content = f"Border\n{c}"
                add_text(content=content, width=150)
                style_id = add_container_style(bkg_rgba=rgba)
                add_container(width=120, height=20, style_id=style_id)

# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------

# Index into (bkg_rgba, text_rgba) palette tuples
TEXT = 1
COLOR = 0

# When creating a palette for the button, you can only use
# Background, Text, and Border.  The other StyleParts belong
# to other widgets with overlap.
# You don't need to use all 3 parts but it's advisable to use
# all statuses for each part.
# Border will ony show when the border width > 0
pal = [
    (WidgetStatus.Active, (
                            (StylePart.Background, PaletteKey.Base, 1.0),
                            (StylePart.Text, PaletteKey.Base, 1.0),
                            (StylePart.Border, PaletteKey.Stronger, 1.0),
                            ),
    ),
    (WidgetStatus.Hovered, (
                            (StylePart.Background, PaletteKey.Strong, 1.0),
                            (StylePart.Text, PaletteKey.Strong, 1.0),
                            (StylePart.Border, PaletteKey.Strongest, 1.0),
                            ),
    ),
    (WidgetStatus.Pressed, (
                            (StylePart.Background, PaletteKey.Base, 1.0),
                            (StylePart.Text, PaletteKey.Base, 1.0),
                            (StylePart.Border, PaletteKey.Stronger, 1.0),
                            ),
    ),
    (WidgetStatus.Disabled, (
                            (StylePart.Background, PaletteKey.Base, 0.5),
                            (StylePart.Text, PaletteKey.Base, 0.5),
                            (StylePart.Border, PaletteKey.Stronger, 0.5),
                            ),
    ),
]

new_color = [0.32, 0.2, 0.13, 1.0]
pal_id = custom_palette(rgba=new_color, statuses=pal)

# This demo will need the colors for the containers, normally
# only the above custom_palette() would be used.
color_pal = get_color_palette(rgba=new_color)

font_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)
btn_style_id = add_button_style(border_width=3)

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/py_button/py_button_text.txt"
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

                add_text(content=("The Button statuses: Active (base), Hovered (strong), "
                                    "Disabled(base alpha 0.5)"))

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
                            add_text(content="Status: Pressed",
                                        size=20, font_id=font_id)
                            make_tiles(pal[2], color_pal)

                        with Column(spacing=5):
                            add_text(content="Status: Disabled",
                                        size=20, font_id=font_id)
                            make_tiles(pal[3], color_pal)

                    with Row(spacing=20):
                        with Column(spacing=5):
                            add_button(label="Custom Palette", padding=[10],
                                    palette_id=pal_id, style_id=btn_style_id)
                            add_button(label="Custom Palette Disabled", padding=[10],
                                        palette_id=pal_id, disabled=True)
                        with Column(spacing=5):
                            add_button(label="Default Palette", padding=[10])
                            add_button(label="Default Palette Disabled", padding=[10],
                                        disabled=True)
start_session()
