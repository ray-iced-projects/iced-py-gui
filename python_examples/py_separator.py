from imports import *

def change_dot_count(btn_id: int):
    update_widget(dot_count_id, IpgSeparatorParam.DotCount, 10)
    
# For change the dot to  a circle, you must add the GotBorderWidth,
# add a style to make the do color transparent and add a color to the border
def change_dot_fill(btn_id: int):
    update_widget(dot_fill_id, IpgSeparatorParam.DotFill, False)
    update_widget(dot_fill_id, IpgSeparatorParam.DotBorderWidth, 2.0)
    update_widget(dot_fill_id, IpgSeparatorParam.StyleId, dot_style)


def change_dot_radius_border_width(btn_id: int):
    update_widget(dot_radius_border_width_id, IpgSeparatorParam.DotBorderWidth, 1.0)
    update_widget(dot_radius_border_width_id, IpgSeparatorParam.DotRadius, 8.0)


# There's an invisible box that contains the separator and the width/height
# defaults to shrink around the separator.  If you cnage the width/height,
# you are changing the box size which acts sort of like padding.
# When you press the button, the dots will separate and the dots will move down.
# The width change is probably not very useful but eh height is needed for the label and 
# horizontal alignment.  If you set the window debug=True, you can better
# see the changes
def change_dot_width(btn_id: int):
    update_widget(dot_width_id, IpgSeparatorParam.Width, 100.0)
    update_widget(dot_width_id, IpgSeparatorParam.Height, 50.0)
    
    
def change_dot_spacing(btn_id: int):
    update_widget(dot_spacing_id, IpgSeparatorParam.Spacing, 10.0)
    
    
def show_dot(btn_id: int):
    update_widget(dot_show_id, IpgSeparatorParam.Show, True)
    
    
    
# This is a simple demo to change the checkmark of the checkbox to an x.
# The default style results in a dot, but by styling
# the dot, you can make the center transparent
# the add_seorator will need to have the border_width set
dot_style = add_separator_style(
                ipg_color=IpgColor.TRANSPARENT,
                border_ipg_color=IpgColor.WHITE)

label_style = add_separator_style(
                ipg_color=IpgColor.LIGHT_YELLOW
                )

line_style = add_separator_style(
                ipg_color=IpgColor.LIGHT_YELLOW
                )


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
    height_fill=True)

# Add a column to hold the widgets
add_column(
    window_id="main", 
    container_id="col",
    parent_id="cont")

# for the dot, the width and height are not used
# unless one wants a sort of padding around the circle.
add_separator(
    parent_id="col",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=5.0,
    dot_count=10,
    spacing=5.0,)

# By using the styling and the border width, 
# one can make a circle
add_separator(
    parent_id="col",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=5.0,
    dot_count=10,
    dot_border_width=2.0,
    spacing=5.0,
    style_id=dot_style)

# The height is needed here and needs to be
# larger than the height of the text.
add_separator(
    parent_id="col",
    separator_type=IpgSeparatorType.Label,
    height=22.0,
    label="Some Label",
    spacing=5.0)

add_separator(
    parent_id="col",
    separator_type=IpgSeparatorType.Label,
    height=22.0,
    label="Some Label",
    spacing=5.0,
    style_id=label_style)

# The line needs both the width and the height
add_separator(
    parent_id="col",
    width=200.0,
    height=20.0,
    separator_type=IpgSeparatorType.Line)

add_separator(
    parent_id="col",
    width=200.0,
    height=20.0,
    separator_type=IpgSeparatorType.Line,
    style_id=line_style)


add_row(window_id="main",
    container_id="dot_count_row",
    parent_id="col")

add_button(
    parent_id="dot_count_row",
    label="Change Dot Count",
    on_press=change_dot_count)

dot_count_id = add_separator(
    parent_id="dot_count_row",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=5.0,
    dot_count=5,
    spacing=5.0)


add_row(
    window_id="main",
    container_id="dot_fill_row",
    parent_id="col")

add_button(
    parent_id="dot_fill_row",
    label="Change Dot Fill",
    on_press=change_dot_fill)

dot_fill_id = add_separator(
    parent_id="dot_fill_row",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=5.0,
    dot_count=5,
    spacing=5.0)


add_row(
    window_id="main",
    container_id="dot_border_width_row",
    parent_id="col")

add_button(
    parent_id="dot_border_width_row",
    label="Change Dot Border Width",
    on_press=change_dot_radius_border_width)

dot_radius_border_width_id = add_separator(
        parent_id="dot_border_width_row",
        separator_type=IpgSeparatorType.Dot,
        dot_radius=5.0,
        dot_count=5,
        dot_border_width=2.0,
        dot_fill=False,
        spacing=5.0,
        style_id=dot_style)

add_text(
    parent_id="col",
    content="See the code comment for what happens here.")

add_row(
    window_id="main",
    container_id="dot_width_row",
    parent_id="col")

add_button(
    parent_id="dot_width_row",
    label="Change Dot Width",
    on_press=change_dot_width)

dot_width_id = add_separator(
    parent_id="dot_width_row",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=5.0,
    dot_count=3,
    spacing=5.0,)


add_row(
    window_id="main",
    container_id="dot_spacing_row",
    parent_id="col")

add_button(
    parent_id="dot_spacing_row",
    label="Change Dot Spacing",
    on_press=change_dot_spacing)

dot_spacing_id = add_separator(
    parent_id="dot_spacing_row",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=5.0,
    dot_count=5,
    spacing=5.0,)


add_row(
    window_id="main",
    container_id="dot_show_row",
    parent_id="col")

add_button(
    parent_id="dot_show_row",
    label="Change Dot Show",
    on_press=show_dot)

dot_show_id = add_separator(
    parent_id="dot_show_row",
    separator_type=IpgSeparatorType.Dot,
    dot_radius=5.0,
    dot_count=5,
    spacing=5.0,
    show=False)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
