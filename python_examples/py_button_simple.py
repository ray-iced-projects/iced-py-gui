from imports import *


# A demo app for the Button widget
# Normally, you would probably use the callbacks
# to do something in your program, but here the
# style of the button was changed.  Maybe in some case you would want to
# change the color to indicate that the button was pressed or something 
# You can change all of the parameters of any widget
# through the callback system.

# callback to print the button id
def print_id(id):
    print(f"Button id {id} pressed")
    
# callback to round the corners of the button
# The style id could have just been added to the button
def round_corners(id):
    update_widget(
        id,                         # id of the widget to be changed
        IpgButtonParam.StyleId,     # The parameter to be changed
        style_id)                   # Each parameter has a value that is sent also, in this cane the id of the style


# a styling widget for the button, it can be used directly in    
# the button's style_id parameter
style_id = add_button_style(border_radius=[10.0]) 


with Window(id="main", title="App", 
            size=(400, 400),center=True):
    
    # To get the centering to work, the container width/height
    # must be greater than the container widgets
    # or the container shrinks to wrap the widgets.
    # In this case, only width needed to be expanded
    # but in other cases if could be height or both.
    with Container(width_fill=True, center=True):
        
        with Column(spacing=20.0, padding=[20.0], 
                    align_x=IpgAlignment.Center):

            add_button(
                label="Click Me to Print Id",
                padding=[5.0],
                on_press=print_id, # callback to print
                style_id=style_id) # styling id used directly here but with a callback below

            add_button(
                label="Press to Update the Corner Radius",
                padding=[5.0], 
                on_press=round_corners)  # callback to round the corners

start_session()