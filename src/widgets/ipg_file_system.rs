
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use iced::Element;
use iced::Task;
use rfd::AsyncFileDialog;

use crate::state::Containers;
use crate::state::access_file_dialog_actions;
use crate::{IpgState, app::Message,
    widgets::{callbacks::{invoke_callback_with_args}, 
    widget_param_update::{WidgetParamUpdate, set_t_value}}};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct FileSystemDialog {
    pub id: usize,
    pub select_file: Option<bool>,
    pub select_folder: Option<bool>,
    pub load_file: Option<bool>,
    pub load_file_for_editor: Option<bool>,
    pub save_file: Option<bool>,
    pub is_loading: bool,
    pub folder_path: Option<String>,
    pub file_path: Option<String>,
    pub file_content: Option<String>,
    pub selected_path: Option<PathBuf>,
}

impl FileSystemDialog {

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {
        
        Some(content.remove(0))
    }
}


#[derive(Debug, Clone)]
pub enum FileSystemMessage {
    FolderPicked(Option<PathBuf>),
    FilePicked(Option<PathBuf>),
    LoadFile(Option<PathBuf>),
    FileLoaded(Option<String>, Option<String>),
    SaveFile(Option<PathBuf>),
    FileSaved(Option<PathBuf>),
}


pub fn fsd_callback(state: &mut IpgState, id: usize, message: FileSystemMessage) {

    match message {
        FileSystemMessage::FilePicked(path_opt) => {
            if let Some(Containers::FileSystemDialog(fsd)) = state.containers.get_mut(&id) {
                fsd.is_loading = false;

                if let Some(path) = path_opt {
                    // Extract file name
                    fsd.file_path = Some(path.display().to_string());
                    
                    // Invoke callback with the selected data
                        invoke_callback_with_args(
                            id,
                            "on_file_selected",
                            "FileSystemDialog",
                            fsd.file_path.clone(),
                            "def on_file_selected(wid: int, file_name)",
                        );
                }
            }
        },
        FileSystemMessage::FolderPicked(path_opt) => {
            if let Some(Containers::FileSystemDialog(fsd)) = state.containers.get_mut(&id) {
                fsd.is_loading = false;

                if let Some(path) = path_opt {
                    // Extract folder path
                    fsd.folder_path = Some(path.display().to_string());
                    
                    // Invoke callback with the selected data
                        invoke_callback_with_args(
                            id,
                            "on_folder_selected",
                            "FileSystemDialog",
                            fsd.folder_path.clone(),
                            "def on_folder_selected(wid: int, folder_name)",
                        );
                }
            }
        },
        FileSystemMessage::LoadFile(path_opt) => {
            if let Some(Containers::FileSystemDialog(fsd)) = state.containers.get_mut(&id) {
                fsd.is_loading = true;
            }
            if let Some(path) = path_opt {
                let task = Task::perform(
                    load_file(path.clone()),
                    move |result| {
                        match result {
                            Ok(content) => {
                                let file_name = path.file_name().map(|f| f.to_string_lossy().to_string());
                                let file_content = content.to_string();
                                Message::FileSystemWindow(
                                    id,
                                    FileSystemMessage::FileLoaded(file_name, Some(file_content)),
                                )
                            },
                            Err(e) => {
                                eprintln!("[ERROR] Failed to load file: {:?}", e);
                                Message::FileSystemWindow(id, FileSystemMessage::FileLoaded(None, None))
                            }
                        }
                    },
                );
                
                let mut file_dialog_actions = access_file_dialog_actions();
                file_dialog_actions.tasks.push(task);
                drop(file_dialog_actions);
            }
        },
        FileSystemMessage::FileLoaded(file_name, file_content) => {
            if let Some(Containers::FileSystemDialog(fsd)) = state.containers.get_mut(&id) {
                fsd.file_path = file_name;
                fsd.file_content = file_content;
                fsd.is_loading = false;
                
                // Invoke callback with the loaded data
                invoke_callback_with_args(
                    id,
                    "on_file_loaded",
                    "FileSystemDialog",
                    fsd.file_content.clone(),
                    "def on_file_loaded(wid: int, file_content: str | None)",
                );
            }
        },
        FileSystemMessage::SaveFile(path_opt) => {
            if let Some(Containers::FileSystemDialog(fsd)) = state.containers.get_mut(&id) {
                fsd.is_loading = true;
            }
            if let Some(path) = path_opt {
                if let Some(Containers::FileSystemDialog(fsd)) = state.containers.get(&id) {
                    if let Some(content) = fsd.file_content.clone() {
                        let task = Task::perform(
                            save_file(Some(path.clone()), content),
                            move |result| {
                                match result {
                                    Ok(saved_path) => {
                                        Message::FileSystemWindow(
                                            id,
                                            FileSystemMessage::FileSaved(Some(saved_path)),
                                        )
                                    },
                                    Err(e) => {
                                        eprintln!("[ERROR] Failed to save file: {:?}", e);
                                        Message::FileSystemWindow(id, FileSystemMessage::FileSaved(None))
                                    }
                                }
                            },
                        );
                        
                        let mut file_dialog_actions = access_file_dialog_actions();
                        file_dialog_actions.tasks.push(task);
                        drop(file_dialog_actions);
                    }
                }
            }
        },
        FileSystemMessage::FileSaved(path_opt) => {
            if let Some(Containers::FileSystemDialog(fsd)) = state.containers.get_mut(&id) {
                fsd.is_loading = false;
                
                if let Some(path) = path_opt {
                    fsd.file_path = Some(path.display().to_string());
                    
                    // Invoke callback with the saved file path
                    invoke_callback_with_args(
                        id,
                        "on_file_saved",
                        "FileSystemDialog",
                        fsd.file_path.clone(),
                        "def on_file_saved(wid: int, file_path: str | None)",
                    );
                }
            }
        },
    }
}


