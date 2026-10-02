//! A popover is a floating piece of content that appears over some element.
//!
//! Unlike a [`tooltip`], a popover is not shown on hover, and it does not
//! control its own visibility. Instead, the `popover` argument is an
//! `Option`: `Some` displays the overlay (open), and `None` hides it (closed).
//! The base is always present.
//!
//! When the user clicks outside of the popover's bounds, the popover notifies
//! the application through its `on_close` handler, which is typically how the
//! application decides to set the `popover` argument to `None` (i.e. to close
//! it).
//!
//! By default, the popover overlay is opaque: mouse button presses inside its
//! bounds are captured, and mouse events do not pass through it to the layers
//! below. Use [`Popover::passthrough`] to make the popover overlay
//! transparent.
//!
//! [`tooltip`]: crate::tooltip::Tooltip
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::{button, container, popover, text};
//!
//! #[derive(Clone)]
//! enum Message {
//!     Close,
//! }
//!
//! struct State {
//!     is_open: bool,
//! }
//!
//! fn view(state: &State) -> Element<'static, Message> {
//!     // The base is always present. The `popover` argument is `Some` when the
//!     // popover is open and `None` when it is closed.
//!     popover(
//!         button(text("Click me!")).on_press(Message::Close),
//!         state.is_open.then(|| {
//!             container(text("This is the popover contents!")).padding(10)
//!         }),
//!     )
//!     .position(popover::Position::Bottom)
//!     .on_close(Message::Close)
//!     .into()
//! }
//! ```
use super::opaque::Opaque;
use iced::advanced::Widget as _;
use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self as adv_widget};
use iced::mouse;
use iced::touch;
use iced::widget::Widget;
use iced::{Element, Event, Length, Pixels, Point, Rectangle, Size, Vector, advanced::Shell};

/// A floating piece of content that appears over another element.
///
/// The popover does not control its own visibility. Instead, the `popover`
/// argument is an `Option`: `Some` displays the overlay (open), and `None`
/// hides it (closed). The base is always present.
///
/// When the user clicks outside of the popover's bounds, the popover notifies
/// the application through its `on_close` handler.
///
/// By default, the popover overlay is opaque: mouse button presses inside its
/// bounds are captured, and mouse events do not pass through it to the layers
/// below. Use [`Popover::passthrough`] to make the popover overlay
/// transparent.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::{button, container, popover, text};
///
/// #[derive(Clone)]
/// enum Message {
///     Close,
/// }
///
/// struct State {
///     is_open: bool,
/// }
///
/// fn view(state: &State) -> Element<'static, Message> {
///     // The base is always present. The `popover` argument is `Some` when the
///     // popover is open and `None` when it is closed.
///     popover(
///         button(text("Click me!")).on_press(Message::Close),
///         state.is_open.then(|| {
///             container("This is the popover contents!").padding(10)
///         }),
///     )
///     .position(popover::Position::Bottom)
///     .on_close(Message::Close)
///     .into()
/// }
/// ```
pub struct Popover<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
    popup: Option<Popup<'a, Message, Theme, Renderer>>,
    position: Position,
    gap: f32,
    snap_within_viewport: bool,
    on_close: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Popover<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    /// Creates a new [`Popover`].
    ///
    /// It expects:
    ///   * the `content` element that the popover is anchored to (the base), and
    ///   * the optional `popover` element to display: `Some` when the popover
    ///     is open and `None` when it is closed.
    ///
    /// The base is always present; it is the `popover` argument that controls
    /// whether the overlay is displayed. By default, the [`Popover`] is
    /// positioned [`Position::Auto`]; use the [`Self::position`] method to set
    /// a specific position.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        popover: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
    ) -> Self {
        Popover {
            content: content.into(),
            popup: popover.map(|popup| Popup::Opaque(Opaque::new(popup))),
            position: Position::default(),
            gap: 0.0,
            snap_within_viewport: true,
            on_close: None,
        }
    }

    /// Sets the message that will be produced when the user clicks outside of
    /// the [`Popover`]'s bounds.
    ///
    /// This is typically how the application decides to close the popover, by
    /// setting the `popover` argument to `None`.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Sets the [`Position`] of the [`Popover`].
    ///
    /// By default, the [`Popover`] is positioned [`Position::Auto`], which
    /// places it on the side of the base with the most available space.
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Sets the gap between the content and its [`Popover`].
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().0;
        self
    }

    /// Sets whether the [`Popover`] is snapped within the viewport.
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    /// Sets whether mouse events pass through the popover overlay.
    ///
    /// By default, the popover overlay is opaque: mouse button presses inside
    /// its bounds are captured, and mouse events do not pass through it to the
    /// layers below.
    pub fn passthrough(mut self, passthrough: bool) -> Self {
        self.popup = self.popup.map(|popover| match popover {
            Popup::Opaque(opaque) if passthrough => Popup::Transparent(opaque.into_inner()),
            Popup::Transparent(element) if !passthrough => Popup::Opaque(Opaque::new(element)),
            content => content,
        });

        self
    }
}

