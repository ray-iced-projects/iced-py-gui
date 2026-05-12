use iced::widget::container;
use iced::advanced::text;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self as widget};
use iced::advanced::overlay::{self as overlay};
use iced::advanced::Overlay as IcedOverlay;
use iced::mouse;
use iced::advanced::renderer;
use iced::advanced::{Shell, Widget};
use iced::{Element, Event, Length, Padding, Pixels, Point, Rectangle, Size, Vector};
use iced::keyboard;
use iced::keyboard::key;


pub struct Menu<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    pub widget: Element<'a, Message, Theme, Renderer>,
    pub content: Element<'a, Message, Theme, Renderer>,
    pub id: Option<widget::Id>,
    pub position: Position,
    pub gap: f32,
    pub padding: f32,
    pub snap_within_viewport: bool,
    pub opened: bool,
    pub on_open: Option<Box<dyn Fn() -> Message + 'a>>,
    pub on_close: Option<Box<dyn Fn() -> Message + 'a>>,
    pub on_click_outside: Option<Box<dyn Fn(Option<widget::Id>) -> Message + 'a>>,
    pub focus_trap: bool,
    pub class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Menu<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    /// The default padding of a [`Menu`] drawn by this renderer.
    const DEFAULT_PADDING: f32 = 5.0;

    /// Creates a new [`Menu`].
    pub fn new(
        widget: impl Into<Element<'a, Message, Theme, Renderer>>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        opened: bool,
    ) -> Self {
        Menu {
            widget: widget.into(),
            content: content.into(),
            id: None,
            opened,
            position: Position::Top,
            gap: 0.0,
            padding: Self::DEFAULT_PADDING,
            snap_within_viewport: true,
            on_open: None,
            on_close: None,
            on_click_outside: None,
            focus_trap: false,
            class: Theme::default(),
        }
    }

    /// Sets the [`widget::Id`] of the [`Menu`].
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the gap between the widget and its [`Menu`].
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().0;
        self
    }

    /// Sets the padding of the [`Menu`].
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().0;
        self
    }

    /// Sets the position of the [`Menu`].
    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    /// Sets whether the [`Menu`] is snapped within the viewport.
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    /// Sets the message fired when a click occurs outside both the Menu
    /// content and the trigger widget. The `Option<widget::Id>` is the id
    /// set on this [`Menu`], useful to distinguish multiple popups.
    pub fn on_click_outside(mut self, message: impl Fn(Option<widget::Id>) -> Message + 'a) -> Self {
        self.on_click_outside = Some(Box::new(message));
        self
    }

    /// Sets whether the [`Menu`] overlay is open.
    pub fn opened(mut self, opened: bool) -> Self {
        self.opened = opened;
        self
    }

    /// Sets the message fired when the Menu opens.
    pub fn on_open(mut self, on_open: impl Fn() -> Message + 'a) -> Self {
        self.on_open = Some(Box::new(on_open));
        self
    }

    /// Sets the message fired when the Menu closes.
    pub fn on_close(mut self, on_close: impl Fn() -> Message + 'a) -> Self {
        self.on_close = Some(Box::new(on_close));
        self
    }

    /// When `true`, Tab and Shift+Tab are captured while the Menu is open,
    /// keeping keyboard focus within the Menu content.
    pub fn focus_trap(mut self, trap: bool) -> Self {
        self.focus_trap = trap;
        self
    }

    /// Sets the style of the [`Menu`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> container::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Menu`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Menu<'_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    fn children(&self) -> Vec<widget::Tree> {
        vec![
            widget::Tree::new(&self.widget),
            widget::Tree::new(&self.content),
        ]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[self.widget.as_widget(), self.content.as_widget()]);
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<PopupState>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(PopupState::default())
    }

    fn size(&self) -> Size<Length> {
        self.widget.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.widget.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.widget
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        {
            let state = tree.state.downcast_mut::<PopupState>();
            if self.opened != state.previous_opened {
                if self.opened {
                    if let Some(on_open) = &self.on_open {
                        shell.publish(on_open());
                    }
                } else if let Some(on_close) = &self.on_close {
                    shell.publish(on_close());
                }
                state.previous_opened = self.opened;
            }
        }

        self.widget.as_widget_mut().update(
            &mut tree.children[0],
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
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.widget.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.widget.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            inherited_style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let mut children = tree.children.iter_mut();

        let widget_overlay = self.widget.as_widget_mut().overlay(
            children.next().unwrap(),
            layout,
            renderer,
            viewport,
            translation,
        );

        let content = if self.opened {
            Some(overlay::Element::new(Box::new(Overlay {
                position: layout.position() + translation,
                content: &mut self.content,
                tree: children.next().unwrap(),
                widget_bounds: layout.bounds(),
                snap_within_viewport: self.snap_within_viewport,
                positioning: self.position,
                gap: self.gap,
                padding: self.padding,
                class: &self.class,
                on_click_outside: self.on_click_outside.as_deref(),
                id: self.id.clone(),
                focus_trap: self.focus_trap,
            })))
        } else {
            None
        };

        if widget_overlay.is_some() || content.is_some() {
            Some(
                overlay::Group::with_children(widget_overlay.into_iter().chain(content).collect())
                    .overlay(),
            )
        } else {
            None
        }
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.widget.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }
}

