
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use iced::Task;
use rfd::AsyncFileDialog;

use crate::state::Widgets;
use crate::state::access_file_dialog_actions;
use crate::config_creator::load_file_filters;
use crate::{IpgState, app::Message,
    widgets::{callbacks::{invoke_callback_with_args}, 
    widget_param_update::{WidgetParamUpdate, set_t_value}}};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;



#[derive(Debug, Clone)]
pub struct FileSystemDialog {
    pub id: usize,
    pub select_file: Option<bool>,
    pub select_files: Option<bool>,
    pub select_folder: Option<bool>,
    pub select_folders: Option<bool>,
    pub load_file: Option<bool>,
    pub load_file_for_editor: Option<bool>,
    pub save_file: Option<bool>,
    pub is_loading: bool,
    pub folder_path: Option<String>,
    pub file_path: Option<String>,
    pub file_paths: Option<Vec<String>>,
    pub file_content: Option<String>,
    pub selected_path: Option<PathBuf>,
    pub filters: Vec<String>,
    pub initial_directory: Option<String>,
    pub show_hidden_files: Option<bool>,
    pub remember_last_directory: Option<bool>,
    pub update_json_file: Option<bool>,
}

// Rules
// Either pick_file(s) or pick_folder(s), setting both gives an error
// .set_can_create_directories(bool) **mac os only** 
// .set_file_name works only for pick_file and save_file
// .set_format_label("File Type:") 
//    macOS only — has no effect on Windows or Linux
//    Use case: save_file() dialogs (not pick_file())
//    When it appears: Only when you register two or more filters with add_filter()
//    Default label: "Format:"
// .set_parent()
//    The dialog appears as a modal child of that parent window
//    The dialog is owned by and dependent on the parent window
//    Users typically can't interact with the parent window while the dialog is open
//    The dialog may be positioned relative to or on top of the parent window
// .set_show_hidden_files(true)
//    Only works if feature[features] default = ["gtk3"] for linux only
//    If not selected then defaults to user filesystem setting.
//    Therefore needs to be set to no display hidden folder if needed.


#[derive(Debug, Clone)]
struct FileDialogSettings {
    #[allow(dead_code)]
    select_file: bool,
    #[allow(dead_code)]
    select_files: bool,
    #[allow(dead_code)]
    select_folder: bool,
    #[allow(dead_code)]
    select_folders: bool,
    initial_directory: Option<String>,
    filters: Vec<String>,
}

impl FileSystemDialog {

    /// Extract dialog settings
    fn extract_dialog_settings(&self) -> FileDialogSettings {
        let mut filters = self.filters.clone();
        
        if !filters.contains(&"All Files".to_string()) {
            filters.push("All Files".to_string());
        }

        // Determine which dialog mode should be active
        let active_count = [
            self.select_file == Some(true),
            self.select_files == Some(true),
            self.select_folder == Some(true),
            self.select_folders == Some(true),
        ].iter().filter(|&&m| m).count();
        
        let (select_file, select_files, select_folder, select_folders) = if active_count > 1 {
            eprintln!("***[WARNING]*** Multiple file(s) or Folder(s) are set. Only one can be true. Defaulting to: select_file = True.");
            (true, false, false, false)
        } else if active_count == 1 {
            (
                self.select_file == Some(true),
                self.select_files == Some(true),
                self.select_folder == Some(true),
                self.select_folders == Some(true),
            )
        } else {
            // No modes set, default to select_file
            (true, false, false, false)
        };
        
        FileDialogSettings {
            initial_directory: self.initial_directory.clone(),
            filters,
            select_file,
            select_files,
            select_folder,
            select_folders,
        }
    }
}


#[derive(Debug, Clone)]
pub enum FileSystemMessage {
    FolderPicked(Option<PathBuf>),
    FilePicked(Option<PathBuf>),
    FilesPicked(Option<Vec<PathBuf>>),
    LoadFile(Option<PathBuf>),
    FileLoaded(Option<String>, Option<String>),
    SaveFile(Option<PathBuf>),
    FileSaved(Option<PathBuf>),
}


// Helper function to create a file dialog with configured filters and settings
fn create_file_dialog(settings: &FileDialogSettings) -> AsyncFileDialog {
    
    let mut dialog = AsyncFileDialog::new();
    
    // Set initial directory if provided
    if let Some(ref init_dir) = settings.initial_directory {
        dialog = dialog.set_directory(init_dir);
    }


    
    // Load filters from configuration
    if let Ok(default_filters) = load_file_filters() {
        // Iterate through requested filters and search in default_filters
        for requested_filter in &settings.filters {
            if let Some((_, extensions)) = default_filters.iter().find(|(name, _)| name == requested_filter) {
                // Parse extensions: split by ';', trim, and remove wildcards
                let ext_list: Vec<String> = extensions
                    .split(';')
                    .map(|e| {
                        e.trim()
                            .trim_start_matches('*')
                            .trim_start_matches('.')
                            .to_string()
                    })
                    .filter(|e| !e.is_empty())
                    .collect();

                dialog = dialog.add_filter(requested_filter, &ext_list);
            } else {
                eprintln!("***[ERROR]*** Filter '{}' could not be found in config", requested_filter);
            }
        }
        
    } else {
        eprintln!("***[Error]*** Unable to load file filters, check the config file is setup properly");
    }
    
    dialog
}

