from icedpygui import Window, Container, Column, Row, Float, start_session, \
    add_button, add_text, add_container_style, \
    IpgColor, update_widget, FloatParam


# A simple float example inspired by the Rust float example.
# Float allows a widget to float over others, optionally scaled
# and translated.

def set_mode(btn_id, name):
    match name:
        case "scale_only":
            print("scale_only")
            update_widget(wid=float, param=FloatParam.Scale, value=1.4)
            update_widget(wid=float, param=FloatParam.Translate, value=[0, 0])
        case "translated_only":
            update_widget(wid=float, param=FloatParam.Translate, value=[80, 200])
        case "translated_scaled":
            update_widget(wid=float, param=FloatParam.Translate, value=[50, 100])
            update_widget(wid=float, param=FloatParam.Scale, value=1.7)
        case "scaled_clamped":
            update_widget(wid=float, param=FloatParam.Scale, value=1.0)
            update_widget(wid=float, param=FloatParam.ScaleClamped, value=10)


with Window(title="Float Example", center=True):

    with Row(padding=[20.0], spacing=20.0, fill=True):

        # Left side: mode buttons
        with Column(width=220, spacing=8.0):
            add_text(content="Float modes:", size=16.0)
            add_button(
                label="Scale only (scale 1.4)",
                on_press=set_mode,
                user_data="scale_only",
                width_fill=True,
            )
            add_button(
                label="Translate only (+80, +200)",
                on_press=set_mode,
                user_data="translated_only",
                width_fill=True,
            )
            add_button(
                label="Translate_scaled (+80, +100 * 1.5)",
                on_press=set_mode,
                user_data="translated_scaled",
                width_fill=True,
            )
            add_button(
                label="Scaled_clamped",
                on_press=set_mode,
                user_data="scaled_clamped",
                width_fill=True,
            )

        # Right side: float card over background items
        with Column(width_fill=True, spacing=12.0, align_center=True):

            # The float card
            card_style = add_container_style(
                background_color=IpgColor.LIGHT_BLUE,
                border_color=IpgColor.BLUE,
                border_width=1.0,
                border_radius=[12.0],
            )

            with Float(scale=1.0, translate=[0.0, 0.0]) as float:
                with Container(
                    padding=[30.0],
                    style_id=card_style,
                    align_center=True,
                ):
                    with Column(spacing=8.0):
                        add_text(content="I'm a Float!", size=20.0)
                        add_text(content="Normal", size=13.0)

            # Background items
            bg_style = add_container_style(
                background_color=IpgColor.DARK_GRAY,
                border_color=IpgColor.GRAY,
                border_width=1.0,
                border_radius=[8.0],
            )

            for i in range(4):
                with Container(
                    width_fill=True,
                    padding=[20.0],
                    style_id=bg_style,
                ):
                    add_text(
                        content=f"Background item {i}",
                        size=14.0,
                        text_color=IpgColor.WHITE,
                    )


start_session()


