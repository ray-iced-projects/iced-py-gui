#!/usr/bin/env python3
"""
Text Editor demo
"""

from icedpygui import Window, Container, start_session, \
    add_text_editor





with Window(title="Text Editor", center=True):
    with Container(fill=True, align_center=True):
        add_text_editor()

start_session()
