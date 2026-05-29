#!/usr/bin/env python3
"""
Button styling demo — shows the colour palette for standard button themes,
with live radio-button switching between colour themes.
"""

from icedpygui import (
    Window,
    WindowParam,
    Container,
    add_container_style,
    Column,
    ColorPicker,
    ContainerParam,
    Row,
    start_session,
    add_button,
    add_pick_list,
    add_radio,
    add_text,
    TextParam,
    update_widget,
    update_widget_params,
    get_color_palette,
    Color,
    window_theme_variants,
)

def on_theme_select(_pl_id: int, theme: str):
    """Select a Theme by PickList"""
    update_widget(wnd_id, WindowParam.Theme, theme)
    (_t, c) = theme.split(".")
    c = c.upper()
    state["current_theme_color"] = Color.c


def color_selected(_cp_id: int, color: list[float]):
    """Color picker callback (reserved for future use)."""
    print(color)

def style_type_selected(_rd_id: int, index: int):
    """Swap text in containers when new style type selected"""
    state["style_type"] = style_types[index]

# pylint: disable=redefined-outer-name
def std_colors_selected(_rd_id: int, index: int):
    """Swap container styles when a standard colour radio button is selected."""
    color_name = std_colors[index]
    for name, (style_id, text_rgba) in all_styles[color_name].items():
        update_widget_params(cont_ids[name], {ContainerParam.StyleId: style_id})
        update_widget_params(text_ids[name], {TextParam.ColorRgba: list(text_rgba)})


def get_variants(color: Color) -> list[tuple]:
    """Return sorted (name, color_rgba, text_rgba) tuples for a colour palette."""
    palette = get_color_palette(color)
    sorted_palette = dict(sorted(palette.items()))
    return [
        (k[:-6], list(v), list(sorted_palette[k[:-6] + "_text"]))
        for k, v in sorted_palette.items()
        if k.endswith("_color")
    ]


def get_label(name: str) -> str:
    """Return the display text for a variant tile."""
    match name:
        case "base":
            return "base\nActive/Pressed"
        case "strong":
            return "strong\nHovered"
        case _:
            return name


def get_styling_label(name: str, style_type: str) -> str:
    """retun labels for palettes for styling types"""
    match (name, style_type):
        case ("base", ""):
            return "base\nActive"
        case ("strong", ""):
            return "strong\nPressed"
        case ("weaker", ""):
            return "weaker\nHovered"
        case _:
            return name

# ---------------------------------------------------------------------------
# Data
# ---------------------------------------------------------------------------
std_colors = ["Primary", "Secondary", "Success", "Warning", "Danger"]
style_types = ["Background", "Subtle"]
cont_ids = {}   # {variant_name: cont_id}
text_ids = {}   # {variant_name: text_id}
state = {"style_type": "",
         "current_theme_color": Color.TOKYO_NIGHT}

# Pre-create all styles for every std color before start_session() so they
# can be swapped at runtime via update_widget_params / ContainerParam.StyleId.
all_styles = {}  # {color_name: {variant_name: (style_id, text_rgba)}}
for _color_name in std_colors:
    _color_enum = getattr(Color, _color_name.upper())
    _per_color = {}
    for (_vname, _color_rgba, _text_rgba) in get_variants(_color_enum):
        _per_color[_vname] = (add_container_style(bkg_rgba=_color_rgba), _text_rgba)
        if _vname == "base":
            _alpha_rgba = list(_color_rgba)
            _alpha_rgba[3] = 0.5
            _alpha_text = list(_text_rgba)
            _alpha_text[3] = 0.5
            _per_color["base alpha"] = (add_container_style(bkg_rgba=_alpha_rgba), _alpha_text)
    all_styles[_color_name] = _per_color

# Pre-create all styles for every style types before start_session() so they
# can be swapped at runtime via update_widget_params / ContainerParam.StyleId.
all_style_types = {}  # {style_type: {variant_name: (style_id, text_rgba)}}
for _style_type in style_types:
    _per_color = {}
    for (_vname, _color_rgba, _text_rgba) in get_variants(state["current_theme_color"]):
        _per_color[_vname] = (add_container_style(bkg_rgba=_color_rgba), _text_rgba)

    all_style_types[_style_type] = _per_color



# ---------------------------------------------------------------------------
# GUI — initial display uses PRIMARY
# ---------------------------------------------------------------------------
with Window(title="Button Styling", size=(800, 600), center=True) as wnd_id:

    with Column(spacing=20, padding=[20], wrap=True):

        add_pick_list(options=window_theme_variants(),
                      placeholder="Select Theme", on_select=on_theme_select)

        with Row(spacing=20, wrap=True):
            for _name, (_style_id, _text_rgba) in all_styles["Primary"].items():
                _label = (
                    "base alpha 0.5\nDisabled"
                    if _name == "base alpha"
                    else get_label(_name)
                )
                with Container(width=120, height=60,
                                align_center=True, style_id=_style_id) as _cont_id:
                    cont_ids[_name] = _cont_id
                    text_ids[_name] = add_text(content=_label, color_rgba=_text_rgba, size=14)

        add_radio(
            labels=std_colors,
            selected_index=0,
            horizontal=True,
            radio_spacing=10,
            on_selected=std_colors_selected,
        )

        with Row(spacing=20, wrap=True):
            for _name, (_style_id, _text_rgba) in all_style_types["Background"].items():

                with Container(width=120, height=60,
                                align_center=True, style_id=_style_id) as _cont_id:
                    cont_ids[_name] = _cont_id
                    name = get_styling_label(_name, "")
                    text_ids[_name] = add_text(content=name, color_rgba=_text_rgba, size=14)

        add_radio(
                labels=style_types,
                selected_index=0,
                horizontal=True,
                radio_spacing=10,
                on_selected=style_type_selected,
            )

        with ColorPicker(on_submit=color_selected):
            # A button is required to open the color picker.
            add_button(label="Color Picker", padding=[3.0])

start_session()
