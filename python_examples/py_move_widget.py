from imports import *

ids = []


def move_widget(btn_id: int, item: any):
    # equate the tuple items to help interpretation
    widget_id = item[0]
    container_id = item[1]
    move_after = item[2]
    move_before = item[3]

    # move the widget
    move_widget(
            window_id="main", 
            widget_id=widget_id, 
            target_container_str_id=container_id, 
            move_after=move_after, 
            move_before=move_before)
    

add_window(id="main",
               title="Move Widget",
    size=(400.0, 600.0),
               pos_centered=True)

add_row(
        window_id="main",
        id="row")

add_column(
        window_id="main",
        id="col_1",
        parent_id="row")

for i in range(0, 10):
    ids.append(add_text(parent_id="col_1", content=f"{i}"))

add_space(
        parent_id="row", 
        width=100.0)

add_column(
        window_id="main",
        id="col_2",
        parent_id="row")


add_column(
        window_id="main",
        id="move_btns")

add_button(
        parent_id="move_btns",
        label="Move number 5 to end",
        on_press=move_widget,
        user_data=(ids[5], "col_1", None, None))

add_button(
        parent_id="move_btns",
        label="Move number 5 after 0",
        on_press=move_widget,
        user_data=(ids[5], "col_1", ids[1], None))

add_button(
        parent_id="move_btns",
        label="Move number 5 before 0",
        on_press=move_widget,
        user_data=(ids[5], "col_1", None, ids[0]))

start_session()
