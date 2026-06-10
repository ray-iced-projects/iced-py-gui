#!/usr/bin/env python3
"""
ToolTip use demo
"""

from icedpygui import (Window, Container, Column, ToolTip,
    add_container_style, add_text, add_checkbox,
    start_session,  ContainerStyleStd, Color)


cont_style = add_container_style(bkg_color=Color.AQUA)


# Add a window first
with Window(title="TooTip Demo", center=True):

    # Add a container to center the widgets in the middle
    with Container(fill=True, align_center=True):
        with Column(spacing=50.0):

            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5,
                style_id= cont_style):

                add_text(content="Some text with a tooltip with custom background style")

            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5,
                position_right=True,
                style_std= ContainerStyleStd.BorderedBox):

                add_text(content="Some text with a tooltip with standard style")

            with ToolTip(
                text="Tip",
                padding=5.0,
                gap=5,
                delay_sec=1):

                add_text(content="Some text with a tooltip with no style and delayed 1 sec")

            with ToolTip(
                text="Tip",
                padding=5.0,
                position_follow_cursor=True,
                gap=5):

                with Container(width=200.0, height=50.0, style_std=ContainerStyleStd.BorderedBox):
                    add_text(content="This container has a tooltip that follows the cursor")

            with ToolTip(
                text="Tip",
                padding=5.0,
                position_bottom=True,
                gap=5):

                add_checkbox(label="Chexbox with tooltip at bottom")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
