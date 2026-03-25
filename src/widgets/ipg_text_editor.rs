

use iced::{highlighter, Length};
use iced::widget::text_editor::{Content, Status};
use iced::widget::{
    text, text_editor,
};

use iced::{Element, Fill};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::IpgState;
use crate::app::Message;
use crate::widgets::ipg_text::TextWrapping;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_opt_usize};


#[derive(Debug, Clone)]
pub struct IpgTextEditor {
    pub id: usize,
    pub content: Content,
    pub place_holder: Option<String>, 
    pub font_id: Option<usize>,
    pub text_size: Option<f32>,
    pub line_height: Option<f32>,
    pub width: Length,
    pub height: Length,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub wrapping: Option<TextWrapping>,
    pub last_status: Status,
}

impl IpgTextEditor {
    pub fn construct<'a>(
        &'a self,
    ) -> Option<Element<'a, Message>> {

        let word_wrap = if let Some(tw) = self.wrapping {
            tw.to_iced()
        } else {
            text::Wrapping::None
        };

        let te: Element<'_, TxtEdMessage> = text_editor(&self.content)
                .placeholder("Type something here...")
                .height(Fill)
                .on_action(TxtEdMessage::ActionPerformed)
                .wrapping(word_wrap)
                .into();

        Some(te.map(move |message| Message::TextEditor(self.id, message)))
    }

}

// #[derive(Debug, Clone)]
// pub enum TxtEdStatus {
//     Active,
//     Hovered,
//     Focused { /* … */ },
//     Disabled,
// }

#[derive(Debug, Clone)]
pub enum TxtEdMessage {
    ActionPerformed(text_editor::Action),
    ThemeSelected(highlighter::Theme),
    WordWrapToggled(bool),
    // NewFile,
    // OpenFile,
    // FileOpened(Result<(PathBuf, Arc<String>), Error>),
    // SaveFile,
    // FileSaved(Result<PathBuf, Error>),
}



pub fn text_ed_callback(id: usize, message: TxtEdMessage, state: &mut IpgState) {
    match message {
        TxtEdMessage::ActionPerformed(action) => {
            let widget = state.widgets.get_mut(&id)
                .expect("text_ed_callback: widget id not found");
            let ed = widget.as_text_editor_mut()
                .expect("text_ed_callback: widget is not an IpgTextEditor");
            ed.content.perform(action);
        },
        TxtEdMessage::ThemeSelected(_theme) => todo!(),
        TxtEdMessage::WordWrapToggled(_) => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextEditorParam {
    FontId, 
    Height, 
    LineHeight, 
    MaxHeight, 
    MinHeight, 
    Padding, 
    PlaceHolder, 
    TextSize, 
    Width, 
    Wrapping, 
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgTextEditor {
    type Param = IpgTextEditorParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgTextEditorParam::FontId => set_opt_usize(&mut self.font_id, value, "IpgTextEditorParam::FontId"),
            IpgTextEditorParam::Height => todo!(),
            IpgTextEditorParam::LineHeight => todo!(),
            IpgTextEditorParam::MaxHeight => todo!(),
            IpgTextEditorParam::MinHeight => todo!(),
            IpgTextEditorParam::Padding => todo!(),
            IpgTextEditorParam::PlaceHolder => todo!(),
            IpgTextEditorParam::TextSize => todo!(),
            IpgTextEditorParam::Width => todo!(),
            IpgTextEditorParam::Wrapping => todo!(),
        }
    }
}


// #[derive(Debug, Clone)]
// pub enum Error {
//     DialogClosed,
//     IoError(io::ErrorKind),
// }

// fn open_file(
//     window: &dyn Window,
// ) -> impl Future<Output = Result<(PathBuf, Arc<String>), Error>> + use<> {
//     let dialog = rfd::AsyncFileDialog::new()
//         .set_title("Open a text file...")
//         .set_parent(&window);

//     async move {
//         let picked_file = dialog.pick_file().await.ok_or(Error::DialogClosed)?;

//         load_file(picked_file).await
//     }
// }

// async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>), Error> {
//     let path = path.into();

//     let contents = tokio::fs::read_to_string(&path)
//         .await
//         .map(Arc::new)
//         .map_err(|error| Error::IoError(error.kind()))?;

//     Ok((path, contents))
// }

// async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
//     let path = if let Some(path) = path {
//         path
//     } else {
//         rfd::AsyncFileDialog::new()
//             .save_file()
//             .await
//             .as_ref()
//             .map(rfd::FileHandle::path)
//             .map(Path::to_owned)
//             .ok_or(Error::DialogClosed)?
//     };

//     tokio::fs::write(&path, contents)
//         .await
//         .map_err(|error| Error::IoError(error.kind()))?;

//     Ok(path)
// }

// fn action<'a, Message: Clone + 'a>(
//     content: impl Into<Element<'a, Message>>,
//     label: &'a str,
//     on_press: Option<Message>,
// ) -> Element<'a, Message> {
//     let action = button(center_x(content).width(30));

//     if let Some(on_press) = on_press {
//         tooltip(
//             action.on_press(on_press),
//             label,
//             tooltip::Position::FollowCursor,
//         )
//         .style(container::rounded_box)
//         .into()
//     } else {
//         action.style(button::secondary).into()
//     }
// }

// fn new_icon<'a, Message>() -> Element<'a, Message> {
//     icon('\u{0e800}')
// }

// fn save_icon<'a, Message>() -> Element<'a, Message> {
//     icon('\u{0e801}')
// }

// fn open_icon<'a, Message>() -> Element<'a, Message> {
//     icon('\u{0f115}')
// }

// fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
//     const ICON_FONT: Font = Font::with_name("editor-icons");

//     text(codepoint)
//         .font(ICON_FONT)
//         .shaping(text::Shaping::Basic)
//         .into()
// }

// const EDITOR: &str = "editor";
