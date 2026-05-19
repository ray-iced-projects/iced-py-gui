//! A sash for resizing containers.

use iced::border::{Border, Radius};
use iced::event::Event;
use iced::advanced::layout;
use iced::window;
use iced::{Background, Element};
use iced::advanced::renderer;
use iced::touch;
use iced::advanced::widget::tree::{self, Tree};
use iced::{
    self, Color, Length, 
    Rectangle, Size, Theme,
};
use iced::advanced::{mouse, Layout, Shell, Widget};


pub fn sash_horizontal<'a, Message, Theme>(
    id: usize,
    widths: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    on_change: impl Fn((usize, usize, f32)) -> Message + 'a,
) -> Sash<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + 'a,
{
    let mut handle_offsets = vec![-handle_width/2.0; widths.len()-1];
        handle_offsets.extend([-handle_width]);
    
    Sash::new(
        id,
        widths, 
        handle_width, 
        handle_height,
        handle_offsets,
        Direction::Horizontal,
        on_change)
}

pub fn sash_vertical<'a, Message, Theme>(
    id: usize,
    heights: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    on_change: impl Fn((usize, usize, f32)) -> Message + 'a,
) -> Sash<'a, Message, Theme>
where
    Message: Clone,
    Theme: Catalog + 'a,
{
    let widths = heights;
    let mut handle_offsets = vec![-handle_height/2.0; widths.len()-1];
        // last offset pulled in to keep in bounds
        handle_offsets.extend([-handle_height]);
        
    Sash::new(
        id,
        widths, 
        handle_width, 
        handle_height,
        handle_offsets,
        Direction::Vertical,
        on_change)
}

pub struct Sash<'a, Message, Theme = iced::Theme>
where
    Theme: Catalog,
{
    id: usize,
    widths: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    on_change: Box<dyn Fn((usize, usize, f32)) -> Message + 'a>,
    on_release: Option<Message>,
    on_release_fn: Option<Box<dyn Fn((usize, usize)) -> Message + 'a>>,
    width: Length,
    height: Length,
    handle_offsets: Vec<f32>,
    include_last_handle: bool,
    direction: Direction,
    class: Theme::Class<'a>,
    /// Per-handle statuses, updated on RedrawRequested.
    statuses: Vec<Status>,
}

