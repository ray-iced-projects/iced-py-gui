from imports import *


def print_id(id):
    print(f"Button id {id} pressed")
    
def round_corners(id):
    update_widget(
        id, 
        IpgButtonParam.StyleId, 
        style_id)
    
style_id = add_button_style(border_radius=[5.0])

add_window(
    window_id="main", 
    title="App", 
    size=(400, 300),
    centered=True)

add_column(
    window_id="main", 
    container_id="col", 
    width_fill=True,
    spacing=20.0,
    padding=[20.0],
    align_x=IpgAlignment.Center)

add_button(
    parent_id="col", 
    label="Click Me to Print Id",
    padding=[5.0],
    on_press=print_id, 
    style_id=style_id)

add_button(
    parent_id="col", 
    label="Press to Update the Corner Radius",
    padding=[5.0], 
    on_press=round_corners)

start_session()