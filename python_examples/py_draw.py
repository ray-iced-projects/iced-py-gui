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
    DrawMode,
    DrawParam,
    DrawWidget,
    add_radio,
    add_pick_list,
    add_text_input,
    TextInputParam,
    add_space,
    start_session,
    update_draw_params,
    update_widget,
)

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/resources/canvas.json"

state = {"id": 0}

def clear_canvas(_btn_id):
    """Clearing Canvas"""
    print("clear canvas")
    update_draw_params(state["id"], { DrawParam.Clear: None })


def widget_selected(_radio_id, index: int):
    """Widget selected"""
    match index:
        case 0:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.Arc})
        case 1:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.Bezier})
        case 2:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.Circle})
        case 3:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.Ellipse})
        case 4:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.Line})
        case 5:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.Polygon})
        case 6:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.PolyLine})
        case 7:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.RightTriangle})
        case 8:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.FreeHand})
        case 9:
            update_draw_params(state["id"], {
                DrawParam.SelectedWidget: DrawWidget.Text})


def set_draw_width(input_id: int, value: str):
    """Set Draw Width"""
    update_draw_params(state["id"], {
                DrawParam.DrawWidth: float(value)})
    update_widget(input_id, TextInputParam.Placeholder, f"Draw Width={value}")


def set_poly_points(input_id: int, value: str):
    """Set Poly Points"""
    print(value)
    update_draw_params(state["id"], {
                DrawParam.PolyPoints: int(value)})
    update_widget(input_id, TextInputParam.Placeholder, f"Polygon Points={value}")


def set_mode(_pl_id: int, mode: str):
    "Set the Draw Mode"
    match mode:
        case "Display":
            update_draw_params(state["id"], {
                DrawParam.DrawMode: DrawMode.Display})
        case "Edit":
            update_draw_params(state["id"], {
                DrawParam.DrawMode: DrawMode.Edit})
        case "New":
            update_draw_params(state["id"], {
                DrawParam.DrawMode: DrawMode.New})
        case"Rotate":
            update_draw_params(state["id"], {
                DrawParam.DrawMode: DrawMode.Rotate})


def save_canvas(_btn_id: int):
    """Save Canvas"""
    update_draw_params(state["id"], {DrawParam.Save: FILE_PATH})


def load_canvas() -> list:
    """Load Canvas"""
    with open(FILE_PATH, 'r', encoding='utf-8') as file:
        return json.load(file)


def load_canvas_with_btn(_btn_id: int):
    """Load Canvas"""
    with open(FILE_PATH, 'r', encoding='utf-8') as file:
        data = json.load(file)
        update_draw_params(state["id"], {DrawParam.Load: data})


def set_draw_color(_cp_id: int, color: list[float, 4]):
    """Set Draw Color"""
    update_draw_params(state["id"], {DrawParam.DrawColor: color})


def set_canvas_color(_cp_id: int, color: list[float, 4]):
    """Set Canvas Color"""
    update_draw_params(state["id"], {DrawParam.CanvasColor: color})


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

                # draw width
                add_text_input(
                    placeholder="Draw width=2.0",
                    on_submit=set_draw_width,
                    width_fill=True,
                    padding=[3],
                )

                # set points for polygon
                add_text_input(
                    placeholder="Polygon Points=3",
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
            state["id"] = add_draw()


start_session()
