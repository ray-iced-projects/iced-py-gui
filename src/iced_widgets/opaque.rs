//! Wraps the given widget and captures any mouse button presses inside the bounds of
//! the widget—effectively making it _opaque_.
//!
//! This widget is meant to be used to mark elements in a [`Stack`] to avoid mouse
//! events from passing through layers.
//!
//! [`Stack`]: crate::Stack
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::{button, opaque, stack, text};
//!
//! #[derive(Clone)]
//! enum Message {
//!     ButtonPressed,
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     stack![
//!         // This button lies below the opaque top layer: mouse events over
//!         // the top layer do not pass through to it.
//!         button("Click me!").on_press(Message::ButtonPressed),
//!         opaque(text("I am opaque")),
//!     ]
//!     .into()
//! }
//! ```
use iced::Widget as _;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::Operation;
use iced::advanced::widget::tree::{self, Tree};
use iced::mouse;
use iced::overlay;
use iced::{Element, Event, Length, Rectangle, Size, Vector, advanced::Layout, advanced::Shell, advanced::Widget};

/// Wraps the given widget and captures any mouse button presses inside the bounds of
/// the widget—effectively making it _opaque_.
///
/// This widget is meant to be used to mark elements in a [`Stack`] to avoid mouse
/// events from passing through layers.
///
/// [`Stack`]: crate::Stack
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{button, opaque, stack, text};
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     stack![
///         // This button lies below the opaque top layer: mouse events over
///         // the top layer do not pass through to it.
///         button("Click me!").on_press(Message::ButtonPressed),
///         opaque(text("I am opaque")),
///     ]
///     .into()
/// }
/// ```
pub struct Opaque<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Opaque<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    /// Creates a new [`Opaque`].
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Opaque {
            content: content.into(),
        }
    }

    /// Returns the wrapped [`Element`].
    pub fn into_inner(self) -> Element<'a, Message, Theme, Renderer> {
        self.content
    }
}

impl<Message, Theme, Renderer> iced::advanced::widget::Meta for Opaque<'_, Message, Theme, Renderer> where
    Renderer: iced::advanced::Renderer
{
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Opaque<'_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        self.content.tag()
    }

    fn state(&self) -> tree::State {
        self.content.state()
    }

    fn diff(&mut self, tree: &mut Tree) {
        self.content.diff(tree);
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(tree, renderer, limits);
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.operate(tree, layout, viewport, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let is_mouse_press = matches!(event, Event::Mouse(mouse::Event::ButtonPressed(_)));

        self.content
            .update(tree, event, layout, cursor, renderer, shell, viewport);

        if is_mouse_press && cursor.is_over(layout.bounds()) {
            shell.capture_event();
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let interaction = self.content.mouse_interaction(tree, layout, cursor, viewport, renderer);

        if interaction == mouse::Interaction::None && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Idle
        } else {
            interaction
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content
            .overlay(tree, layout, renderer, viewport, translation, window)
    }
}

impl<'a, Message, Theme, Renderer> From<Opaque<'a, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(opaque: Opaque<'a, Message, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        opaque.boxed()
    }
}
