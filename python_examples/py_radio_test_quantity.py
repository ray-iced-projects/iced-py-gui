from imports import *

def radio_cb(id: int, selected: tuple):
    radio_index = selected[0]
    radio_label=selected[1]
    print(id, radio_index, radio_label)

add_window(
    window_id="main", 
    title="Radio Demo",
    size=(1400, 600),
    position=(100, 25))

add_container(
    window_id="main",
    container_id="cont",
    center=True,
)

add_radio(
    parent_id="cont", 
    labels=["a", "b", "c"],
    direction=IpgRadioDirection.Horizontal,
    size=15.0,
    on_select=radio_cb)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
