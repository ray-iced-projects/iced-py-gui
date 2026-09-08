

from icedpygui import (
    Window,
    Container,
    add_file_system_dialog,
    FileSystemDialogParam as fsdParam,
    add_button,
    start_session,
    update_widget,
)


def on_selected_file_path(_fsd_id: int, fp: str):
    """Select only the file path"""
    print(fp)


fsd_id = add_file_system_dialog(
    results_callback=on_selected_file_path
)

def config_fsd(_btn_id: int):
    """Update FSD"""
    update_widget(fsd_id, fsdParam.SelectFile, True)


with Window(title="FileSystemDialog Example", center=True):

    with Container(width_fill=True, height_fill=True, align_center=True):
        add_button(label="Select file Path",
                   on_press=config_fsd)

start_session()
