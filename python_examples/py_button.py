#!/usr/bin/env python3
"""
Button use demo

By repeatedly pressing the button, the parameters and styles are cycled through
"""

from icedpygui import Window, Column, Container, \
    add_button, add_button_style, add_text, ButtonParam, ButtonStyleStd, \
    ButtonStyleParam, TextParam, Arrow, Color, \
    update_widget, update_widget_params, start_session

counter = 1
txt_id = 0

# Various style to demonstrate
# Note, unlike the parameter updating, the style resets all of the style parameters
# back to there default values, so they are not additive like the params.
# see how the alpha was used below, the bkg color needed to be added back in.
bkg_color = add_button_style(background_color=Color.LIGHT_BLUE)
bkg_color_alpha = add_button_style(background_color=Color.LIGHT_BLUE, background_color_alpha=0.5)
bkg_rgba = add_button_style(background_color_alpha=0.5)
bkg_gradient = add_button_style(
    background_gradient_color_stop=Color.BLUE,
    background_gradient_degrees=45.0,
    background_color=Color.RED)

border_color = add_button_style(border_color=Color.RED, border_width=10.0)
border_color_alpha = add_button_style(border_color=Color.RED, border_width=10.0, border_color_alpha=0.5)
border_rgba = add_button_style(border_rgba=[0.0, 0.8, 0.2, 1.0], border_width=10.0)
border_radius_style = add_button_style(border_radius=[20.0], border_color=Color.BLUE, border_width=2.0)
border_width_style = add_button_style(border_width=10.0, border_color=Color.GREEN)

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
shadow_blur_style = add_button_style(
    shadow_color=Color.LIGHT_BLUE, shadow_offset_xy=[3.0, 3.0], shadow_blur_radius=20.0)

text_color_style = add_button_style(text_color=Color.RED)
text_color_alpha = add_button_style(text_color=Color.RED, text_color_alpha=0.5)
text_rgba_style = add_button_style(text_rgba=[0.0, 0.6, 0.0, 1.0])

