//! Color Picker
use iced::Widget as _;
use iced::advanced::Overlay as IcedOverlay;
use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay::{self as overlay};
use iced::advanced::renderer;
use iced::advanced::text;
use iced::advanced::widget::{self as widget, Tree};
use iced::advanced::{Shell, Widget};
use iced::mouse;
use iced::widget::container;
use iced::{Element, Event, Length, Padding, Pixels, Point, Rectangle, Size, Vector};

pub struct DatePicker<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    pub button: Element<'a, Message, Theme, Renderer>,
    pub content: Element<'a, Message, Theme, Renderer>,
    pub selected_date: String,
    pub position: Position,
    pub gap: f32,
    pub padding: f32,
    pub snap_within_viewport: bool,
    pub opened: bool,
    pub on_open: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> DatePicker<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    /// The default padding of a [`DatePicker`] drawn by this renderer.
    const DEFAULT_PADDING: f32 = 5.0;

    /// Creates a new [`DatePicker`].
    ///
    /// [`DatePicker`]: struct.DatePicker.html
    pub fn new(
        button: impl Into<Element<'a, Message, Theme, Renderer>>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        selected_date: String,
        position: Position,
    ) -> Self {
        DatePicker {
            button: button.into(),
            content: content.into(),
            selected_date,
            position,
            gap: 0.0,
            padding: Self::DEFAULT_PADDING,
            snap_within_viewport: true,
            opened: false,
            on_open: None,
            class: Theme::default(),
        }
    }

    /// Sets the gap between the button and its [`DatePicker`].
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().0;
        self
    }

    /// Sets the padding of the [`DatePicker`].
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().0;
        self
    }

    /// Sets whether the [`DatePicker`] is snapped within the viewport.
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    /// Sets whether the [`DatePicker`] overlay is open.
    pub fn opened(mut self, opened: bool) -> Self {
        self.opened = opened;
        self
    }

    /// Sets the callback fired when the button is clicked.
    /// Receives `true` when opening, `false` when closing.
    pub fn on_open(mut self, on_open: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open = Some(Box::new(on_open));
        self
    }

    /// Sets the style of the [`DatePicker`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> container::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`DatePicker`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> widget::Meta for DatePicker<'_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for DatePicker<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    fn diff(&mut self, tree: &mut widget::Tree) {
        if tree.children.len() != 2 {
            tree.children = vec![widget::Tree::new(&self.button), widget::Tree::new(&self.content)];
        }
        tree.children[0].diff(&mut self.button);
        tree.children[1].diff(&mut self.content);
    }

    fn size(&self) -> Size<Length> {
        self.button.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.button.layout(&mut tree.children[0], renderer, limits);
        tree.size = tree.children[0].size;
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event
            && cursor.is_over(layout.bounds())
        {
            if let Some(on_open) = &self.on_open {
                shell.publish((on_open)(!self.opened));
            }
        }

        self.button
            .update(&mut tree.children[0], event, layout, cursor, renderer, shell, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.button
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.button.draw(
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
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let mut children = tree.children.iter_mut();
        let button_tree = children.next().unwrap();
        let content_tree = children.next().unwrap();

        let mut overlays = self
            .button
            .overlay(button_tree, layout, renderer, viewport, translation, window);

        if self.opened {
            self.content.layout(
                content_tree,
                renderer,
                &layout::Limits::new(
                    Size::ZERO,
                    if self.snap_within_viewport {
                        window
                    } else {
                        Size::INFINITE
                    },
                )
                .shrink(Padding::new(self.padding)),
            );
            let content_size = content_tree.size;

            let position = layout.position() + translation;
            let button_bounds = layout.bounds();
            let cursor_position = button_bounds.center();
            let x_center = position.x + (button_bounds.width - content_size.width) / 2.0;
            let y_center = position.y + (button_bounds.height - content_size.height) / 2.0;

            let mut container_bounds = {
                let offset = match self.position {
                    Position::Top => Vector::new(x_center, position.y - content_size.height - self.gap - self.padding),
                    Position::Bottom => {
                        Vector::new(x_center, position.y + button_bounds.height + self.gap + self.padding)
                    }
                    Position::Left => Vector::new(position.x - content_size.width - self.gap - self.padding, y_center),
                    Position::Right => {
                        Vector::new(position.x + button_bounds.width + self.gap + self.padding, y_center)
                    }
                    Position::FollowCursor => {
                        let t = position - button_bounds.position();
                        Vector::new(cursor_position.x, cursor_position.y - content_size.height) + t
                    }
                    Position::Center => Vector::new(x_center, y_center),
                };
                Rectangle {
                    x: offset.x - self.padding,
                    y: offset.y - self.padding,
                    width: content_size.width + self.padding * 2.0,
                    height: content_size.height + self.padding * 2.0,
                }
            };

            let viewport_rect = Rectangle::with_size(window);
            if self.snap_within_viewport {
                if container_bounds.x < viewport_rect.x {
                    container_bounds.x = viewport_rect.x;
                } else if viewport_rect.x + viewport_rect.width < container_bounds.x + container_bounds.width {
                    container_bounds.x = viewport_rect.x + viewport_rect.width - container_bounds.width;
                }
                if container_bounds.y < viewport_rect.y {
                    container_bounds.y = viewport_rect.y;
                } else if viewport_rect.y + viewport_rect.height < container_bounds.y + container_bounds.height {
                    container_bounds.y = viewport_rect.y + viewport_rect.height - container_bounds.height;
                }
            }

            let content_layout = Layout::new(content_size).move_to(Point {
                x: container_bounds.x + self.padding,
                y: container_bounds.y + self.padding,
            });
            let container_layout = Layout::new(container_bounds.size()).move_to(container_bounds.position());

            overlays.push(overlay::Element::new(Box::new(Overlay {
                container_layout,
                content_layout,
                content: &mut self.content,
                tree: content_tree,
                class: &self.class,
                window,
            })));
        }

        overlays
    }

    fn operate(
        &mut self,
        tree: &mut widget::Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds(), viewport);
        operation.traverse(&mut |operation| {
            self.button
                .operate(&mut tree.children[0], layout, viewport, renderer, operation);
        });
    }
}

