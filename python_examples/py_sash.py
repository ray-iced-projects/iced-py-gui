    #!/usr/bin/env python3
"""
Sash use demo
"""

from icedpygui import (
    Window,
    Column,
    Container,
    Row,
    start_session,
    Sash,
    SashParam,
    add_checkbox,
    CheckboxParam,
    add_radio,
    add_text,
    add_container_style,
    update_widget,)

cont_styles = []
cont_styles.append(add_container_style(bkg_rgba=[0.25, 0.35, 0.55, 1.0]))
cont_styles.append(add_container_style(bkg_rgba=[0.30, 0.50, 0.35, 1.0]))
cont_styles.append(add_container_style(bkg_rgba=[0.55, 0.35, 0.25, 1.0]))

h_contents = ["Left", "Center", "Right"]
v_contents = ["Top", "Center", "Bottom"]

h_sizes = [150.0, 200.0, 150.0]
v_sizes = [100.0, 200.0, 100.0]

state = {"is_checked": False,
         "sync_sashes": False,
         "on_cross_checked": False,
         "sync_cross_sashes": False,
         "sash_ids": [],
         }


def sync_resize_h(chk_id: int, checked: bool):
    """Set the checkbox"""
    update_widget(chk_id, CheckboxParam.IsChecked, checked)
    state["is_checked"] = checked
    for s_id in state["sash_ids"]:
        update_widget(s_id, SashParam.SyncSashes, checked)


def sync_resize_cross_h(chk_id: int, checked: bool):
    """Sets the Resize Cross Hash"""
    update_widget(chk_id, CheckboxParam.IsChecked, checked)
    state["is_checked"] = checked
    for s_id in state["sash_ids"]:
        update_widget(s_id, SashParam.SyncCrossSashes, checked)


def on_resize_sash_h(sh_id: int, data: tuple[int, float]):
    """Resizing Horizontal Sash"""
    print(sh_id, data)


def on_resize_outer(sh_id: int, size: float):
    """Resizing with outer"""
    print(sh_id, size)

def on_radio_selected(_rd_id: int, index: int):
    """Selecting resizing type"""
    for sid in state["sash_ids"]:
        match index:
            case 0:
                update_widget(sid, SashParam.ResizeModeLastOnly, True)
            case 1:
                update_widget(sid, SashParam.ResizeModeUniform, True)
            case 2:
                update_widget(sid, SashParam.ResizeModeProportional, True)


#  First add a window
with Window(title="Sash Demo", center=True, size=(900, 600)):
    with Row(spacing=20.0, fill=True):
        with Column(spacing=20.0, height_fill=True):
            add_text(content="Use the mouse to drag the sashes left or right\n"
                     + "drag the cross sashes using bottom sashes")
            with Column(spacing=20.0, width=650, height=350):
                for i in range(2):
                    with Sash(
                        initial_sizes=h_sizes,
                        size=150.0,
                        sash_size=4.0,
                        # once set, enables cross sizing
                        cross_handle_size=5.0,
                        outer_handle_size=4.0,
                        # for the sashes, no callbacks are needed
                        # unless dynamic setting are wanted
                        on_resize=on_resize_sash_h,
                        on_resize_outer=on_resize_outer) as sash_id:

                        state["sash_ids"].append(sash_id)

                        for i in range(3):
                            # Add containers to the Sash
                            with Container(
                                fill=True,
                                align_center=True,
                                style_id=cont_styles[i]):

                                add_text(content=h_contents[i])

            with Row(spacing=20):
                # Add some controls
                with Column(spacing=10.0, padding=[10]):
                    add_checkbox(
                        label="Sync SashesH",
                        is_checked=False,
                        on_toggle=sync_resize_h)

                    add_checkbox(
                        label="Sync Cross SashH",
                        is_checked=False,
                        on_toggle=sync_resize_cross_h)

                with Column(spacing=10.0, padding=[10]):
                    add_text(content="Outer Sash (right side) drag parameters")

                    add_radio(
                        labels=["Last Only (default)", "Uniform", "Proportional"],
                        on_selected=on_radio_selected)

        with Column(spacing=20.0, height_fill=True):
            add_text(content="Use the mouse to drag the sashes up or down")
            with Sash(
                initial_sizes=v_sizes,
                size=200.0,
                sash_size=4.0,
                vertical_direction=True):

                for i in range(3):
                    # Add containers to the Sash
                    with Container(
                        fill=True,
                        align_center=True,
                        style_id=cont_styles[i]):

                        add_text(content=v_contents[i])

start_session()
