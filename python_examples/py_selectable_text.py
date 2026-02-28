from imports import *

# Some global vars for this short demo
left_button_pcount = 0
left_button_rcount = 0

right_button_pcount = 0
right_button_rcount = 0

middle_button_pcount = 0
middle_button_rcount = 0

debug = False


# Some specific errors in callback functions may indicate that the parameter count
# is not correct.  However, these can also occur when a non-fatal python errors
# occur like incrementing a non-initialized variable.  Comment out the global
# statement below and you should get an error about parameter count.  However, 
# this is a non-fatal python error, in this case.  You can figure this out by
# commenting out your code adding None as a placeholder and running it.  If the
# error doesn't happen, look for python non-fatal error.
def on_press(_st_id):
    global left_button_pcount
    left_button_pcount += 1
    update_widget(left_pressed_id, IpgTextParam.Content, f"Left button pressed {left_button_pcount} times")


def on_release(_st_id):
    global left_button_rcount
    left_button_rcount += 1
    update_widget(left_released_id, IpgTextParam.Content, f"Left button pressed {left_button_rcount} times")


def on_right_press(_st_id):
    global right_button_pcount
    right_button_pcount += 1
    update_widget(right_pressed_id, IpgTextParam.Content, f"Right button pressed {right_button_pcount} times")


def on_right_release(_st_id):
    global right_button_rcount
    right_button_rcount += 1
    update_widget(right_released_id, IpgTextParam.Content, f"Right button pressed {right_button_rcount} times")


def on_middle_press(_st_id):
    global middle_button_pcount
    middle_button_pcount += 1
    update_widget(middle_pressed_id, IpgTextParam.Content, f"Middle button pressed {middle_button_pcount} times")


def on_middle_release(_st_id):
    global middle_button_rcount
    middle_button_rcount += 1
    update_widget(middle_released_id, IpgTextParam.Content, f"Middle button pressed {middle_button_rcount} times")


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.
def change_text(btn_id):
    update_widget(st_id, IpgSelectableTextParam.Text, "You have changed the selectable text! Click On Me!")


def change_width(btn_id):
    update_widget(st_id, IpgSelectableTextParam.Width, 150.0)


def change_height(btn_id):
    update_widget(st_id, IpgSelectableTextParam.Height, 60.0)


def change_h_align(btn_id):
    # Expand the width so the right alignment can be seen
    update_widget(st_id, IpgSelectableTextParam.WidthFill, True)
    update_widget(st_id, IpgSelectableTextParam.HorizontalAlign, IpgAlignmentX.Right)


def change_v_align(btn_id):
    # Expand the width so the bottom alignment can be seen
    update_widget(st_id, IpgSelectableTextParam.HeightFill, True)
    update_widget(st_id, IpgSelectableTextParam.VerticalAlign, IpgAlignmentY.Bottom)


def change_line_height(btn_id):
    update_widget(st_id, IpgSelectableTextParam.LineHeight, 3.0)


def change_size(btn_id):
    update_widget(st_id, IpgSelectableTextParam.Size, 20.0)


def change_color(btn_id):
    update_widget(st_id, IpgSelectableTextParam.TextColor, IpgColor.BLUE)


def toggle_debug(btn_id):
    global debug
    debug = not debug
    update_widget(wnd_id, IpgWindowParam.Debug, debug)


# Add window must be the first widget. Other windows can be added
# at anytime, as long as their widgets follow.
wnd_id = add_window(
                id="main", 
                title="Selectable Text Demo",
    size=(800, 600),  
                position=(100, 25))

# All widgets need to be added to a container, so a container
# is the second widget needed.
add_column(
        window_id="main", 
        id="col",
        align=IpgAlignment.Center,
        width_fill=True, 
        height_fill=True,
        spacing=5.0)

add_space(
        parent_id="col", 
        height=20.0)

add_text(
        parent_id="col", 
        content="When using the buttons, use left to right, top to bottom for best results.",)

add_space(
        parent_id="col", 
        height=20.0)

# add_row(window_id="main", id="col0", parent_id="col", 
#                height=50.0, align_items=IpgRowAlignment.Center)
# A selectable is a bit more versatile than using a button styled as text.
# In this case, you can detect the left, right, and muddle buttons of the mouse.
# A single callback was used in this case but you could use individual ones also.
# Any are none of these callbacks can be used.
st_id = add_selectable_text(
                parent_id="col", 
                text="Click Me With Your Mouse Buttons!! To see the changes below.",
                on_press=on_press,
                on_release=on_release,
                on_right_press=on_right_press,
                on_right_release=on_right_release,
                on_middle_press=on_middle_press,
                on_middle_release=on_middle_release,
                text_color=IpgColor.YELLOW)

add_space(
        parent_id="col", 
        height=20.0)

add_row(
        window_id="main", 
        id="left", 
        parent_id="col")

left_pressed_id = add_text(
                        parent_id="left", 
                        content=f"Left button pressed {left_button_pcount} times")

left_released_id = add_text(
                            parent_id="left", 
                            content=f"Left button released {left_button_rcount} times")

add_row(
        window_id="main", 
        id="right", 
        parent_id="col")

right_pressed_id = add_text(
                        parent_id="right", 
                        content=f"Right button pressed {right_button_pcount} times")

right_released_id = add_text(
                        parent_id="right", 
                        content=f"Left button released {right_button_rcount} times")

add_row(
        window_id="main", 
        id="middle", 
        parent_id="col")

middle_pressed_id = add_text(
                        parent_id="middle", 
                        content=f"Middle button pressed {middle_button_pcount} times")

middle_released_id = add_text(
                        parent_id="middle", 
                        content=f"Middle button released {middle_button_rcount} times")

add_row(
        window_id="main", 
        id="row_1", 
        parent_id="col")

add_button(
        parent_id="row_1", 
        label="Change Selectable Text", 
        on_press=change_text)

add_button(
        parent_id="row_1", 
        label="Toggle Debug", 
        on_press=toggle_debug)

# TODO: height not working
add_row(
        window_id="main", 
        id="row_2", 
        parent_id="col")

add_button(
        parent_id="row_2", 
        label="Change the text width", 
        on_press=change_width)

add_button(
        parent_id="row_2", 
        label="Change the text Height", 
        on_press=change_height)

add_row(
        window_id="main", 
        id="row_3", 
        parent_id="col")

add_button(
        parent_id="row_3", 
        label="Change the H Align", 
        on_press=change_h_align)

add_button(
        parent_id="row_3", 
        label="Change the V align", 
        on_press=change_v_align)

add_row(
        window_id="main", 
        id="row_4", 
        parent_id="col")

add_button(
        parent_id="row_4", 
        label="Change the Line Height", 
        on_press=change_line_height)

add_button(
        parent_id="row_4", 
        label="Change the Size", 
        on_press=change_size)

add_row(
        window_id="main", 
        id="row_5", 
        parent_id="col")

add_button(
        parent_id="row_5", 
        label="Change the Color", 
        on_press=change_color)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
