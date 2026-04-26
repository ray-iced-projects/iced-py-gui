#!/usr/bin/env python3
"""
Stack demo

Add a stack container widget.

Stack allows one the stack widgets on top of each other
If you are stacking and have issues with any mouse events being transferred
to the widgets below them, then add an opaque layer to block.

Parameters
----------
window_id : str
    Sets the window this stack belongs to.
container_id : str
    Sets the Unique string identifier for the stack.
parent_id : str, Optional
    Sets the parent container ID.  Defaults to the window itself.
width : float, Optional
    Sets the Fixed width in logical pixels.
height : float, Optional
    Sets the Fixed height in logical pixels.
width_fill : bool, default False
    Whether the stack fills available width.
height_fill : bool, default False
    Whether the stack fills available height.
fill : bool, Optional
    Whether to fill both the available width and height.
hide_index : int, Optional
    Sets the index of the child to hide.
show : bool, default True
    Whether the stack is visible.

Returns
-------
int
    The numeric widget ID of the newly created stack.
"""
import os
from icedpygui import Window, Column, Container, Stack, start_session, \
    add_image, add_text, add_space, update_widget, TextParam, \
    ContentFit, MouseArea, MousePointer


def card_selected(_card_id, name):
    """Card callback"""
    update_widget(
        text_id,
        TextParam.Content,
        f"Card selected is {name}")


cwd = os.getcwd()
path = path = cwd + "/python_examples/resources/cards/hearts/"

names = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"]

with Window(title="Stack", center=True):

    with Container(fill=True, align_center=True):

        with Column(width=200, height_fill=True):
            add_space(height=30)
            text_id = add_text(content="Card Selected is None")
            add_space(height=30)
            # Adds the stack container to the window.
            with Stack(height_fill=True):

                for i in range(1, 14):
                    # Adds the column to the stack to hold the space and card.
                    with Column():

                        # The space, which grows with each card, allows for an offset
                        # to be able to see all of the cards.  If not used, they are
                        # stacked on top of each other.
                        add_space(height=35*i-35)
                        with Container(width=150, height=250):
                            with MouseArea(
                                mouse_pointer=MousePointer.Grab,
                                on_press=card_selected,
                                user_data=names[i-1]):
                                add_image(
                                    path=f"{path}{i}.png",
                                    content_fit=ContentFit.Fill)


start_session()
