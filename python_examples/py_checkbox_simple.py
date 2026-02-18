from icedpygui import (
    add_window, generate_id, update_widget, start_session,
    add_container, add_column, add_checkbox, IpgIcon,
    IpgCheckboxParam, IpgAlignment
    )


# The callback used for the first checkbox.
# The callback id for this is not used since we are
# changing the other checkbox.  Therefore we needed to know
# its id.  Normally you would probably use a class to store theses
# needed ids for later use.
# The is_checked is a boolean which will toggle ther first 
# checkbox when you want todo something, in this case change the Icon.
def on_toggle(_chkbx_id: int, is_checked: bool, user_data: any):
    update_widget(
            wid=x_id, 
            param=IpgCheckboxParam.Icon, 
            value=IpgIcon.Asterisk)


# Add a window first
add_window(
    window_id="main", 
    title="CheckBox Demo",
    width=600, 
    height=600,  
    pos_centered=True)

# Add a container to center the widgets in the middle
# Centering is the default behavior.
add_container(
    window_id="main", 
    container_id="cont", 
    width_fill=True,
    height_fill=True)

# Since a container can only hold one widget, use a column to hold the
# two checkboxes.  We let the width and height default to shrink, so no entry.
# The alignment defaults to Start but for demonstration purpose, we
# added the IpgColumnAlignment.Start
add_column(
    window_id="main", 
    container_id="col", 
    parent_id="cont",
    align=IpgAlignment.Start)

# Add the first checkbox with the callback on_toggle.
# The user data is optional, we just sow it here.
add_checkbox(
    parent_id="col", 
    label="Check Me!!!",
    on_toggle=on_toggle,
    user_data="Something") # not used in this demo

# Add the second checkbox.  This has no callback since it not needed.
# We need it's id, so we will equate it and use it in the callback
# to change the icon to an Asterisk.  We set the is_check to true
# so that you can see the Icon.
x_id = add_checkbox(
    parent_id="col", 
    label="See my check check change to an x",
    icon=IpgIcon.X,
    icon_size=16.0,
    is_checked=True)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
