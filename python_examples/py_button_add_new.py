from imports import *

# The button demo add a new button to the window when the first
# button is pressed.  You can dynamically add any widget through a callback

count = 0
def on_press(btn_id, user_data: int):
    global count
    count += 1
    print(count)
    # any widget can be added
    # you would probably use move_widget
    # to place the new widget properly
    # in this case, the new button just
    # appends to the column
    add_button(
        parent_id="col", 
        label=f"Button_{count}", 
        on_press=on_press,
        user_data=0,
        )


# Add the windows
add_window(
    id="main", 
    title="Button", 
    size=(400, 400),  
    center=True,
    )

 # Add a container to hold everything aligning all in the center
add_container(
    window_id="main", 
    id="cont", 
    width_fill=True, 
    height_fill=True,
    center=True,
    )

# Add a column to hold multiple widgets, vertically.
add_column(
    window_id="main", 
    id="col", 
    parent_id="cont",
    spacing=20.0,
    )

add_button(
    parent_id="col", 
    label="Press Me to Add A Button", 
    on_press=on_press,
    user_data=0
    )

start_session()
