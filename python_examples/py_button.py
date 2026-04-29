#!/usr/bin/env python3
"""
Button use demo

By repeatedly pressing the button, the parameters and styles are cycled through

Adds a button widget.
A clickable button used for some gui action.

Parameters
----------
parent_id : str, Optional
    Sets the parent container ID that this button belongs to.
label : str,  Optional
    Sets the Text label displayed on the button.
on_press : callable,  Optional
    Sets the Callback method to invoke when the button is pressed.
width : float,  Optional
    Sets the Fixed Width in logical pixels.
width_fill : bool, default False
    Whether the button fills available width.
height : float,  Optional
    Sets the Fixed Height in logical pixels.
height_fill : bool, default False
    Whether the button fills available height.
fill : bool, Optional
    Whether the button fills available width and height.
padding : list of float,  Optional
    Sets the Padding as [all], [vertical, horizontal], or
    [top, right, bottom, left].
text_top_left : bool,  Optional
    Whether to Align the label to the top-left.
text_top_center : bool,  Optional
    Whether to Align the label to the top-centre.
text_top_right : bool,  Optional
    Whether to Align the label to the top-right.
text_center_left : bool,  Optional
    Whether to Align the label to the centre-left.
text_center : bool,  Optional
    Whether to Align the label to the centre (default True).
text_center_right : bool,  Optional
    Whether to Align the label to the centre-right.
text_bottom_left : bool,  Optional
    Whether to Align the label to the bottom-left.
text_bottom_center : bool,  Optional
    Whether to Align the label to the bottom-centre.
text_bottom_right : bool,  Optional
    Whether to Align the label to the bottom-right.
text_size : float,  Optional
    Sets the Font size for the label text.
 if_menu_btn: bool, Optional
     Whether the button is used in the menu widget, effects the alignment.
clip : bool,  Optional
    Whether to clip content that overflows the button.
style_id : int,  Optional
    Stes the ID of a custom style created with ``add_button_style``.
style_std : ButtonStyleStd,  Optional
    Sets the a predefined standard style variant.
style_arrow : Arrow,  Optional
    Sets an arrow icon style for the button.
user_data : Any,  Optional
    Sets an arbitrary data forwarded to callbacks.
show : bool, default True
    Whether the button is visible.
gen_id : int,  Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.
Returns
-------
int
   The numeric widget ID of the newly created button.
"""

from icedpygui import Window, Column, Container, Row, \
    add_button, add_button_style, add_text, ButtonParam, \
    ButtonStyleStd, TextParam, Arrow, Color, \
    update_widget, update_widget_params, start_session

state = {"txt_id": 0}


# Various style to demonstrate
# Note, unlike the parameter updating, the style resets all of the style parameters
# back to there default values, so they are not additive like the params.
# see how the alpha was used below, the bkg color needed to be added back in.
bkg_color = add_button_style(background_color=Color.RED)

bkg_color_alpha = add_button_style(background_color=Color.RED, background_color_alpha=0.5)

bkg_gradient = add_button_style(
    gradient_color_stops=[Color.BLUE, Color.RED],
    gradient_offset_stops=[0.0, 1.0],
    gradient_degrees=45.0)

border_color = add_button_style(border_color_active=Color.RED, border_width=10.0)

border_color_alpha = add_button_style(border_color_active=Color.RED,
                                      border_width=10.0,
                                      border_color_alpha_active=0.5)
border_rgba = add_button_style(border_rgba_active=[0.0, 0.8, 0.2, 1.0], border_width=10.0)

border_radius_style = add_button_style(border_radius=[20.0],
                                       border_color_active=Color.BLUE,
                                       border_width=2.0)
border_width_style = add_button_style(border_width=10.0, border_color_active=Color.GREEN)

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

text_color_style = add_button_style(background_color=Color.RED)

text_color_alpha = add_button_style(text_color=Color.RED, text_color_alpha=0.5)
text_rgba_style = add_button_style(text_rgba=[0.0, 0.6, 0.0, 1.0])

def on_press(btn_id: int):
    """Callback to change the state of the button"""
    print(btn_id)

#  First add a window
with Window(title="Button Parameters",
            size=(800, 700), center=True):

    # Add container to hold the button
    with Container(fill=True, padding=[20]):
        with Row(width_fill=True):
            with Column(spacing=20):
                add_button(label="Default Button", on_press=on_press)
                add_button(label="Width=150", width=150)
                add_button(label="Height=50", height=50)
                add_text(content="Note: The spacing around the label")
                add_button(label="Padding=[10]", padding=[10])
                add_button(label="Padding=[top=10, right=15,\nbottom=0, left=5]", padding=[10, 15, 0, 5])
                add_button(label="This should be clipped", width=140, clip=True)
                add_button(label="status=Disabled", status_disabled=True)

            with Column(spacing=20):
                add_text(content="Some Styling", width=400, align_center=True)
                with Row(spacing=10):
                    add_button(label="Bkg Color std=Subtle", width=100, height=50,
                                style_std=ButtonStyleStd.Subtle)
                    add_button(label="Color.RED, status=active", status_active=True, style_id=bkg_color)
                    # add_button(label="with color alpha=0.5", style_id=bkg_color_alpha)

start_session()
