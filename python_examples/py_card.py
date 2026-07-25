#!/usr/bin/env python3
"""
Card use demo
"""
from icedpygui import (
    Window, Container, Column, Color,
    Card, CardParam, CardStyleStd,
    add_button, add_card, ButtonParam, ButtonStyleStd, TextParam,
    add_separator, add_separator_style, add_text, update_widget,
    start_session)

state = {"card_id": 0}

# The callback will minimizes the card.
def minimize_card(card_id: int):
    """Card callback to minimize the card"""
    # Hide the card
    update_widget(card_id, CardParam.IsOpen, False)
    # Show the button
    update_widget(btn_id, ButtonParam.Show, True)
    # Hide the isntructions
    update_widget(instr_id, TextParam.Show, False)

def maximize_card(_btn_id: int):
    """Button callback to maximize the card"""
    #  Show the card
    update_widget(state["card_id"], CardParam.IsOpen, True)
    # Hide the button
    update_widget(btn_id, ButtonParam.Show, False)
    # Show the instructions
    update_widget(instr_id, TextParam.Show, True)

sep_style = add_separator_style(color=Color.LIGHT_YELLOW)

# window added first
with Window(title="Card Demo", center=True):

    with Container(fill=True, align_center=True):
        # Add the column to hold the widgets
        # Even though only one widget shows, multiple widgets needed
        # a parent id, so column was used.
        with Column(spacing=10.0, padding=[10.0]):
            instr_id = add_text(content="Press the 'x' to close the card")
            # add the the card
            with Card(
                width=300.0,
                height=200.0,
                padding=[5],
                style_std=CardStyleStd.Success,
                on_close=minimize_card
                ) as state["card_id"]:

                # adds first widget to header
                add_text(content="Card")
                # adds second widget to body.  These can be more than 1 by using a Column
                # so that there is only one top level widget
                add_text(content="This is the body of the card.")

                # adds the third widget to the footer.
                # added a seperator to help show the bottom footer
                # and then a text widget
                with Column(width_fill=True, height=30):
                    add_separator(
                        line=True,
                        line_length=300.0,
                        line_thickness=2.0,
                        style_id=sep_style)

                    add_text(content="Foot content")

            # adds a simple card that you could hide and show with a buttton
            # but the button callback will have to do the show and hide
            add_card(header="Card Header", body="Card Body",
                     footer="Card Footer",
                     width=300.0, height=200.0)

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
