from imports import *

with Window(id="main", title="Scrollable", center=True):

    with Container(center=True, width_fill=True, height_fill=True):

        with Column(spacing=20.0):

            # if no add_scrollbar() use, the default is 
            # in the y direction
            add_scrollable(
                id="scroll_y", 
                width=200.0, 
                height=100.0,
            )

            txt = ("This is Some Text \n")*20

            add_text(
                parent_id="scroll_y",
                content=txt,
            )

            # to use the scrollbar in the x direction,
            bar_x = add_scrollbar()

            add_scrollable(
                id="scroll_x",
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
