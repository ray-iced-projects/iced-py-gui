#!/usr/bin/env python3
"""
File Dialog use demo
"""

from icedpygui import (
    Window,
    Container,
    start_session,
    add_button,
    add_file_system_dialog,
    FileSystemDialogParams,
    update_widget,
    )


def open_dialog(_btn_id: int):
    """Open Dialog"""
    print("open dialog")
    update_widget(fsd_id, FileSystemDialogParams.SelectFolder, True)


def folder_selected(_fsd_id: int, folder_name: str):
    """Folder selected"""
    print("folder_name", folder_name)


with Window(title="Float Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        fsd_id = add_file_system_dialog(on_folder_selected=folder_selected)
        add_button(label="Open File Dialog", on_press=open_dialog)



start_session()
