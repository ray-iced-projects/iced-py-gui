
#!/usr/bin/env python3
"""
Arrow helper, displays all of the arrows in built in bootstrap-icons
"""

from icedpygui import (
    Window,
    Column,
    Container,
    Row,
    Scrollable,
    start_session,
    arrow_to_str,
    arrow_variants,
    add_text,
    add_space,
    )



# Add the window
with Window(
        title="Table Demo",
        size=(500, 600),
        center=True,
        ):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):
        with Scrollable(width_fill=True):
            with Column(fill=True, padding=[20]):

                add_text(content=(
                    "These are the standard arrows that are in the built in bootstrap-icons."
                    "If you want other arrows, you will need to define the font and add it"
                    "to the add_text(font_id=my_font)"))

                add_space(height=20)

                for arrow in arrow_variants():
                    glyph = arrow_to_str(arrow)
                    with Row(spacing=20):
                        add_text(content=str(arrow))
                        add_text(content=glyph, size=20)


start_session()
