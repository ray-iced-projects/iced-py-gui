from imports import *

# When ctrl, alt, logo(MAC), or shift are processed, they are keys.  Therefore modifier is "None".
# When a key follows the command key, then modifier is the command key and key is the key.
# So to reduce confusion with the text widget values, when the command key is first pressed,
# the modifier is set to "" versus printing out a "None".
# user_data was set in this case just to show how it works.
def key_pressed(
    _event_id: int, 
    key: dict, 
    user_data: any):
    
    modifier = key.get("modifier")
    if key.get("modifier") == "None":
        modifier = ""
        
    name = key.get("name")
    
    update_widget(
        wid=text_pressed, 
        param=IpgTextParam.Content, 
        value=f"{name}: {modifier} {key.get('key')}")
    
    update_widget(
        wid=text_user_data, 
        param=IpgTextParam.Content, 
        value=f"user data is {user_data}")


# key released callback.  Even though user_data is not used, it still needs to be
# in the parameters because it was supplied as a parameter when added
def key_released(
    _event_id: int, 
    key: dict, 
    _user_data: any):
    
    name = key.get("name")
    
    update_widget(
            wid=text_released, 
            param=IpgTextParam.Content, 
            value=f"{name}: {key.get('modifier')} {key.get('key')}")


# add the event and the two callbacks along with the user_data if needed.
add_event_keyboard(
        enabled=True, 
        on_key_press=key_pressed, 
        on_key_release=key_released, 
        user_data=[25])


# Unlike widgets, the events can be added at any time since they are not widgets
# A window is added before any widgets
add_window(
        window_id="main", 
        title="KeyBoard Handler Demo",
    size=(600, 400),
        pos_centered=True)

# A column is added to hold the widgets
add_column(
        window_id="main", 
        container_id="col",
        align=IpgAlignment.Center,
        width_fill=True, 
        height_fill=True)

# a space for readability
add_space(
        parent_id="col", 
        height=50.0)

# Some text to display the output
text_pressed = add_text(
                    parent_id="col", 
                    content="Key presses will show here")

text_released = add_text(
                    parent_id="col", 
                    content="Key releases will show here")

add_space(
        parent_id="col", 
        height=50.0)

text_user_data = add_text(
                    parent_id="col", 
                    content="Some user data will be displayed here")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
