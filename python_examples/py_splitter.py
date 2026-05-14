#!/usr/bin/env python3
"""
Splitter demo

Demonstrates SplitterH (horizontal split — side-by-side panels separated by
vertical drag handles) and SplitterV (vertical split — stacked panels separated
by horizontal drag handles).

Both splitters manage their own internal sizes and call optional Python callbacks
when the user drags or releases a sash handle.

Parameters common to SplitterH and SplitterV
---------------------------------------------
window_id : str
    Window this splitter belongs to.
container_id : str
    Unique string identifier for the splitter.
parent_id : str, Optional
    Parent container ID.  Defaults to the window.
sizes : list[float]
    Initial size (width for SplitterH, height for SplitterV) of each panel.
    The number of panels equals len(sizes); the number of sash handles is
    len(sizes) - 1.
height (SplitterH) / width (SplitterV) : float
    Fixed cross-axis dimension of the splitter in logical pixels.
min_size : float, default 20.0
    Minimum size any panel can be dragged to.
sash_width (SplitterH) / sash_height (SplitterV) : float, default 8.0
    Hit-target thickness of each drag handle in logical pixels.
on_resize : callable, Optional
    Called while dragging.  Signature: cb(widget_id, (index, value, sizes))
    where index is the moved handle, value is the new boundary position, and
    sizes is the updated sizes list.
on_release : callable, Optional
    Called when the mouse is released after a drag.  Same signature.
style_id : int, Optional
    ID returned by add_splitter_style().
show : bool, default True
    Whether the splitter is visible.

Returns
-------
int
    The numeric widget ID of the newly created splitter.
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

# --- shared labels updated from callbacks ------------------------------------
h_label_id = None
v_label_id = None


def on_h_resize(_wid, data):
    """Called while dragging a SplitterH handle."""
    index, value, sizes = data
    update_widget(h_label_id, TextParam.Content,
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


# =============================================================================
# Window
# =============================================================================
with Window(title="Splitter Demo", size=(900, 650), center=True):

    with Column(padding=[20], spacing=16):

        # --- SplitterH: three side-by-side panels ----------------------------
        add_text(content="SplitterH — three side-by-side panels (drag the handles)")

        with SplitterH(sizes=[220.0, 220.0, 220.0], height=200.0,
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

        h_label_id = add_text(content="SplitterH: drag a handle to see sizes")

        add_space(height=20)

        # --- SplitterV: two stacked panels -----------------------------------
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