impl<'a, Message, Theme> Sash<'a, Message, Theme>
where
    Theme: Catalog,
{
    /// Sets the release message of the [`Sash`].
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }

    /// Sets a release callback of the [`Sash`] that receives `(id, handle_index)`.
    /// Use this instead of [`on_release`] when you need to know which handle was released.
    pub fn on_release_fn(mut self, f: impl Fn((usize, usize)) -> Message + 'a) -> Self {
        self.on_release_fn = Some(Box::new(f));
        self
    }
    /// Sets the width of the [`Sash`] which usually spans the entire width of the items.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Sash`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the handle offsets for alignment of the [`Sash`].
    pub fn handle_offsets(mut self, handle_offsets: Vec<f32>) -> Self {
        self.handle_offsets = handle_offsets;
        self
    }

    /// Sets the include_last_handle of the [`Sash`].
    /// If not included, the total width or height will not change
    pub fn include_last_handle(mut self, include: bool) -> Self {
        self.include_last_handle = include;
        self
    }

    /// Sets the direction of the [`Sash`].
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the style of the [`Sash`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Sash`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

#[derive(Default)]
struct State {
    is_dragging: bool,
    index: usize,
    handle_bounds: Vec<Rectangle>,
    width_height_bounds: Vec<Rectangle>,
}

impl<'a, Message, Theme> Sash<'a, Message, Theme>
where
    Theme: Catalog,
{
    /// The default height of a [`Sash`].
    pub const DEFAULT_HEIGHT: f32 = 21.0;

    /// Creates a new [`Sash`].
    pub fn new<F>(
        id: usize,
        widths: Vec<f32>,
        handle_width: f32,
        handle_height: f32,
        handle_offsets: Vec<f32>,
        direction: Direction, 
        on_change: F) 
        -> Self
    where
        F: 'a + Fn((usize, usize, f32)) -> Message,
    {
        Sash {
            id,
            widths,
            handle_width,
            handle_height,
            on_change: Box::new(on_change),
            on_release: None,
            on_release_fn: None,
            width: Length::Fill,
            height: Length::Fill,
            handle_offsets,
            include_last_handle: true,
            direction,
            class: Theme::default(),
            statuses: vec![],
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Sash<'_, Message, Theme>
where
    Theme: Catalog,
    Renderer: renderer::Renderer,
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        update::<Message, Theme, Renderer>(self, tree, event, layout, cursor, shell);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        _layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let is_mouse_over = 
            find_mouse_over_handle_bounds(
                &state.handle_bounds,  
                cursor);

        if state.is_dragging || is_mouse_over.is_some(){
            match self.direction {
                Direction::Horizontal => mouse::Interaction::ResizingHorizontally,
                Direction::Vertical => mouse::Interaction::ResizingVertically,
            }
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _renderer_style: &renderer::Style,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        
        for i in 0..self.widths.len() {
            let status = self.statuses.get(i).copied().unwrap_or(Status::Active);
            let style = theme.style(&self.class, status);
            
            renderer.fill_quad(
                renderer::Quad {
                    bounds: state.width_height_bounds[i],
                    ..renderer::Quad::default()
                },
                Background::Color(Color::TRANSPARENT),
            );
            // fill with the handle
            if !self.include_last_handle && i == self.widths.len()-1{
                break;
            }
            renderer.fill_quad(
                renderer::Quad {
                    bounds: state.handle_bounds[i],
                    border: Border {
                        radius: style.border_radius,
                        width: style.border_width,
                        color: style.border_color,
                    },
                    ..renderer::Quad::default()
                },
                style.background,
            );
        }
    }

}

impl<'a, Message, Theme, Renderer> From<Sash<'a, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(
        sash: Sash<'a, Message, Theme>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(sash)
    }
}

/// Processes the given [`Event`] and updates the [`State`] of an [`Sash`]
/// accordingly.
fn update<Message: Clone, Theme, Renderer>(
    widget: &mut Sash<'_, Message, Theme>,
    tree: &mut Tree,
    event: &Event,
    layout: Layout<'_>,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
) 
where
    Theme: Catalog,
    Renderer: iced::advanced::Renderer,
{
    
    let state = tree.state.downcast_mut::<State>();
    let is_dragging = state.is_dragging;
    let total_bounds = layout.bounds();
    
    // stores the state
    let mut widths = vec![];
    for width in widget.widths.iter() {
        match widget.direction {
            Direction::Horizontal => {
                widths.push(*width);
            },
            Direction::Vertical => {
                widths.push(*width);
            },
        }
    }
    state.handle_bounds = 
        get_handle_bounds(
            total_bounds,
            &widths,
            widget.handle_width, 
            widget.handle_height,
            &widget.handle_offsets,
            widget.include_last_handle,
            widget.direction);

    state.width_height_bounds =
        get_width_height_bounds(
            total_bounds,
            &widths,
            widget.handle_width, 
            widget.handle_height, 
            widget.direction);

    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            let index = 
                find_mouse_over_handle_bounds(
                    &state.handle_bounds, cursor);
            
            if index.is_some() {
                state.is_dragging = true;
                state.index = index.unwrap();
                shell.capture_event();
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerLifted { .. })
        | Event::Touch(touch::Event::FingerLost { .. }) => {
            if is_dragging {
                if let Some(f) = &widget.on_release_fn {
                    shell.publish(f((widget.id, state.index)));
                } else if let Some(on_release) = widget.on_release.clone() {
                    shell.publish(on_release);
                }
                state.is_dragging = false;
                state.index = 0;

                shell.request_redraw();
                shell.capture_event();
            }
        }
        Event::Mouse(mouse::Event::CursorMoved { position })
        | Event::Touch(touch::Event::FingerMoved { id: _, position }) => {
            if is_dragging {
                let end_x = total_bounds.x+total_bounds.width;
                let end_y = total_bounds.y+total_bounds.height;
                let handle_bounds = state.handle_bounds[state.index];
                let w_h_bounds = state.width_height_bounds[state.index];
                let handle_count = state.handle_bounds.len();
                let w_h_count = state.width_height_bounds.len();

                match widget.direction {
                    Direction::Horizontal => {
                        if (position.x - handle_bounds.x + handle_bounds.width/2.0).abs() > 0.99 {
                            let new_value = 
                                // Moving left
                                if position.x < w_h_bounds.x && state.index == 0 {

                                    state.handle_bounds[state.index].x = w_h_bounds.x;
                                    (state.index, 0.0)
                                } else 
                                // Moving left stopping at next sash
                                if state.index > 0 && position.x < state.handle_bounds[state.index-1].x {

                                    state.handle_bounds[state.index].x = state.handle_bounds[state.index-1].x;
                                    (state.index, 0.0)
                                } else
                                // Moving right: stop at next sash
                                if  state.index < handle_count-1 && (state.index < handle_count) && 
                                    (position.x > state.handle_bounds[state.index+1].x) {

                                    state.handle_bounds[state.index].x = state.handle_bounds[state.index+1].x;
                                    let new_value = (state.handle_bounds[state.index+1].x - w_h_bounds.x).round();
                                    (state.index, new_value)
                                } else 
                                // Moving right: last index and no sash at end
                                if (handle_count < w_h_count) && 
                                    (position.x > end_x-handle_bounds.width/2.0) {

                                    state.handle_bounds[state.index].x = end_x-handle_bounds.width/2.0;
                                    let new_value = (end_x-handle_bounds.width/2.0-w_h_bounds.x).round();
                                    (state.index, new_value)
                                }
                                    else {
                                    // moving
                                    state.handle_bounds[state.index].x = position.x;
                                    let new_value = (position.x - w_h_bounds.x).round();
                                    (state.index, new_value)
                                };
                            let new_value = (widget.id, new_value.0, new_value.1);
                            shell.publish((widget.on_change)(new_value));
                            shell.capture_event();
                        }
                    },
                    Direction::Vertical => {
                        if (position.y - handle_bounds.y + handle_bounds.height/2.0).abs() > 0.99 {
                            let new_value = 
                                // Moving up
                                if position.y < w_h_bounds.y && state.index == 0 {

                                    state.handle_bounds[state.index].y = w_h_bounds.y;
                                    (state.index, 0.0)
                                } else 
                                // Moving left stopping at next sash
                                if state.index > 0 && position.y < state.handle_bounds[state.index-1].y {

                                    state.handle_bounds[state.index].y = state.handle_bounds[state.index-1].y;
                                    (state.index, 0.0)
                                } else
                                // Moving right: stop at next sash
                                if  state.index < handle_count-1 && (state.index < handle_count) && 
                                    (position.y > state.handle_bounds[state.index+1].y) {

                                    state.handle_bounds[state.index].y = state.handle_bounds[state.index+1].y;
                                    let new_value = (state.handle_bounds[state.index+1].y - w_h_bounds.y).round();
                                    (state.index, new_value)
                                } else 
                                // Moving right: last index and no sash at end
                                if (handle_count < w_h_count) && 
                                    (position.y > end_y-handle_bounds.height/2.0) {
                                        
                                    state.handle_bounds[state.index].y = end_y-handle_bounds.height/2.0;
                                    let new_value = (end_y-handle_bounds.height/2.0-w_h_bounds.y).round();
                                    (state.index, new_value)
                                }
                                    else {
                                    // moving
                                    state.handle_bounds[state.index].y = position.y;
                                    let new_value = (position.y - w_h_bounds.y).round();
                                    (state.index, new_value)
                                };
                            let new_value = (widget.id, new_value.0, new_value.1);
                            shell.publish((widget.on_change)(new_value));
                            shell.capture_event();
                        }
                    },
                }
            }
        },
        _ => {}
    }

    // Status tracking — mirrors iced's Button/Scrollable pattern.
    // Compute the current per-handle status from live state + cursor.
    let is_mouse_over = find_mouse_over_handle_bounds(&state.handle_bounds, cursor);
    let current_statuses: Vec<Status> = (0..widget.widths.len())
        .map(|i| {
            if state.is_dragging && i == state.index {
                Status::Dragged
            } else if Some(i) == is_mouse_over {
                Status::Hovered
            } else {
                Status::Active
            }
        })
        .collect();

    if let Event::Window(window::Event::RedrawRequested(_)) = event {
        widget.statuses = current_statuses;
    } else if widget.statuses != current_statuses {
        shell.request_redraw();
    }
}

fn get_handle_bounds(
    bounds: Rectangle,
    widths_heights: &[f32],
    handle_width: f32,
    handle_height: f32,
    handle_offsets: &[f32],
    include_last_handle: bool,
    direction: Direction,
    ) -> Vec<Rectangle> 
{
    let mut handle_bounds = vec![];
    let mut start = match direction {
            Direction::Horizontal => bounds.x,
            Direction::Vertical => bounds.y,
        };
        for (i, width_height) in widths_heights.iter().enumerate() {
            
            if i == widths_heights.len()-1 {
                if include_last_handle {
                    start += width_height;
                } else {
                    break;
                }
            } else {
                start += width_height;
            }

            let rect = match direction {
                Direction::Horizontal => {
                    Rectangle{ 
                        x: start+handle_offsets[i], 
                        y: bounds.y, 
                        width: handle_width, 
                        height: handle_height,
                    }
                },
                Direction::Vertical => {
                    Rectangle{
                        x: bounds.x,
                        y: start+handle_offsets[i],
                        width: handle_width,
                        height: handle_height,
                    }
                },
            };
                
            handle_bounds.push(rect);

        }
        handle_bounds
}

fn get_width_height_bounds(
    bounds: Rectangle,
    widths_heights: &[f32],
    handle_width: f32,
    handle_height: f32,
    direction: Direction,
    ) -> Vec<Rectangle> 
{
    let mut w_h_bounds = vec![];
    let mut start = match direction {
            Direction::Horizontal => bounds.x,
            Direction::Vertical => bounds.y,
        };
        for width_height in widths_heights.iter() {
            let rect = match direction {
                Direction::Horizontal => {
                    Rectangle{ 
                        x: start, 
                        y: bounds.y, 
                        width: *width_height, 
                        height: handle_height,
                    }
                },
                Direction::Vertical => {
                    Rectangle{
                        x: bounds.x,
                        y: start,
                        width: handle_width,
                        height: *width_height,
                    }
                },
            };
                
            w_h_bounds.push(rect);

            match direction {
                Direction::Horizontal => {
                    start += width_height;
                },
                Direction::Vertical => {
                    start += width_height;
                },
            }
            
        }
        w_h_bounds
}

fn find_mouse_over_handle_bounds(
    handle_bounds: &[Rectangle],
    cursor: mouse::Cursor) 
    -> Option<usize> {
        for (index, bounds) in handle_bounds.iter().enumerate() {
            if cursor.is_over(*bounds) {
                return Some(index)
            }
        }
        None
}

/// The direction of [`Sash`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Direction {
    /// Horizontal resizing
    #[default]
    Horizontal,
    /// Vertical resizing
    Vertical,
}

/// The possible status of a [`Sash`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Sash`] can be interacted with.
    Active,
    /// The [`Sash`] is being hovered.
    Hovered,
    /// The [`Sash`] is being dragged.
    Dragged,
    /// The [`Sash`] is disabled.
    Disabled,
}

