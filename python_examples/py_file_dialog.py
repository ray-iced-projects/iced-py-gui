#!/usr/bin/env python3
"""
File Dialog use demo
"""

from icedpygui import (
    Window,
    Column,
    Container,
    start_session,
    add_button,
    add_file_system_dialog,
    add_text_editor,
    FileSystemDialogParams,
    update_widget,
    )

state = {"file_content": ""}

def open_dialog_folder(_btn_id: int):
    """Open Dialog to select a folder"""
    update_widget(fsd_id, FileSystemDialogParams.SelectFolder, True)


def open_dialog_file(_btn_id: int):
    """Open Dialog to select a folder"""
    update_widget(fsd_id, FileSystemDialogParams.SelectFile, True)


def folder_selected(_fsd_id: int, folder_path: str):
    """Folder selected"""
    print("folder_path_name", folder_path)


def file_selected(_fsd_id: int, file_path: str):
    """File selected"""
    print("file_name", file_path)


def load_file(_fsd_id: int, file: str):
    """Load file content"""
    state["file_content"] = file
    update_widget()


with Window(title="Float Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        with Column(spacing=20):
            fsd_id = add_file_system_dialog(on_folder_selected=folder_selected,
                                            on_file_selected=file_selected)
            add_button(label="Open File Dialog for Folder", on_press=open_dialog_folder)
            add_button(label="Open File Dialog For Filename", on_press=open_dialog_file)
            add_button(label="Load file", on_press=load_file)
            editor_id = add_text_editor(content=state["file_content"], fill=True)

start_session()
