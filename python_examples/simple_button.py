from icedpygui import (
    add_window, add_column, start_session, update_widget,
    add_button, add_button_style, IpgButtonStyleParam
    )

def print_id(id):
    print(f"Button id {id} pressed")
    
def round_corners(_, radius: float):
    update_widget(style_id, IpgButtonStyleParam.BorderRadius, [20.0])

style_id = add_button_style(border_radius=[12.0])

add_window(window_id="main", title="App", width=400, height=300)
add_column(window_id="main", container_id="col", width_fill=True)

add_button(parent_id="col", label="Click Me", on_press=print_id, style_id=style_id)

add_button(parent_id="col", label="Update Radius", on_press=round_corners, user_data="my data")

start_session()
