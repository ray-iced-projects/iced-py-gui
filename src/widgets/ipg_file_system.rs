
use std::path::PathBuf;

use iced::Element;
use iced::Task;
use rfd::AsyncFileDialog;

use crate::state::Widgets;
use crate::state::access_file_dialog_actions;
use crate::{IpgState, app::Message,
    widgets::{callbacks::{invoke_callback, invoke_callback_with_args}, 
    widget_param_update::{WidgetParamUpdate, set_t_value}}};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct FileSystemDialog {
    pub id: usize,
    pub opened: bool,
    pub select_file: Option<bool>,
    pub select_folder: Option<bool>,
    pub load_content: Option<bool>,
    pub is_loading: bool,
    pub folder_name: Option<String>,
    pub file_name: Option<String>,
    pub file_content: Option<String>,
    pub selected_path: Option<PathBuf>,
}

impl FileSystemDialog {

    pub fn construct<'a>(
        &'a self,
        ) -> Option<Element<'a, Message>> {
        

        None
    }
}


#[derive(Debug, Clone)]
pub enum FileSystemMessage {
    OpenFile,
    FileOpened(Option<PathBuf>),
    OnClose,
    OnOpen,
    FolderPicked(Option<PathBuf>),
}


pub fn fsw_callback(state: &mut IpgState, id: usize, message: FileSystemMessage) {

    match message {
        FileSystemMessage::OnClose => {
            invoke_callback(id, "on_close", "FileSystemWindow");
        },
        FileSystemMessage::OnOpen => {
            invoke_callback(id, "on_open", "FileSystemWindow");
        },
        FileSystemMessage::OpenFile => {
            // Mark as loading
            if let Some(Widgets::FileSystemDialog(fsw)) = state.widgets.get_mut(&id) {
                fsw.is_loading = true;
                
                // Determine dialog type based on current flag configuration
                // let dialog_type = fsw.get_dialog_type();
                
                // Spawn the async file dialog task based on dialog type
                // return Some(Task::perform(
                //     async move {
                //         let dialog = AsyncFileDialog::new();
                        
                //         match dialog_type {
                //             Some(DialogType::File { load_content: _ }) => {
                //                 dialog.pick_file().await.map(|h| h.path().to_path_buf())
                //             },
                //             Some(DialogType::Folder) => {
                //                 dialog.pick_folder().await.map(|h| h.path().to_path_buf())
                //             },
                //             None => None,
                //         }
                //     },
                //     move |path| Message::FileSystemWindow(id, FileSystemMessage::FileOpened(path)),
                // ));
            }
        },
        FileSystemMessage::FileOpened(path_opt) => {
            if let Some(Widgets::FileSystemDialog(fsw)) = state.widgets.get_mut(&id) {
                fsw.is_loading = false;
                
                // Process the selected path
                if let Some(path) = path_opt {
                    fsw.selected_path = Some(path.clone());
                    
                    // Extract folder name
                    if let Some(parent) = path.parent() {
                        fsw.folder_name = Some(parent.to_string_lossy().to_string());
                    }
                    
                    // Extract file name
                    if let Some(file_name) = path.file_name() {
                        fsw.file_name = Some(file_name.to_string_lossy().to_string());
                    }
                    
                    // Read file content if both select_file and load_content are true
                    let select_file = fsw.select_file.unwrap_or(false);
                    let load_content = fsw.load_content.unwrap_or(false);
                    
                    if select_file && load_content && path.is_file() {
                        match std::fs::read_to_string(&path) {
                            Ok(content) => {
                                fsw.file_content = Some(content);
                            },
                            Err(e) => {
                                eprintln!("[ERROR] Failed to read file {:?}: {}", path, e);
                            }
                        }
                    }
                    
                    // Invoke callback with the selected data
                    invoke_callback_with_args(
                        id,
                        "on_file_selected",
                        "FileSystemWindow",
                        (
                            fsw.folder_name.clone(),
                            fsw.file_name.clone(),
                            fsw.file_content.clone(),
                        ),
                        "def on_file_selected(wid: int, data: tuple[str | None, str | None, str | None])",
                    );
                }
            }
        },
        FileSystemMessage::FolderPicked(path_opt) => {
            if let Some(Widgets::FileSystemDialog(fsw)) = state.widgets.get_mut(&id) {
                fsw.is_loading = false;

                if let Some(path) = path_opt {
                    // Extract folder name
                    fsw.folder_name = Some(path.display().to_string());
                    
                    // Invoke callback with the selected data
                        invoke_callback_with_args(
                            id,
                            "on_folder_selected",
                            "FileSystemDialog",
                            (
                                fsw.folder_name.clone(),
                            ),
                            "def on_folder_selected(wid: int, folder_name)",
                        );
                }
            }
        },
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum FileSystemDialogParams {
    Opened,
    SelectFile,
    SelectFolder,
    LoadContent,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for FileSystemDialog {
    type Param = FileSystemDialogParams;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FileSystemDialogParams::Opened => set_t_value(&mut self.opened, value, "FileSystemWindowParams::Opened"),
            FileSystemDialogParams::SelectFile => set_t_value(&mut self.select_file, value, "FileSystemWindowParams::SelectFile"),
            FileSystemDialogParams::SelectFolder => {
                set_t_value(&mut self.select_folder, value, "FileSystemWindowParams::SelectFolder");
                
                if self.select_folder == Some(true) {
                    let id = self.id;
                    let task = Task::perform(
                            async move {
                                let dialog = AsyncFileDialog::new();
                                dialog.pick_folder().await.map(|h| h.path().to_path_buf())
                            },
                            move |path| Message::FileSystemWindow(id, FileSystemMessage::FolderPicked(path)),
                        );

                    let mut state = access_file_dialog_actions();
                            state.tasks.push(task);
                            drop(state);
                }
            },
            FileSystemDialogParams::LoadContent => set_t_value(&mut self.load_content, value, "FileSystemWindowParams::LoadContent"),
        }
    }
}
