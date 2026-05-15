#!/usr/bin/env python3
"""
Divider demo
"""

from icedpygui import (
    Window,
    Row,
    start_session,
    Container,
    add_text,
    add_divider,
    DividerDirection,
    DividerParam,
    Stack,
    update_widget
    )


def divider_change(_div_id: int, data: tuple[int, float]):
    """Divider change"""
    _idx, size1 = data
    size2 = 200 - size1 + sizes[1]
    sizes = [size1, size2]
    update_widget(div_id, DividerParam.Sizes, sizes)


sizes = [200.0, 200.0]


# Add a window first
with Window(
        title="Divider Demo",
        size=(600,600),
        center=True):

    with Stack():
        with Row(width_fill=True, height=100):
            with Container(width=200, height_fill=True, align_center=True):
                add_text(content="Container 1")

            with Container(width=200, height_fill=True, align_center=True):
                add_text(content="Container 2")

        div_id = add_divider(
            direction=DividerDirection.Horizontal,
            sizes=sizes,
            handle_width=5,
            handle_height=100,
            on_change=divider_change)



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