impl<Message, Theme, Renderer> adv_widget::Meta for Popover<'_, Message, Theme, Renderer> where
    Renderer: iced::advanced::Renderer
{
}

impl<Message, Theme, Renderer> iced::advanced::Widget<Message, Theme, Renderer>
    for Popover<'_, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer,
{
    fn diff(&mut self, tree: &mut adv_widget::Tree) {
        if let Some(popup) = self.popup.as_mut() {
            if tree.children.len() != 2 {
                let mut content_tree = adv_widget::Tree::new(&self.content);
                self.content.diff(&mut content_tree);
                let mut popup_tree = adv_widget::Tree::new(popup);
                popup.diff(&mut popup_tree);
                tree.children = vec![content_tree, popup_tree];
            } else {
                tree.children[0].diff(&mut self.content);
                tree.children[1].diff(popup);
            }
        } else if tree.children.len() != 1 {
            let mut content_tree = adv_widget::Tree::new(&self.content);
            self.content.diff(&mut content_tree);
            tree.children = vec![content_tree];
        } else {
            tree.children[0].diff(&mut self.content);
        }
    }

    fn size(&self) -> Size<Length> {
        self.content.size()
    }

    fn layout(&mut self, tree: &mut adv_widget::Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.content.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }

    fn update(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content
            .update(&mut tree.children[0], event, layout, cursor, renderer, shell, viewport);
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.draw(
            &tree.children[0],
            renderer,
            theme,
            inherited_style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &iced::advanced::widget::Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            self.content
                .operate(&mut tree.children[0], layout, viewport, renderer, operation);
        });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut iced::advanced::widget::Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let Some(popup) = self.popup.as_mut() else {
            return Vec::new();
        };

        let (base, rest) = tree.children.split_at_mut(1);
        let Some(popup_tree) = rest.first_mut() else {
            return Vec::new();
        };

        // Pre-compute the popup layout
        popup.layout(
            popup_tree,
            renderer,
            &layout::Limits::new(
                Size::ZERO,
                if self.snap_within_viewport {
                    window
                } else {
                    Size::INFINITE
                },
            ),
        );
        let popup_size = popup_tree.size;
        let content_bounds = layout.bounds() + translation;

        let popup_position = {
            let vp = Rectangle::with_size(window);
            let x_center = content_bounds.x + (content_bounds.width - popup_size.width) / 2.0;
            let y_center = content_bounds.y + (content_bounds.height - popup_size.height) / 2.0;
            let (x, y) = match self.position {
                Position::Auto => {
                    let space_below = (vp.y + vp.height) - (content_bounds.y + content_bounds.height);
                    let space_above = content_bounds.y - vp.y;
                    let space_right = (vp.x + vp.width) - (content_bounds.x + content_bounds.width);
                    let space_left = content_bounds.x - vp.x;
                    if space_below >= space_above && space_below >= space_right && space_below >= space_left {
                        (x_center, content_bounds.y + content_bounds.height + self.gap)
                    } else if space_above >= space_right && space_above >= space_left {
                        (x_center, content_bounds.y - popup_size.height - self.gap)
                    } else if space_right >= space_left {
                        (content_bounds.x + content_bounds.width + self.gap, y_center)
                    } else {
                        (content_bounds.x - popup_size.width - self.gap, y_center)
                    }
                }
                Position::Top => (x_center, content_bounds.y - popup_size.height - self.gap),
                Position::Bottom => (x_center, content_bounds.y + content_bounds.height + self.gap),
                Position::Left => (content_bounds.x - popup_size.width - self.gap, y_center),
                Position::Right => (content_bounds.x + content_bounds.width + self.gap, y_center),
            };
            let mut pos = Point::new(x, y);
            if self.snap_within_viewport {
                let vp = Rectangle::with_size(window);
                pos.x = pos.x.max(vp.x).min(vp.x + vp.width - popup_size.width);
                pos.y = pos.y.max(vp.y).min(vp.y + vp.height - popup_size.height);
            }
            pos
        };

        let popup_layout = Layout::new(popup_size).move_to(popup_position);

        let mut overlays = self
            .content
            .overlay(&mut base[0], layout, renderer, viewport, translation, window);

        overlays.push(overlay::Element::new(Box::new(Overlay {
            popup,
            tree: popup_tree,
            content_bounds,
            popup_layout,
            on_close: self.on_close.clone(),
            viewport: *viewport,
            window,
        })));

        overlays
    }
}

impl<'a, Message, Theme, Renderer> From<Popover<'a, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(popover: Popover<'a, Message, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        popover.boxed()
    }
}

enum Popup<'a, Message, Theme, Renderer> {
    Opaque(Opaque<'a, Message, Theme, Renderer>),
    Transparent(Element<'a, Message, Theme, Renderer>),
}

/// The position of the popover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// The popover will appear on the side of the widget with the most
    /// available space.
    #[default]
    Auto,
    /// The popover will appear on the top of the widget.
    Top,
    /// The popover will appear on the bottom of the widget.
    Bottom,
    /// The popover will appear on the left of the widget.
    Left,
    /// The popover will appear on the right of the widget.
    Right,
}

