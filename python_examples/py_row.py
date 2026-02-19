from imports import *

def align_center(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgRowParam.Align, 
            value=IpgAlignment.Center)

     
# Moves the text to the end position
def align_end(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgRowParam.Align, 
            value=IpgAlignment.End)
    
    
# Moves the text back to the start position
def align_start(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgRowParam.Align, 
            value=IpgAlignment.Start)
    

# Moves text off start because padding on the left side
# padding = [top, right, bottom, left]
def padding(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgRowParam.Padding, 
            value=[0.0, 0.0, 0.0, 50.0])  
    
# change container width
def width(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgRowParam.Width, 
            value=350.0)
    
    
# change container height
def height(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgRowParam.Height, 
            value=100.0)
 
# change container height
def spacing(btn_id):
    update_widget(
            wid=col_id, 
            param=IpgRowParam.Spacing, 
            value=20.0)
    
    
cont_style = add_container_style(
                    border_width=2.0,
                    border_color=IpgColor.WHITE)



# Add the windows
add_window(
        window_id="main", 
        title="Container Styling",
    size=(600, 600),  
        pos_centered=True,
        debug=True)

# Add column to hold everything
add_column(
        window_id="main",
        container_id="col",
        width_fill=True)


# Add a row to hold the text widgets
col_id = add_row(
                window_id="main",
                container_id="row_txt",
                parent_id="col",
                align=IpgAlignment.Start,
                width_fill=True,
                height=50.0
                )

add_text(
        parent_id="row_txt",
        content="Some Text")

add_text(
        parent_id="row_txt",
        content="Some Text")

add_text(
        parent_id="row_txt",
        content="Some Text")


add_column(
        window_id="main",
        container_id="col_bottom",
        parent_id="col",
        width_fill=True,
        height=400.0
        )

# Add a button the center the alignment 
add_button(
        parent_id="col_bottom",
        label="Align Center",
        on_press=align_center)

# Add a button align end 
add_button(
        parent_id="col_bottom",
        label="Align End",
        on_press=align_end)

# Add a button align back to the start 
add_button(
        parent_id="col_bottom",
        label="Align Start",
        on_press=align_start)

# Add a button add padding of the contained items
add_button(
        parent_id="col_bottom",
        label="Padding",
        on_press=padding)

# Add a button change the container width
add_button(
        parent_id="col_bottom",
        label="Width",
        on_press=width)

# Add a button change the container height
add_button(
        parent_id="col_bottom",
        label="Height",
        on_press=height)

# Add a button change the solumn spacing
add_button(
        parent_id="col_bottom",
        label="Spacing",
        on_press=spacing)

# last thing is to start the session
start_session()