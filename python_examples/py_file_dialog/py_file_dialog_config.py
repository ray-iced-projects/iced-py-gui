#!/usr/bin/env python3
"""
File System Dialog Updating use demo
"""

from icedpygui import (
    Window,
    Column,
    Container,
    FileSystemDialog,
    Row,
    start_session,
    add_button,
    add_combobox,
    add_text,
    TextParam,
    FileSystemDialogParam,
    get_dialog_filters,
    update_widget,
    )


state = {"selection_list": []}

def set_filter(_btn_id: int):
    """Set dialog filter"""
    update_widget(fsd_id, FileSystemDialogParam.DefaultFilter, "Text Files")


def open_dialog_folder(_btn_id: int):
    """Open Dialog to select a folder"""
    update_widget(fsd_id, FileSystemDialogParam.SelectFolder, True)


def open_dialog_file(_btn_id: int):
    """Open Dialog to select a folder"""
    update_widget(fsd_id, FileSystemDialogParam.SelectFile, True)


def folder_selected(_fsd_id: int, folder_path: str):
    """Folder selected"""
    print("folder_path_name", folder_path)


def file_selected(_fsd_id: int, file_path: str):
    """File selected"""
    print("file_name", file_path)

filters = get_dialog_filters()

def add_selection_list(_btn_id: int, selection: str):
    """Adding to a selection list for filters"""
    state["selection_list"].append(selection)
    print(state["selection_list"])
    update_widget(selection_id, TextParam.Content,
                  f"Your selection list: {str(state["selection_list"])}")

def clear_selection_list(_btn_id: int):
    """Clearing selection list"""
    state["selection_list"] = []
    update_widget(selection_id, TextParam.Content,
                  f"Your selection list: {str(state['selection_list'])}")


with Window(title="Float Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        with Column(spacing=20, padding=[20]):
            add_text(content=(
                ""))
            with FileSystemDialog(on_folder_selected=folder_selected,
                                  on_file_selected=file_selected) as fsd_id:
                with Row(spacing=20):
                    add_button(label="Open for file", on_press=open_dialog_file)
                    add_button(label="Clear Selection List", on_press=clear_selection_list)

            add_combobox(options=filters,placeholder="Add a Filter",
                         width=200, on_select=add_selection_list)
            selection_id = add_text(
                content=f"Your selection list: {state['selection_list']}")

start_session()