def on_press(btn_id: int):
    global counter
    global txt_id
    match counter:
        # Buttom parameters, changed using the ButtonParam class
        case 0:
            update_widget(txt_id, TextParam.Content, "Param = None, keep pressing")
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Width: None,
                ButtonParam.Height: None,
                ButtonParam.StyleId: None,
                ButtonParam.Label: "Press Me"})
        case 1:
            update_widget(txt_id, TextParam.Content, "Param = Clip, keep pressing")
            update_widget_params(btn_id, {
                ButtonParam.Clip: True,
                ButtonParam.Width: 75.0, # width needs to be set or button shrinks to size or wraps if container is smaller
                ButtonParam.Label: "This is a clipped label"})
        case 2:
            update_widget(txt_id, TextParam.Content, "Param = Height 50.0, keep pressing")
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Clip: False,
                ButtonParam.Width: None,
                ButtonParam.Label: "Height 50.0",
                # param to set
                ButtonParam.Height: 50.0})
        case 3:
            update_widget(txt_id, TextParam.Content, "Param = Padding - all sides[20], keep pressing")
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Height: None,
                ButtonParam.Label: "Padding all sides",
                # param to set
                ButtonParam.Padding: [20.0]})
        case 4:
            update_widget(txt_id, TextParam.Content, "Param = Padding - top/bottom [20, 0, 20, 0], keep pressing")
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Padding top/bottom",
                # param to set
                ButtonParam.Padding: [20, 0, 20, 0]})
        case 5:
            update_widget(txt_id, TextParam.Content, "Param = Padding - left/right [0, 20, 0, 20], keep pressing")
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Padding left/right",
                # param to set
                ButtonParam.Padding: [0, 20, 0, 20]})
        case 6:
            update_widget(txt_id, TextParam.Content, "Param = StyleArrow, keep pressing") 
            update_widget_params(btn_id, {
                ButtonParam.Padding: [10], # just the show the arrow better
                # param to set
                ButtonParam.StyleArrow: Arrow.ArrowRight})
        case 7:
            update_widget(txt_id, TextParam.Content, "Param = StyleStd - Danger, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.StyleArrow: Arrow.ArrowNone, # resets the arrow style to None
                ButtonParam.Label: "I'm a Danger colored button",
                # param to set
                ButtonParam.StyleStd: ButtonStyleStd.Danger})
        case 8:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignBottomCenter=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Padding: None,
                ButtonParam.Label: "Param = TextAlignBottomCenter, keep pressing",
                # Make the button bigger to show alingment better
                ButtonParam.Width: 600.0,
                ButtonParam.Height: 200.0,
                ButtonParam.StyleStd: ButtonStyleStd.Primary, # default style
                # param to set
                ButtonParam.TextAlignBottomCenter: True})
        case 9:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignBottomLeft=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignBottomLeft, keep pressing",
                ButtonParam.TextAlignBottomCenter: False,
                # param to set
                ButtonParam.TextAlignBottomLeft: True})
        case 10:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignBottomRight=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignBottomRight, keep pressing",
                ButtonParam.TextAlignBottomLeft: False,
                # param to set
                ButtonParam.TextAlignBottomRight: True})
        case 11:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignCenter=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignCenter, keep pressing",
                ButtonParam.TextAlignBottomRight: False,
                # param to set
                ButtonParam.TextAlignCenter: True})
        case 12:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignCenterLeft=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignCenterLeft, keep pressing",
                ButtonParam.TextAlignCenter: False,
                # param to set
                ButtonParam.TextAlignCenterLeft: True})
        case 13:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignCenterRight=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignCenterRight, keep pressing",
                ButtonParam.TextAlignCenterLeft: False,
                # param to set
                ButtonParam.TextAlignCenterRight: True})
        case 14:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignTopCenter=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignTopCenter, keep pressing",
                ButtonParam.TextAlignCenterRight: False,
                # param to set
                ButtonParam.TextAlignTopCenter: True})
        case 15:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignTopLeft=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignTopLeft, keep pressing",
                ButtonParam.TextAlignTopCenter: False,
                # param to set
                ButtonParam.TextAlignTopLeft: True})
        case 16:
            update_widget(txt_id, TextParam.Content, "Param = TextAlignTopRight=True, keep pressing") 
            update_widget_params(btn_id, {
                # reset some of the previous parameters
                ButtonParam.Label: "Param = TextAlignTopRight, keep pressing",
                ButtonParam.TextAlignTopLeft: False,
                # param to set
                ButtonParam.TextAlignTopRight: True})
        case 17:
            update_widget(txt_id, TextParam.Content, "Param = default, keep pressing") 
            update_widget_params(btn_id, {
            # reset some of the previous parameters
            ButtonParam.Width: None,
            ButtonParam.Height: None,
            ButtonParam.TextAlignTopRight: None,
            ButtonParam.Label: "Press Me to see some custom styling"})
        
        # Button Style parameters changed using StyleId
        case 18:
            update_widget(txt_id, TextParam.Content, "Param = background_color, keep pressing") 
            update_widget_params(btn_id, {
            # reset some of the previous parameters
            ButtonParam.Width: 300,
            ButtonParam.Height: 100,
            ButtonParam.TextAlignCenter: True,
            ButtonParam.Label: "background color",
            # param to set
            ButtonParam.StyleId: bkg_color})
        case 19:
            update_widget(txt_id, TextParam.Content, "Param = background_color_alpha, keep pressing") 
            update_widget_params(btn_id, {
            # reset some of the previous parameters
            ButtonParam.Label: "background color alpha",
            # param to set
            ButtonParam.StyleId: bkg_color_alpha})
        case 20:
            update_widget(txt_id, TextParam.Content, "Param = background_rgba, keep pressing") 
            update_widget_params(btn_id, {
            # reset some of the previous parameters
            ButtonParam.Label: "background rgba",
            # param to set
            ButtonParam.StyleId: bkg_rgba})
        case 21:
            update_widget(txt_id, TextParam.Content, "Param = background_gradient, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "background gradient",
            # param to set
            ButtonParam.StyleId: bkg_gradient})
        case 22:
            update_widget(txt_id, TextParam.Content, "Param = border_color, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "border color",
            # param to set
            ButtonParam.StyleId: border_color})
        case 23:
            update_widget(txt_id, TextParam.Content, "Param = border_color alpha, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "border color alpha",
            # param to set
            ButtonParam.StyleId: border_color_alpha})
        case 24:
            update_widget(txt_id, TextParam.Content, "Param = border_rgba, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "border rgba",
            # param to set
            ButtonParam.StyleId: border_rgba})
        case 25:
            update_widget(txt_id, TextParam.Content, "Param = border_radius, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "border radius",
            # param to set
            ButtonParam.StyleId: border_radius_style})
        case 26:
            update_widget(txt_id, TextParam.Content, "Param = border_width, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "border width",
            # param to set
            ButtonParam.StyleId: border_width_style})
        case 27:
            update_widget(txt_id, TextParam.Content, "Param = shadow_color, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "shadow color",
            # param to set
            ButtonParam.StyleId: shadow_color_style})
        case 28:
            update_widget(txt_id, TextParam.Content, "Param = shadow_color alpha, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "shadow color alpha",
            # param to set
            ButtonParam.StyleId: shadow_color_alpha})
        case 29:
            update_widget(txt_id, TextParam.Content, "Param = shadow_rgba, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "shadow rgba",
            # param to set
            ButtonParam.StyleId: shadow_rgba_style})
        case 30:
            update_widget(txt_id, TextParam.Content, "Param = shadow_offset_xy, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "shadow offset",
            # param to set
            ButtonParam.StyleId: shadow_offset_style})
        case 31:
            update_widget(txt_id, TextParam.Content, "Param = shadow_blur_radius, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "shadow blur radius",
            # param to set
            ButtonParam.StyleId: shadow_blur_style})
        case 32:
            update_widget(txt_id, TextParam.Content, "Param = text_color, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "text color",
            # param to set
            ButtonParam.StyleId: text_color_style})
        case 33:
            update_widget(txt_id, TextParam.Content, "Param = text_color_alpha, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "text color alpha",
            # param to set
            ButtonParam.StyleId: text_color_alpha})
        case 34:
            update_widget(txt_id, TextParam.Content, "Param = text_rgba, keep pressing") 
            update_widget_params(btn_id, {
            ButtonParam.Label: "text rgba",
            # param to set
            ButtonParam.StyleId: text_rgba_style})
    
    
    counter += 1
    if counter > 34:
        counter = 0
    
#  First add a window
with Window(title="Button Parameters",
            size=(800, 600), center=True):

    # Add container to hold the button
    with Container(align_center=True, fill=True):
        with Column(spacing=20):
            txt_id = add_text(content="Default Button, press start")
            add_button(label="Press Me", on_press=on_press)

start_session()
