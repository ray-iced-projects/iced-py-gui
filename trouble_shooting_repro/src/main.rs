use iced::widget::{self, center, column, combo_box, scrollable, space, text};
use iced::{Center, Element, Fill};

pub fn main() -> iced::Result {
    iced::run(Example::update, Example::view)
}

struct Example {
    languages: combo_box::State<Language>,
    selected_language_1: Option<Language>,
    selected_language_2: Option<Language>,
    text_1: String,
    text_2: String,
    cb1_id: widget::Id,
    cb2_id: widget::Id,
    selected_count: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Selected(widget::Id, Language),
    OptionHovered(widget::Id, Language),
    Closed(widget::Id),
}

impl Example {
    fn new() -> Self {
        Self {
            languages: combo_box::State::new(Language::ALL.to_vec()),
            selected_language_1: None,
            selected_language_2: None,
            text_1: String::new(),
            text_2: String::new(),
            cb1_id: widget::Id::unique(),
            cb2_id: widget::Id::unique(),
            selected_count: 0,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(id, language) => {
                self.selected_count += 1;
                println!("Selected #{} published: id={id:?} language={language}", self.selected_count);
                if id == self.cb1_id {
                    self.selected_language_1 = Some(language);
                    self.text_1 = language.hello().to_string();
                } else {
                    self.selected_language_2 = Some(language);
                    self.text_2 = language.hello().to_string();
                }
            }
            Message::OptionHovered(id, language) => {
                if id == self.cb1_id {
                    self.text_1 = language.hello().to_string();
                } else {
                    self.text_2 = language.hello().to_string();
                }
            }
            Message::Closed(id) => {
                if id == self.cb1_id {
                    self.text_1 = self
                        .selected_language_1
                        .map(|language| language.hello().to_string())
                        .unwrap_or_default();
                } else {
                    self.text_2 = self
                        .selected_language_1
                        .map(|language| language.hello().to_string())
                        .unwrap_or_default();
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let combo_box_1 = combo_box(
            &self.languages,
            "Type a language...",
            self.selected_language_1.as_ref(),
            {
                let id = self.cb1_id.clone();
                move |language| Message::Selected(id.clone(), language)
            },
        )
        .id(self.cb1_id.clone())
        .on_option_hovered({
            let id = self.cb1_id.clone();
            move |language| Message::OptionHovered(id.clone(), language)
        })
        .on_close({
            let id = self.cb1_id.clone();
            Message::Closed(id)
        })
        .width(250);

        let combo_box_2 = combo_box(
            &self.languages,
            "Type a language...",
            self.selected_language_2.as_ref(),
            {
                let id = self.cb2_id.clone();
                move |language| Message::Selected(id.clone(), language)
            },
        )
        .id(self.cb2_id.clone())
        .on_option_hovered({
            let id = self.cb2_id.clone();
            move |language| Message::OptionHovered(id.clone(), language)
        })
        .on_close({
            let id = self.cb2_id.clone();
            Message::Closed(id)
        })
        .width(250);

        let content = column![
            text(&self.text_1),
            "What is your language?",
            combo_box_1,
            text(&self.text_2),
            "What is your language?",
            combo_box_2,
            space().height(150),
        ]
        .width(Fill)
        .align_x(Center)
        .spacing(10);

        center(scrollable(content)).into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Example::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    Danish,
    #[default]
    English,
    French,
    German,
    Italian,
    Japanese,
    Portuguese,
    Spanish,
    Other,
}

impl Language {
    const ALL: [Language; 9] = [
        Language::Danish,
        Language::English,
        Language::French,
        Language::German,
        Language::Italian,
        Language::Japanese,
        Language::Portuguese,
        Language::Spanish,
        Language::Other,
    ];

    fn hello(&self) -> &str {
        match self {
            Language::Danish => "Halloy!",
            Language::English => "Hello!",
            Language::French => "Salut!",
            Language::German => "Hallo!",
            Language::Italian => "Ciao!",
            Language::Japanese => "こんにちは!",
            Language::Portuguese => "Olá!",
            Language::Spanish => "¡Hola!",
            Language::Other => "... hello?",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Danish => "Danish",
                Language::English => "English",
                Language::French => "French",
                Language::German => "German",
                Language::Italian => "Italian",
                Language::Japanese => "日本語",
                Language::Portuguese => "Portuguese",
                Language::Spanish => "Spanish",
                Language::Other => "Some other language",
            }
        )
    }
}