impl<'a, Message, Theme, Renderer> adv_widget::Meta for Popup<'a, Message, Theme, Renderer> where
    Renderer: iced::advanced::Renderer
{
}

impl<'a, Message, Theme, Renderer> iced::advanced::Widget<Message, Theme, Renderer>
    for Popup<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> adv_widget::tree::Tag {
        match self {
            Popup::Opaque(o) => o.tag(),
            Popup::Transparent(e) => e.tag(),
        }
    }

    fn state(&self) -> adv_widget::tree::State {
        match self {
            Popup::Opaque(o) => o.state(),
            Popup::Transparent(e) => e.state(),
        }
    }

    fn size(&self) -> Size<Length> {
        match self {
            Popup::Opaque(o) => o.size(),
            Popup::Transparent(e) => e.size(),
        }
    }
    fn diff(&mut self, tree: &mut adv_widget::Tree) {
        match self {
            Popup::Opaque(o) => o.diff(tree),
            Popup::Transparent(e) => e.diff(tree),
        }
    }
    fn layout(&mut self, tree: &mut adv_widget::Tree, renderer: &Renderer, limits: &layout::Limits) {
        match self {
            Popup::Opaque(o) => o.layout(tree, renderer, limits),
            Popup::Transparent(e) => e.layout(tree, renderer, limits),
        }
    }
    fn update(
        &mut self,
        tree: &mut adv_widget::Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        match self {
            Popup::Opaque(o) => o.update(tree, event, layout, cursor, renderer, shell, viewport),
            Popup::Transparent(e) => e.update(tree, event, layout, cursor, renderer, shell, viewport),
        }
    }
    fn draw(
        &self,
        tree: &adv_widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        match self {
            Popup::Opaque(o) => o.draw(tree, renderer, theme, style, layout, cursor, viewport),
            Popup::Transparent(e) => e.draw(tree, renderer, theme, style, layout, cursor, viewport),
        }
    }
    fn mouse_interaction(
        &self,
        tree: &adv_widget::Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        match self {
            Popup::Opaque(o) => o.mouse_interaction(tree, layout, cursor, viewport, renderer),
            Popup::Transparent(e) => e.mouse_interaction(tree, layout, cursor, viewport, renderer),
        }
    }
    fn operate(
        &mut self,
        tree: &mut adv_widget::Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn adv_widget::Operation,
    ) {
        match self {
            Popup::Opaque(o) => o.operate(tree, layout, viewport, renderer, operation),
            Popup::Transparent(e) => e.operate(tree, layout, viewport, renderer, operation),
        }
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut adv_widget::Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        match self {
            Popup::Opaque(o) => o.overlay(tree, layout, renderer, viewport, translation, window),
            Popup::Transparent(e) => e.overlay(tree, layout, renderer, viewport, translation, window),
        }
    }
}

struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    popup: &'b mut Popup<'a, Message, Theme, Renderer>,
    tree: &'b mut adv_widget::Tree,
    content_bounds: Rectangle,
    popup_layout: Layout,
    on_close: Option<Message>,
    viewport: Rectangle,
    window: Size,
}

impl<Message, Theme, Renderer> iced::advanced::overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'_, '_, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer,
{
    fn update(&mut self, event: &Event, cursor: mouse::Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>) {
        let cursor_position = cursor.position();
        let is_inside = cursor_position.is_some_and(|p| self.popup_layout.bounds().contains(p));
        let is_over_base = cursor_position.is_some_and(|p| self.content_bounds.contains(p));

        if matches!(
            event,
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. })
        ) && !is_inside
        {
            if is_over_base {
                return;
            }
            if let Some(on_close) = self.on_close.take() {
                shell.publish(on_close);
            }
            return;
        }

        self.popup.update(
            self.tree,
            event,
            self.popup_layout,
            cursor,
            renderer,
            shell,
            &self.viewport,
        );
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        cursor_position: mouse::Cursor,
    ) {
        self.popup.draw(
            self.tree,
            renderer,
            theme,
            inherited_style,
            self.popup_layout,
            cursor_position,
            &Rectangle::with_size(self.window),
        );
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        if !cursor.is_over(self.popup_layout.bounds()) {
            return mouse::Interaction::None;
        }
        self.popup.mouse_interaction(
            self.tree,
            self.popup_layout,
            cursor,
            &Rectangle::with_size(self.window),
            renderer,
        )
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn adv_widget::Operation) {
        let viewport = self.popup_layout.bounds();
        self.popup
            .operate(self.tree, self.popup_layout, &viewport, renderer, operation);
    }

    fn overlay<'a>(&'a mut self, renderer: &Renderer) -> Vec<overlay::Element<'a, Message, Theme, Renderer>> {
        self.popup.overlay(
            self.tree,
            self.popup_layout,
            renderer,
            &self.viewport,
            Vector::ZERO,
            self.window,
        )
    }

    fn index(&self) -> f32 {
        2.0
    }
}
