#!/usr/bin/env python3
"""
Button styling demo — shows the colour palette for standard button themes,
with live radio-button switching between colour themes.
"""

from icedpygui import (
    Window,
    WindowParam,
    WindowTheme,
    add_container,
    add_container_style,
    ContainerParam,
    Column,
    Row,
    start_session,
    add_combobox,
    add_combobox_menu_style,
    add_combobox_input_style,
    add_pick_list,
    Scrollable,
    add_text,
    update_widget,
    get_styling_palette,
    StdColorStyle,
    window_theme_names,
    add_font_style,
    FontWeight,
    generate_id,
    add_space,
)


def update_tiles(tiles: list[(str, str, str, int, int)]):
    """Update the tiles after theme change"""
    # ("bkg", "Background", "bkg_base", COLOR, gen_id)
    for (color, _parameter, key, c_t, tile_id) in tiles:
        style_id = add_container_style(
                bkg_rgba=state[color][key][c_t])
        update_widget(tile_id, ContainerParam.StyleId, style_id)



def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    update_widget(state["wnd_id"], WindowParam.Theme, theme_name)
    state["selected_theme"] = theme_name
    state["bkg"] = get_styling_palette(theme_name, StdColorStyle.Primary)
    state["primary"] = get_styling_palette(theme_name, StdColorStyle.Primary)
    state["secondary"] = get_styling_palette(theme_name, StdColorStyle.Secondary)
    update_tiles(input_active)
    update_tiles(input_hovered)
    update_tiles(input_focused)
    update_tiles(input_disabled)
    update_tiles(menu_tiles)
    update_tiles(bkg_tiles)
    update_tiles(bkg_tiles_text)
    update_tiles(primary_tiles)
    update_tiles(secondary_tiles)


def make_tiles(tiles: list[(str, str, str, int, int)]):
    """Make the tiles"""
    # ("bkg", "Background", "bkg_base", COLOR, gen_id)
    with Column():
        with Row(spacing=5):
            for (color_pal, parameter, key, c_t, tile_id) in tiles:
                content=f"{parameter}\n{key}\n{(f"{color_pal.capitalize()} Color"
                        if c_t is COLOR else f"{color_pal.capitalize()} Text")}"
                add_text(content=content, width=120)
        with Row(spacing=5):
            for (color_pal, parameter, key, c_t, tile_id) in tiles:
                style_id = add_container_style(
                bkg_rgba=state[color_pal][key][c_t])
                add_container(width=120, height=20, style_id=style_id, gen_id=tile_id)


# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------

state = {"selected_style": "",
         "selected_theme": "Light",
         "wnd_id": 0,
         "bkg":  get_styling_palette("Light", StdColorStyle.Primary),
         "primary": get_styling_palette("Light", StdColorStyle.Primary),
         "secondary": get_styling_palette("Light", StdColorStyle.Secondary),
         }

# Index into (bkg_rgba, text_rgba) palette tuples
TEXT = 1
COLOR = 0


input_active =[
    ("bkg",         "Background",   "bkg_base",     COLOR,  generate_id() ),
    ("bkg",         "Border",       "bkg_strong",   COLOR,  generate_id() ),
    ("bkg",         "Icon",         "bkg_weak",     TEXT,   generate_id() ),
    ("secondary",   "Placeholder",  "base",         COLOR,  generate_id() ),
    ("bkg",         "Value" ,       "bkg_base",     TEXT,   generate_id() ),
    ("primary",     "Selection",    "weak",         COLOR,  generate_id() ),
    ]

input_hovered = [
    ("bkg", "Border", "bkg_base", TEXT, generate_id()),
]

input_focused= [
    ("primary", "Border", "strong", COLOR, generate_id()),
]

input_disabled= [
    ("bkg",         "Background",   "bkg_weak",      COLOR,  generate_id() ),
    ("secondary",   "Value",        "base",          COLOR,  generate_id() ),
    ("bkg",         "Placeholder",  "bkg_strongest", COLOR,  generate_id() ),
]

menu_tiles = [
    ("bkg",     "Background",   "bkg_weak",     COLOR,  generate_id() ),
    ("bkg",     "Border",       "bkg_strong",   COLOR,  generate_id() ),
    ("bkg",     "Text",         "bkg_weak",     TEXT,   generate_id() ),
    ("primary", "Selected_Text","strong",       TEXT,   generate_id() ),
    ("primary", "Selected_Bkg", "strong",       COLOR,  generate_id() ),
]

# ComboBox data
combo_list = ["Hello", "World"]

def selected_item(_cid: int, _idx: int):
    """Callback for Combobox"""

