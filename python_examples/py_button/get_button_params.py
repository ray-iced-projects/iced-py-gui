"""
Demo: read a button's parameters (and its style, font, and palette) during
its on_press callback, plus any other widget by id (a checkbox here).
"""
from icedpygui import (
    Window,
    Container,
    Column,
    start_session,
    add_button,
    add_button_style,
    add_checkbox,
    add_font_style,
    custom_palette,
    FontWeight,
    get_widget_parameters,
    get_widget_style_parameters,
    get_widget_font_parameters,
    get_widget_palette_parameters,
)

# Holds ids created at build time so callbacks can query any widget by id.
state = {"checkbox_id": None}


def _print_dict(title: str, data: dict):
    print(title)
    for key, value in data.items():
        print(f"  {key} = {value!r}")


def on_toggle(wid: int, is_checked: bool):
    """Checkbox toggle handler."""
    print(f"Checkbox {wid} toggled -> {is_checked}")


def on_press(wid: int):
    """Get the pressed button's parameters, its resources, and the checkbox."""
    params = get_widget_parameters(wid)
    _print_dict(f"Button {wid} parameters:", params)

    if params.get("style_id") is not None:
        _print_dict("  style parameters:",
                    get_widget_style_parameters(params["style_id"]))
    if params.get("font_id") is not None:
        _print_dict("  font parameters:",
                    get_widget_font_parameters(params["font_id"]))
    if params.get("palette_id") is not None:
        _print_dict("  palette parameters:",
                    get_widget_palette_parameters(params["palette_id"]))

    # Get any widget by id, not just the pressed button.
    if state["checkbox_id"] is not None:
        _print_dict(f"Checkbox {state['checkbox_id']} parameters:",
                    get_widget_parameters(state["checkbox_id"]))
    print("-" * 40)


with Window(title="Get Button Params", center=True):
    with Container(align_center=True, fill=True):
        with Column(spacing=20):
            style_id = add_button_style(bkg_rgba=[0.2, 0.4, 0.8, 1.0],
                                        border_width=2.0)
            font_id = add_font_style(weight=FontWeight.Bold)
            palette_id = custom_palette(rgba=[0.32, 0.2, 0.13, 1.0])

            add_button(label="Styled Button", width=200, padding=[10],
                       style_id=style_id, font_id=font_id,
                       palette_id=palette_id, on_press=on_press)
            add_button(label="Plain Button", on_press=on_press)

            state["checkbox_id"] = add_checkbox(label="Enable feature",
                                                is_checked=True,
                                                on_toggle=on_toggle)


start_session()
