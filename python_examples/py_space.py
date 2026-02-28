from imports import *

# Add the window, debug is set to true to be able to see the space widget.
add_window(
        id="main1", 
        title="Space Demo 1",
    size=(400, 600),
        position=(100, 25), 
        debug=True)

# Add the column for the widgets, centered
add_column(
        window_id="main1", 
        id="col1",
        align=IpgAlignment.Center,
        width_fill=True, 
        height_fill=True)

# Add the space
add_space(
    parent_id="col1", 
    width=50.0, 
    height=200.0)

# Add some info text
add_text(
    parent_id="col1", 
    content="\nThere is a space outlined above,"
            " \nwidth=100.0, height=50.0\n "
            "If you resize the window, the spacing does not change because it is set, "
            "unlike on window 2")

# Add another window
add_window(
    id="main2", 
    title="Space Demo 2",
    size=(400, 600),
    position=(600, 25), 
    debug=True)

# Add the column for the widgets, centered
add_column(
    window_id="main2", 
    id="col2",
    align=IpgAlignment.Center,
    width_fill=True, 
    height_fill=True)

# Add the space, since the window debug=True you will be able to see it.
add_space(
    parent_id="col2", 
    width_fill=True, 
    height=100.0)

# Add some text for info
add_text(
    parent_id="col2", 
    content="\nThere is a space outlined above, \nwidth_fill=True, height=100.0\n ")

add_text(
    parent_id="col2", 
    content="\nif you drag the window width, the spacing grows because width_fill=True\n ")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
