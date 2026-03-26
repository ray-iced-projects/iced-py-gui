from icedpygui import Window, Column, start_session, \
    update_widget, IpgTextParam, add_event_mouse, \
        add_text, add_space

# make a global var to hold some scroll data
scroll_total_line_y = 0


# The mouse move callback will fire when the window opens.
# The user_data is not used here, but needed since it was supplied as a parameter
# The mouse_id is not used since we're just updating the text widget.
# The move data is a dictionary as all of the events data are.
def mouse_move(_mouse_id: int, point: dict, user_data: any):
    update_widget(text_for_moved, IpgTextParam.Content, f"Moved {point}")


# Mouse callbacks
def left_mouse_button_pressed(_mouse_id: int, user_data: any):
    update_widget(text_for_pressed, IpgTextParam.Content, f"Left Button pressed")
    update_widget(text_for_user_data, IpgTextParam.Content, f"user data = {user_data}")
    
def middle_mouse_button_pressed(_mouse_id: int, user_data: any):
    update_widget(text_for_pressed, IpgTextParam.Content, f"Middle Button pressed")
    update_widget(text_for_user_data, IpgTextParam.Content, f"user data = {user_data}")

def right_mouse_button_pressed(_mouse_id: int, user_data: any):
    update_widget(text_for_pressed, IpgTextParam.Content, f"Right Button pressed")
    update_widget(text_for_user_data, IpgTextParam.Content, f"user data = {user_data}")
    
def left_mouse_button_released(_mouse_id: int, _user_data: any):
    update_widget(text_for_released, IpgTextParam.Content, f"Left Button released")

def middle_mouse_button_released(_mouse_id: int, _user_data: any):
    update_widget(text_for_released, IpgTextParam.Content, f"Middle Button released")
    
def right_mouse_button_released(_mouse_id: int, _user_data: any):
    update_widget(text_for_released, IpgTextParam.Content, f"Right Button released")


# The scroll data is a dictionary
def mouse_button_scrolled_line(_mouse_id: int, scroll: dict, _user_data: any):
    global scroll_total_line_y
    scroll_total_line_y += scroll.get("y")
    update_widget(text_for_scroll_line, IpgTextParam.Content, 
                    f"Scrolled {scroll} total = {scroll_total_line_y}")


# An event can be added at any time since they are not widgets or containers.
add_event_mouse(enabled=True,
                    on_move=mouse_move,
                    on_left_press=left_mouse_button_pressed,
                    on_left_release=left_mouse_button_released,
                    on_middle_press=middle_mouse_button_pressed,
                    on_middle_release=middle_mouse_button_released,
                    on_right_press=right_mouse_button_pressed,
                    on_right_release=right_mouse_button_released,
                    on_middle_scroll_line=mouse_button_scrolled_line,
                    user_data="Some Data",
                    )

# Adding a window
with Window(
        title="Mouse Handler Demo",
        size=(600, 600),
        center=True):

    # Add a column to hold the widgets
    with Column(align_center=True, fill=True):

        # Add some spacing
        add_space(height=150.0)

        # Add all of the text widget for info display
        text_for_moved = add_text(content="Mouse position will be here")
        text_for_pressed = add_text(content="Button presses will show here")
        text_for_released = add_text(content="Button releases will show here")
        text_for_scroll_line = add_text(content="Button scroll line data will show here")
        text_for_user_data = add_text(content="Button user data will show here")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
