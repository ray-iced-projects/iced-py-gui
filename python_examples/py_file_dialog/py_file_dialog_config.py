#!/usr/bin/env python3
"""
File System Dialog Updating use demo
"""
from enum import Enum
from icedpygui import (
    Window,
    Column,
    Container,
    ContainerStyleStd,
    Row,
    start_session,
    add_combobox,
    ComboBoxParam,
    add_file_system_dialog,
    add_radio,
    add_text_input,
    add_text,
    TextParam,
    FileSystemDialogParam as FsdParam,
    FileSystemDialogCallbackType as FsdCallType,
    get_dialog_filters,
    update_widget,
    update_widget_params,
    generate_id,
    )

state = {"filter_list": ["All Files"],
         "file_name": None,}


def set_file_name(_it_id: int, name: str):
    """Data from input text to set file name"""
    update_widget(fsd_id, FsdParam.FileName, name)
    update_widget(it_text_id, TextParam.Content, f"File name: {name}")


filters = get_dialog_filters()

def add_to_filter_list(_btn_id: int, filter_: str):
    """Adding to thea filter list for filters"""
    state["filter_list"].insert(0, filter_)
    update_widget(filter_text_id, TextParam.Content,
                  f"Your filter list: {str(state["filter_list"])}")
    update_widget(fsd_id, FsdParam.Filters, state["filter_list"])
    update_widget(cb_id, ComboBoxParam.Selected, None)


def clear_filter_list(_btn_id: int):
    """Clearing filter list"""
    state["filter_list"] = ["All Files"]
    update_widget(filter_text_id, TextParam.Content,
                  f"Your filter list: {str(state['filter_list'])}")
    update_widget(fsd_id, FsdParam.Filters, [])


def set_default_directory(_btn_id: int):
    """Setting default directory bool value
    When the dialog opens and a foler is selected, then
    the folder will be added to the FSD to set it"""
    state["set_default_folder"] = True

radio_labels = ["Select a single file name", "Select many file names",
                "Select a single folder name", "Select many folder names",
                "Load a file", "Set file name", "Save a file",
                "Set default Directory", "Add a Filter", "Clear Filter List",]



fsd_callbacks = ["file", "files", "folder", "folders"]

SOME_FILE_CONTENT = "This is some file content\n"

def dialog_callbacks(_rd_id: int, idx: int):
    """Seting Dialog parameters"""
    match idx:
        case 0: # Select a file name
            update_widget(fsd_id, FsdParam.SelectFile, True)
        case 1: # Select many file names
            update_widget(fsd_id, FsdParam.SelectFiles, True)
        case 2: # Select a folder name
            update_widget(fsd_id, FsdParam.SelectFolder, True)
        case 3: # Select many folder names
            update_widget(fsd_id, FsdParam.SelectFolders, True)
        case 4: # Load a file
            update_widget(fsd_id, FsdParam.LoadFile, True)
        case 5: # Save a file
            if state["file_name"] is None:
                update_widget(ms_fs_id, TextParam.Content,
                    "***[WARNING]: File name not set so you have to enter it")
            update_widget_params(fsd_id, {
                FsdParam.FileContent: SOME_FILE_CONTENT,
                FsdParam.SaveFile: True,
                })
        case 6: # "Set default Directory"results_callback
            print("case 6 Set directory")
            update_widget(fsd_id, FsdParam.DefaultDirectory, "/home/charles/Documents")


