#!/usr/bin/env python3
"""
File System Dialog use demo
"""

from icedpygui import (
    Window,
    Column,
    Container,
    FileSystemDialog,
    Row,
    start_session,
    add_button,
    add_text,
    add_text_editor,
    FileSystemDialogParam,
    TextEditorParam,
    update_widget,
    )


state = {"file_content": ""}

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


def load_file(_btn_id: int):
    """Load file content"""
    update_widget(fsd_id, FileSystemDialogParam.SelectFileForLoad, True)


def file_loaded(_fsd_id: int, content: str):
    """File loaded"""
    update_widget(editor_id, TextEditorParam.Content, content)


def save_file(_btn_id: int):
    """Save an Editor File"""
    update_widget(fsd_id, FileSystemDialogParam.SaveFile, True)


with Window(title="Float Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        with Column(spacing=20, padding=[20]):
            add_text(content=(
                "Since this demo is for a FileSystemDialog where you may just "
                "want to get a file name or folder name, you probably wouldn't "
                "normally use buttons for things like getting a file name or getting a folder "
                "name for the text editor as done here. "
                "See the py_text_editor.py example for using icons and such to loading a file "
                "using the FileSystemDialog.\n\n[NOTE]: The folder name and file name will be "
                "printed when the appropriate button is pressed."))
            with FileSystemDialog(on_folder_selected=folder_selected,
                                  on_file_selected=file_selected,
                                  on_file_loaded=file_loaded) as fsd_id:
                with Row(spacing=20):
                    add_button(label="Open File Dialog for Folder", on_press=open_dialog_folder)
                    add_button(label="Open File Dialog For Filename", on_press=open_dialog_file)
                    add_button(label="Load file for text editor", on_press=load_file)
                    add_button(label="Save Editor File", on_press=save_file)

            editor_id = add_text_editor(content=state["file_content"],
                                        placeholder="Select a file",
                                        fill=True)

start_session()
