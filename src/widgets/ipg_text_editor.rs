

use std::collections::HashMap;

use iced::Length;
use iced::Padding;
use iced::highlighter;
use iced::widget;

use iced::Element;
use iced::widget::text::Wrapping;
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::get_len;
use crate::py_api::helpers::get_padding;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};


#[derive(Debug, Clone)]
pub struct TextEditor {
    pub id: usize,
    pub content: widget::text_editor::Content,
    pub place_holder: Option<String>, 
    pub font_id: Option<usize>,
    pub text_size: Option<f32>,
    pub line_height: Option<f32>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub wrapping_none: Option<bool>,
    pub wrapping_glyph: Option<bool>,
    pub wrapping_word_glyph: Option<bool>,
    pub last_status: TxtEdStatus,
}

impl TextEditor {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {

        // default is word so not checked
        let wrapping = 
            if self.wrapping_none.is_some() {
                Wrapping::None
            } else if self.wrapping_glyph.is_some() {
                Wrapping::Glyph
            } else if self.wrapping_word_glyph.is_some() {
                Wrapping::WordOrGlyph
            } else { Wrapping::Word };

        let hgt = if self.fill.is_none() && 
                        self.height_fill.is_none() && 
                        self.height.is_none() {
            Length::Fill
        } else {
            get_len(self.fill, self.height_fill, self.height)
        };

        let ph = if let Some(ph) = &self.place_holder {
            ph
        } else { "Type something here..." };

        let font_opt = 
            self.lookup(widgets, self.font_id)
                .and_then(Widgets::as_font).cloned();

        let te = widget::text_editor(&self.content)
                .placeholder(ph)
                .height(hgt)
                .min_height(self.min_height.unwrap_or_default())
                .max_height(self.max_height.unwrap_or(f32::INFINITY))
                .on_action(TxtEdMessage::ActionPerformed)
                .wrapping(wrapping);

        let te = if let Some(ft) = font_opt {
            te.font(ft.to_iced())
        } else { te };

        let te = if let Some(lh) = self.line_height {
            te.line_height(lh)
        } else { te };

        let te = if let Some(ts) = self.text_size {
            te.size(ts)
        } else { te };

        let te = if let Some(wd) = self.width {
            te.width(wd)
        } else { te };

        let te = if self.padding.is_some() {
            te.padding(get_padding(&self.padding))
        } else {
            te.padding(Padding::new(5.0))
        };

        let te: Element<'_, TxtEdMessage> = te.into();
        Some(te.map(move |message| Message::TextEditor(self.id, message)))
    }

}

#[derive(Debug, Clone)]
pub enum TxtEdStatus {
    Active,
    Hovered,
    Focused { /* … */ },
    Disabled,
}

#[derive(Debug, Clone)]
pub enum TxtEdMessage {
    ActionPerformed(widget::text_editor::Action),
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
                .expect("text_ed_callback: widget is not an TextEditor");
            ed.content.perform(action);
        },
        TxtEdMessage::ThemeSelected(_theme) => todo!(),
        TxtEdMessage::WordWrapToggled(_) => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextEditorParam {
    Fill,
    FontId,
    Height,
    HeightFill,
    LineHeight,
    MaxHeight,
    MinHeight,
    Padding,
    PlaceHolder, 
    TextSize,
    Width,
    WidthFill,
    WrappingGlyph,
    WrappingNone,
    WrappingWordGlyph,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for TextEditor {
    type Param = TextEditorParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextEditorParam::Fill => set_t_value(&mut self.fill, value, "TextEditorParam::Fill"),
            TextEditorParam::FontId => set_t_value(&mut self.font_id, value, "TextEditorParam::FontId"),
            TextEditorParam::Height => set_t_value(&mut self.height, value, "TextEditorParam::Height"),
            TextEditorParam::HeightFill => set_t_value(&mut self.height_fill, value, "TextEditorParam::HeightFill"),
            TextEditorParam::LineHeight => set_t_value(&mut self.line_height, value, "TextEditorParam::LineHeight"),
            TextEditorParam::MaxHeight => set_t_value(&mut self.max_height, value, "TextEditorParam::MaxHeightname"),
            TextEditorParam::MinHeight => set_t_value(&mut self.min_height, value, "TextEditorParam::MinHeight"),
            TextEditorParam::Padding => set_t_value(&mut self.padding, value, "TextEditorParam::Padding"),
            TextEditorParam::PlaceHolder => set_t_value(&mut self.place_holder, value, "TextEditorParam::PlaceHolder"),
            TextEditorParam::TextSize => set_t_value(&mut self.text_size, value, "TextEditorParam::TextSize"),
            TextEditorParam::Width => set_t_value(&mut self.width, value, "TextEditorParam::Width"),
            TextEditorParam::WidthFill => set_t_value(&mut self.width_fill, value, "TextEditorParam::WidthFill"),
            TextEditorParam::WrappingGlyph => set_t_value(&mut self.wrapping_glyph, value, "TextEditorParam::WrappingGlyph"),
            TextEditorParam::WrappingNone => set_t_value(&mut self.wrapping_none, value, "TextEditorParam::WrappingNone"),
            TextEditorParam::WrappingWordGlyph => set_t_value(&mut self.wrapping_word_glyph, value, "TextEditorParam::WrappingWordGlyph"),
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
