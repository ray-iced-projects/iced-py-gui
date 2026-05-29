
#!/usr/bin/env python3
"""
Arrow helper
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
    add_font_style,
    add_text,
    Arrow,
    )


# The Bootstrap Icons font is loaded automatically, but add_text needs a
# font_id to know which font to render the glyph with.  Without this,
# the system default font is used and the codepoint
# renders as whatever glyph that font happens to have there (a ribbon, etc).
bootstrap_font = add_font_style(family_name="bootstrap-icons")

arrow = arrow_to_str(arrow=Arrow.ArrowBarLeft)

def on_input(_wid: int, data: str):
    """text input"""
    print(data)



# Add the window
with Window(
        title="Table Demo",
        size=(1000, 600),
        center=True,
        # theme=WindowTheme.Light
        ):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):
        with Scrollable(width=700):
            with Column(fill=True):

                for arrow in arrow_variants():
                    glyph = arrow_to_str(arrow)
                    with Row(spacing=20):
                        add_text(content=str(arrow))
                        add_text(content=glyph, font_id=bootstrap_font, size=20)


start_session()
