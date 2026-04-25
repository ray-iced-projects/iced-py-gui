#!/usr/bin/env python3
"""
Card use demo

Add a card container.

Card excepts the addition of 1 to 3 widgets, head, body, and optional foot.
if only 1 widget is added, then it's assumed to be the body
if 2 widgets are added, then they are head, body, respectively.
if 3 widgets are added, then they are head, body, foot, respectively.

Parameters
----------
parent_id : str
    Sets the parent container ID that this card belongs to.
is_open : bool, default True
    Whether the card is open (expanded).
close_icon : bool, Optional
    Whether to have a close icon.
close_icon_size : float, Optional
    Sets the Size of the close button in logical pixels.
on_close : callable, Optional
    Sets the Callback method to invoke when the card is closed.
width : float, Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the card fills available width.
height : float, Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the card fills available height.
max_width : float, Optional
    Sets the Maximum width in logical pixels.
max_height : float, Optional
    Sets the Maximum height in logical pixels.
padding : list of float, Optional
    Sets the Padding for all sections as [all], [vertical, horizontal], or
    [top, right, bottom, left].
padding_head : list of float, Optional
    Sets the Padding for the header section.
padding_body : list of float, Optional
    Sets the Padding for the body section.
padding_foot : list of float, Optional
    Sets the Padding for the footer section.
style_id : int, Optional
    Sets the ID of a custom style created with ``add_card_style``.
style_std : CardStyleStd, Optional
    Sets a predefined standard style variant.
style_button : int, Optional
    Sets the ID of a button style for the close button.
show : bool, default True
    Whether the card is visible.
user_data : Any, Optional
    Sets the Arbitrary data forwarded to callbacks.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.
Returns
-------
int
    The numeric widget ID of the newly created card.
"""
from icedpygui import Window, Container, Column, \
    Card, CardParam, CardStyleStd, \
    add_button, ButtonParam, ButtonStyleStd, TextParam, \
    add_separator, add_text, update_widget, start_session

state = {"card_id": 0}

# The callback will minimizes the first card, the button at the bottom left will maximize it.
def minimize_card(card_id: int):
    """Card callback to minimize the card"""
    update_widget(card_id, CardParam.IsOpen, False)
    update_widget(btn_id, ButtonParam.Show, True)
    update_widget(instr_id, TextParam.Show, False)

def maximize_card(_btn_id: int):
    """Button callback to maximize the card"""
    update_widget(state["card_id"], CardParam.IsOpen, True)
    update_widget(btn_id, ButtonParam.Show, False)
    update_widget(instr_id, TextParam.Show, True)


# window added first
with Window(title="Card Demo", center=True):

    with Container(fill=True, align_center=True):
        # Add the column to hold the widgets
        # Even though only one widget shows, multiple widgets needed
        # a parent id, so column was used.
        with Column(spacing=10.0, padding=[10.0]):
            instr_id = add_text(content="Close the Card to see next action")
            # add the the card
            with Card(
                width=300.0,
                height=200.0,
                padding=[5],
                style_std=CardStyleStd.Success,
                on_close=minimize_card
                ) as state["card_id"]:

                add_text(content="Card")
                add_text(content="This is the body of the card.")

                with Column(width_fill=True, height=30):
                    add_separator(line_length=300)
                    add_text(content="Foot content")

            # add the button but make show=False
            # The button can go anyplace you like,
            # you can make it unhidden all the time
            # and just change the label, let your
            # imagination go wild :)
            btn_id = add_button(
                label="Open Card",
                show=False,
                padding=[10],
                on_press=maximize_card,
                style_std=ButtonStyleStd.Success)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
