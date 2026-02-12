from icedpygui import add_window, add_button, start_session

def handler1(id):
    """Handler that works with or without user_data"""
    print(f"Button {id} pressed")
    
def handler2(id, user_data):
    """Handler that works with or without user_data"""
    print(f"Button {id} pressed, user_data={user_data}")

add_window(window_id="main", title="App", width=400, height=300)
add_button(parent_id="main", label="Click Me", on_press=handler1)
add_button(parent_id="main", label="click Me2", on_press=handler2, user_data="my data")
start_session()