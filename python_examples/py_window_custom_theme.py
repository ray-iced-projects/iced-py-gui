#!/usr/bin/env python3
"""
Window use demo

Adds a window to the application.

"""
from pathlib import Path
from icedpygui import (
    Window,
    WindowParam,
    Column,
    Container,
    ContainerParam,
    Row,
    Scrollable,
    start_session,
    add_container_style,
    add_icon,
    Arrow,
    add_pick_list,
    add_text_editor,
    window_theme_names,
    update_widget,
    update_widget_params,
    create_custom_theme,
    add_text,
    TextParam,
    get_styling_palette,
    StdColorStyle,
    )

TEXT = 1  # index into (bkg_rgba, text_rgba) palette tuples

# Tile id storage — populated during GUI construction, used in update_tiles()
std_cont_ids = []   # flat list: [color0_base, color0_weak, color0_strong, color1_base, ...]
std_text_ids = []
bkg_cont_ids = []
bkg_text_ids = []

state = {"theme": "MyTheme"}


def update_std_tiles(theme_name: str):
    """Refresh the standard-colour tile containers for a new theme."""
    i = 0
    for _std_color in std_colors:
        _palette = get_styling_palette(theme_name, _std_color)
        for _key in theme_std_colors:
            _bkg_rgba = list(_palette[_key][0])
            _text_rgba = list(_palette[_key][TEXT])
            _style_id = add_container_style(bkg_rgba=_bkg_rgba)
            update_widget(std_cont_ids[i], ContainerParam.StyleId, _style_id)
            update_widget_params(std_text_ids[i], {TextParam.ColorRgba: _text_rgba})
            i += 1


def update_bkg_tiles(theme_name: str):
    """Refresh the background palette tile containers for a new theme."""
    _palette = get_styling_palette(theme_name, StdColorStyle.Primary)
    for i, _key in enumerate(theme_bkg):
        _bkg_rgba = list(_palette[_key][0])
        _text_rgba = list(_palette[_key][TEXT])
        _style_id = add_container_style(bkg_rgba=_bkg_rgba)
        update_widget(bkg_cont_ids[i], ContainerParam.StyleId, _style_id)
        update_widget_params(bkg_text_ids[i], {TextParam.ColorRgba: _text_rgba})


def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    state["theme"] = theme_name
    update_widget(wnd1_id, WindowParam.Theme, theme_name)
    update_widget(wnd2_id, WindowParam.Theme, theme_name)
    update_std_tiles(theme_name)
    update_bkg_tiles(theme_name)


create_custom_theme(
    name="MyTheme",
    background=[0.08, 0.08, 0.12, 1.0],
    text=[0.9, 0.9, 0.9, 1.0],
    primary=[0.4, 0.6, 1.0, 1.0],
    success=[0.3, 0.8, 0.4, 1.0],
    warning=[1.0, 0.7, 0.2, 1.0],
    danger=[0.9, 0.3, 0.3, 1.0],
)

create_custom_theme(
    name="MyBrownTheme",
    background=[0.15, 0.10, 0.07, 1.0],   # dark espresso
    text=     [0.93, 0.87, 0.78, 1.0],    # warm cream
    primary=  [0.72, 0.45, 0.20, 1.0],    # saddle brown
    success=  [0.45, 0.60, 0.25, 1.0],    # muted olive green
    warning=  [0.85, 0.60, 0.15, 1.0],    # amber
    danger=   [0.75, 0.25, 0.18, 1.0],    # terracotta
)

std_colors = [
    StdColorStyle.Primary,
    StdColorStyle.Success,
    StdColorStyle.Warning,
    StdColorStyle.Danger,
]

color_strings = ["Primary", "Success", "Warning", "Danger"]

theme_std_colors = ["base", "weak", "strong"]
theme_bkg = [
    "bkg_base",
    "bkg_weak",
    "bkg_weaker",
    "bkg_weakest",
    "bkg_neutral",
    "bkg_strong",
    "bkg_stronger",
    "bkg_strongest",
]

def get_instructions():
    """Loading instructions for header"""
    with open(Path(__file__).with_suffix(".txt"), encoding="utf-8") as file:
        return file.read()

icon_open = add_icon(arrow=Arrow.CaretDownFill)
icon_closed = add_icon(arrow=Arrow.CaretLeftFill)

with Window(
    title="Window Custom Theme",
    size=(550.0, 500.0),
    position=(1100, 300),
    theme_name="MyTheme") as wnd1_id:

    # add a container to center things
    with Container(fill=True):
        with Column(spacing=20, padding=[20], wrap=True):
            add_text(content="Select theme from picklist")
            names = window_theme_names()
            themes = ["MyTheme", "MyBrownTheme"] + names
            add_pick_list(options=themes, selected="MyTheme",
                        handle_dynamic_closed_icon_id=icon_closed,
                        handle_dynamic_open_icon_id=icon_open,
                      placeholder="Select Theme", on_select=on_theme_select)

            add_text(content="Standard colors")
            with Row(spacing=10.0, wrap=True):
                for (idx, std_color) in enumerate(std_colors):
                    palette = get_styling_palette(state["theme"], std_color)
                    for key in theme_std_colors:
                        bkg_rgba = list(palette[key][0])
                        text_rgba = list(palette[key][TEXT])
                        style_id = add_container_style(bkg_rgba=bkg_rgba)
                        with Container(width=150, height=30, align_center=True,
                                       style_id=style_id) as _cid:
                            std_cont_ids.append(_cid)
                            std_text_ids.append(add_text(
                                content=f"{color_strings[idx]} {key}",
                                color_rgba=text_rgba))

            # bkg palette only depends on the theme, so any std color works
            palette = get_styling_palette(state["theme"], StdColorStyle.Primary)
            add_text(content="Theme bkg palette")
            with Row(spacing=10.0, wrap=True):
                for key in theme_bkg:
                    bkg_rgba = list(palette[key][0])
                    text_rgba = list(palette[key][TEXT])
                    style_id = add_container_style(bkg_rgba=bkg_rgba)
                    with Container(width=150, height=30, align_center=True,
                                   style_id=style_id) as _cid:
                        bkg_cont_ids.append(_cid)
                        bkg_text_ids.append(add_text(content=key, color_rgba=text_rgba))

with Window(
    title="Window Custom Theme",
    size=(700.0, 500.0),
    position=(300, 300),
    theme_name="MyTheme") as wnd2_id:

    # add a container to center things
    with Container(fill=True):
        with Scrollable(height_fill=True):
            add_text_editor(content=get_instructions())

start_session()
