#!/usr/bin/env python3
"""Simple clipboard read/write demo."""

from icedpygui import (
    Window,
    Column,
    Container,
    add_button,
    add_space,
    add_text,
    add_text_editor,
    clipboard_write,
    clipboard_callback,
    start_session,
)

state = { "status_id": 0 }


def copy_pressed(_btn_id: int):
    """Write text to system clipboard."""
    txt = "Hello from iced-py-gui clipboard!"
    clipboard_write(txt)
    # in this demo, this is trivial but if you have a situation
    # where didn't use a button where no cllback occurs, you could invoke
    # the clipboard callback which will send the id of the clipboard along with the text
    # and any user data.
    clipboard_callback(on_clipboard)


def on_clipboard(cb_id: int, txt: str):
    """Callback"""
    print(f"clipboard callback {cb_id} {txt}")


with Window(title="Clipboard Demo", center=True):
    with Container(width_fill=True, height_fill=True, align_center=True):
        with Column(spacing=12.0, padding=[20.0], width=520.0):
            add_text(content="Clipboard API Demo")
            add_space(height=6.0)

            add_button(label="Press me to Copy the message\n into the Clipboard",
                       on_press=copy_pressed)

            add_text_editor(content=("Use ctrl v to paste the text here after "
                                     "you press the copy button. Click below this text to "
                                     "show the cursor before pasting.\n\n\n\n\n\n"),
                            width=300, height=200)

start_session()
