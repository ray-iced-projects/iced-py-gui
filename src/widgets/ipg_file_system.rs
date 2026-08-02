use std::{collections::HashMap, io::Error, path::PathBuf, sync::Arc};

use iced::{Element, Task, Window, window};

use crate::{app::Message, state::Widgets, widgets::{callbacks::invoke_callback, widget_param_update::{WidgetParamUpdate, set_t_value}}};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct FileSystemWindow {
    pub id: usize,
    pub opened: bool,
    pub select_file: Option<bool>,
    pub select_folder: Option<bool>,
}

impl FileSystemWindow {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        ) -> Option<Element<'a, Message>> {
        
            
            None
        }
}

#[derive(Debug, Clone)]
pub enum FileSystemMessage {
    OpenFile,
    FileOpened,
    OnClose,
    OnOpen,
}


pub fn fsw_callback(id: usize, message: FileSystemMessage) {
    match message {
        // FileSystemMessage::ClickedOutside => {
        //     invoke_callback(id, "on_click_outside", "PopUp");
        // },
        FileSystemMessage::OnClose => {
            invoke_callback(id, "on_close", "PopUp");
        },
        FileSystemMessage::OnOpen => {
            invoke_callback(id, "on_open", "PopUp");
        },
        FileSystemMessage::OpenFile => {
            self.is_loading = true;
            window::oldest()
                .and_then(|id| window::run(id, open_file))  // ← Runs the picker async
                .then(Task::future)
                .map(FileSystemMessage::FileOpened);  // ← Returns result back as message
        },
        FileSystemMessage::FileOpened =>  {
            open_file(&window::oldest());
        },
    }
}

async fn open_file(window: &dyn Window) -> Result<(PathBuf, Arc<String>), Error> {
    let dialog = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .set_parent(&window);  // ← Attaches to window
    
    let picked_file = dialog.pick_file().await?;  // ← Awaits user selection
    load_file(picked_file).await
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum FileSystemWindowParams {
    Opened,
    SelectFile,
    SelectFolder,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for FileSystemWindow {
    type Param = FileSystemWindowParams;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FileSystemWindowParams::Opened => set_t_value(&mut self.opened, value, "FileSystemWindowParams::Opened"),
            FileSystemWindowParams::SelectFile => set_t_value(&mut self.select_file, value, "FileSystemWindowParams::SelectFile"),
            FileSystemWindowParams::SelectFolder => set_t_value(&mut self.select_folder, value, "FileSystemWindowParams::SelectFolder"),
        }
    }
}
