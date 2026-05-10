#!/usr/bin/env python3
"""Draw demo."""
import os, json
from icedpygui import (
    Window,
    ColorPicker,
    Column,
    Container,
    Row,
    add_button,
    add_button_style,
    add_draw,
    DrawParam,
    add_radio,
    add_pick_list,
    add_text_input,
    add_space,
    start_session,
    update_widget,
)

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/resources/canvas.json"

def clear_canvas(_btn_id):
    """Clearing Canvas"""
    print("clear canvas")

def widget_selected(_input_id, widget: int):
    """Widget selected"""
    print(f"widget selected {widget}")

def set_line_width(_input_id: int, value: str):
    """Set Line Width"""
    print(f"line_width value {value}")

def set_poly_points(_input_id: int, value: str):
    """Set Poly Points"""
    print(f"poly points value {value}")

def set_mode(_pl_id: int, mode: int):
    "Set the Draw Mode"
    print(f"Draw mode = {mode}")

def save_canvas(_btn_id: int):
    """Save Canvas"""
    print("saving canvas")

def load_canvas() -> list:
    """Load Canvas"""
    with open(FILE_PATH, 'r', encoding='utf-8') as file:
        return json.load(file)

def load_canvas_with_btn(_btn_id: int):
    """Load Canvas"""
    with open(FILE_PATH, 'r', encoding='utf-8') as file:
        data = json.load(file)
        update_widget(draw_id, DrawParam.Curves, data)

def set_draw_color(_cp_id: int, color: str):
    """Set Draw Color"""
    print(f"Setting draw color = {color}")

def set_canvas_color(_cp_id: int, color: str):
    """Set Canvas Color"""
    print(f"Setting canvas color = {color}")

btn_style = add_button_style(border_radius=[5.0])

with Window(title="Draw Demo", center=True):
    with Row(width_fill=True, height_fill=True):

        with Container(width=200, height_fill=True):
            with Column(padding=[10], spacing=20):
                add_space(height=20)
                add_button(label="Clear Canvas", on_press=clear_canvas)

                # radio buttons for canvas widget selection
                add_radio(
                    labels=["Arc", "Bezier", "Circle", "Ellipse", "Line",
                            "Polygon", "PolyLine", "Right Triangle",
                            "FreeHand", "Text"],
                    radio_spacing=5,
                    on_selected=widget_selected
                )

                # line width
                add_text_input(
                    placeholder="Line width=2.0",
                    on_submit=set_line_width,
                    width_fill=True,
                    padding=[3],
                )

                # set points for polygon
                add_text_input(
                    placeholder="Polygon Points = 3",
                    on_submit=set_poly_points,
                    width_fill=True,
                    padding=[3]
                )

                add_pick_list(
                    options=["Display", "Edit", "New", "Rotate"],
                    placeholder="Select Draw Mode",
                    on_select=set_mode,
                    selected="None",
                )

                with ColorPicker(on_submit=set_draw_color):
                    add_button(label="Set Draw Color",
                            padding=[3.0],
                            style_id=btn_style)

                with ColorPicker(on_submit=set_canvas_color):
                    add_button(label="Set Canvas Color",
                            padding=[3.0],
                            style_id=btn_style)

                add_button(
                    label="Save Canvas",
                    on_press=save_canvas,
                    padding=[3.0],
                    style_id=btn_style,
                )

                add_button(
                    label="Load Canvas",
                    on_press=load_canvas_with_btn,
                    padding=[3.0],
                    style_id=btn_style,
                )

        with Container(width_fill=True, height_fill=True):
            draw_id = add_draw()


start_session()
