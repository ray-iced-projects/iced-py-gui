//! A simple Quad widget using iced's native renderer::Quad.

use iced::{
    advanced::{
        layout::{Limits, Node},
        renderer,
        widget::Tree,
        Layout, Widget,
    },
    border::Radius,
    mouse::Cursor,
    Background, Border, Element, Length, Rectangle, Shadow, Size,
};

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    /// Width of the quad
    pub width: Length,
    /// Height of the quad
    pub height: Length,
    /// Methods for creating inner bounds
    pub inner_bounds: InnerBounds,
    /// Color of the quad
    pub quad_color: Background,
    /// Border of the quad
    pub quad_border: Border,
    /// Shadow of the quad
    pub quad_shadow: Shadow,
    /// To add clipping to the quad
    pub quad_snap: bool,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fill,
            inner_bounds: InnerBounds::Ratio(0.5, 0.5),
            quad_color: iced::Color::from([0.5; 3]).into(),
            quad_border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::default(),
            },
            quad_shadow: Shadow::default(),
            quad_snap: false,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Quad
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let intrinsic = match self.inner_bounds {
            InnerBounds::Square(l) => Size::new(l, l),
            InnerBounds::Ratio(_, _) => Size::ZERO,
        };
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(self.width, self.height, intrinsic);
        Node::new(size)
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: self.inner_bounds.get_bounds(layout.bounds()),
                border: self.quad_border,
                shadow: self.quad_shadow,
                snap: self.quad_snap,
            },
            self.quad_color,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Quad> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer,
    Theme: 'a,
{
    fn from(value: Quad) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InnerBounds {
    /// Inner bounds as a ratio of the outer bounds
    Ratio(f32, f32),
    /// Square inner bounds with the given side length
    Square(f32),
}

impl InnerBounds {
    pub fn get_bounds(&self, outer_bounds: Rectangle) -> Rectangle {
        match self {
            InnerBounds::Ratio(w, h) => {
                let width = w * outer_bounds.width;
                let height = h * outer_bounds.height;
                let x = outer_bounds.x + (outer_bounds.width - width) * 0.5;
                let y = outer_bounds.y + (outer_bounds.height - height) * 0.5;
                Rectangle { x, y, width, height }
            }
            InnerBounds::Square(l) => {
                let width = *l;
                let height = *l;
                let x = outer_bounds.x + (outer_bounds.width - width) * 0.5;
                let y = outer_bounds.y + (outer_bounds.height - height) * 0.5;
                Rectangle { x, y, width, height }
            }
        }
    }
}
