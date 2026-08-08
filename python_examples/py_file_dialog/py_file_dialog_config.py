#!/usr/bin/env python3
"""
File System Dialog Updating use demo
"""

from icedpygui import (
    Window,
    Column,
    Container,
    Row,
    start_session,
    add_button,
    add_combobox,
    add_file_system_dialog,
    ComboBoxParam,
    add_text,
    TextParam,
    FileSystemDialogParam,
    get_dialog_filters,
    update_widget,
    )


state = {"filter_list": ["All Files"],
         "set_default_folder": False,}


def open_dialog_folder(_btn_id: int):
    """Open Dialog to select a folder"""
    update_widget(fsd_id, FileSystemDialogParam.SelectFolder, True)


def open_dialog_file(_btn_id: int):
    """Open Dialog to select a folder"""
    update_widget(fsd_id, FileSystemDialogParam.SelectFile, True)

def open_dialog_files(_btn_id: int):
    """Select multiple files"""
    update_widget(fsd_id, FileSystemDialogParam.SelectFiles, True)

def folder_selected(_fsd_id: int, folder_path: str):
    """Folder selected"""
    print(folder_path)
    if state["set_default_folder"]:
        update_widget(fsd_id, FileSystemDialogParam.InitialDirectory, folder_path)


def file_selected(_fsd_id: int, file_path: str):
    """File selected"""
    print("file_name", file_path)

filters = get_dialog_filters()

def add_to_filter_list(_btn_id: int, filter_: str):
    """Adding to thea filter list for filters"""
    state["filter_list"].insert(0, filter_)
    update_widget(selection_id, TextParam.Content,
                  f"Your filter list: {str(state["filter_list"])}")
    update_widget(fsd_id, FileSystemDialogParam.Filters, state["filter_list"])
    update_widget(cb_id, ComboBoxParam.Selected, None)

def clear_filter_list(_btn_id: int):
    """Clearing filter list"""
    state["filter_list"] = ["All Files"]
    update_widget(selection_id, TextParam.Content,
                  f"Your filter list: {str(state['filter_list'])}")
    update_widget(fsd_id, FileSystemDialogParam.Filters, [])

def set_default_directory(_btn_id: int):
    """Setting default directory bool value
    When the dialog opens and a foler is selected, then
    the folder will be added to the FSD to set it"""
    state["set_default_folder"] = True

fsd_id = add_file_system_dialog(on_folder_selected=folder_selected,
                                  on_file_selected=file_selected)

with Window(title="FileSystemDialog Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        with Column(spacing=20, padding=[20]):

            with Row(spacing=20):
                with Column(spacing=10):
                    add_button(label="Open for a file", on_press=open_dialog_file)
                    add_button(label="Open for files", on_press=open_dialog_files)
                add_button(label="Open for a Folder", on_press=open_dialog_folder)
                add_button(label="Clear Filter List", on_press=clear_filter_list)
                add_button(label="Set default Directory", on_press=set_default_directory)

            cb_id = add_combobox(options=filters, placeholder="Add a Filter",
                         width=200, on_select=add_to_filter_list)
            selection_id = add_text(
                content=f"Your filter list: {state['filter_list']}")

start_session()
