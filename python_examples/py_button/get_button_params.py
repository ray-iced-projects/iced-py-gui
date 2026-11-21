"""
Demo: read a button's parameters (and its style, font, and palette) during
its on_press callback, plus any other widget by id (a checkbox here).
"""
from icedpygui import (
    Window, start_session,
    Container, Column, ColumnParam, Row,
    add_button, ButtonParam,
    add_space,
    add_text,
    TextParam,
    get_widget_parameters,
    update_widget,
)

def print_dict(data: dict):
    """Displaying the parameters"""
    _content = ("Any time parameters are needed,\n"
               "You can get them by using the\n"
               "get_widget_paramters(wid=num)\n\n")
    for key, value in data.items():
        _content += (f"{key} = {value}\n")
    update_widget(txt_id, TextParam.Content, _content)
    update_widget(right_col_id, ColumnParam.Show, True)


def on_press(btn_id: int):
    """Get the button's parameters"""
    params = get_widget_parameters(btn_id)
    print_dict(params)

btn_params = [
    ButtonParam.Clip, ButtonParam.Fill, ButtonParam.FontId,
    ButtonParam.Height, ButtonParam.HeightFill, ButtonParam.Label,
    ButtonParam.Padding, ButtonParam.PaletteId, ButtonParam.Show,
    ButtonParam.StatusActive, ButtonParam.StatusDisabled, ButtonParam.StatusHovered,
    ButtonParam.StatusPressed, ButtonParam.StyleArrow, ButtonParam.StyleId,
    ButtonParam.StyleStd, ButtonParam.Width, ButtonParam.WidthFill]


with Window(title="Get Button Params", center=True):

    with Container(align_center=True, fill=True):
        with Row(width_fill=True, padding=[50], spacing=50):
            with Column(spacing=20, align_center=True):
                wid = add_button(label="Press to get the Button parameters",
                                on_press=on_press)
                txt_id = add_text(content="")

            with Column(spacing=20, align_center=True, show=False) as right_col_id:
                add_text(content=("When using the update_widget() function\n"
                                  "The param is the UpperCamelCase of the\n"
                                  "parameter name, preceeded by widget param name.\n"
                                  "Which in this case is 'ButtonParam', as seen below"))
                for (i, p) in enumerate(btn_params):
                    with Row(spacing=10):
                        add_text(content=str(btn_params[i]), width=200)
                        if i < len(btn_params)-1:
                            add_text(content=str(btn_params[i+1]), width=200)
                        else:
                            add_space(width=200)

start_session()
