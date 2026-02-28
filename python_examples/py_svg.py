from imports import *

width = 400.0
height = 400.0


# Since user data is include, it will need to be
# added to all whether used on or.
def on_press(btn_id, _user_data):
    print("on press", btn_id)


def on_release(btn_id: int, _user_data):
    print("on release", btn_id)


def on_right_press(btn_id: int, _user_data):
    print("on right press, btn_id")


def on_right_release(id: int, _user_data):
    print("on right release", id)


def on_middle_press(btn_id: int, _user_data):
    print("on middle press", btn_id)


def on_middle_release(btn_id: int, _user_data):
    print("on middle release", btn_id)


def on_enter(btn_id: int, user_data: any):
    print("entered", btn_id, user_data)


def on_move(btn_id: int, point: dict, _user_data):
    print("on move", btn_id, point)


def on_exit(btn_id, _user_data):
    print("on exit", btn_id)


# The six of the svg will only get as big as the size of
# the container it's in.
def increase_size(btn_id):
    global width, height
    width += 10
    height += 10
    update_widget(svg_id, IpgSvgParam.Width, width)
    update_widget(svg_id, IpgSvgParam.Height, height)


def decrease_size(btn_id):
    global width, height
    width -= 10
    height -= 10
    update_widget(svg_id, IpgSvgParam.Width, width)
    update_widget(svg_id, IpgSvgParam.Height, height)


add_window(
        id="main", 
        title="Main",
    size=(600, 600),
        pos_centered=True,
        debug=True)

add_container(
        window_id="main", 
        id="cont",
        width_fill=True,
        centered=True)

# Setting up the image path
cwd = os.getcwd()
tiger_path = cwd + "/python_examples/resources/tiger_0.svg"

add_column(
        window_id="main", 
        id="col",
        align=IpgAlignment.Center)


svg_id = add_svg(
                parent_id="col",
                svg_path=tiger_path,
                width=width,
                height=height,
                on_enter=on_enter,
                on_exit=on_exit,
                on_move=on_move,
                on_press=on_press,
                on_release=on_release,
                on_middle_press=on_middle_press,
                on_middle_release=on_middle_release,
                on_right_press=on_right_press,
                on_right_release=on_right_release,
                user_data="Some Data")

add_button(
        parent_id="col", 
        label="Increase Size", 
        on_press=increase_size)

add_button(
        parent_id="col", 
        label="Decrease Size", 
        on_press=decrease_size)

start_session()
