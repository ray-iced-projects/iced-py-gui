#!/usr/bin/env python3
"""
Button use demo
"""

from icedpygui import Window, Column, Container, Scrollable, Row,\
    add_button, add_text,\
    IpgContainerStyleStd, start_session



def print_id(btn_id):
    """Callback to print the button id"""
    print(f"Button id {btn_id} pressed")

def print_user_data(_btn_id, some_data):
    """Callback to prints user data"""
    print(f"User data {some_data}")


#  First add a window
with Window(title="App",
            size=(800, 600), center=True):

    # Need a Scrollable since long content
    with Scrollable(width_fill=True):

        # Need a Container to hold everything, in this case a Column
        with Column(spacing=20.0, padding=[20.0]):

            add_text(content="Button with padding variations [Top, Right, Bottom, Left] [all]")

            with Row(spacing=20.0):
                # button with only a label parameter all other parameters are defaults
                add_button(label="No Padding")

                # padding
                add_button(
                    label="All sides Padding\n [10.0]",
                    padding=[10.0])

                # padding
                add_button(
                    label="Top/Bottom Padding\n[10.0, 0.0, 10.0, 0.0]",
                    padding=[10.0, 0.0, 10.0, 0.0]) # top & botton

                # padding
                add_button(
                    label="Left/Right Padding\n[0.0, 10.0, 0.0, 10.0]",
                    padding=[0.0, 10.0, 0.0, 10.0]) # left & right

            add_text(content="Button label Alignment: bool")

            with Row(spacing=20.0):

                add_button(
                    label="top left",
                    width=150.0,
                    height=50.0,
                    text_top_left=True,
                )

                add_button(
                    label="top center",
                    width=150.0,
                    height=50.0,
                    text_top_center=True,
                )

                add_button(
                    label="top right",
                    width=200.0,
                    height=50.0,
                    text_top_right=True,
                )

            with Row(spacing=20.0):

                add_button(
                    label="center left",
                    width=200.0,
                    height=50.0,
                    text_center_left=True,
                )

                add_button(
                    label="center (default)",
                    width=200.0,
                    height=50.0,
                    text_center=True,
                )

                add_button(
                    label="center right",
                    width=200.0,
                    height=50.0,
                    text_center_right=True,
                )

            with Row(spacing=20.0):

                add_button(
                    label="bottom left",
                    width=200.0,
                    height=50.0,
                    text_bottom_left=True,
                )

                add_button(
                    label="bottom center",
                    width=200.0,
                    height=50.0,
                    text_bottom_center=True,
                )

                add_button(
                    label="bottom right",
                    width=200.0,
                    height=50.0,
                    text_bottom_right=True,
                )

            add_text(content="Button label text size")

            with Row(spacing=20.0):

                add_button(label="Size default")

                add_button(label="Size", text_size=20.0)


            add_text(content="Button Width and Height parameters, note the spacing around the button label")

            # Container needed to show the outline
            with Container(width_fill=True, height=150, style_std=IpgContainerStyleStd.BorderedBox):
                # Need a coloumn to hold the row and then the lone button
                with Column(width_fill=True, height=200.0, spacing=10.0):
                    # Need row for the first two buttons
                    with Row(spacing=20.0, width_fill=True):

                        add_button(label="Width Height=Shrink, no space")

                        add_button(
                            label="Width=200 Height=50",
                            width=200.0,
                            height=50.0)

                    add_button(label="width height=Fill - button fills the remaining area in Column",
                                fill=True)

            add_text(content="Button Clipping")

            # Need row to hold 2 buttons
            with Row(spacing=20.0, width_fill=True):

                add_button(label="The text on this button will wrap",
                        width=200.0)

                add_button(label="The text on this button is clipped....",
                        width=200.0,
                        clip=True)

            add_text(content="Button Callbacks")

            # Need column to hold the buttons
            with Column(spacing=20.0, width_fill=True, height=200.0):

                add_button(label="Press me to run the callback to print id",
                           padding=[10.0],
                           on_press=print_id)

                add_button(label="Press me to run the callback to print user data",
                           padding=[10.0],
                           on_press=print_user_data,
                           user_data="Some data")

                add_button(label="Press me to run the callback to print more user data",
                           padding=[10.0],
                           on_press=print_user_data,
                           user_data=[10.0, 20.0])

start_session()
