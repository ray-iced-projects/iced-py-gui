#!/usr/bin/env python3
"""
Button use demo

Adds a button widget.
A clickable button used for some gui action.

"""

from icedpygui import Window, Column, Container, Row, Scrollable,\
    add_button, add_button_style, add_text, add_space, \
    ButtonStyleStd, Color, start_session

state = {"txt_id": 0}


# Various style to demonstrate
# Note, unlike the parameter updating, the style resets all of the style parameters
# back to there default values, so they are not additive like the params.
# see how the alpha was used below, the bkg color needed to be added back in.

# Standard styles are:
# Background,
# Danger,
# Primary,
# Secondary,
# Subtle (unique settings),
# Success,
# Warning,
# Text,

# Status    |  Standard Styles
# Active    |  base
# Hovered   |  strong
# Pressed   |  base
# Disabled  |  base => background scale_alpha(0.5)

# Status    |  Text button
# Active    |  base
# Hovered   |  base text scale alpha(0.8)
# Pressed   |  base
# Disabled  |  base => background scale_alpha(0.5)

# Status    |  Background Custom Colors
# Active    |  base
# Hovered   |  weak
# Pressed   |  strong
# Disabled  |  base => background scale_alpha(0.5)

# Status    |  Standard Style Subtle (unique)
# Active    |  base
# Hovered   |  strong
# Pressed   |  base
# Disabled  |  base => background scale_alpha(0.5)


bkg_color = add_button_style(bkg_color=Color.RED)

bkg_color_alpha = add_button_style(bkg_color=Color.RED, bkg_color_alpha=0.5)

bkg_gradient = add_button_style(
    gradient_color_stops=[Color.BLUE, Color.RED],
    gradient_offset_stops=[0.0, 1.0],
    gradient_degrees=45.0)


border = add_button_style(
            border_color_active=Color.BLUE,
            border_radius=[20.0],
            border_width=2.0)


shadow_color_style = add_button_style(
    shadow_color=Color.TAN, shadow_offset_xy=[10.0, 15.0])

shadow_color_alpha = add_button_style(
    shadow_color=Color.TAN,
    shadow_offset_xy=[10.0, 15.0],
    shadow_color_alpha=0.5)

shadow_rgba_style = add_button_style(
    shadow_rgba=[0.5, 0.0, 0.5, 0.8], shadow_offset_xy=[5.0, 5.0], shadow_blur_radius=10.0)

shadow_offset_style = add_button_style(
    shadow_color=Color.LIGHT_BLUE, shadow_offset_xy=[10.0, 10.0], shadow_blur_radius=5.0)

hadow_blur_style = add_button_style(
    shadow_color=Color.LIGHT_BLUE, shadow_offset_xy=[3.0, 3.0], shadow_blur_radius=20.0)

text_color_style = add_button_style(bkg_color=Color.RED)

text_color_alpha = add_button_style(text_color=Color.RED, text_color_alpha=0.5)
text_rgba_style = add_button_style(text_rgba=[0.0, 0.6, 0.0, 1.0])

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


CONTENT_STATUS="You can lock your button into a status state,\n\
if you want custom colors, then use the statuses to see the differences.\n\
See the color_custom for more details."

def on_press(btn_id: int):
    """Callback to change the state of the button"""
    print(btn_id)

