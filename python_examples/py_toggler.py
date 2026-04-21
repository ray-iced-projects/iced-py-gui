#!/usr/bin/env python3
"""
Toggle use demo
"""

from icedpygui import Window, Column, Container, Row, start_session, \
        add_toggler, add_toggler_style, add_text, ContainerStyleStd, Color \


# Callback from toggler
def toggled(tg_id, is_toggled):
    """Toggler callback"""
    print(tg_id, is_toggled)


state = {"wnd_width": 700}


# Add the window
with Window(
    title="Toggler Demo",
    size=(state["wnd_width"], 700),
    center=True):

    # Add a main row to hold two columns
    with Row(fill=True):
        with Column(
            width = state["wnd_width"]/2.0,
            height_fill=True,
            padding=[20.0],
            spacing=20.0):

            add_text(content="Label alignment:\nvalid if width > text width",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=ContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0],
                            width_fill=True, height_fill=True):
                    # text alignment only works if width > length of text
                    add_toggler(label="Text left (default)", width=200.0)
                    add_toggler(label="Text center", width=200.0, text_center=True)
                    add_toggler(label="Text right", width=200.0, text_right=True)

            add_text(content="Toggler Size",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=ContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0],
                            width_fill=True, height_fill=True):
                    add_toggler(label="Size (default)", width=200.0)
                    add_toggler(label="Size: 30", width=200.0, size=30.0)
                    add_toggler(label="Size: 10", width=200.0, size=10.0)

            add_text(content="Toggler Text Size",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=ContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0],
                            width_fill=True, height_fill=True):
                    add_toggler(label="Text Size (default)")
                    add_toggler(label="Text Size: 20", text_size=20.0)
                    add_toggler(label="Text Size: 5", text_size=5.0)

            add_text(content="Label spacing:",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=100.0,
                        style_std=ContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0],
                            width_fill=True, height_fill=True):

                    add_toggler(label="Text spacing (default)")
                    add_toggler(label="Text spacing 0", spacing=0.0)
                    add_toggler(label="Text spacing 30", spacing=30.0)

        with Column(
            width_fill=True,
            height_fill=True,
            padding=[20.0],
            spacing=20.0):

            add_text(content="Label text Line Height:",
                    align_center_left=True)

            # add container for the background
            with Container(width_fill=True, height=175.0,
                        style_std=ContainerStyleStd.BorderedBox):
                # add the row to hold the togglers
                with Column(spacing=10.0, padding=[10.0],
                            width_fill=True, height_fill=True):

                    # adding a container with outline to show how the
                    # line height works
                    with Container(style_std=ContainerStyleStd.RoundedBox):
                        add_toggler(label="Text Line Height (default=0)")
                    with Container(style_std=ContainerStyleStd.RoundedBox):
                        add_toggler(label="Text Line Height 5", text_line_height=5.0)
                    with Container(style_std=ContainerStyleStd.RoundedBox):
                        add_toggler(label="Text Line Height 2", text_line_height=2.0)

            # add container for the background
            with Container(width_fill=True, height=70.0, padding=[20.0],
                        style_std=ContainerStyleStd.BorderedBox):

                # Add some styling to the toggler, this can go anywhere
                tog_style = add_toggler_style(
                                background_color=Color.GREEN,
                                foreground_color=Color.ANTIQUE_WHITE)

                # Add the toggler and change size to see styling better
                add_toggler(
                    label="Styled Toggler",
                    style_id=tog_style
                    )


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
