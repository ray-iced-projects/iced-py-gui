"""
Scrollable demo

Allows scrolling when widgets exceed the container's size
"""

from icedpygui import Window, Container, Column, start_session, \
        Scrollable, add_scroller, add_text


with Window(title="Scrollable Demo", center=True):

    with Container(align_center=True, fill=True):

        with Column(spacing=50.0):

            # default in the y direction, if you want to change the parameters
            # of the y scroller, use add_scroller(scroller parameters)
            with Scrollable( width=200.0, height=100.0):

                add_text(content=("This is Some Text \n")*20)

            # The y direction scroller will show when the height of the widgets exceed
            # the height of the container.  Therefore, the only time you need to use the
            # scroller_y_id is when you want to change the parameters.  The scroller_x
            # will always need to be added when needed.
            with Scrollable(
                width=200.0,
                height=50.0,
                scroller_x_id=add_scroller(),
                ):

                add_text(content=("This is Some Text ")*20)

start_session()
