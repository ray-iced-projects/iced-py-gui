from imports import *


# Add a window first
# if you make debug=True, you can see 
# the outline of the box containing
# the separators and the effect of the 
# width and height settings.
add_window(
    window_id="main", 
    title="Separator Demo",
    size=(600, 600),  
    center=True,
    debug=False)

add_container(
    window_id="main",
    container_id="cont",
    width_fill=True,
    height_fill=True,
    center=True)

add_column(
    window_id="main",
    parent_id="cont",
    container_id="col",
    spacing=20.0,
    align_x=IpgAlignment.Center
)


add_separator(
    parent_id="col",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=3.0,
    dot_count=10,
    spacing=10.0,)

add_separator(
    parent_id="col",
    separator_type=IpgSeparatorType.Line,
    line_length=150.0,
    line_thickness=2.0)

add_separator(
    parent_id="col",
    separator_type=IpgSeparatorType.Label,
    label="label",
    label_left_width=2.0,
    label_right_width=2.0,
)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
