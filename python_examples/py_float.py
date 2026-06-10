#!/usr/bin/env python3
"""
Float use demo
"""

from icedpygui import (
    Window,
    Container,
    Column,
    Row,
    Float,
    start_session,
    add_button,
    add_container_style,
    add_text,
    Color,
    ContainerStyleStd,
    FloatParam,
    TextParam,
    update_widget_params,
    update_widget,
    )



# global value for the text_id
state = {"text_id": 0}

# Changes the mode when buttons pressed
def set_mode(_btn_id, name):
    """Sets the mode of the Float"""
    match name:
        case "normal":
            update_widget_params(flt, {FloatParam.Scale: 1.0,
                                       FloatParam.Translate: [0, 0],
                                       FloatParam.ScaleClamped: None})
            update_widget(state["text_id"], TextParam.Content, "Normal")
        case "scale_only":
            update_widget_params(flt, {FloatParam.Scale: 2.0,
                                       FloatParam.Translate: [0, 0],
                                       FloatParam.ScaleClamped: None})
            update_widget(state["text_id"],
                          TextParam.Content,
                          "Scaled Only but outside of window, see clamped")
        case "translated_only":
            update_widget_params(flt, {FloatParam.Translate: [80, 200],
                                FloatParam.ScaleClamped: None})
            update_widget(state["text_id"], TextParam.Content, "Translated Only")
        case "translated_scaled":
            update_widget_params(flt, {FloatParam.Scale: 1.7,
                                       FloatParam.Translate: [50, 100],
                                       FloatParam.ScaleClamped: None})
            update_widget(state["text_id"], TextParam.Content, "Translated and Scaled")
        case "scaled_clamped":
            update_widget_params(flt, {FloatParam.Scale: 1.0,
                                       FloatParam.Translate: [0, 0],
                                       FloatParam.ScaleClamped: 2})
            update_widget(state["text_id"],
                          TextParam.Content,
                          "Clamped the Scaling so I remained in window with a padding of [10]")


# Container styling
bg_style = add_container_style(
    bkg_color=Color.DARK_GRAY,
    border_color=Color.GRAY,
    border_width=1.0,
    border_radius=[8.0],
)

with Window(title="Float Example", center=True):

    with Row(padding=[20.0], spacing=20.0, fill=True):

        # Left side: mode buttons
        with Column(width=220, spacing=10.0):
            add_text(content="Float modes:", size=16.0)
            add_button(
                label="Normal",
                on_press=set_mode,
                user_data="normal",
                width_fill=True,
            )
            add_button(
                label="Scale only (scale 2.0)",
                on_press=set_mode,
                user_data="scale_only",
                width_fill=True,
            )
            add_button(
                label="Scaled_clamped 2.0",
                on_press=set_mode,
                user_data="scaled_clamped",
                width_fill=True,
            )
            add_button(
                label="Translate only (+80, +200)",
                on_press=set_mode,
                user_data="translated_only",
                width_fill=True,
            )
            add_button(
                label="Translate_scaled (+50, +100 * 1.7)",
                on_press=set_mode,
                user_data="translated_scaled",
                width_fill=True,
            )


        # Right side: float card over background items
        with Column(width_fill=True, spacing=12.0, align_center=True):

            with Float(scale=1.0, translate=[0.0, 0.0], clamped_padding=[10]) as flt:
                with Container(
                    width=200,
                    height=100,
                    style_std=ContainerStyleStd.Primary,
                    align_center=True,
                ):
                    with Column(spacing=10.0):
                        add_text(content="I'm a Float!", size=20.0)
                        state["text_id"] = add_text(content="Normal", size=13.0)

            # Add some containers to show the float overlay better
            for i in range(4):
                with Container(
                    width=300,
                    padding=[20.0],
                    style_id=bg_style,
                ):
                    add_text(
                        content=f"Background item {i}",
                        size=14.0,
                        color=Color.WHITE,
                    )

# Must be last
start_session()
