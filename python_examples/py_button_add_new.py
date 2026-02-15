from icedpygui import (
    add_window, start_session,
    add_container, add_column,
    add_button, 
)

count = 0
def on_press(btn_id, user_data: int):
    global count
    count += 1
    print(count)
    add_button(
        parent_id="col", 
        label=f"Button_{count}", 
        on_press=on_press,
        user_data=0)


# Add the windows
add_window(
    window_id="main", 
    title="Button", 
    width=400, 
    height=400,  
    pos_centered=True)

 # Add a container to hold everything aligning all in the center
add_container(
    window_id="main", 
    container_id="cont", 
    width_fill=True, 
    height_fill=True,
    centered=True)

# Add a column to hold multiple widgets, vertically.
add_column(
    window_id="main", 
    container_id="col", 
    parent_id="cont")

add_button(
    parent_id="col", 
    label="Press Me to Add A Button", 
    on_press=on_press,
    user_data=0)

start_session()
