//! ipg_opaque

use iced::Element;
use iced::widget::opaque;

use crate::app::Message;


#[derive(Clone, Debug)]
pub struct IpgOpaque {
    pub id: usize,
}

impl IpgOpaque {
    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Element<'a, Message> {
            opaque(content.remove(0))
    }
}
