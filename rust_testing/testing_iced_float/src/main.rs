
use iced::widget::{column, container, float, stack, text};
use iced::{Element, Fill, Length, Theme, Vector};

pub fn main() -> iced::Result {

    iced::application(Gradient::default, Gradient::update, Gradient::view)
        .theme(Theme::Dark)
        .run()
}

#[derive(Debug, Default, Clone, Copy)]
struct Gradient {
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Gradient {
    
    fn update(&mut self, message: Message) {
        match message {
            
        }
    }

    fn view(&self) -> Element<'_, Message> {

        let mut content: Vec<Element<Message>> = vec![];
        for _ in 0..10 {
            content.push(text("Float Test").into());
        }
        let col: Element<Message> = container(column(content))
            .into();

        let flt = float(
            container(text("I'm a float"))
                .width(200.0)
                .height(200.0)
                .center(Length::Fixed(200.0))
                .style(move | theme| { container::rounded_box(theme)})
            )
            .scale(1.0)
            .translate(|_bounds, _viewport| Vector::new(160.0, 40.0));

        stack![col, flt].width(Length::Fill).height(Length::Fill).into()
        
    }

}


