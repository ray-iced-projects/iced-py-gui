#!/usr/bin/env python3
"""
Checkbox styling use demo
"""

from icedpygui import Window, Column, Row, start_session, \
    add_checkbox, add_checkbox_style, add_text, \
    CheckboxStyleStd, Color


# Style IDs can be created anywhere before use
custom_bkg = add_checkbox_style(
    background_color=Color.LIGHT_BLUE)


with Window(title="Checkbox Styling",
            size=(700, 600), center=True):

    with Column(spacing=20.0, padding=[20.0]):

        # ***********************Style standard types***************************

        add_text(content="Checkboxes with style_std types (check and hover to see changes)")

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

        # ***********************Background Color***************************

        add_text(content="Checkbox Background Color")

        with Row(spacing=20.0):

            add_checkbox(
                label="Custom Background",
                is_checked=True,
                style_id=custom_bkg)

            bkg_yellow = add_checkbox_style(
                background_color=Color.LIGHT_YELLOW)

            add_checkbox(
                label="Yellow Background",
                is_checked=True,
                style_id=bkg_yellow)

        # ***********************Border Options***************************

        add_text(content="Checkbox Border Options")

        with Row(spacing=20.0):

            border_color = add_checkbox_style(
                border_color=Color.RED,
                border_width=2.0)

            border_radius = add_checkbox_style(
                border_color=Color.BLUE,
                border_width=2.0,
                border_radius=[8.0])

            border_thick = add_checkbox_style(
                border_color=Color.DARK_GREEN,
                border_width=4.0,
                border_radius=[4.0])

            add_checkbox(
                label="Border Color + Width",
                is_checked=True,
                style_id=border_color)

            add_checkbox(
                label="Border Radius",
                is_checked=True,
                style_id=border_radius)

            add_checkbox(
                label="Thick Rounded Border",
                is_checked=True,
                style_id=border_thick)

        # ***********************Icon Color***************************

        add_text(content="Checkbox Icon Color")

        with Row(spacing=20.0):

            icon_red = add_checkbox_style(
                icon_color=Color.RED)

            icon_green = add_checkbox_style(
                icon_color=Color.GREEN)

            add_checkbox(
                label="Red Icon",
                is_checked=True,
                style_id=icon_red)

            add_checkbox(
                label="Green Icon",
                is_checked=True,
                style_id=icon_green)

        # ***********************Text Color***************************

        add_text(content="Checkbox Text Color")

        with Row(spacing=20.0):

            text_blue = add_checkbox_style(
                text_color=Color.BLUE)

            text_red = add_checkbox_style(
                text_color=Color.RED)

            add_checkbox(
                label="Blue Text",
                is_checked=True,
                style_id=text_blue)

            add_checkbox(
                label="Red Text",
                is_checked=True,
                style_id=text_red)

        # ***********************Combined Styling***************************

        add_text(content="Checkbox Combined Styling")

        with Row(spacing=20.0):

            combined = add_checkbox_style(
                background_color=Color.DARK_BLUE,
                border_color=Color.GOLD,
                border_width=2.0,
                border_radius=[6.0],
                icon_color=Color.GOLD,
                text_color=Color.DARK_BLUE)

            add_checkbox(
                label="All Custom Styles",
                is_checked=True,
                style_id=combined)

            combined_std = add_checkbox_style(
                border_color=Color.RED,
                border_width=2.0,
                icon_color=Color.ORANGE)

            add_checkbox(
                label="Success + Custom Overrides",
                is_checked=True,
                style_id=combined_std,
                style_std=CheckboxStyleStd.Success)


start_session()
