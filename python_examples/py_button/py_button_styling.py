#!/usr/bin/env python3
"""
Button use demo

Adds a button widget.
A clickable button used for some gui action.

"""

from icedpygui import (Window, Column, Container, Row,
    add_button, add_text, add_button_style,
    ButtonStyleStd, Color, start_session)

state = {"txt_id": 0}


# Various style to demonstrate
# Note, unlike the parameter updating, the style resets all of the style parameters
# back to there default values, so they are not additive like the params.
# see how the alpha was used below, the bkg color needed to be added back in.

border = add_button_style(
            border_color=Color.BLUE,
            border_radius=[20.0],
            border_width=2.0)

# shadow_blur_radius must be also set along with color, offset is optional
shadow_color_style = add_button_style(
    shadow_color=Color.LIGHT_BLUE, shadow_blur_radius=5, shadow_offset_xy=[10.0, 15.0])

def on_press(btn_id: int):
    """Callback to change the state of the button"""
    print(btn_id)

#  First add a window
with Window(title="Button Styling Parameters",
            size=(600, 500), center=True):

    # Add container to hold the button
    with Container(fill=True, padding=[20]):
        with Column(width_fill=True, spacing=10):
            add_text(content="Some Styling", width_fill=True,
                     width=400, align_center=True)
            with Row(width_fill=True, spacing=20):
                with Column(spacing=20):
                    add_button(label="Default Button", on_press=on_press)
                    add_button(label="Width=150", width=150)
                    add_button(label="Height=50", height=50)
                    add_text(content="Note: The spacing around the label")
                    add_button(label="Padding=[10]", padding=[10])
                    add_button(label="Padding=[top=10, right=15,\nbottom=0, left=5]",
                            padding=[10, 15, 0, 5])
                    add_text(content="Clip=True, label clipped")
                    add_button(label="This should be clipped", width=140, clip=True)

                with Column(spacing=20):

                    with Row(spacing=10):
                        with Column(spacing=10):
                            add_button(label="ButtonStyleStd.Subtle", width=200,
                                        style_std=ButtonStyleStd.Subtle)
                            add_button(label="ButtonStyleStd.Danger", width=200,
                                        style_std=ButtonStyleStd.Danger)
                            add_button(label="ButtonStyleStd.Text", width=200,
                                        style_std=ButtonStyleStd.Text)
                            add_button(label="Some Border Styling", padding=[5], style_id=border)
                            add_button(label="Button with Shadow",
                                       padding=[5],
                                       style_std=ButtonStyleStd.Primary,
                                       style_id=shadow_color_style
                                       )


start_session()