pub fn fsd_callback(state: &mut IpgState, id: usize, message: FileSystemMessage) {

    match message {
        FileSystemMessage::FilePicked(path_opt) => {
            if let Some(Widgets::FileSystemDialog(fsd)) = state.widgets.get_mut(&id) {
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
        FileSystemMessage::FilesPicked(path_opt) => {
            if let Some(Widgets::FileSystemDialog(fsd)) = state.widgets.get_mut(&id) {
                fsd.is_loading = false;

                if let Some(paths) = path_opt {
                    // Extract file names and convert to strings
                    fsd.file_paths = Some(paths.iter().map(|p| p.display().to_string()).collect());
                    
                    // Invoke callback with the selected data
                        invoke_callback_with_args(
                            id,
                            "on_file_selected",
                            "FileSystemDialog",
                            fsd.file_paths.clone(),
                            "def on_file_selected(wid: int, [file_name])",
                        );
                }
            }
        },
        FileSystemMessage::FolderPicked(path_opt) => {
            if let Some(Widgets::FileSystemDialog(fsd)) = state.widgets.get_mut(&id) {
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
            if let Some(Widgets::FileSystemDialog(fsd)) = state.widgets.get_mut(&id) {
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
            if let Some(Widgets::FileSystemDialog(fsd)) = state.widgets.get_mut(&id) {
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
            if let Some(path) = path_opt {
                if let Some(Widgets::FileSystemDialog(fsd)) = state.widgets.get_mut(&id) {
                    fsd.is_loading = true;
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
            if let Some(Widgets::FileSystemDialog(fsd)) = state.widgets.get_mut(&id) {
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
    SelectFiles,
    SelectFolder,
    SelectFileForLoad,
    SaveFile,
    Filters,
    InitialDirectory,
    ShowHiddenFiles,
    RememberLastDirectory,
    UpdateJsonFile,
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
                    
                    let settings = self.extract_dialog_settings();
                    
                    let task = Task::perform(
                            async move {
                                create_file_dialog(&settings)
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
            FileSystemDialogParam::SelectFiles => {
                set_t_value(&mut self.select_files, value, "FileSystemWindowParams::SelectFiles");
                let id = self.id;
                if self.select_files == Some(true) && !self.is_loading {
                    self.is_loading = true;
                    
                    let settings = self.extract_dialog_settings();
                    
                    let task = Task::perform(
                            async move {
                                create_file_dialog(&settings)
                                    .set_title("Select Files")
                                    .pick_files().await.map(|handles| handles.iter().map(|h| h.path().to_path_buf()).collect())
                            },
                            move |paths| Message::FileSystemWindow(id, FileSystemMessage::FilesPicked(paths)),
                        );

                        let mut state = access_file_dialog_actions();
                        state.tasks.push(task);
                        drop(state);
                }
            },
            FileSystemDialogParam::SelectFolder => {
                set_t_value(&mut self.select_folder, value, "FileSystemWindowParams::SelectFolder");
                self.select_file = Some(false);
                if self.select_folder == Some(true) && !self.is_loading {
                    self.is_loading = true;
                    let id = self.id;

                    let settings = self.extract_dialog_settings();

                    let task = Task::perform(
                            async move {
                                create_file_dialog(&settings)
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
                    
                    let settings = self.extract_dialog_settings();
                    
                    let task = Task::perform(
                            async move {
                                create_file_dialog(&settings)
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
                    
                    let settings = self.extract_dialog_settings();
                    
                    let task = Task::perform(
                            async move {
                                create_file_dialog(&settings)
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
            FileSystemDialogParam::Filters => {
                set_t_value(&mut self.filters, value, "FileSystemWindowParams::Filters");
            },
            FileSystemDialogParam::InitialDirectory => {
                set_t_value(&mut self.initial_directory, value, "FileSystemWindowParams::InitialDirectory");
            },
            FileSystemDialogParam::ShowHiddenFiles => {
                set_t_value(&mut self.show_hidden_files, value, "FileSystemWindowParams::ShowHiddenFiles");
            },
            FileSystemDialogParam::RememberLastDirectory => {
                set_t_value(&mut self.remember_last_directory, value, "FileSystemWindowParams::RememberLastDirectory");
            },
            FileSystemDialogParam::UpdateJsonFile => {
                set_t_value(&mut self.update_json_file, value, "FileSystemWindowParams::UpdateJsonFile");
            },
        }
    }
}
