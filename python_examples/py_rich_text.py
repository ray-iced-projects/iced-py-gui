from imports import *

"""
Rich text example demonstrating styled spans of text.
"""

with Window(
    title="Rich Text Demo",
    center=True):
    
    with Container(align_center=True, fill=True):
    
        with Column(spacing=10.0):
            add_text(content="Rich Text Demo", size=24.0)

            # Basic rich text with different colored spans
            rt_id = add_rich_text(size=16.0)
            add_span(rt_id, "Hello ")
            add_span(rt_id, "World", text_color=Color.RED, bold=True)
            add_span(rt_id, " in ")
            add_span(rt_id, "rich text!", text_color=Color.BLUE, italic=True)

            # Rich text with mixed styling
            rt2_id = add_rich_text(size=20.0)
            add_span(rt2_id, "Normal, ")
            add_span(rt2_id, "Bold, ", bold=True)
            add_span(rt2_id, "Italic, ", italic=True)
            add_span(rt2_id, "Bold+Italic, ", bold=True, italic=True)
            add_span(rt2_id, "Underline, ", underline=True)
            add_span(rt2_id, "Strikethrough", strikethrough=True)

            # Rich text with different sizes
            rt3_id = add_rich_text()
            add_span(rt3_id, "Small ", size=12.0)
            add_span(rt3_id, "Medium ", size=18.0)
            add_span(rt3_id, "Large ", size=24.0)
            add_span(rt3_id, "XLarge", size=32.0)

            # Rich text with custom rgba colors
            rt4_id = add_rich_text(size=18.0)
            add_span(rt4_id, "Custom ", text_rgba=[1.0, 0.5, 0.0, 1.0])
            add_span(rt4_id, "RGBA ", text_rgba=[0.0, 0.8, 0.2, 1.0])
            add_span(rt4_id, "Colors", text_rgba=[0.5, 0.0, 1.0, 1.0])


start_session()
