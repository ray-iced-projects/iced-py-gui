#!/usr/bin/env python3
"""
Rich text demo
"""

import webbrowser

from icedpygui import (Window, Container, start_session,
    RichText, add_span, Color)


OPEN_URL = 1

def open_url(_rt_id, link_id: int):
    """RichText link callback"""
    if link_id == OPEN_URL:
        webbrowser.open("https://www.google.com")



with Window(title="Text Editor", center=True):
    with Container(fill=True, align_center=True):
        with RichText(on_link_click=open_url):
            add_span(
                text="I am Light Blue and have an underline!\n\n",
                color=Color.LIGHT_BLUE,
                underline=True)

            add_span(
                text="I am Yellow and have an strike through!\n\n",
                color=Color.YELLOW,
                strikethrough=True)

            add_span(
                text="I am Light Blue and have a background color!\n\n",
                color=Color.LIGHT_BLUE,
                background_color=Color.BLUE)

            add_span(text="link: ")
            add_span(
                text="google.com",
                underline=True,
                link=OPEN_URL)

start_session()
