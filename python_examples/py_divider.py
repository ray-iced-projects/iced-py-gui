#!/usr/bin/env python3
"""
Divider demo
"""

from icedpygui import (
    Window,
    Column,
    ColumnParam,
    Container,
    add_container_style,
    start_session,
    Stack,
    add_button,
    add_text,
    TextParam,
    add_divider,
    DividerParam,
    DividerDirection,
    add_divider_style,
    add_toggler,
    update_widget,
    Color,
    )


def divider_change(div_id: int, data: tuple[int, float]):
    """Divider change"""
    # Get the difference to be added to the right column
    idx, value = data
    diff = heights[idx] - value

    # Update the top locally and in ipg
    heights[idx] = value
    update_widget(
            wid=column_ids[idx],
            param=ColumnParam.Height,
            value=value)

    # Update the bottom locally and in ipg
    if idx < len(heights)-1:
        heights[idx+1] += diff
        update_widget(
            wid=column_ids[idx+1],
            param=ColumnParam.Height,
            value=heights[idx+1])

    # Update the divider
    update_widget(
                wid=div_id,
                param=DividerParam.Sizes,
                value=heights)

    # Update the two text items
    update_widget(wid=text_ids[idx],
                    param=TextParam.Content,
                    value=f"Width={value}")

    if idx < len(heights)-1:
        update_widget(wid=text_ids[idx+1],
                    param=TextParam.Content,
                    value=f"Width={heights[idx+1]}")


heights = [175.0, 175.0]
column_ids = []
text_ids = []
HANDLE_WIDTH = 200.0
HANDLE_HEIGHT = 4.0

cont_style_id = add_container_style(border_color=Color.WHITE,
                                        border_width=1.0)

div_style = add_divider_style(background_color=Color.WHITE)

# Add a window first
with Window(
        title="Divider Demo",
        size=(600,600),
        center=True):

    # Add a container to center the widgets in the middle
    with Container(fill=True, align_center=True):

        # add a column to hold the text and the stack
        with Column():
            add_text(content="Place the cursor over the highlighted divider and drag")

            # make the stack to lay the dividers over the containers
            with Stack():

                # make a column to hold the two columns
                # this is added to stack
                # The outer container used in the stack
                # cannot have any padding, since divider
                # cannot detect whether padding is used
                # it becomes misaligned.
                with Column(width=HANDLE_WIDTH):

                    for index, height in enumerate(heights):
                        # add a container for styling purposes
                        with Container(
                            # style_id=cont_style_id
                            ):

                            with Column(
                                    width=HANDLE_WIDTH,
                                    height=height) as col_id:

                                column_ids.append(col_id)

                                text_ids.append(add_text(content=f"Width={height}"))

                                add_button(label="Some Button")

                                add_button(label="Another Button")

                                add_toggler(label="Toggler")



                # Make the divider
                add_divider(
                    direction=DividerDirection.Vertical,
                    sizes=heights,
                    handle_width=HANDLE_WIDTH,
                    handle_height=HANDLE_HEIGHT,
                    include_last_handle=True,
                    style_id=div_style,
                    on_change=divider_change)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
