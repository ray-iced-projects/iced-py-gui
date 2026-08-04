
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use iced::Element;
use iced::Task;
use iced::Window;
use rfd::AsyncFileDialog;

use crate::state::Widgets;
use crate::state::access_file_dialog_actions;
use crate::{IpgState, app::Message,
    widgets::{callbacks::{invoke_callback_with_args}, 
    widget_param_update::{WidgetParamUpdate, set_t_value}}};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct FileSystemDialog {
    pub id: usize,
    pub opened: bool,
    pub select_file: Option<bool>,
    pub select_folder: Option<bool>,
    pub load_file: Option<bool>,
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
    FolderPicked(Option<PathBuf>),
    FilePicked(Option<PathBuf>),
    LoadFile(Option<PathBuf>),
}


pub fn fsw_callback(state: &mut IpgState, id: usize, message: FileSystemMessage) {

    match message {
        FileSystemMessage::FilePicked(path_opt) => {
            if let Some(Widgets::FileSystemDialog(fsw)) = state.widgets.get_mut(&id) {
                fsw.is_loading = false;

                if let Some(path) = path_opt {
                    // Extract file name
                    fsw.file_name = Some(path.display().to_string());
                    
                    // Invoke callback with the selected data
                        invoke_callback_with_args(
                            id,
                            "on_file_selected",
                            "FileSystemDialog",
                            (
                                fsw.file_name.clone(),
                            ),
                            "def on_file_selected(wid: int, file_name)",
                        );
                }
            }
        },
        FileSystemMessage::FolderPicked(path_opt) => {
            if let Some(Widgets::FileSystemDialog(fsw)) = state.widgets.get_mut(&id) {

                if let Some(path) = path_opt {
                    // Extract folder path
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
        FileSystemMessage::LoadFile(path_opt) => {
            

            // Invoke callback with the selected data
                // invoke_callback_with_args(
                //     id,
                //     "on_file_selected",
                //     "FileSystemDialog",
                //     (
                //         fsw.file_name.clone(),
                //         fsw.file_content.clone(),
                //     ),
                //     "def on_file_selected(wid: int, file_name)",
                // );
                
        },
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

fn open_file(
    window: &dyn Window,
) -> impl Future<Output = Result<(PathBuf, Arc<String>), Error>> + use<> {
    let dialog = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .set_parent(&window);

    async move {
        let picked_file = dialog.pick_file().await.ok_or(Error::DialogClosed)?;

        load_file(picked_file).await
    }
}

async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum FileSystemDialogParams {
    SelectFile,
    SelectFolder,
    LoadFile,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for FileSystemDialog {
    type Param = FileSystemDialogParams;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FileSystemDialogParams::SelectFile => {
                set_t_value(&mut self.select_file, value, "FileSystemWindowParams::SelectFile");
                let id = self.id;
                if self.select_file == Some(true) {
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
            FileSystemDialogParams::SelectFolder => {
                set_t_value(&mut self.select_folder, value, "FileSystemWindowParams::SelectFolder");
                
                if self.select_folder == Some(true) {
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
            FileSystemDialogParams::LoadFile => {
                set_t_value(&mut self.load_file, value, "FileSystemWindowParams::LoadContent");
                // let file = open_file(window)
                // let task = Task::perform(
                // load_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs",)),
                // Message::FileOpened,
                // );

                // let mut state = access_file_dialog_actions();
                //     state.tasks.push(task);
                //     drop(state);
                
            }
        }
    }
}
