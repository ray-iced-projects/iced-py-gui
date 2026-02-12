//!ipg_modal
use iced::advanced::widget::Text;
use iced::{Padding, Renderer, Theme};
use iced::widget::{Button, Column, center, container, mouse_area, opaque, stack};
use iced::{Color, Element, Length};

use ipg_alignment::{IpgAlignment, get_alignment};
use ipg_types::{Message, ModalMessage};



#[derive(Debug)]
pub struct IpgModal {
    pub id: usize,
    pub label: String,
    pub show: bool,
    pub spacing: f32,
    pub padding: Padding,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub align_items: IpgAlignment,
    pub clip: bool,
}

impl IpgModal {
    pub fn new(
        id: usize,
        label: String,
        show: bool,
        spacing: f32,
        padding: Padding,
        width: Length,
        height: Length,
        max_width: f32,
        align_items: IpgAlignment,
        clip: bool,
    ) -> Self {
        Self {
            id,
            label,
            show,
            spacing,
            padding,
            width,
            height,
            max_width,
            align_items,
            clip,
        }
    }
}

pub fn construct_modal<'a>(mdl: &'a IpgModal, 
                            content: Vec<Element<'a, Message>> ) 
                            -> Element<'a, Message, Theme, Renderer> {

    let label = Text::new(mdl.label.clone());            
    let button: Element<ModalMessage> = Button::new(label)
                                        .on_press(ModalMessage::OnOpen)
                                        .into();


    let btn: Element<Message, Theme, Renderer> = button.map(move |message| 
                                                    Message::Modal(mdl.id, message));
                                                                             
    if mdl.show {
        let align_items = get_alignment(mdl.align_items.clone());

        let col: Element<Message, Theme, Renderer> = Column::with_children(content)
                                            .align_x(align_items)
                                            .width(mdl.width)
                                            .height(mdl.height)
                                            .padding(mdl.padding)
                                            .spacing(mdl.spacing)
                                            .clip(mdl.clip)
                                            .into();
        
        let ml: Element<'a, Message, Theme, Renderer> = 
            opaque(
                mouse_area(center(opaque(col)).style(|_theme| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.8,
                                ..Color::BLACK
                            }
                            .into(),
                        ),
                        ..container::Style::default()
                    }
                }))
            );
        ml
    } else {
        btn
    }            

}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
