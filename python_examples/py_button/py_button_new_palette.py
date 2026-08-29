#!/usr/bin/env python3
"""
Button creating a new palette.
"""
import os
from icedpygui import (
    Window,
    WindowTheme,
    Column,
    Scrollable,
    Row,
    start_session,
    add_button,
    add_button_style,
    add_pick_list,
    add_text,
    custom_palette,
    get_color_palette,
    PaletteKey,
    WidgetStatus,
    StateVariant,
    StylePart,
    add_text_editor,
    add_text_editor_style,
    update_widget,
    WindowParam,
    window_theme_names,
)

new_color = [0.32, 0.2, 0.13, 1.0]

color_pal = get_color_palette(rgba=new_color)

# See the py_button_text for an explanation or run this example to see.
pal = [
    ((WidgetStatus.Active, StateVariant.NoVariant), (
                            (StylePart.Background, PaletteKey.Base, 1.0),
                            (StylePart.Text, PaletteKey.BaseText, 0.8),
                            (StylePart.Border, PaletteKey.Stronger, 1.0),
                            ),
    ),
    ((WidgetStatus.Hovered, StateVariant.NoVariant), (
                            (StylePart.Background, PaletteKey.Strong, 1.0),
                            (StylePart.Text, PaletteKey.StrongText, 0.8),
                            (StylePart.Border, PaletteKey.Strongest, 1.0),
                            ),
    ),
    ((WidgetStatus.Pressed, StateVariant.NoVariant), (
                            (StylePart.Background, PaletteKey.Base, 1.0),
                            (StylePart.Text, PaletteKey.BaseText, 0.8),
                            (StylePart.Border, PaletteKey.Stronger, 1.0),
                            ),
    ),
    ((WidgetStatus.Disabled, StateVariant.NoVariant), (
                            (StylePart.Background, PaletteKey.Base, 0.5),
                            (StylePart.Text, PaletteKey.BaseText, 0.5),
                            (StylePart.Border, PaletteKey.Stronger, 0.5),
                            ),
    ),
]

def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    update_widget(wnd_id, WindowParam.Theme, theme_name)

pal_id = custom_palette(rgba=new_color, statuses=pal)

btn_style_id = add_button_style(border_width=3)

cwd = os.getcwd()
FILE_PATH = os.path.join(cwd, "python_examples", "py_button", "py_button_text.txt")
state = {"file": ""}

try:
    with open(FILE_PATH, "r", encoding='utf-8') as file:
        state["file"] = file.read()
except FileNotFoundError:
    print(f"*********The file does not exist using {FILE_PATH}.*******")

# ---------------------------------------------------------------------------
# GUI — Display with a TokyoNight background
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

        with Column(spacing=20, width_fill=True, padding=[0, 0, 0, 20]):
            add_text(content=("***Select a theme to see how the buttons look\n"
                              "***with a different background."))

            add_pick_list(options=window_theme_names(), selected="TokyoNight",
                                placeholder="Select Theme", on_select=on_theme_select)

            add_text(content="***Buttons where the Status is locked***")

            with Row(spacing=20, width_fill=True):
                add_button(label="Status: Active", active=True,
                           palette_id=pal_id, padding=[10])

                add_button(label="Status: Hovered", hovered=True,
                           palette_id=pal_id, padding=[10])

                add_button(label="Status: Pressed", pressed=True,
                           palette_id=pal_id, padding=[10])

                add_button(label="Status: Disabled", disabled=True,
                           palette_id=pal_id, padding=[10])


            add_button(label="Custom Button Palette", padding=[10],
                        palette_id=pal_id)

            add_button(label="Custom Button Palette with border", padding=[10],
                                    palette_id=pal_id, style_id=btn_style_id)


start_session()
