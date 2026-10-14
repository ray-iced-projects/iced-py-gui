
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::mouse;
use iced::{Border, Color, Length, Rectangle, Shadow, Size};


#[derive(Debug, Default)]
pub struct CustomQuad {
    pub width: f32,
    pub height: f32,
    pub fill_color: Color,
    pub border: Border,
    pub shadow: Shadow,
    pub snap: bool,
}

impl CustomQuad
{
    pub fn new(
        width: f32,
        height: f32,
        fill_color: Color,
    ) -> Self {
        Self {
            width,
            height,
            fill_color,
            ..Default::default()
        }
    }

    /// Sets the border of the [`CustomQuad`].
    #[must_use]
    #[allow(dead_code)]
    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Sets the shadow of the [`CustomQuad`].
    #[must_use]
    #[allow(dead_code)]
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }

    /// Sets the snap of the [`CustomQuad`].
    #[must_use]
    #[allow(dead_code)]
    pub fn snap(mut self, snap: bool) -> Self {
        self.snap = snap;
        self
    }

}

impl widget::Meta for CustomQuad {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for CustomQuad
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) {
        tree.size = Size::new(self.width, self.height);
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: self.border,
                shadow: self.shadow,
                snap: self.snap,
            },
            self.fill_color,
        );
    }
}