impl<'a, Message, Theme, Renderer> From<Menu<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        content: Menu<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(content)
    }
}

#[derive(Debug, Clone, Default)]
struct PopupState {
    previous_opened: bool,
}

/// The position of the content. Defaults to following the cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// The content will appear on the top of the widget.
    #[default]
    Top,
    /// The content will appear on the bottom of the widget.
    Bottom,
    /// The content will appear on the left of the widget.
    Left,
    /// The content will appear on the right of the widget.
    Right,
    /// The content will be centered over the widget.
    Center,
}


struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    position: Point,
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut widget::Tree,
    widget_bounds: Rectangle,
    snap_within_viewport: bool,
    positioning: Position,
    gap: f32,
    padding: f32,
    class: &'b Theme::Class<'a>,
    on_click_outside: Option<&'b (dyn Fn(Option<widget::Id>) -> Message + 'a)>,
    id: Option<widget::Id>,
    focus_trap: bool,
}

impl<Message, Theme, Renderer> IcedOverlay<Message, Theme, Renderer>
    for Overlay<'_, '_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let viewport = Rectangle::with_size(bounds);

        let content_layout = self.content.as_widget_mut().layout(
            self.tree,
            renderer,
            &layout::Limits::new(
                Size::ZERO,
                if self.snap_within_viewport {
                    viewport.size()
                } else {
                    Size::INFINITE
                },
            )
            .shrink(Padding::new(self.padding)),
        );

        let text_bounds = content_layout.bounds();
        let x_center = self.position.x + (self.widget_bounds.width - text_bounds.width) / 2.0;
        let y_center = self.position.y + (self.widget_bounds.height - text_bounds.height) / 2.0;

        let mut content_bounds = {
            let offset = match self.positioning {
                Position::Top => Vector::new(
                    x_center,
                    self.position.y - text_bounds.height - self.gap - self.padding,
                ),
                Position::Bottom => Vector::new(
                    x_center,
                    self.position.y + self.widget_bounds.height + self.gap + self.padding,
                ),
                Position::Left => Vector::new(
                    self.position.x - text_bounds.width - self.gap - self.padding,
                    y_center,
                ),
                Position::Right => Vector::new(
                    self.position.x + self.widget_bounds.width + self.gap + self.padding,
                    y_center,
                ),
                Position::Center => Vector::new(x_center, y_center),
            };

            Rectangle {
                x: offset.x - self.padding,
                y: offset.y - self.padding,
                width: text_bounds.width + self.padding * 2.0,
                height: text_bounds.height + self.padding * 2.0,
            }
        };

        if self.snap_within_viewport {
            if content_bounds.x < viewport.x {
                content_bounds.x = viewport.x;
            } else if viewport.x + viewport.width < content_bounds.x + content_bounds.width {
                content_bounds.x = viewport.x + viewport.width - content_bounds.width;
            }

            if content_bounds.y < viewport.y {
                content_bounds.y = viewport.y;
            } else if viewport.y + viewport.height < content_bounds.y + content_bounds.height {
                content_bounds.y = viewport.y + viewport.height - content_bounds.height;
            }
        }

        layout::Node::with_children(
            content_bounds.size(),
            vec![content_layout.translate(Vector::new(self.padding, self.padding))],
        )
        .translate(Vector::new(content_bounds.x, content_bounds.y))
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: mouse::Cursor,
    ) {
        let style = theme.style(self.class);

        container::draw_background(renderer, &style, layout.bounds());

        let defaults = renderer::Style {
            text_color: style.text_color.unwrap_or(inherited_style.text_color),
        };

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            &defaults,
            layout.children().next().unwrap(),
            cursor_position,
            &Rectangle::with_size(Size::INFINITE),
        );
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        if let Some(on_click_outside) = self.on_click_outside {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    let over_popup = cursor.is_over(layout.bounds());
                    let over_widget = cursor.is_over(self.widget_bounds);
                    if !over_popup && !over_widget {
                        shell.publish(on_click_outside(self.id.clone()));
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    shell.publish(on_click_outside(self.id.clone()));
                    shell.capture_event();
                }
                _ => {}
            }
        }

        self.content.as_widget_mut().update(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            shell,
            &Rectangle::with_size(Size::INFINITE),
        );

        if self.focus_trap {
            if let Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Tab),
                ..
            }) = event {
                shell.capture_event();
            }
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            &Rectangle::with_size(Size::INFINITE),
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            &Rectangle::with_size(Size::INFINITE),
            Vector::ZERO,
        )
    }
}
