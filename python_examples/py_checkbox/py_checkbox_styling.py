#!/usr/bin/env python3
"""
Checkbox styling use demo

"""

from icedpygui import (Window, Column, Row, start_session,
    add_checkbox, add_checkbox_style, add_text,
    CheckboxStyleStd, Color)


with Window(title="Checkbox Styling",
            size=(700, 600), center=True):

    with Column(spacing=20.0, padding=[20.0]):

        # ***********************Style standard types***************************

        add_text(content="Checkboxes with style_std types (check and hover to see changes)")
        add_text(content="For additional color mods, see py_checkbox_new_palette.y demo")

        with Row(spacing=20.0):

            add_checkbox(
                label="Primary (Default)",
                is_checked=True,
                style_std=CheckboxStyleStd.Primary)

            add_checkbox(
                label="Secondary",
                is_checked=True,
                style_std=CheckboxStyleStd.Secondary)

            add_checkbox(
                label="Success",
                is_checked=True,
                style_std=CheckboxStyleStd.Success)

            add_checkbox(
                label="Danger",
                is_checked=True,
                style_std=CheckboxStyleStd.Danger)

        # ***********************Border Options***************************

        add_text(content="Checkbox Border Options")

        with Row(spacing=20.0):

            border_radius = add_checkbox_style(
                border_width=2.0,
                border_radius=8.0)

            border_thick = add_checkbox_style(
                border_width=4.0,
                border_radius=4.0)

            add_checkbox(
                label="Border Radius",
                is_checked=True,
                style_id=border_radius)

            add_checkbox(
                label="Thick Rounded Border",
                is_checked=True,
                style_id=border_thick)

        # ***********************Text Color***************************

        add_text(content="Checkbox Text Color")

        with Row(spacing=20.0):

            text_blue = add_checkbox_style(
                text_color=Color.LIGHT_BLUE)

            text_red = add_checkbox_style(
                text_color=Color.RED)

            add_checkbox(
                label="Light Blue Text",
                style_id=text_blue)

            add_checkbox(
                label="Red Text",
                style_id=text_red)

start_session()
