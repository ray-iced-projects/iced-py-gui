#!/usr/bin/env python3
"""Simple clipboard read/write demo."""

from icedpygui import (
    Window,
    Column,
    Row,
    Container,
    add_button,
    add_space,
    add_text,
    clipboard_read,
    clipboard_write,
    update_widget,
    TextParam,
    start_session,
)

state = { "status_id": 0 }


def copy_pressed(_btn_id: int):
    """Write fixed text to system clipboard."""
    clipboard_write("Hello from iced-py-gui clipboard!")
    update_widget(state["status_id"], TextParam.Content, "Status: wrote text to clipboard")


def on_clipboard_read(_req_id: int, text: str | None):
    """Receive clipboard text and show it in the status label."""
    if text is None:
        update_widget(state["status_id"], TextParam.Content, "Status: clipboard had no text")
    else:
        update_widget(state["status_id"], TextParam.Content, f"Clipboard: {text}")


def paste_pressed(_btn_id: int):
    """Queue a clipboard read request."""
    clipboard_read(on_clipboard_read)
    update_widget(state["status_id"], TextParam.Content, "Status: reading clipboard...")


with Window(title="Clipboard Demo", center=True):
    with Container(width_fill=True, height_fill=True, align_center=True):
        with Column(spacing=12.0, padding=[20.0], max_width=520.0):
            add_text(content="Clipboard API Demo")
            add_text(content="Press Copy, then Paste to read current text clipboard value")
            add_space(height=6.0)

            with Row(spacing=10.0):
                add_button(label="Copy", on_press=copy_pressed)
                add_button(label="Paste", on_press=paste_pressed)

            add_space(height=8.0)
            status_id = add_text(content="Status: ready")

start_session()
