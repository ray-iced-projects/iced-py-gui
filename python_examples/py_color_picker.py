from imports import *

def color_selected(cp_id: int, color: list, user_data: any):
    # Need to change the list color to a str type
    # Since the color is being displayed as text
    # otherwise use as is
    string = "["
    for i in range(0, len(color)):
        string += str(color[i]) + ", "
    string += "]"
    
    # update the text
    update_widget(
        wid=text_id, 
        param=IpgTextParam.Content, 
        value=string)
    
    # update the text color
    update_widget(
        wid=text_id, 
        # Note the type is TextRgba, not TextColor
        # You could use a value of type IpgColor
        # then use the TextColor without using the colorpicker
        param=IpgTextParam.TextRgba, 
        value=color)
    


def cp_opened(cp_id: int, user_data: any):
    print("color picker opened")
    

def cp_canceled(cp_id: int, user_data: any):
    print("color picker canceled")

# Since the color_picker is using a button
# to open the selection widget, then
# to style the button you use the button styling
cp_style = add_button_style(border_radius=[5.0])


# Add a window first
add_window(
    id="main", 
    title="Canvas",
    size=(500.0, 500.0),
    center=True)

# Add the container to center both x and y (default).  Holds only one widget.
add_container(
    window_id="main", 
    id="cont",
    width_fill=True, 
    height_fill=True,
    center=True)

# Add a column to hold multiple widgets
add_column(
    window_id="main",
    id="col",
    parent_id="cont",
    spacing=20.0)

add_color_picker(
    parent_id="col",
    on_press=cp_opened, # Button to open color picker
    on_select=color_selected, # the color selection selected
    on_cancel=cp_canceled, # the color selection cancel
    padding=[5.0],
    style_id=cp_style,
    user_data="Something") #user data not used but supplied for testing
    # If you use user_data, all callback will require the user_data parameter
    # or whatever name you want for it.

text_id = add_text(
            parent_id="col",
            content="Color value here")

start_session()