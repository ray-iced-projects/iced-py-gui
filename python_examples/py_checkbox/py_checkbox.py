#!/usr/bin/env python3
"""
Checkbox use demo

A checkbox with a text label that can be toggled on and off.

"""

from icedpygui import (Window, Column, Container, Row, Scrollable, start_session,
    add_checkbox, add_text, ContainerStyleStd, Icon)


# This callback simply prints if checkbox is checked.
# The checkbox always returns an id and a bool. If user_data
# is used, you must have a parameter for it.
# Normally the chk_id would not be used, the user would
# be performing some other operation.  Therefore, it's
# good practise to remind yourself, that the id is the source
# and that you probably need other ids to do the operations.
def on_toggle(chk_id: int, is_checked: bool):
    """Checkbox Callback"""
    print(f"Checkbox id {chk_id} toggled to {is_checked}")


# You must always have 2 paramters defined, id and a bool for the checked.
# If the add_chechbox is using user_data, then a parameter must be supplied.
# You can name the parameters anything you like.
# if you want to use the same callback for checkboxes with or without the
# user data, simple pass a None value, i.e. user_data=None
def on_toggle_user_data(chk_id: int, is_checked: bool, data: str):
    """Checkbox Callback"""
    print(f"Checkbox id {chk_id}, checked={is_checked}, user_data={data}")

# Window size=(width, height)
with Window(title="Checkbox Demo",
            size=(900, 600), center=True):

    with Scrollable(width_fill=True):

        with Column(spacing=20.0, padding=[20.0], width_fill=True):

            # ***********************Checkbox Basics***************************

            add_text(content=("Basic Checkbox\n"
                              "Check the ones for callback and "
                              "user_data to see results printed"))

            with Row(spacing=20.0):

                add_checkbox(label="Default Checkbox")

                add_checkbox(
                    label="With Callback, on_toggle=on_toggle",
                    on_toggle=on_toggle)

                add_checkbox(
                    label="With use_data='some data'",
                    on_toggle=on_toggle_user_data,
                    user_data="some info"
                    )

            # ***********************Checkbox state***************************

            add_text(content="Checkbox is_checked State")

            with Row(spacing=20.0):

                add_checkbox(
                    label="is_checked=False (default)",
                    is_checked=False)

                add_checkbox(
                    label="is_checked=True",
                    is_checked=True)

            # ***********************Checkbox Sizing***************************

            add_text(content="Checkbox Size (box size)")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Size",
                    is_checked=True)

                add_checkbox(
                    label="size=25",
                    size=25.0,
                    is_checked=True)

                add_checkbox(
                    label="size=40",
                    size=40.0,
                    is_checked=True)

            # ***********************Spacing***************************

            add_text(content="Checkbox Spacing (gap between box and label)")

            with Row(spacing=40.0):

                add_checkbox(
                    label="Default Spacing",
                    is_checked=True)

                add_checkbox(
                    label="spacing=5",
                    spacing=5.0,
                    is_checked=True)

                add_checkbox(
                    label="spacing=30",
                    spacing=30.0,
                    is_checked=True)

            # ***********************Text sizes***************************

            add_text(content="Checkbox Text Size")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Text Size",
                    is_checked=True)

                add_checkbox(
                    label="text_size=20",
                    text_size=20.0,
                    is_checked=True)

                add_checkbox(
                    label="text_size=30",
                    text_size=30.0,
                    is_checked=True)

            # *************************Widths/Heights**********************

            add_text(content="Checkbox Width")

            with Row(spacing=20.0, width_fill=True):
                # Added to container to better show background
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_checkbox(
                        label="Default Width=Shrink",
                        is_checked=True)

                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_checkbox(
                        label="width=200",
                        width=200.0,
                        is_checked=True)

                with Container(style_std=ContainerStyleStd.BorderedBox, width_fill=True):
                    add_checkbox(
                        label="Fill=True, fills remaining area",
                        fill=True,
                        is_checked=True)

            # ***********************Checkbox icons***************************

            add_text(content="Checkbox Icon Variations")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Icon (Check)",
                    is_checked=True)

                add_checkbox(
                    label="Icon.X",
                    icon=Icon.X,
                    is_checked=True)

                add_checkbox(
                    label="Icon.Asterisk",
                    icon=Icon.Asterisk,
                    is_checked=True)

                add_checkbox(
                    label="Icon.Heart",
                    icon=Icon.HeartFill,
                    is_checked=True)

            # ***********************Checkbox Icon Sizing***************************

            add_text(content="Checkbox Icon Size")

            with Row(spacing=20.0):

                add_checkbox(
                    label="Default Icon Size",
                    is_checked=True)

                add_checkbox(
                    label="icon_size=25",
                    icon_size=25.0,
                    is_checked=True)

                add_checkbox(
                    label="icon_size=40",
                    icon_size=40.0,
                    size=40.0,
                    is_checked=True)

            # ***********************Checkbox No label***************************

            add_text(content="Checkbox No Label")

            with Row(spacing=20.0):

                add_checkbox(is_checked=True)

                add_checkbox(is_checked=False)

# Must be the last executed step to start things off.
start_session()
