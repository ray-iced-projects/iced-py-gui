"""
Designing CP
"""
from icedpygui import Window, Container, Column, Row, Float, start_session, \
    add_container, add_container_style, add_text, add_slider, add_button,\
    Color, update_widget, ContainerStyleParam, Color, TextParam, ButtonParam, \
    FloatParam

cont = {"color": [0.0, 0.0, 1.0, 1.0]}
cont_style_id = add_container_style(background_color=Color.BLUE)
BLUE = [0.0, 0.0, 1.0, 1.0]
SLIDER_WIDTH = 200
CP_SHOW = False

def slider_r_on_change(_slider_id: int, data: float):
    """Slide changed"""
    cont["color"][0] = round(data, 2)
    update_widget(cont_style_id, ContainerStyleParam.BackgroundRgba, cont["color"])
    update_widget(r_val, TextParam.Content, f"{cont["color"][0]}")

def slider_g_on_change(_slider_id: int, data: float):
    """Slide changed"""
    cont["color"][1] = round(data, 2)
    update_widget(cont_style_id, ContainerStyleParam.BackgroundRgba, cont["color"])
    update_widget(g_val, TextParam.Content, f"{cont["color"][1]}")

def slider_b_on_change(_slider_id: int, data: float):
    """Slide changed"""
    cont["color"][2] = round(data, 2)
    update_widget(cont_style_id, ContainerStyleParam.BackgroundRgba, cont["color"])
    update_widget(b_val, TextParam.Content, f"{cont["color"][2]}")

def slider_a_on_change(_slider_id: int, data: float):
    """Slide changed"""
    cont["color"][3] = round(data, 2)
    update_widget(cont_style_id, ContainerStyleParam.BackgroundRgba, cont["color"])
    update_widget(a_val, TextParam.Content, f"{cont["color"][3]}")

def color_submitted(_btn_id: int):
    """Submitted"""
    update_widget(open_id, ButtonParam.Show, True)
    update_widget(flt_id, FloatParam.Show, False)
    print(cont["color"])

def hide_cp(_btn_id: int):
    """Hide CP"""
    update_widget(open_id, ButtonParam.Show, True)
    update_widget(flt_id, FloatParam.Show, False)

def open_cp(btn_id: int):
    """Open CP"""
    update_widget(btn_id, ButtonParam.Show, False)
    update_widget(flt_id, FloatParam.Show, True)


with Window(title="Float Example", center=True):

    with Container(fill=True, align_center=True):

        with Column(spacing=20):

            add_text(content="Some Text to Show it Floated")
            add_text(content="Some Text to Show it Floated")
            add_text(content="Some Text to Show it Floated")

            open_id = add_button(label="Select Color", on_press=open_cp)

            with Float(scale=0.0, show=True) as flt_id:
                with Row(spacing=10):
                    with Column(spacing=10):
                        cont_id = add_container(width=200, height=100,
                            style_id=cont_style_id)
                        with Row(spacing=20):
                            add_button(label="Submit", on_press=color_submitted)
                            add_button(label="Cancel", on_press=hide_cp)
                        # sliders
                    with Column(spacing=5, width=250.0):
                        with Row(spacing=10):
                            add_text(content="r")
                            sl_r = add_slider(
                                min=0.0, max=1.0,
                                step=0.02, value=BLUE[0],
                                width=SLIDER_WIDTH,
                                on_change=slider_r_on_change)
                            r_val = add_text(content=f"{BLUE[0]}")
                        with Row(spacing=10):
                            add_text(content="g")
                            sl_g = add_slider(
                                min=0.0, max=1.0,
                                step=0.02, value=BLUE[1],
                                width=SLIDER_WIDTH,
                                on_change=slider_g_on_change)
                            g_val = add_text(content=f"{BLUE[1]}")
                        with Row(spacing=10):
                            add_text(content="b")
                            sl_b = add_slider(
                                min=0.0, max=1.0,
                                step=0.02, value=BLUE[2],
                                width=SLIDER_WIDTH,
                                on_change=slider_b_on_change)
                            b_val = add_text(content=f"{BLUE[2]}")
                        with Row(spacing=10):
                            add_text(content="a")
                            sl_a = add_slider(
                                min=0.0, max=1.0,
                                step=0.02, value=BLUE[3],
                                width=SLIDER_WIDTH,
                                on_change=slider_a_on_change)
                            a_val = add_text(content=f"{BLUE[3]}")

            add_text(content="Some Text to Show it Floated")
            add_text(content="Some Text to Show it Floated")
            add_text(content="Some Text to Show it Floated")

start_session()