def results_callback(_fsd_id: int, results: tuple[FsdCallType, any]):
    """Results from Dialog"""
    (fsd_type, data) = results
    match fsd_type:
        case FsdCallType.File:
            update_widget(file_text_id, TextParam.Content,
                          f"[MESSAGE]: Selected file name is: {data}")
        case FsdCallType.Files:
            update_widget(files_text_id, TextParam.Content,
                          "[MESSAGE]: Selected file names are:\n" + "\n".join(data))
        case FsdCallType.Folder:
            update_widget(folder_text_id, TextParam.Content,
                          f"[MESSAGE]: Selected folder name is: {data}")
        case FsdCallType.Folders:
            update_widget(folders_text_id, TextParam.Content,
                          "[MESSAGE]: Selected folder names are:\n" + "\n".join(data))
        case FsdCallType.FileLoaded:
            update_widget(file_loaded_id, TextParam.Content,
                            f"[MESSAGE]: Loaded file is [30 chars]:\n{data[:30]}")
        case FsdCallType.FileSaved:
            update_widget(ms_fs_id, TextParam.Content,
                            f"[MESSAGE]: Saved file name is: {data}")
        case FsdCallType.DefaultDirectory:
            update_widget(ms_dir_id, TextParam.Content, f"[MESSAGE]: Default Directory is: {data}")


fsd_id = add_file_system_dialog(results_callback=results_callback)
ROW_WIDTH = 300

with Window(title="FileSystemDialog Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        with Row():
            # left column
            with Column(spacing=20, padding=[0,0,0,10]): # left_padding set
                # Using the group_id in a radio button, with adding only 1 button,
                # one can separate the radios to allowing the results close by or
                # help with any alignment issues.  More than one button can be
                # added so that groups of buttons can act as one set of radios where
                # the selection remains as one.
                group_id = generate_id()
                for (index, label) in enumerate(radio_labels):
                    match index:
                        case 0: # Select a single file name
                            with Row(spacing=20):
                                with Container(width=ROW_WIDTH):
                                    add_radio(labels=[label], group_id=group_id,
                                            on_selected=dialog_callbacks)
                                file_text_id = add_text(content="File name: None")

                        case 1: # Select many file names
                            with Row(spacing=20):
                                with Container(width=ROW_WIDTH):
                                    add_radio(labels=[label], group_id=group_id,
                                                on_selected=dialog_callbacks)
                                files_text_id = add_text(content="File names: None")

                        case 2: # Select a folder name
                            with Row(spacing=20):
                                with Container(style_std=ContainerStyleStd.BorderedBox,
                                                width=ROW_WIDTH):
                                    add_radio(labels=[label], group_id=group_id,
                                                on_selected=dialog_callbacks)
                                folder_text_id = add_text(content="Folder name: None")

                        case 3: # Select many folder names
                            with Row(spacing=20):
                                with Container(style_std=ContainerStyleStd.BorderedBox,
                                           width=ROW_WIDTH):

                                    add_radio(labels=[label], group_id=group_id,
                                                on_selected=dialog_callbacks)
                                folders_text_id = add_text(content="Folder names: None")

                        case 4: # Load a file
                            with Row(spacing=20):
                                with Container(style_std=ContainerStyleStd.BorderedBox,
                                           width=ROW_WIDTH):

                                    add_radio(labels=[label], group_id=group_id,
                                                on_selected=dialog_callbacks)
                                file_loaded_id = add_text(content="First 30 chars of file: None")

                        case 5: # Set file name
                            with Row(spacing=20):
                                with Container(style_std=ContainerStyleStd.BorderedBox,
                                           width=ROW_WIDTH):
                                    add_text_input(placeholder="Set Filename: Input file name",
                                                   on_submit=set_file_name)
                                it_text_id = add_text(content="File name: None")

                        case 6: # Save a file
                            with Row(spacing=20):
                                with Container(style_std=ContainerStyleStd.BorderedBox,
                                           width=ROW_WIDTH):
                                    add_radio(labels=[label], group_id=group_id,
                                                on_selected=dialog_callbacks)
                                ms_fs_id = add_text(content="[MESSAGE]: None")

                        case 7: # "Set default Directory"
                            with Row(spacing=20):
                                with Container(style_std=ContainerStyleStd.BorderedBox,
                                            width=ROW_WIDTH):
                                    add_radio(labels=[label], group_id=group_id,
                                                on_selected=dialog_callbacks)
                                ms_dir_id = add_text(content="[MESSAGE]: None")


                cb_id = add_combobox(options=filters, placeholder="Add a Filter",
                            width=200, on_select=add_to_filter_list)

                filter_text_id = add_text(
                    content=f"Your filter list: {state['filter_list']}")

start_session()
