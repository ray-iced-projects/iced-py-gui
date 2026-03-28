from icedpygui import Window, Container, Column, Row, Float, start_session, \
    add_button, add_text, add_container_style, \
    IpgColor, update_widget, FloatParam, IpgContainerStyleStd, \
    IpgTextParam


# Float allows a widget to float over others, optionally scaled
# and translated.

# global value for the text_id
text_id = 0

def set_mode(btn_id, name):
    match name:
        case "normal":
            update_widget(wid=flt, param=FloatParam.Scale, value=1.0)
            update_widget(wid=flt, param=FloatParam.Translate, value=[0, 0])
            update_widget(wid=flt, param=FloatParam.ScaleClamped, value=None)
            update_widget(wid=text_id, param=IpgTextParam.Content, value="Normal")
        case "scale_only":
            update_widget(wid=flt, param=FloatParam.Scale, value=2.0)
            update_widget(wid=flt, param=FloatParam.Translate, value=[0, 0])
            update_widget(wid=flt, param=FloatParam.ScaleClamped, value=None)
            update_widget(wid=text_id, param=IpgTextParam.Content, value="Scaled Only but outside of window, see clamped")
        case "translated_only":
            update_widget(wid=flt, param=FloatParam.Translate, value=[80, 200])
            update_widget(wid=flt, param=FloatParam.ScaleClamped, value=None)
            update_widget(wid=text_id, param=IpgTextParam.Content, value="Translated Only")
        case "translated_scaled":
            update_widget(wid=flt, param=FloatParam.Translate, value=[50, 100])
            update_widget(wid=flt, param=FloatParam.Scale, value=1.7)
            update_widget(wid=flt, param=FloatParam.ScaleClamped, value=None)
            update_widget(wid=text_id, param=IpgTextParam.Content, value="Translated and Scaled")
        case "scaled_clamped":
            update_widget(wid=flt, param=FloatParam.Scale, value=1.0)
            update_widget(wid=flt, param=FloatParam.Translate, value=[0, 0])
            update_widget(wid=flt, param=FloatParam.ScaleClamped, value=2.0)
            update_widget(wid=text_id, param=IpgTextParam.Content, value="Clamped the Scaling so I remained in window with a padding of [10]")


# Container styling
bg_style = add_container_style(
    background_color=IpgColor.DARK_GRAY,
    border_color=IpgColor.GRAY,
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
                    style_std=IpgContainerStyleStd.Primary,
                    align_center=True,
                ):
                    with Column(spacing=10.0):
                        add_text(content="I'm a Float!", size=20.0)
                        text_id = add_text(content="Normal", size=13.0)
                        
            # Add the container to show the float overlay better
            for i in range(4):
                with Container(
                    width=300,
                    padding=[20.0],
                    style_id=bg_style,
                ):
                    add_text(
                        content=f"Background item {i}",
                        size=14.0,
                        text_color=IpgColor.WHITE,
                    )



start_session()


