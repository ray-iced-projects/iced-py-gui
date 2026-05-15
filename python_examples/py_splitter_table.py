#!/usr/bin/env python3
"""
Splitter demo

Demonstrates SplitterH (horizontal split — side-by-side panels separated by
vertical drag handles) and SplitterV (vertical split — stacked panels separated
by horizontal drag handles).

Both splitters manage their own internal sizes and call optional Python callbacks
when the user drags or releases a sash handle.

"""
from icedpygui import (
    Window,
    Container,
    ContainerStyleStd,
    start_session,
    add_text,
    SplitterH,
    SplitterV,
)


# =============================================================================
# Window
# =============================================================================
with Window(title="Splitter Demo", center=True):
    with Container(style_std=ContainerStyleStd.BorderedBox):
        with SplitterH(sizes=[100.0, 100.0], height=300.0, sash_size=4.0, max_size=200):

            with SplitterV(sizes=[30]*10, width=100.0, sash_size=4.0, max_size=300):
                # Panel 0
                for i in range(10):
                    with Container(fill=True, align_center=True):
                        add_text(content=f"{i}")

            with SplitterV(sizes=[30]*10, width=100.0, sash_size=4.0, max_size=300):
                # Panel 0
                for i in range(10):
                    with Container(fill=True, align_center=True):
                        add_text(content=f"{i}")

start_session()