# Palette keys the first 3 keys are associated with Primary and Secondary colors
# The bkg keys are associated with the theme.
# 'base',
# 'strong',
# 'weak',
bkg_tiles = [
    ("bkg", "Background", "bkg_base",       COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_neutral',    COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_strong',     COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_stronger',   COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_strongest',  COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_weak',       COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_weaker',     COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_weakest',    COLOR,  generate_id() ),
    ]

bkg_tiles_text = [
    ("bkg", "Background", "bkg_base",       TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_neutral',    TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_strong',     TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_stronger',   TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_strongest',  TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_weak',       TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_weaker',     TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_weakest',    TEXT,   generate_id() ),
    ]

primary_tiles = [
    ("primary", "",  "base",   COLOR, generate_id() ),
    ("primary", "",  "base",   TEXT,  generate_id() ),
    ("primary", "",  "weak",   COLOR, generate_id() ),
    ("primary", "",  "weak",   TEXT,  generate_id() ),
    ("primary", "",  "strong", COLOR, generate_id() ),
    ("primary", "",  "strong", TEXT,  generate_id() ),
]

secondary_tiles = [
    ("secondary", "",  "base",   COLOR, generate_id() ),
    ("secondary", "",  "base",   TEXT,  generate_id() ),
    ("secondary", "",  "weak",   COLOR, generate_id() ),
    ("secondary", "",  "weak",   TEXT,  generate_id() ),
    ("secondary", "",  "strong", COLOR, generate_id() ),
    ("secondary", "",  "strong", TEXT,  generate_id() ),
]

font_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)

# ---------------------------------------------------------------------------
# GUI — initial display with a Light background (Selectable by PickList)
# ---------------------------------------------------------------------------
with Window(title="ComboBox Theme Palette Styles",
            size=(1100, 850), center=True, theme=WindowTheme.Light) as wnd_id:
    state["wnd_id"] = wnd_id
    with Column(spacing=10, padding=[20], width_fill=True, height=850):
        add_text(content="Select a Theme to see the differences in the styling.")
        add_pick_list(options=window_theme_names(), selected="Light",
                    placeholder="Select Theme", on_select=on_theme_select)
        with Scrollable(height=825):
            with Column(spacing=10, width_fill=True):
                add_text(content=(
                    "The ComboBox uses the Background, Primary, and Secondary "
                    "schemes for styling.\n"
                    "Theses color schemes have a base, strong, and weak palettes."
                    "Each palette has two colors,\nColor and a contrasting Text color.\n"
                    "The text above the containers will indicate the color.\n"
                    "Each theme has it's own Primary, Secondary,... colors.\n"
                    "if theme=GRUVBOX_LIGHT, primary=light blue, ...\n"
                    "if theme=GRUVBOX_DARK, primary=dark blue, ...\n"
                    "See the py_window_custom_theme.py and other widget palette examples."))

                add_text(content=(
                    "The ComboBox statuses: Active, Hovered, Focused(dropdown active), Disabled\n"
                    "The ComboBox has two style types, input and dropdown menu.\n"
                    "The input covers all styling except any dropdown items and such.\n"
                    "The menu style does not have any statuses"))

                add_text(content="ComboBoxes for seeing the actual styling")
                with Row(spacing=10):
                    menu_style_id = add_combobox_menu_style(border_width=3)
                    input_style_id = add_combobox_input_style(border_width=3)
                    with Row(spacing=20):
                        add_combobox(options=combo_list, placeholder="Default Styling...",
                            on_select=selected_item, width=150, menu_style_id=menu_style_id,
                            input_style_id=input_style_id)
                        add_combobox(options=combo_list, placeholder="Disabled Styling...",
                            width=150, menu_style_id=menu_style_id, disabled=True,
                            input_style_id=input_style_id)


                with Column(spacing=20, width_fill=True, height_fill=True):
                    add_text(content="******Input Styling******")

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Status: Active",
                                 size=20, font_id=font_id)
                        make_tiles(input_active)

                    with Row(spacing=20, width_fill=True):
                        with Column(spacing=5):
                            add_text(content="Status: Hovered",
                                     size=20, font_id=font_id)
                            make_tiles(input_hovered)

                        with Column(spacing=5):
                            add_text(content="Status: Focused",
                                     size=20, font_id=font_id)
                            make_tiles(input_focused)

                        with Column(spacing=5):
                            add_text(content="Status: Disabled",
                                     size=20, font_id=font_id)
                            make_tiles(input_disabled)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Menu Style - No Statuses",
                                 size=20, font_id=font_id)
                        make_tiles(menu_tiles)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="All Background Palette Colors",
                                 size=20, font_id=font_id)
                        make_tiles(bkg_tiles)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="All Background Palette Text Colors",
                                 size=20, font_id=font_id)
                        make_tiles(bkg_tiles_text)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Primary Palette Text & Colors",
                                 size=20, font_id=font_id)
                        make_tiles(primary_tiles)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Secondary Palette Text & Colors",
                                 size=20, font_id=font_id)
                        make_tiles(secondary_tiles)
                        add_space(height=30)

start_session()
