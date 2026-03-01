from imports import *


# The callback used for the first checkbox.
# The callback id for this is not used since we are
# changing the other checkbox.  Therefore we needed to know
# its id.  Normally you would probably use a class to store theses
# needed ids for later use.
# The is_checked is a boolean which will toggle ther first 
# checkbox when you want todo something, in this case change the Icon.
def on_toggle(_chkbx_id: int, is_checked: bool, user_data: any):
    print("Some user data: ", user_data)
    update_widget(
            wid=x_id, 
            param=IpgCheckboxParam.Icon, 
            value=IpgIcon.Asterisk)

cont_style = add_container_style(
                background_color=IpgColor.LIGHT_BLUE)
chk_style = add_checkbox_style(
    background_color=IpgColor.LIGHT_BLUE)


# Add a window first
with Window(id="main", title="CheckBox Demo",
            size=(600, 600),  center=True):

    # Add a container to center the widgets in the middle
    # Centering is the default behavior.
    with Container(id="cont", width_fill=True,
                   height_fill=True, center=True):

        # Since a container can only hold one widget, use a column to hold the
        # two checkboxes.  We let the width and height default to shrink, so no entry.
        # The alignment defaults to Start but for demonstration purpose, we
        # added the IpgColumnAlignment.Start
        with Column(id="col", align_x=IpgAlignment.Start, spacing=20.0):

            # Add the first checkbox with the callback on_toggle.
            # The user data is optional, we just sow it here.
            add_checkbox(
                label="Check Me!!!",
                on_toggle=on_toggle,
                user_data="Something") # just printed in this demo

            with Container(id="custom", style_id=cont_style):

                # Add the second checkbox.  This has no callback since it not needed.
                # We need it's id, so we will equate it and use it in the callback
                # to change the icon to an Asterisk.  We set the is_check to true
                # so that you can see the Icon.
                x_id = add_checkbox(
                    label="See my check change to an Asterisk",
                    icon=IpgIcon.X,
                    icon_size=16.0,
                    is_checked=True,
                    style_id=chk_style)
  
# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
