#!/usr/bin/env python3
"""
Button crating a new palette.
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
    add_button,
    add_pick_list,
    Scrollable,
    add_text,
    update_widget,
    get_button_palette,
    ButtonStyleStd,
    window_theme_names,
    add_font_style,
    FontWeight,
    generate_id,
    add_space,
)


def update_tiles(tiles: list[(str, str, str, int, int)]):
    """Update the tiles after theme change"""
    # ("bkg", "Background", "bkg_base", alpha, COLOR, gen_id)
    for (color, _parameter, key, alpha, c_t, tile_id) in tiles:
        bkg_rgba = state[color][key][c_t]
        bkg_rgba[3] = alpha
        style_id = add_container_style(bkg_rgba=bkg_rgba)
        update_widget(tile_id, ContainerParam.StyleId, style_id)



def on_theme_select(_pl_id: int, theme_name: str):
    """Select a Theme by PickList"""
    update_widget(state["wnd_id"], WindowParam.Theme, theme_name)
    state["selected_theme"] = theme_name
    state["bkg"] = get_button_palette(theme_name, ButtonStyleStd.Primary)
    state["primary"] = get_button_palette(theme_name, ButtonStyleStd.Primary)
    state["secondary"] = get_button_palette(theme_name, ButtonStyleStd.Secondary)
    state["success"] = get_button_palette(theme_name, ButtonStyleStd.Success),
    state["warning"] = get_button_palette(theme_name, ButtonStyleStd.Warning),
    state["danger"] = get_button_palette(theme_name, ButtonStyleStd.Danger),
    update_tiles(active)
    update_tiles(hovered)
    update_tiles(disabled)
    update_tiles(bkg_tiles)
    update_tiles(bkg_tiles_text)
    update_tiles(primary_tiles)
    update_tiles(secondary_tiles)


def make_tiles(tiles: list[(str, str, str, float, int, int)]):
    """Make the tiles"""
    # ("bkg", "Background", "bkg_base", alpha, COLOR, gen_id)
    with Column():
        with Row(spacing=5):
            for (color_pal, parameter, key, alpha, c_t, tile_id) in tiles:
                c = "Color" if alpha == 1.0 else f"Color\nAlpha {alpha}"
                t = "Text" if alpha == 1.0 else f"Text\nAlpha {alpha}"
                content=f"{parameter}\n{key}\n{(f"{color_pal.capitalize()} {c}"
                        if c_t is COLOR else f"{color_pal.capitalize()} {t}")}"
                add_text(content=content, width=120)
        with Row(spacing=5):
            for (color_pal, parameter, key, alpha, c_t, tile_id) in tiles:
                bkg_rgba = state[color_pal][key][c_t]
                bkg_rgba[3] = alpha
                style_id = add_container_style(bkg_rgba=bkg_rgba)
                add_container(width=120, height=20, style_id=style_id, gen_id=tile_id)


# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------

state = {"selected_style": "",
         "selected_theme": "TokyoNight",
         "wnd_id": 0,
         "bkg":  get_button_palette("TokyoNight", ButtonStyleStd.Primary),
         "primary": get_button_palette("TokyoNight", ButtonStyleStd.Primary),
         "secondary": get_button_palette("TokyoNight", ButtonStyleStd.Secondary),
         "success": get_button_palette("TokyoNight", ButtonStyleStd.Success),
         "warning": get_button_palette("TokyoNight", ButtonStyleStd.Warning),
         "danger": get_button_palette("TokyoNight", ButtonStyleStd.Danger),
         }

# Index into (bkg_rgba, text_rgba) palette tuples
TEXT = 1
COLOR = 0


active = [
    ("primary", "Background", "base", 1.0, COLOR, generate_id() ),
    ("primary", "Label", "base", 1.0, TEXT, generate_id() ),
    ]

hovered = [
    ("primary", "Background", "strong", 1.0, COLOR, generate_id()),
    ("primary", "Background", "strong", 1.0, TEXT, generate_id()),
]

pressed = [
    ("primary", "Background", "base", 1.0, COLOR, generate_id() ),
    ("primary", "Background", "base", 1.0, TEXT, generate_id() ),
    ]

disabled= [
    ("primary", "Background", "base", 0.5, COLOR, generate_id() ),
    ("primary", "Label", "base", 0.5, TEXT,  generate_id() )
]


# Palette keys the first 3 keys are associated with Primary and Secondary colors
# The bkg keys are associated with the theme.
# 'base',
# 'strong',
# 'weak',
bkg_tiles = [
    ("bkg", "Background", "bkg_base",       1.0, COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_neutral',    1.0, COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_strong',     1.0, COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_stronger',   1.0, COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_strongest',  1.0, COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_weak',       1.0, COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_weaker',     1.0, COLOR,  generate_id() ),
    ("bkg", "Background", 'bkg_weakest',    1.0, COLOR,  generate_id() ),
    ]

bkg_tiles_text = [
    ("bkg", "Background", "bkg_base",       1.0, TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_neutral',    1.0, TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_strong',     1.0, TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_stronger',   1.0, TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_strongest',  1.0, TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_weak',       1.0, TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_weaker',     1.0, TEXT,   generate_id() ),
    ("bkg", "Background", 'bkg_weakest',    1.0, TEXT,   generate_id() ),
    ]

primary_tiles = [
    ("primary", "",  "base",   1.0, COLOR, generate_id() ),
    ("primary", "",  "base",   1.0, TEXT,  generate_id() ),
    ("primary", "",  "weak",   1.0, COLOR, generate_id() ),
    ("primary", "",  "weak",   1.0, TEXT,  generate_id() ),
    ("primary", "",  "strong", 1.0, COLOR, generate_id() ),
    ("primary", "",  "strong", 1.0, TEXT,  generate_id() ),
]

secondary_tiles = [
    ("secondary", "",  "base",  1.0, COLOR, generate_id() ),
    ("secondary", "",  "base",  1.0, TEXT,  generate_id() ),
    ("secondary", "",  "weak",  1.0, COLOR, generate_id() ),
    ("secondary", "",  "weak",  1.0, TEXT,  generate_id() ),
    ("secondary", "",  "strong",1.0, COLOR, generate_id() ),
    ("secondary", "",  "strong",1.0, TEXT,  generate_id() ),
]

success_tiles = [
    ("success", "",  "base",  1.0, COLOR, generate_id() ),
    ("success", "",  "base",  1.0, TEXT,  generate_id() ),
    ("success", "",  "weak",  1.0, COLOR, generate_id() ),
    ("success", "",  "weak",  1.0, TEXT,  generate_id() ),
    ("success", "",  "strong",1.0, COLOR, generate_id() ),
    ("success", "",  "strong",1.0, TEXT,  generate_id() ),
]

warning_tiles = [
    ("warning", "",  "base",  1.0, COLOR, generate_id() ),
    ("warning", "",  "base",  1.0, TEXT,  generate_id() ),
    ("warning", "",  "weak",  1.0, COLOR, generate_id() ),
    ("warning", "",  "weak",  1.0, TEXT,  generate_id() ),
    ("warning", "",  "strong",1.0, COLOR, generate_id() ),
    ("warning", "",  "strong",1.0, TEXT,  generate_id() ),
]

danger_tiles = [
    ("danger", "",  "base",  1.0, COLOR, generate_id() ),
    ("danger", "",  "base",  1.0, TEXT,  generate_id() ),
    ("danger", "",  "weak",  1.0, COLOR, generate_id() ),
    ("danger", "",  "weak",  1.0, TEXT,  generate_id() ),
    ("danger", "",  "strong",1.0, COLOR, generate_id() ),
    ("danger", "",  "strong",1.0, TEXT,  generate_id() ),
]

font_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)

# ---------------------------------------------------------------------------
# GUI — Initial display with a TokyoNight background (Selected by PickList)
# ---------------------------------------------------------------------------
with Window(title="Button Theme Palette Styles",
            size=(1100, 850), center=True, theme=WindowTheme.TokyoNight) as wnd_id:
    state["wnd_id"] = wnd_id
    with Column(spacing=10, padding=[20], width_fill=True, height=850):
        add_text(content="Select a Theme to see the differences in the styling.")
        add_pick_list(options=window_theme_names(), selected="TokyoNight",
                    placeholder="Select Theme", on_select=on_theme_select)
        with Scrollable(height=825):
            with Column(spacing=10, width_fill=True):
                add_text(content=(
                    "The Button uses the Background, Primary, Secondary, Success, "
                    "The palettes are base, strong, weak"
                    "Warning and Danger schemes for styling.\n"
                    "Each palette has two colors,\nColor and a contrasting Text color.\n"
                    "The text above the containers will indicate the color.\n"
                    "Each theme has it's own Primary, Secondary,... colors.\n"
                    "if theme=GRUVBOX_LIGHT, primary=light blue, ...\n"
                    "if theme=GRUVBOX_DARK, primary=dark blue, ...\n"
                    "See the py_window_custom_theme.py and other widget palette examples."))

                add_text(content=("The Button statuses: Active (base), Hovered (strong), "
                                  "Disabled(base alpha 0.5)"))

                add_text(content="Buttons for seeing the actual styling")
                with Row(spacing=10):
                    with Row(spacing=20, wrap=True):
                        with Column(spacing=5):
                            add_button(label="Primary Styling", padding=[10])
                            add_button(label="Disabled Styling",disabled=True, padding=[10])

                        with Column(spacing=5):
                            add_button(label="Secondary Styling", padding=[10],
                                    style_std=ButtonStyleStd.Secondary)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Secondary)

                        with Column(spacing=5):
                            add_button(label="Success Styling", padding=[10],
                                    style_std=ButtonStyleStd.Success)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Success)

                        with Column(spacing=5):
                            add_button(label="Warning Styling", padding=[10],
                                    style_std=ButtonStyleStd.Warning)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Warning)

                        with Column(spacing=5):
                            add_button(label="Danger Styling", padding=[10],
                                    style_std=ButtonStyleStd.Danger)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Danger)

                        with Column(spacing=5):
                            add_button(label="Background Styling", padding=[10],
                                    style_std=ButtonStyleStd.Background)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Background)

                        with Column(spacing=5):
                            add_button(label="Subtle Styling", padding=[10],
                                    style_std=ButtonStyleStd.Subtle)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Subtle)

                        with Column(spacing=5):
                            add_button(label="Text Styling", padding=[10],
                                    style_std=ButtonStyleStd.Text)
                            add_button(label="Disabled Styling", disabled=True, padding=[10],
                                    style_std=ButtonStyleStd.Text)


                with Column(spacing=20, width_fill=True, height_fill=True):
                    add_text(content="******Status Styling******")

                    with Row(spacing=20, width_fill=True):
                        with Column(spacing=5):
                            add_text(content="Status: Active",
                                    size=20, font_id=font_id)
                            make_tiles(active)

                        with Column(spacing=5):
                            add_text(content="Status: Hovered",
                                     size=20, font_id=font_id)
                            make_tiles(hovered)

                        with Column(spacing=5):
                            add_text(content="Status: Pressed",
                                     size=20, font_id=font_id)
                            make_tiles(pressed)

                        with Column(spacing=5):
                            add_text(content="Status: Disabled",
                                     size=20, font_id=font_id)
                            make_tiles(disabled)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Primary Palette Text & Colors",
                                 size=20, font_id=font_id)
                        make_tiles(primary_tiles)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Secondary Palette Text & Colors",
                                 size=20, font_id=font_id)
                        make_tiles(secondary_tiles)
                        add_space(height=30)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Success Palette Text & Colors",
                                 size=20, font_id=font_id)
                        make_tiles(success_tiles)
                        add_space(height=30)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Warning Palette Text & Colors",
                                 size=20, font_id=font_id)
                        make_tiles(warning_tiles)
                        add_space(height=30)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="Danger Palette Text & Colors",
                                 size=20, font_id=font_id)
                        make_tiles(danger_tiles)
                        add_space(height=30)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="All Background Palette Colors",
                                 size=20, font_id=font_id)
                        make_tiles(bkg_tiles)

                    with Column(spacing=5, width_fill=True):
                        add_text(content="All Background Palette Text Colors",
                                 size=20, font_id=font_id)
                        make_tiles(bkg_tiles_text)

start_session()
