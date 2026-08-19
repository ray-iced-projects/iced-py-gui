#!/usr/bin/env python3
"""
Button creating a new palette.
"""

from icedpygui import (
    Window,
    WindowTheme,
    Container,
    add_container_style,
    Column,
    start_session,
    get_color_palette,
    add_pick_list,
    add_text,
    update_widget,
    window_theme_names,
    WindowParam,
    PaletteKey)

def on_theme_select(_pl_id: int, theme: str):
    """Selcting a new theme"""
    update_widget(wnd_id, WindowParam.Theme, theme)


def get_text_color(key_: PaletteKey, palette: dict):
    """Get the text color for a given palette key"""
    key_name = str(key_).rsplit(".", maxsplit=1)[-1]  # Extract just the name
    text_key_name = f"{key_name}Text"
    try:
        text_key = getattr(PaletteKey, text_key_name)
        return palette.get(text_key, [0, 0, 0, 1])  # Default to black if not found
    except AttributeError:
        return [0, 0, 0, 1]  # Default fallback


color_pal = get_color_palette(rgba=[0.32, 0.2, 0.13, 1.0])

# ---------------------------------------------------------------------------
# GUI — Display with a TokyoNight background
# ---------------------------------------------------------------------------
with Window(title="Color Palette",
            size=(1100, 850), center=True,
            theme=WindowTheme.TokyoNight) as wnd_id:
    with Container(align_center=True, fill=True):
        with Column(spacing=10):
            add_pick_list(options=window_theme_names(), selected="TokyoNight",
                                            placeholder="Select Theme", on_select=on_theme_select)
            for (key, color) in sorted(color_pal.items(), key=lambda x: str(x[0])):
                if "Text" not in str(key):
                    cnt_st_id = add_container_style(bkg_rgba=color)
                    text_color = get_text_color(key, color_pal)
                    with Container(width=300, height=50, align_center=True, style_id=cnt_st_id):
                        add_text(content=str(key), color_rgba=text_color)

start_session()