/// The appearance of a Sash.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Background`] of the handle.
    pub background: Background,
    /// The border width of the handle.
    pub border_width: f32,
    /// The border [`Color`] of the handle.
    pub border_color: Color,
    /// The border [`Radius`] of the handle.
    pub border_radius: Radius,
}

/// The theme catalog of a [`Sash`].
pub trait Catalog: Sized {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// A styling function for a [`Sash`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(subtle)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`Sash`].
pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let color = match status {
        Status::Active => palette.primary.strong.color,
        Status::Hovered => palette.primary.base.color,
        Status::Dragged => palette.primary.strong.color,
        Status::Disabled => palette.primary.weak.color,
    };

    Style {
        background: color.into(),
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 0.0.into()
    }
}

pub fn transparent(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let color = match status {
        Status::Active => Color::TRANSPARENT,
        Status::Hovered => palette.background.weak.color,
        Status::Dragged => palette.background.weakest.color,
        Status::Disabled => palette.background.base.color,
    };

    Style {
        background: color.into(),
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 0.0.into()
    }
}

pub fn subtle(theme: &Theme, status: Status) -> Style {
    let palette = theme.palette();

    let color = match status {
        Status::Active => palette.background.weak.color,
        Status::Hovered => palette.background.weaker.color,
        Status::Dragged => palette.background.weakest.color,
        Status::Disabled => palette.background.base.color,
    };

    Style {
        background: color.into(),
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 0.0.into()
    }
}

