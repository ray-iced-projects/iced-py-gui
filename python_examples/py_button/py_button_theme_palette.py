#!/usr/bin/env python3
"""
Button crating a new palette.
"""
import os
from icedpygui import (
    Window,
    WindowParam,
    WindowTheme,
    Column,
    Row,
    start_session,
    add_button,
    add_pick_list,
    Scrollable,
    add_text,
    update_widget,
    ButtonStyleStd,
    window_theme_names,
    add_font_style,
    FontWeight,
    add_text_editor,
    add_text_editor_style,
)



def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    update_widget(state["wnd_id"], WindowParam.Theme, theme_name)

# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------

state = {"wnd_id": 0}

font_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/py_button/py_button_text2.txt"
state = {"file": ""}

try:
    with open(FILE_PATH, "r", encoding='utf-8') as file:
        state["file"] = file.read()
except FileNotFoundError:
    print(f"*********The file does not exist using {FILE_PATH}.*******")

# ---------------------------------------------------------------------------
# GUI — Initial display with a TokyoNight background (Selected by PickList)
# ---------------------------------------------------------------------------
with Window(title="Button Theme Palette Styles",
            size=(750, 700), center=True, theme=WindowTheme.TokyoNight) as wnd_id:
    state["wnd_id"] = wnd_id
    with Column(spacing=10, padding=[20], width_fill=True, height=850):
        add_text(content="Select a Theme to see the differences in the styling.",
                 font_id=font_id)
        add_pick_list(options=window_theme_names(), selected="TokyoNight",
                    placeholder="Select Theme", on_select=on_theme_select)
        with Scrollable(height=825):
            with Column(spacing=10, width_fill=True):
                with Scrollable(height=300):
                    txt_ed_style_id = add_text_editor_style(border_width=0)
                    add_text_editor(content=state["file"],
                                    width_fill=True,
                                    style_id=txt_ed_style_id)

                add_text(content="Hover and Press Buttons to see color changes",
                         font_id=font_id)
                with Row(spacing=10):
                    with Row(spacing=20, wrap=True):
                        with Column(spacing=5):
                            # The default style standard is Primary
                            add_button(label="Primary Styling", padding=[10])
                            add_button(label="Disabled Styling",disabled=True, padding=[10])

                        with Column(spacing=5):
                            add_button(label="Secondary Styling", padding=[10],
                                    style_std=ButtonStyleStd.Secondary)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Secondary)

                        with Column(spacing=5):
                            add_button(label="Success Styling", padding=[10],
                                    style_std=ButtonStyleStd.Success)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Success)

                        with Column(spacing=5):
                            add_button(label="Warning Styling", padding=[10],
                                    style_std=ButtonStyleStd.Warning)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Warning)

                        with Column(spacing=5):
                            add_button(label="Danger Styling", padding=[10],
                                    style_std=ButtonStyleStd.Danger)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Danger)

                        with Column(spacing=5):
                            add_button(label="Background Styling", padding=[10],
                                    style_std=ButtonStyleStd.Background)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Background)

                        with Column(spacing=5):
                            add_button(label="Subtle Styling", padding=[10],
                                    style_std=ButtonStyleStd.Subtle)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Subtle)

                        with Column(spacing=5):
                            add_button(label="Text Styling", padding=[10],
                                    style_std=ButtonStyleStd.Text)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Text)


start_session()
