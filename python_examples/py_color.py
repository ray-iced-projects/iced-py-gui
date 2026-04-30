#!/usr/bin/env python3
"""
Creating color palettes

"""

from icedpygui import Window, Column, Container, Scrollable, start_session, \
    get_color_palette, Color, add_text, add_container_style

palette = get_color_palette(Color.BLUE)
sorted_palette = dict(sorted(palette.items()))

# {
#  'base_color': [0.0, 0.0, 1.0, 1.0],
#  'base_text': [1.0, 1.0, 1.0, 1.0],
#  'neutral_color': [0.15, 0.0, 1.0, 1.0],
#  'neutral_text': [1.0, 1.0, 1.0, 1.0],
#  'strong_color': [0.19, 0.0, 1.0, 1.0],
#  'strong_text': [1.0, 1.0, 1.0, 1.0],
#  'stronger_color': [0.22, 0.0, 1.0, 1.0],
#  'stronger_text': [1.0, 1.0, 1.0, 1.0],
#  'strongest_color': [0.26, 0.0, 1.0, 1.0],
#  'strongest_text': [1.0, 1.0, 1.0, 1.0],
#  'weak_text': [1.0, 1.0, 1.0, 1.0],
#  'weaker_color': [0.08, 0.0, 1.0, 1.0],
#  'weaker_text': [1.0, 1.0, 1.0, 1.0],
#  'weakest_color': [0.03, 0.0, 1.0, 1.0],
#  'weakest_text': [1.0, 1.0, 1.0, 1.0],
#  'weak_color': [0.12, 0.0, 1.0, 1.0],
#  }


with Window(title="Colors", size=(600, 600), center=True):
    with Container(align_center=True, fill=True):
        with Scrollable():
            with Column(spacing=20):
                variants = [(k[:-6], v, sorted_palette[k[:-6] + "_text"])
                            for k, v in sorted_palette.items() if k.endswith("_color")]
                for (name, color_rgba, text_rgba) in variants:
                    bkg = add_container_style(background_rgba=color_rgba)
                    with Container(width=200, height=50, align_center=True, style_id=bkg):
                        add_text(content=name, color_rgba=text_rgba)

start_session()
