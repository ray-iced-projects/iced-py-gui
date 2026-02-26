from imports import *



add_window(
    window_id="main", 
    title="Scrollable",
    center=True,
)

add_container(
    window_id="main",
    container_id="cont",
    center=True,
    width_fill=True,
    height_fill=True,
)

add_column(
    window_id="main",
    container_id="col",
    parent_id="cont",
    spacing=20.0,
)

# if no add_scrollbar() use, the default is 
# in the y direction
add_scrollable(
    window_id="main",
    parent_id="col",
    container_id="scroll_default", 
    width=200.0, 
    height=100.0,
)

txt = ("This is Some Text \n")*20

add_text(
    parent_id="scroll_default",
    content=txt,
)

# to use the scrollbar in the x direction,

bar_x = add_scrollbar()

add_scrollable(
    window_id="main",
    container_id="scroll_x",
    parent_id="col",
    width=200.0, 
    height=50.0,
    scrollbar_x_id=bar_x, 
    )

txt = ("This is Some Text ")*20

add_text(
    parent_id="scroll_x",
    content=txt,
)

start_session()
