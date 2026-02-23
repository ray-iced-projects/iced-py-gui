from imports import *



add_window(
    window_id="main", 
    title="Radio Demo",
    size=(500, 600),
    position=(100, 25))

add_container(
    window_id="main", 
    container_id="cont",
    width_fill=True, 
    height_fill=True)

add_column(
    window_id="main", 
    container_id="col", 
    parent_id="cont",
    align_x=IpgAlignment.Center, 
    spacing=50.0)


add_radio(
    parent_id="col", 
    labels=["one", "two", "three", "four"],)

add_radio(
    parent_id="col", 
    labels=["1", "2", "3", "4"],)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