#  First add a window
with Window(title="Button Parameters",
            size=(1000, 700), center=True):

    # Add container to hold the button
    with Container(fill=True, padding=[20]):
        with Row(width_fill=True, spacing=20):
            with Column(spacing=20):
                add_button(label="Default Button", on_press=on_press)
                add_button(label="Width=150", width=150)
                add_button(label="Height=50", height=50)
                add_text(content="Note: The spacing around the label")
                add_button(label="Padding=[10]", padding=[10])
                add_button(label="Padding=[top=10, right=15,\nbottom=0, left=5]",
                           padding=[10, 15, 0, 5])
                add_button(label="This should be clipped", width=140, clip=True)

            with Column(spacing=20):
                add_text(content="Some Styling", width=400, align_center=True)
                with Row(spacing=10):
                    with Column(spacing=10):
                        add_text(content="Some Standard styling")
                        add_button(label="ButtonStyleStd.Subtle", width=200,
                                    style_std=ButtonStyleStd.Subtle)
                        add_button(label="ButtonStyleStd.Danger", width=200,
                                    style_std=ButtonStyleStd.Danger)
                        add_button(label="ButtonStyleStd.Text", width=200,
                                    style_std=ButtonStyleStd.Text)
                        add_text(content="Border styling")
                        add_button(label="Some Border Styling", padding=[5], style_id=border)

                    with Scrollable():
                        with Column(spacing=10):
                            add_text(content=CONTENT_STATUS)
                            add_button(label="status=Active", status_active=True,
                                    style_id=bkg_color)
                            add_button(label="status=Hovered", status_hovered=True,
                                    style_id=bkg_color)
                            add_button(label="status=Pressed", status_pressed=True,
                                    style_id=bkg_color)
                            add_button(label="status=Disabled", status_disabled=True,
                                    style_id=bkg_color)

                            add_space(height=30)

                            # Scenario 1: global text_color=BLUE —
                            # all four statuses should show blue text
                            add_text(content="Scenario 1: global text_color=BLUE (all statuses)")
                            add_button(label="Active — expect BLUE text",
                                    status_active=True, style_id=style_global_text)
                            add_button(label="Hovered — expect BLUE text",
                                    status_hovered=True, style_id=style_global_text)
                            add_button(label="Pressed — expect BLUE text",
                                    status_pressed=True, style_id=style_global_text)
                            add_button(label="Disabled — expect BLUE text",
                                    status_disabled=True, style_id=style_global_text)

                            # Scenario 2: text_color_active=GREEN only —
                            # all statuses fall back to GREEN
                            add_text(content="Scenario 2: text_color_active=GREEN \
                                (all statuses fall back)")
                            add_button(label="Active — expect GREEN text",
                                    status_active=True, style_id=style_active_base)
                            add_button(label="Hovered — expect GREEN text",
                                    status_hovered=True, style_id=style_active_base)
                            add_button(label="Pressed — expect GREEN text",
                                    status_pressed=True, style_id=style_active_base)
                            add_button(label="Disabled — expect GREEN text",
                                    status_disabled=True, style_id=style_active_base)

                            # Scenario 3: active=YELLOW, hovered=BLUE, disabled=DARK_GRAY,
                            # pressed falls back to YELLOW
                            add_text(content="Scenario 3: active=YELLOW, hovered=BLUE, \
                                disabled=DARK_GRAY")
                            add_button(label="Active — expect YELLOW text",
                                    status_active=True, style_id=style_partial)
                            add_button(label="Hovered — expect BLUE text",
                                    status_hovered=True, style_id=style_partial)
                            add_button(label="Pressed — expect YELLOW text",
                                    status_pressed=True, style_id=style_partial)
                            add_button(label="Disabled — expect DARK_GRAY text",
                                    status_disabled=True, style_id=style_partial)

                            # Scenario 4: global text_color=WHITE,
                            # text_color_active=BLUE — active overrides global
                            add_text(content="Scenario 4: global=WHITE but \
text_color_active=BLUE overrides")
                            add_button(label="Active — expect BLUE text",
                                    status_active=True, style_id=style_global_and_active)
                            add_button(label="Hovered — expect BLUE text",
                                    status_hovered=True, style_id=style_global_and_active)
                            add_button(label="Pressed — expect BLUE text",
                                    status_pressed=True, style_id=style_global_and_active)
                            add_button(label="Disabled — expect BLUE text",
                                    status_disabled=True, style_id=style_global_and_active)

                            # Scenario 5: only pressed=YELLOW —
                            # active/hovered/disabled use theme default
                            add_text(content="Scenario 5: only text_color_pressed=YELLOW")
                            add_button(label="Active — expect theme default text",
                                    status_active=True, style_id=style_pressed_only)
                            add_button(label="Hovered — expect theme default text",
                                    status_hovered=True, style_id=style_pressed_only)
                            add_button(label="Pressed — expect YELLOW text",
                                    status_pressed=True, style_id=style_pressed_only)
                            add_button(label="Disabled — expect theme default text",
                                    status_disabled=True, style_id=style_pressed_only)


start_session()
