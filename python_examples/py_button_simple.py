from imports import *


def print_id(id):
    print(f"Button id {id} pressed")
    
def round_corners(id):
    update_widget(
        id, 
        IpgButtonParam.StyleId, 
        style_id)
    
style_id = add_button_style(border_radius=[10.0])
cont_style = add_container_style(background_color=IpgColor.LIGHT_BLUE)

with Window(id="main", title="App", 
            size=(400, 400),center=True):

    with Column(id="col", width_fill=True, spacing=20.0,
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

        with Container(id="cont", style_id=cont_style, 
                    width=200, center=True):
            add_button(
                label="Backgound Color Changes on Container",
                padding=[5.0],
                style_standard=IpgButtonStyleStandard.Text)        

start_session()