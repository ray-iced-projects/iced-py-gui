#!/usr/bin/env python3
"""
Image use demo with interactive cropping via MouseArea.

The Image widget displays raster images (PNG, JPG, GIF, BMP, ICO,
TIFF, WebP, AVIF, PNM, DDS, TGA, EXR, Farbfeld, QOI) while preserving
aspect ratio by default.


Usage:
  1. Click "Crop Mode" to enter cropping mode.
  2. Press and drag on the image to select a crop rectangle.
  3. Release the mouse — the button changes to "Press to Crop".
  4. Click the button to apply the crop.
  5. Click "Undo Crop" to restore the original image.

Container Sizing & Cropping
----------------------------
During interactive crop selection this demo switches to
ContentFit.Fill so the displayed image fills the fixed Container
exactly, giving a 1-to-1 mapping from mouse coordinates to widget
pixels.  After the crop is applied ContentFit is reset to the default
so the cropped result displays with its natural aspect ratio.

The image widget size is normally driven by its source dimensions and
ContentFit.  When placing an image inside a Container for interactive
cropping, you may need to fix the Container's width and
height to the desired size.  Without fixed dimensions the
Container may resize when sibling widgets (buttons, text) change size,
which breaks the coordinate mapping between mouse position and source
pixels.  If you set the sizing of all containers or have a simpler layout
where nothing causes the container to resize, then you could probably
eliminate to ContentFit setting/resetting.  If you crop manually, preset the
width and height of the image, you won't have any issues.

In this demo, only cropping was performed but additional methods
like scaling and even mouse rotation could be added using the same
methods as cropping.
"""
import os
from icedpygui import (
    Window, Container, Column, Row, Stack, MouseArea, Float,
    add_image, add_button, add_text, add_container_style,
    update_widget, update_widget_params,
    ImageParam, ButtonParam, TextParam, FloatParam, ContainerParam,
    ContentFit, ContainerStyleStd, start_session,
)

# ── paths ──────────────────────────────────────────────────────────────
cwd = os.getcwd()
ferris = cwd + "/python_examples/resources/png_svg/ferris.png"

# ── state ──────────────────────────────────────────────────────────────
ACTUAL_IMG_W, ACTUAL_IMG_H = 1200.0, 800.0
IMG_W, IMG_H = 1200*0.3, 800.0*0.3

state = {
    "crop_mode": False,
    "dragging": False,
    "selected": False,
    "start_x": 0.0,
    "start_y": 0.0,
    "end_x": 0.0,
    "end_y": 0.0,
    "cur_x": 0.0,
    "cur_y": 0.0,
    "img_id": 0,
    "crop_btn_id": 0,
    "undo_btn_id": 0,
    "status_id": 0,
    "float_id": 0,
    "overlay_id": 0,
}

# ── overlay style ──────────────────────────────────────────────────────
overlay_style = add_container_style(
    bkg_rgba=[0.3, 0.5, 1.0, 0.2],
    border_rgba=[0.3, 0.5, 1.0, 0.8],
    border_width=1.5,
)

# ── helpers ────────────────────────────────────────────────────────────
def _crop_rect():
    """Return (x, y, w, h) scaled to source image pixels, as ints.

    With ContentFit::Fill active during crop mode, the image stretches
    to exactly IMG_W x IMG_H, so mouse coords map directly.
    """
    scale_x = ACTUAL_IMG_W / IMG_W
    scale_y = ACTUAL_IMG_H / IMG_H

    x1 = max(0.0, min(state["start_x"], state["end_x"]))
    y1 = max(0.0, min(state["start_y"], state["end_y"]))
    x2 = min(IMG_W, max(state["start_x"], state["end_x"]))
    y2 = min(IMG_H, max(state["start_y"], state["end_y"]))

    return (int(x1 * scale_x), int(y1 * scale_y),
            int((x2 - x1) * scale_x), int((y2 - y1) * scale_y))


# ── callbacks ──────────────────────────────────────────────────────────
def on_crop_btn(btn_id: int):
    """Toggle crop mode or apply the crop."""
    if not state["crop_mode"]:
        # Enter crop mode — switch to Fill so mouse maps 1:1 to widget
        state["crop_mode"] = True
        state["dragging"] = False
        update_widget(state["img_id"], ImageParam.ContentFit, ContentFit.Fill)
        update_widget(btn_id, ButtonParam.Label, "Select area on image…")
        update_widget(state["status_id"], TextParam.Content,
                      "Drag on the image to select crop area")
    else:
        # Apply crop
        x, y, w, h = _crop_rect()
        # print(f"[DEBUG] Applying crop: x={x}, y={y}, w={w}, h={h}")
        if w > 0 and h > 0:
            update_widget_params(state["img_id"], {
                ImageParam.CropX: x,
                ImageParam.CropY: y,
                ImageParam.CropWidth: w,
                ImageParam.CropHeight: h,
                ImageParam.ContentFit: None,  # restore default
            })
            update_widget(state["status_id"], TextParam.Content,
                          f"Cropped to x={x} y={y} w={w} h={h}")
        else:
            update_widget(state["img_id"], ImageParam.ContentFit, None)

        state["crop_mode"] = False
        state["dragging"] = False
        update_widget(btn_id, ButtonParam.Label, "Crop Mode")
        update_widget(state["overlay_id"], ContainerParam.Show, False)


