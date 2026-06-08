//! A context menu for showing actions on right click.
//!
use iced::{
    Element, Event, advanced::Layout, Length, Point, Rectangle, advanced::Shell, Vector, advanced::Widget,
    advanced::layout::{Limits, Node},
    mouse::{self, Button, Cursor},
    overlay, advanced::renderer,
    advanced::widget::{Operation, Tree, tree},
};

pub use crate::iced_aw_widgets::menu::style::context_menu::{Catalog, Style};
pub use crate::iced_aw_widgets::menu::style::status::{Status, StyleFn};

use crate::iced_aw_widgets::menu::overlay::context_menu::ContextMenuOverlay;

/// A context menu
///
///
/// # Example
/// ```ignore
/// # use iced_widget::{Text, Button};
/// # use iced_aw::ContextMenu;
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     Action1,
/// }
///
/// let underlay = Text::new("right click me");
///
/// let cm = ContextMenu::new(
///     underlay,
///     || Button::new("action1").on_press(Message::Action1).into()
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct ContextMenu<
    'a,
    Overlay,
    Message,
    Theme = iced::Theme,
    Renderer = iced::Renderer,
> where
    Overlay: Fn() -> Element<'a, Message, Theme, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// The underlying element.
    underlay: Element<'a, Message, Theme, Renderer>,
    /// The content of [`ContextMenuOverlay`].
    overlay: Overlay,
    overlay_instance: Option<Element<'a, Message, Theme, Renderer>>,
    /// The style of the [`ContextMenu`].
    class: Theme::Class<'a>,
    /// Force the menu to be shown (for testing purposes). If None, uses internal state.
    force_open: Option<bool>,
}

impl<'a, Overlay, Message, Theme, Renderer> ContextMenu<'a, Overlay, Message, Theme, Renderer>
where
    Overlay: Fn() -> Element<'a, Message, Theme, Renderer>,
    Message: Clone,
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    /// Creates a new [`ContextMenu`]
    ///
    /// `underlay`: The underlying element.
    ///
    /// `overlay`: The content of [`ContextMenuOverlay`] which will be displayed when `underlay` is clicked.
    pub fn new<U>(underlay: U, overlay: Overlay) -> Self
    where
        U: Into<Element<'a, Message, Theme, Renderer>>,
    {
        ContextMenu {
            underlay: underlay.into(),
            overlay,
            overlay_instance: None,
            class: Theme::default(),
            force_open: None,
        }
    }

    /// Sets the style of the [`ContextMenu`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme, Style>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme, Style>).into();
        self
    }

    /// Sets the class of the input of the [`ContextMenu`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Forces the menu to be open or closed, overriding the internal state.
    /// This is primarily useful for testing purposes.
    /// If `None`, the menu uses its internal state (toggled by right-click).
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.force_open = Some(open);
        self
    }
}

impl<'a, Content, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ContextMenu<'a, Content, Message, Theme, Renderer>
where
    Content: 'a + Fn() -> Element<'a, Message, Theme, Renderer>,
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> iced::Size<Length> {
        self.underlay.as_widget().size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.underlay
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.children[0].diff(&mut self.underlay);
        if let Some(overlay) = self.overlay_instance.as_mut() {
            tree.children[1].diff(overlay);
        }
    }

    fn operate<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let s: &mut State = state.state.downcast_mut();
        let show = self.force_open.unwrap_or(s.show);

        if show {
            let content = self.overlay_instance.get_or_insert_with(&self.overlay);
            state.children[1].diff(&mut *content);

            content
                .as_widget_mut()
                .operate(&mut state.children[1], layout, renderer, operation);
        } else {
            self.overlay_instance = None;
            self.underlay.as_widget_mut().operate(
                &mut state.children[0],
                layout,
                renderer,
                operation,
            );
        }
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if *event == Event::Mouse(mouse::Event::ButtonPressed(Button::Right)) {
            let bounds = layout.bounds();

            if cursor.is_over(bounds) {
                let s: &mut State = state.state.downcast_mut();
                s.cursor_position = cursor.position().unwrap_or_default();
                s.show = !s.show;

                if !s.show {
                    self.overlay_instance = None;
                }

                shell.capture_event();
                shell.request_redraw();
            }
        }

        self.underlay.as_widget_mut().update(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let s: &mut State = tree.state.downcast_mut();
        let show = self.force_open.unwrap_or(s.show);

        if !show {
            self.overlay_instance = None;
            return self.underlay.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        let position = s.cursor_position;
        let content = self.overlay_instance.get_or_insert_with(&self.overlay);
        tree.children[1].diff(&mut *content);
        Some(
            ContextMenuOverlay::new(
                position + translation,
                &mut tree.children[1],
                content,
                &self.class,
                s,
            )
            .overlay(),
        )
    }
}

impl<'a, Content, Message, Theme, Renderer> From<ContextMenu<'a, Content, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Content: 'a + Fn() -> Self,
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
    Theme: 'a + Catalog,
{
    fn from(modal: ContextMenu<'a, Content, Message, Theme, Renderer>) -> Self {
        Element::new(modal)
    }
}

/// The state of the ``context_menu``.
#[derive(Debug, Default)]
pub(crate) struct State {
    /// The visibility of the [`ContextMenu`] overlay.
    pub show: bool,
    /// Use for showing the overlay where the click was made.
    pub cursor_position: Point,
}

impl State {
    /// Creates a new [`State`] containing the given state data.
    pub const fn new() -> Self {
        Self {
            show: false,
            cursor_position: Point::ORIGIN,
        }
    }
}
