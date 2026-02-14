from icedpygui import add_window, add_button, add_button_style, start_session

def handler1(id):
    """Handler that works with or without user_data"""
    print(f"Button id {id} pressed")
    
def handler2(id, user_data):
    """Handler that works with or without user_data"""
    print(f"Button id {id} pressed, user_data={user_data}")

custom = add_button_style(border_radius=[12.0])

add_window(window_id="main", title="App", width=400, height=300)

add_button(parent_id="main", label="Click Me", on_press=handler1, style_id=custom)

add_button(parent_id="main", label="click Me2", on_press=handler2, user_data="my data")

start_session()