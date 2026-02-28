from imports import *


def print_id(id):
    print(f"Button id {id} pressed")
    
def round_corners(id):
    update_widget(
        id, 
        IpgButtonParam.StyleId, 
        style_id)
    
style_id = add_button_style(border_radius=[10.0])

with Window(window_id="main", title="App", 
            size=(400, 300),center=True):

    with Column(container_id="col", width_fill=True, spacing=20.0,
                padding=[20.0], align_x=IpgAlignment.Center):

        add_button(
            label="Click Me to Print Id",
            padding=[5.0],
            on_press=print_id, 
            style_id=style_id)

        add_button(
            label="Press to Update the Corner Radius",
            padding=[5.0], 
            on_press=round_corners)



start_session()