impl<'a, Message, Theme, Renderer> From<DatePicker<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(content: DatePicker<'a, Message, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        content.boxed()
    }
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
    /// The content will follow the cursor.
    FollowCursor,
    /// The content will be centered over the button.
    Center,
}

struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    container_layout: Layout,
    content_layout: Layout,
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut widget::Tree,
    class: &'b Theme::Class<'a>,
    window: Size,
}

impl<Message, Theme, Renderer> IcedOverlay<Message, Theme, Renderer> for Overlay<'_, '_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        cursor_position: mouse::Cursor,
    ) {
        let style = theme.style(self.class);
        let viewport = Rectangle::with_size(self.window);

        renderer.with_layer(viewport, |renderer| {
            container::draw_background(renderer, &style, self.container_layout.bounds());

            let defaults = renderer::Style {
                text_color: style.text_color.unwrap_or(inherited_style.text_color),
            };

            self.content.draw(
                self.tree,
                renderer,
                theme,
                &defaults,
                self.content_layout,
                cursor_position,
                &viewport,
            );
        });
    }

    fn update(&mut self, event: &Event, cursor: mouse::Cursor, renderer: &Renderer, shell: &mut Shell<'_, Message>) {
        let is_mouse_press = matches!(event, Event::Mouse(mouse::Event::ButtonPressed(_)));

        self.content.update(
            self.tree,
            event,
            self.content_layout,
            cursor,
            renderer,
            shell,
            &Rectangle::with_size(self.window),
        );

        if is_mouse_press && cursor.is_over(self.container_layout.bounds()) {
            shell.capture_event();
        }
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        self.content.mouse_interaction(
            self.tree,
            self.content_layout,
            cursor,
            &Rectangle::with_size(self.window),
            renderer,
        )
    }

    fn overlay<'c>(&'c mut self, renderer: &Renderer) -> Vec<overlay::Element<'c, Message, Theme, Renderer>> {
        self.content.overlay(
            self.tree,
            self.content_layout,
            renderer,
            &Rectangle::with_size(self.window),
            Vector::ZERO,
            self.window,
        )
    }
}
