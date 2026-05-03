
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

use pyo3::pyclass;


/// Represents a color in one of the supported output formats.
#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, hash, frozen)]
pub enum ColorValue {
    /// Normalized float components [r, g, b, a] in 0.0..=1.0
    Float([f32; 4]),
    /// 8-bit integer components [r, g, b, a] in 0..=255
    Integer([u8; 4]),
    /// Hex string e.g. "#RRGGBBAA"
    Hex(String),
    /// Percentage components [r, g, b, a] in 0.0..=100.0
    Percent([f32; 4]),
}

impl std::hash::Hash for ColorValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ColorValue::Float(c) => {
                0u8.hash(state);
                for f in c { f.to_bits().hash(state); }
            }
            ColorValue::Integer(c) => { 1u8.hash(state); c.hash(state); }
            ColorValue::Hex(s)     => { 2u8.hash(state); s.hash(state); }
            ColorValue::Percent(c) => {
                3u8.hash(state);
                for f in c { f.to_bits().hash(state); }
            }
        }
    }
}

impl ColorValue {
    /// Convert to normalized [r, g, b, a] (0.0..=1.0).
    pub fn to_normalized(&self) -> [f32; 4] {
        match self {
            ColorValue::Float(c) => *c,
            ColorValue::Integer([r, g, b, a]) => [
                *r as f32 / 255.0,
                *g as f32 / 255.0,
                *b as f32 / 255.0,
                *a as f32 / 255.0,
            ],
            ColorValue::Hex(s) => parse_hex_color(s),
            ColorValue::Percent([r, g, b, a]) => {
                [r / 100.0, g / 100.0, b / 100.0, a / 100.0]
            }
        }
    }
}

fn parse_hex_color(s: &str) -> [f32; 4] {
    let s = s.trim_start_matches('#');
    let byte = |i: usize| u8::from_str_radix(&s[i..i + 2], 16).unwrap_or(0) as f32 / 255.0;
    match s.len() {
        6 => [byte(0), byte(2), byte(4), 1.0],
        8 => [byte(0), byte(2), byte(4), byte(6)],
        _ => [0.0, 0.0, 0.0, 1.0],
    }
}

impl From<[f32; 4]> for ColorValue {
    fn from(c: [f32; 4]) -> Self { ColorValue::Float(c) }
}

impl From<[u8; 4]> for ColorValue {
    fn from(c: [u8; 4]) -> Self { ColorValue::Integer(c) }
}

impl From<String> for ColorValue {
    fn from(s: String) -> Self { ColorValue::Hex(s) }
}

impl From<&str> for ColorValue {
    fn from(s: &str) -> Self { ColorValue::Hex(s.to_owned()) }
}

pub struct ColorPicker<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    pub button: Element<'a, Message, Theme, Renderer>,
    pub content: Element<'a, Message, Theme, Renderer>,
    pub selected_color: ColorValue,
    pub position: Position,
    pub gap: f32,
    pub padding: f32,
    pub snap_within_viewport: bool,
    pub opened: bool,
    pub on_open: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> ColorPicker<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    /// The default padding of a [`ColorPicker`] drawn by this renderer.
    const DEFAULT_PADDING: f32 = 5.0;

    /// Creates a new [`ColorPicker`].
    ///
    /// [`ColorPicker`]: struct.ColorPicker.html
    pub fn new(
        button: impl Into<Element<'a, Message, Theme, Renderer>>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        selected_color: impl Into<ColorValue>,
        position: Position,
    ) -> Self {
        ColorPicker {
            button: button.into(),
            content: content.into(),
            selected_color: selected_color.into(),
            position,
            gap: 0.0,
            padding: Self::DEFAULT_PADDING,
            snap_within_viewport: true,
            opened: false,
            on_open: None,
            class: Theme::default(),
        }
    }

    /// Sets the gap between the button and its [`ColorPicker`].
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().0;
        self
    }

    /// Sets the padding of the [`ColorPicker`].
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().0;
        self
    }

    /// Sets whether the [`ColorPicker`] is snapped within the viewport.
    pub fn snap_within_viewport(mut self, snap: bool) -> Self {
        self.snap_within_viewport = snap;
        self
    }

    /// Sets whether the [`ColorPicker`] overlay is open.
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

    /// Sets the style of the [`ColorPicker`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> container::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`ColorPicker`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ColorPicker<'_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    fn children(&self) -> Vec<widget::Tree> {
        vec![
            widget::Tree::new(&self.button),
            widget::Tree::new(&self.content),
        ]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[self.button.as_widget(), self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.button.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.button.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.button
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
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if cursor.is_over(layout.bounds()) {
                if let Some(on_open) = &self.on_open {
                    shell.publish((on_open)(!self.opened));
                }
            }
        }

        self.button.as_widget_mut().update(
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
        self.button.as_widget().mouse_interaction(
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
        self.button.as_widget().draw(
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

        let button = self.button.as_widget_mut().overlay(
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
                cursor_position: layout.bounds().center(),
                button_bounds: layout.bounds(),
                snap_within_viewport: self.snap_within_viewport,
                positioning: self.position,
                gap: self.gap,
                padding: self.padding,
                class: &self.class,
            })))
        } else {
            None
        };

        if button.is_some() || content.is_some() {
            Some(
                overlay::Group::with_children(button.into_iter().chain(content).collect())
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
            self.button.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        });
    }
}

impl<'a, Message, Theme, Renderer> From<ColorPicker<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        content: ColorPicker<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(content)
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
}


struct Overlay<'a, 'b, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    position: Point,
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut widget::Tree,
    cursor_position: Point,
    button_bounds: Rectangle,
    snap_within_viewport: bool,
    positioning: Position,
    gap: f32,
    padding: f32,
    class: &'b Theme::Class<'a>,
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
        let x_center = self.position.x + (self.button_bounds.width - text_bounds.width) / 2.0;
        let y_center = self.position.y + (self.button_bounds.height - text_bounds.height) / 2.0;

        let mut content_bounds = {
            let offset = match self.positioning {
                Position::Top => Vector::new(
                    x_center,
                    self.position.y - text_bounds.height - self.gap - self.padding,
                ),
                Position::Bottom => Vector::new(
                    x_center,
                    self.position.y + self.button_bounds.height + self.gap + self.padding,
                ),
                Position::Left => Vector::new(
                    self.position.x - text_bounds.width - self.gap - self.padding,
                    y_center,
                ),
                Position::Right => Vector::new(
                    self.position.x + self.button_bounds.width + self.gap + self.padding,
                    y_center,
                ),
                Position::FollowCursor => {
                    let translation = self.position - self.button_bounds.position();

                    Vector::new(
                        self.cursor_position.x,
                        self.cursor_position.y - text_bounds.height,
                    ) + translation
                }
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
        self.content.as_widget_mut().update(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            shell,
            &Rectangle::with_size(Size::INFINITE),
        );
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
}
