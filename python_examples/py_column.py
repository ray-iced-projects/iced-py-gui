from imports import *


# Moves the text to the center position
def align_center(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgColumnParam.AlignX, 
            value=IpgAlignment.Center)

     
# Moves the text to the end position
def align_end(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgColumnParam.AlignX, 
            value=IpgAlignment.End)
    
    
# Moves the text back to the start position
def align_start(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgColumnParam.AlignX, 
            value=IpgAlignment.Start)
    

# Moves text off start because padding on the left side
# padding = [top, right, bottom, left]
def padding(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgColumnParam.Padding, 
            value=[0.0, 0.0, 0.0, 50.0])  
    
# change container width
def width(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgColumnParam.Width, 
            value=200.0)
    
    
# change container height
def height(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgColumnParam.Height, 
            value=300.0)
 
# change container height
def spacing(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgColumnParam.Spacing, 
            value=20.0)
    
# add a container style, these can be added at any time
cont_style = add_container_style(
                    border_width=2.0,
                    border_color=IpgColor.WHITE)



# Add the windows
add_window(
    id="main", 
    title="Container Styling", 
    size=(600, 600),  
    centered=True,
    debug=True)

add_row(
    window_id="main",
    id="row",
    width_fill=True)


# Add a column to hold the text widgets
# The column id is needed since the column
# is being modified in the callbacks.
col_id = add_column(
            window_id="main",
            id="col_txt",
            parent_id="row",
            align_x=IpgAlignment.Start,
            width_fill=True,
            height=400.0,
            spacing=10.0,)

add_text(
        parent_id="col_txt",
        content="Some Text")

add_text(
        parent_id="col_txt",
        content="Some Text")

add_text(
        parent_id="col_txt",
        content="Some Text")


add_column(
        window_id="main",
        id="col_right",
        parent_id="row",
        width_fill=True,
        height=400.0,
        spacing=10.0,
        )

add_text(
    parent_id="col_right",
    content="Press buttons below to change the style of the left Column\n"
)
# Add a button the center the alignment 
add_button(
        parent_id="col_right",
        label="Align Center",
        on_press=align_center)

# Add a button align end 
add_button(
        parent_id="col_right",
        label="Align End",
        on_press=align_end)

# Add a button align back to the start 
add_button(
        parent_id="col_right",
        label="Align Start",
        on_press=align_start)

# Add a button add padding of the contained items
add_button(
        parent_id="col_right",
        label="Padding",
        on_press=padding)

# Add a button change the container width
add_button(
        parent_id="col_right",
        label="Width",
        on_press=width)

# Add a button change the container height
add_button(
        parent_id="col_right",
        label="Height",
        on_press=height)

# Add a button change the solumn spacing
add_button(
        parent_id="col_right",
        label="Spacing",
        on_press=spacing)

# last thing is to start the session
start_session()