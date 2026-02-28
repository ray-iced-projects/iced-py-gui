from imports import *

# Add the window
add_window(
        id="main", 
        title="Toggler Demo",
    size=(700, 625),  
        pos_centered=True)

# Add a main column to hold everything
add_column(
        window_id="main", 
        id="col", 
        width_fill=True, 
        height_fill=True,
        align=IpgAlignment.Center, 
        spacing=5.0)

add_space(
        parent_id="col", 
        height=50.0)

# Add some styling to the toggler
tog_style = add_toggler_style(
                      background_color=IpgColor.GREEN,
                      background_color_toggled=IpgColor.LIGHT_GREEN,
                      foreground_color=IpgColor.ANTIQUE_WHITE,
                      foreground_color_toggled=IpgColor.BLUE,
                      )

# Add the toggler and change size to see styling better
tog_id = add_toggler(
                parent_id="col",
                label="Some Toggler Label",
                size=40.0,
                text_size=25.0,
                style_id=tog_style)



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