async fn load_file(path: impl Into<PathBuf>) -> io::Result<Arc<String>> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)?;

    Ok(contents)
}

async fn save_file(path: Option<PathBuf>, contents: String) -> io::Result<PathBuf> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dialog closed"))?
    };

    tokio::fs::write(&path, contents)
        .await?;

    Ok(path)
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum FileSystemDialogParam {
    SelectFile,
    SelectFolder,
    SelectFileForLoad,
    SaveFile,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for FileSystemDialog {
    type Param = FileSystemDialogParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FileSystemDialogParam::SelectFile => {
                set_t_value(&mut self.select_file, value, "FileSystemWindowParams::SelectFile");
                let id = self.id;
                if self.select_file == Some(true) && !self.is_loading {
                    self.is_loading = true;
                    let task = Task::perform(
                            async move {
                                AsyncFileDialog::new()
                                    .set_title("Select File")
                                    .pick_file().await.map(|h| h.path().to_path_buf())
                            },
                            move |path| Message::FileSystemWindow(id, FileSystemMessage::FilePicked(path)),
                        );

                    let mut state = access_file_dialog_actions();
                            state.tasks.push(task);
                            drop(state);
                }
            },
            FileSystemDialogParam::SelectFolder => {
                set_t_value(&mut self.select_folder, value, "FileSystemWindowParams::SelectFolder");
                
                if self.select_folder == Some(true) && !self.is_loading {
                    self.is_loading = true;
                    let id = self.id;
                    let task = Task::perform(
                            async move {
                                AsyncFileDialog::new()
                                    .set_title("Select Folder")
                                    .pick_folder().await.map(|h| h.path().to_path_buf())
                            },
                            move |path| Message::FileSystemWindow(id, FileSystemMessage::FolderPicked(path)),
                        );

                    let mut state = access_file_dialog_actions();
                            state.tasks.push(task);
                            drop(state);
                }
            },
            FileSystemDialogParam::SelectFileForLoad => {
                set_t_value(&mut self.select_file, value, "FileSystemWindowParams::SelectFileForLoad");
                let id = self.id;
                if self.select_file == Some(true) && !self.is_loading {
                    self.is_loading = true;
                    let task = Task::perform(
                            async move {
                                AsyncFileDialog::new()
                                    .set_title("Select File")
                                    .pick_file().await.map(|h| h.path().to_path_buf())
                            },
                            move |path| Message::FileSystemWindow(id, FileSystemMessage::LoadFile(path)),
                        );

                    let mut state = access_file_dialog_actions();
                            state.tasks.push(task);
                            drop(state);
                }
            },
            FileSystemDialogParam::SaveFile => {
                set_t_value(&mut self.save_file, value, "FileSystemWindowParams::SaveFile");
                let id = self.id;
                if self.save_file == Some(true) && !self.is_loading {
                    self.is_loading = true;
                    let task = Task::perform(
                            async move {
                                AsyncFileDialog::new()
                                    .set_title("Save File")
                                    .save_file().await.map(|h| h.path().to_path_buf())
                            },
                            move |path| Message::FileSystemWindow(id, FileSystemMessage::SaveFile(path)),
                        );

                    let mut state = access_file_dialog_actions();
                            state.tasks.push(task);
                            drop(state);
                }
            },
        }
    }
}