def on_undo_btn(_btn_id: int):
    """Restore the original (un-cropped) image."""
    update_widget_params(state["img_id"], {
        ImageParam.CropX: None,
        ImageParam.CropY: None,
        ImageParam.CropWidth: None,
        ImageParam.CropHeight: None,
        ImageParam.ContentFit: None,  # restore default
    })
    state["crop_mode"] = False
    state["dragging"] = False
    update_widget(state["crop_btn_id"], ButtonParam.Label, "Crop Mode")
    update_widget(state["overlay_id"], ContainerParam.Show, False)
    update_widget(state["status_id"], TextParam.Content, "Crop undone")


def on_press(_ma_id: int):
    """Mouse press — lock the start point and begin dragging."""
    if not state["crop_mode"]:
        return
    state["dragging"] = True
    state["selected"] = False
    # Lock start to wherever the cursor is right now
    state["start_x"] = state["cur_x"]
    state["start_y"] = state["cur_y"]
    state["end_x"] = state["cur_x"]
    state["end_y"] = state["cur_y"]
    # Show the selection overlay at the press point
    update_widget(state["overlay_id"], ContainerParam.Show, True)
    update_widget(state["float_id"], FloatParam.Translate,
                  [state["cur_x"], state["cur_y"]])
    update_widget_params(state["overlay_id"], {
        ContainerParam.Width: 0.0,
        ContainerParam.Height: 0.0,
    })


def on_release(_ma_id: int):
    """Mouse release — end of drag selection."""
    if not state["crop_mode"] or not state["dragging"]:
        return
    state["dragging"] = False
    state["selected"] = True
    x, y, w, h = _crop_rect()
    if w > 0 and h > 0:
        update_widget(state["crop_btn_id"], ButtonParam.Label, "Press to Crop")
        update_widget(state["status_id"], TextParam.Content,
                      f"Selected: x={x} y={y} w={w} h={h}")


def on_move(_ma_id: int, point: tuple[float, float]):
    """Track mouse movement for drag selection."""
    px, py = point
    state["cur_x"] = px
    state["cur_y"] = py
    if state["crop_mode"] and state["dragging"]:
        state["end_x"] = px
        state["end_y"] = py
        # Update the visible selection rectangle
        x1 = min(state["start_x"], px)
        y1 = min(state["start_y"], py)
        w = abs(px - state["start_x"])
        h = abs(py - state["start_y"])
        update_widget(state["float_id"], FloatParam.Translate, [x1, y1])
        update_widget_params(state["overlay_id"], {
            ContainerParam.Width: w,
            ContainerParam.Height: h,
        })


# *** GUI *********************************************************
with Window(title="Image Crop Demo", size=(600, 550), center=True):

    with Container(align_center=True, fill=True):
        with Column(spacing=10.0, width=400):

            # Status text
            state["status_id"] = add_text(
                content="Click 'Crop Mode' to start")

            # Buttons row
            with Row(spacing=10.0):
                state["crop_btn_id"] = add_button(
                    label="Crop Mode", on_press=on_crop_btn)
                state["undo_btn_id"] = add_button(
                    label="Undo Crop", on_press=on_undo_btn)


            # A stack is used to stack all of the content on top of each other.
            # If there was a case where you don't want the mouse interactions
            # to bleed through to activate a lower widget, you would block
            # with a opaque widget during any mouse action.
            with Stack():
                with MouseArea(
                    on_press=on_press,
                    on_release=on_release,
                    on_move=on_move,
                ):
                    with Container(width=IMG_W, height=IMG_H,
                                   style_std=ContainerStyleStd.BorderedBox):
                        # If you are not doing any image manipulations,
                        # you could add the IMG_W and IMG_H to add_image
                        # and the size will be adjusted accordingly.
                        # In this case, FitContext is used so
                        # the Container as the size controller.
                        state["img_id"] = add_image(path=ferris)

                # the float holds the outline container and allows positioning
                # see the float demo for more info
                with Float(translate=[0.0, 0.0]) as flt_id:
                    state["float_id"] = flt_id
                    # The container styling allows one to see the mouse rectangle area
                    with Container(width=0.0, height=0.0,
                                   style_id=overlay_style) as ov_id:
                        state["overlay_id"] = ov_id
                        add_text(content="")

# Required last call
start_session()
