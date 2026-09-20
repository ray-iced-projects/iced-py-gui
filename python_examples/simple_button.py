"""
texting
"""
from icedpygui import (
    Window,
    Column,
    Container,
    start_session,
    add_button,
    add_button_style,
    get_widget_default_statuses,
    custom_palette,
    )

def on_press(btn_id: int):
    """Button"""
    print(btn_id)

# the style_id is needed to get the correct default statuses
# for the button.  You could add other style parameters as needed
# but if you don't want to modify the default status, this is the
# easiest approach.  See palette_create.py for a more complex example.
style_id = add_button_style()

new_color = [0.32, 0.2, 0.13, 1.0]

style_border = add_button_style(border_rgba=new_color, border_width=6.0, border_radius=[6.0])

default_statuses = get_widget_default_statuses(style_id=style_id)


def print_default_statuses(statuses):
    """Print the default statuses in a readable, indented format."""
    for (status, variant), parts in statuses:
        print(f"{status} / {variant}")
        for part, key, alpha in parts:
            print(f"    {str(part):<28} {str(key):<24} alpha={alpha:.2f}")
        print()

print_default_statuses(default_statuses)


pal_id = custom_palette(rgba=new_color, statuses=default_statuses)

with Window(title="Testing", center=True):
    with Container(align_center=True, fill=True):
        with Column(spacing=20):
            add_button(label="Normal Default Button",
                    padding=[10],
                    on_press=on_press)
            add_button(label="Button",
                    padding=[10],
                    on_press=on_press,
                    palette_id=pal_id)
            add_button(label="Button",
                            padding=[10],
                            on_press=on_press,
                            style_id=style_border,
                            )

start_session()
