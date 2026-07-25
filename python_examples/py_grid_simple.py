#!/usr/bin/env python3
"""
Grid use demo
"""

from icedpygui import (Window, Container, ContainerStyleStd, Grid, start_session, add_text)

with Window(title="Demo"):
    with Container(fill=True, align_center=True):
        with Grid(width=400, columns_amount=4, spacing=3.0):
            # typical row/column iteration
            for row in range(2):
                for col in range(4):
                    with Container(height=40, align_center=True,
                                   style_std=ContainerStyleStd.BorderedBox):
                        add_text(content=f"Grid {row} {col}")

start_session()
