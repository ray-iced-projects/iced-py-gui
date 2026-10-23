#!/usr/bin/env python3
"""
Button use demo

Adds a button widget.
A clickable button used for some gui action.

"""

from icedpygui import (
    Window, WindowParam, window_theme_names,
    Column, Container, Row, start_session,
    add_button, add_button_style, ButtonStyleStd,
    add_font_style, FontWeight,
    add_pick_list,
    add_space,
    add_text,
    Color, update_widget)


def on_theme_select(_pl_id: int, theme_name: str):
    """Update the Window Theme"""
    update_widget(wnd_id, WindowParam.Theme, theme_name)


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

font_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)

#  First add a window
with Window(title="Button Styling Parameters",
            size=(600, 700), center=True) as wnd_id: # id needed for window theme change

    # Add container to hold the button
    with Container(fill=True, padding=[20]):
        with Column(width_fill=True, spacing=10):
            add_text(content=("Select a Theme to see the differences in the styling.\n"
                              "Note: The border and shadow colors won't change with\n"
                              "the theme changes because they were set.\n"
                              "To over come this:\n"
                              "1. Create a palette (see palette examples) for the border\n"
                              "    and/or shadow colors.\n"
                              "2. Update the colors when your theme changes.\n"),
                              font_id=font_id)
            add_text(content="Select a Theme")
            add_pick_list(options=window_theme_names(), selected="TokyoNight",
                        placeholder="Select Theme", on_select=on_theme_select)

            add_text(content="***Some Styling***", width_fill=True,
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
                                       style_id=shadow_color_style
                                       )
                            add_space(height=30)
                            add_text(content="Clip=True, label clipped")
                            add_button(label="This should be clipped", width=140, clip=True)


start_session()
