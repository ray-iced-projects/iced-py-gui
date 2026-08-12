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
    add_combobox,
    ComboBoxParam,
    add_file_system_dialog,
    add_radio,
    add_text_input,
    add_text,
    TextParam,
    FileSystemDialogParam,
    FileSystemDialogCallbackType as FsdCallType,
    get_dialog_filters,
    update_widget,
    update_widget_params,
    )


state = {"filter_list": ["All Files"],
         "file_name": None,}

def set_file_name(_it_id: int, data: str):
    """Data from input text to set file name"""
    state["file_name"] = data
    print(data)


filters = get_dialog_filters()

def add_to_filter_list(_btn_id: int, filter_: str):
    """Adding to thea filter list for filters"""
    state["filter_list"].insert(0, filter_)
    update_widget(text_id, TextParam.Content,
                  f"Your filter list: {str(state["filter_list"])}")
    update_widget(fsd_id, FileSystemDialogParam.Filters, state["filter_list"])
    update_widget(cb_id, ComboBoxParam.Selected, None)


def clear_filter_list(_btn_id: int):
    """Clearing filter list"""
    state["filter_list"] = ["All Files"]
    update_widget(text_id, TextParam.Content,
                  f"Your filter list: {str(state['filter_list'])}")
    update_widget(fsd_id, FileSystemDialogParam.Filters, [])


def set_default_directory(_btn_id: int):
    """Setting default directory bool value
    When the dialog opens and a foler is selected, then
    the folder will be added to the FSD to set it"""
    state["set_default_folder"] = True

radio_labels = ["Select a single file name", "Select many file names",
                "Select a single folder name", "Select many folder names",
                "Load a file", "Set file name (first use input text below)", "Save a file",
                "Set default Directory", "Clear Filter List",]

fsd_callbacks = ["file", "files", "folder", "folders"]

SOME_FILE_CONTENT = "This is some file content"

def dialog_callbacks(_rd_id: int, index: int):
    """Seting Dialog parameters"""
    match index:
        case 0: # Select a file name
            update_widget(fsd_id, FileSystemDialogParam.SelectFile, True)
        case 1: # Select many file names
            update_widget(fsd_id, FileSystemDialogParam.SelectFiles, True)
        case 2: # Select a folder name
            update_widget(fsd_id, FileSystemDialogParam.SelectFolder, True)
        case 3: # Select many folder names
            update_widget(fsd_id, FileSystemDialogParam.SelectFolders, True)
        case 4: # Load a file
            update_widget(fsd_id, FileSystemDialogParam.LoadFile, True)
        case 5: # Set file name
            update_widget(fsd_id, FileSystemDialogParam.FileName, state["file_name"])
        case 6: # Save a file
            if state["file_name"] is None:
                update_widget(warning_id, TextParam.Content,
                    "***[WARNING]: File name not set so you have to enter it")
            update_widget_params(fsd_id, {
                FileSystemDialogParam.FileContent: SOME_FILE_CONTENT,
                FileSystemDialogParam.SaveFile: True,
                })
        case 7: # "Set default Directory"
            update_widget(fsd_id, FileSystemDialogParam.DefaultDirectory, "/home/charles/Documents")


def results_callback(_fsd_id: int, results: tuple[FsdCallType, any]):
    """Results from Dialog"""
    (fsd_type, data) = results
    match fsd_type:
        case FsdCallType.File:
            update_widget(text_id, TextParam.Content,
                          f"Selected file name is: {data}")
        case FsdCallType.Files:
            update_widget(text_id, TextParam.Content,
                          "Selected file names are:\n" + "\n".join(data))
        case FsdCallType.Folder:
            update_widget(text_id, TextParam.Content,
                          f"Selected folder name is: {data}")
        case FsdCallType.Folders:
            update_widget(text_id, TextParam.Content,
                          "Selected folder names are:\n" + "\n".join(data))
        case FsdCallType.FileLoaded:
            update_widget(file_id, TextParam.Content,
                            f"Loaded file is:\n{data}")
        case FsdCallType.FileSaved:
            update_widget(text_id, TextParam.Content,
                            f"Saved file name is:\n{data}")


fsd_id = add_file_system_dialog(results_callback=results_callback)

with Window(title="FileSystemDialog Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        with Column(spacing=20, padding=[20]):

            with Row(spacing=20):
                with Column(spacing=10):
                    add_radio(labels=radio_labels, radio_spacing=5, on_selected=dialog_callbacks)

            cb_id = add_combobox(options=filters, placeholder="Add a Filter",
                         width=200, on_select=add_to_filter_list)
            text_id = add_text(
                content=f"Your filter list: {state['filter_list']}")

            add_text(content="Use the text input below to set a file name")
            add_text_input(placeholder="Set file name", width=200, on_submit=set_file_name)

            warning_id = add_text(content="***[WARNINGS]: None")

            file_id = add_text(content="File content: None")
start_session()
