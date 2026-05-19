#!/usr/bin/env python3
"""
Splitter demo
"""
from icedpygui import (
    Window,
    Column,
    Row,
    Container,
    ContainerStyleStd,
    start_session,
    add_text,
    add_button,
    add_space,
    update_widget,
    TextParam,
    SplitterH,
    SplitterV,
)

# shared labels updated from callbacks
h_label_id_0 = None
h_label_id_1 = None
v_label_id = None


def on_h_resize(_wid, data):
    """Called while dragging a SplitterH handle."""
    index, value, sizes = data
    match index:
        case 0:
            update_widget(h_label_id_0, TextParam.Content,
                f"SplitterH: handle {index}  pos={value:.1f}  sizes={[f'{s:.0f}' for s in sizes]}")
        case 1:
            update_widget(h_label_id_1, TextParam.Content,
                f"SplitterH: handle {index}  pos={value:.1f}  sizes={[f'{s:.0f}' for s in sizes]}")


def on_v_resize(_wid, data):
    """Called while dragging a SplitterV handle."""
    index, value, sizes = data
    update_widget(v_label_id, TextParam.Content,
        f"SplitterV: handle {index}  pos={value:.1f}  sizes={[f'{s:.0f}' for s in sizes]}")


def on_release(wid, data):
    """On Release"""
    index, value, sizes = data
    print(f"Released: wid={wid}  handle={index}  pos={value:.1f}  "
          f"sizes={[f'{s:.0f}' for s in sizes]}")


# add the window
with Window(title="Splitter Demo", size=(900, 650), center=True):

    with Column(padding=[20], spacing=16):

        # SplitterH: three side-by-side panels
        add_text(content="SplitterH — three side-by-side panels (drag the handles)")

        with SplitterH(sizes=[220.0, 220.0, 220.0], height=200.0, max_size=660.0,
                       on_resize=on_h_resize, on_release=on_release):

            # Panel 0
            with Container(fill=True, padding=[8],
                    style_std=ContainerStyleStd.BorderedBox):
                with Column():
                    add_text(content="Panel 0, this is some long text to see the wrapping change")
                    add_button(label="Button A")

            # Panel 1
            with Container(fill=True, padding=[8],
                    style_std=ContainerStyleStd.BorderedBox):
                with Column():
                    add_text(content="Panel 1")
                    add_button(label="Button B")

            # Panel 2
            with Container(fill=True, padding=[8],
                    style_std=ContainerStyleStd.BorderedBox):
                with Column():
                    add_text(content="Panel 2")
                    add_button(label="Button C")

        h_label_id_0 = add_text(content="SplitterH: drag a handle to see sizes")
        h_label_id_1 = add_text(content="SplitterH: drag a handle to see sizes")

        add_space(height=20)

        # SplitterV: two stacked panels
        add_text(content="SplitterV — two stacked panels (drag the handle)")

        with SplitterV(sizes=[150.0, 150.0], width=660.0,
                       on_resize=on_v_resize, on_release=on_release):

            # Top panel
            with Container(fill=True, padding=[8],
                    style_std=ContainerStyleStd.BorderedBox):
                with Row():
                    add_text(content="Top panel — resize me")

            # Bottom panel
            with Container(fill=True, padding=[8],
                    style_std=ContainerStyleStd.BorderedBox):
                with Row():
                    add_text(content="Bottom panel")

        v_label_id = add_text(content="SplitterV: drag the handle to see sizes")


start_session()
