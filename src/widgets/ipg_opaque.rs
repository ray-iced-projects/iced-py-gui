//! ipg_opaque

use iced::Element;
use iced::widget::opaque;

use crate::app::Message;


#[derive(Clone, Debug)]
pub struct Opaque {
    pub id: usize,
    pub show: bool,
}

impl Opaque {
    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {
            if !self.show { return None }
            Some(opaque(content.remove(0)))
    }
}
