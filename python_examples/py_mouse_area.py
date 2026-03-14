from imports import *

def on_press(id, _user_data):
    print("on press", id)


def on_release(id, _user_data):
    print("on release", id)


def on_right_press(id, _user_data):
    print("on right press, id")


def on_right_release(id, _user_data):
    print("on right release", id)


def on_middle_press(id, _user_data):
    print("on middle press", id)


def on_middle_release(id, _user_data):
    print("on middle release", id)


def on_enter(id, user_data):
    print("entered", id, user_data)    


def on_move(id: int, point: dict, _user_data):
    print("on move", id, point)


def on_exit(id, _user_data):
    print("on exit", id)


with Window(
    title="Main",
    size=(400, 400), 
    center=True, 
    debug=True):

    with Container(
        id="cont", 
        width_fill=True, 
        height_fill=True,
        align_center=True,):

        add_mouse_area(
            id="ma",
            on_enter=on_enter,
            on_exit=on_exit,
            on_move=on_move,
            on_press=on_press,
            on_release=on_release,
            on_middle_press=on_middle_press,
            on_middle_release=on_middle_release,
            on_right_press=on_right_press,
            on_right_release=on_right_release,
            user_data="Some Data"
            )

        # A text widget was added here but you can also add containers or other widgets too.
        add_text(parent_id="ma", content="my content 1")

        # you will probably rarely add more than one item to a mousearea
        # but the option for more is there.
        add_text(parent_id="ma", content="my content 2")

start_session()
