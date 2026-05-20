#!/usr/bin/env python3
"""
Button use demo
"""

from icedpygui import (
    Window,
    Container,
    Column,
    Scrollable,
    start_session,
    add_button,
    add_button_style,
    Color)



def on_press(btn_id: int):
    """Callback to change the state of the button"""
    print(btn_id)

bkg_color = add_button_style(bkg_color=Color.RED)

# Scenario 1: global text_color only — all statuses should show BLUE text
style_global_text = add_button_style(bkg_color=Color.RED, text_color=Color.BLUE)

# Scenario 2: text_color_active only — active, hovered, pressed, disabled all fall back to GREEN
style_active_base = add_button_style(bkg_color=Color.RED, text_color_active=Color.GREEN)

# Scenario 3: text_color_active (YELLOW) + selective overrides: hovered=BLUE, disabled=DARK_GRAY
style_partial = add_button_style(
    bkg_color=Color.RED,
    text_color_active=Color.YELLOW,
    text_color_hovered=Color.BLUE,
    text_color_disabled=Color.DARK_GRAY,
)

# Scenario 4: global text_color (WHITE) + text_color_active (BLUE) overrides all statuses
style_global_and_active = add_button_style(
    bkg_color=Color.RED,
    text_color=Color.WHITE,
    text_color_active=Color.BLUE,
)

# Scenario 5: only text_color_pressed=YELLOW set —
# pressed should be YELLOW, others use theme default
style_pressed_only = add_button_style(bkg_color=Color.RED, text_color_pressed=Color.YELLOW)

#  First add a window
with Window(title="Button Parameters",
            size=(600, 600), center=True):

    # Add container to hold the button
    with Container(fill=True, align_center=True):
        with Scrollable():
            with Column(spacing=10):
                add_button(label="status=Active", status_active=True, style_id=bkg_color)
                add_button(label="status=Hovered", status_hovered=True, style_id=bkg_color)
                add_button(label="status=Pressed", status_pressed=True, style_id=bkg_color)
                add_button(label="status=Disabled", status_disabled=True, style_id=bkg_color)



start_session()
