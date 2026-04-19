"""
Scrollable demo

Allows scrolling when widgets exceed the container's size
"""

from icedpygui import Window, Container, Column, start_session, \
        Scrollable, add_scroller, add_rail_style, add_scrollable_style, \
        add_text, Color, ContainerStyleStd, add_space

# The rail id is used in the add_scrollable_style
rail_x = add_rail_style (
            background_color=Color.GREEN)

# Add the style ids to the scrollable style
scr_style = add_scrollable_style(
    horizontal_rail_style_id=rail_x,
    )

# the scroller return a large amount of data in dict format
# just select what you need to use.
# The scroller callback is activated on startup, if you don't
# want the callback to be executed at that point, add a condition
# as been done here either using the x or y absolute to prevent
# the callback from executing before any scrolling occus.
def on_scroll(_scroller_id, data: dict):
    """Scroller callback"""
    if data['abs_y'] != 0:
        print(data)



with Window(title="Scrollable Demo", center=True):

    with Container(align_center=True, fill=True):

        with Column(spacing=50.0):

            # default in the y direction, if you want to change the parameters
            # of the y scroller, use add_scroller(scroller parameters)
            with Scrollable( width=200.0, height=100.0, on_scroll=on_scroll):
                with Container(
                    width_fill=True,
                    style_std=ContainerStyleStd.BorderedBox):

                    add_text(content=("This is Some Text \n")*20)

            # The y direction scroller will show when the height of the widgets exceed
            # the height of the container.  Therefore, the only time you need to use the
            # scroller_y_id is when you want to change the parameters.  The scroller_x
            # will always need to be added when needed.
            with Scrollable(width=200.0,
                            height=50.0,
                            scroller_x_id=add_scroller()):

                add_text(content=("A lot of text, ")*20)

            # Some styling added
            # Auto scroll is set to True which allows one to
            # use the mouse to scroll, click the middle mouse button
            # move the mouse and when done, click any other mouse button.
            with Scrollable(
                width=200.0,
                height=100.0,
                scroller_x_id=add_scroller(),
                scroller_y_id=add_scroller(),
                auto_scroll=True,
                style_id=scr_style
                ):

                # When the x scroller is added, a container
                # is needed with some padding so that the
                # last line shows or add a space, as done here.
                for _ in range(100):
                    add_text(content="A lot of text")

                add_text(content=("Some styling added, ")*10,
                        wrapping_none=True)
                add_space(height=10)

start_session()
