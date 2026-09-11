//! A sash for resizing containers.

use iced::border::{Border, Radius};
use iced::event::Event;
use iced::advanced::layout;
use iced::{Background, Element};
use iced::advanced::renderer;
use iced::touch;
use iced::advanced::widget::tree::{self, Tree};
use iced::{
    self, Color, Length, 
    Rectangle, Size, Theme,
};
use iced::advanced::{mouse, overlay, Layout, Shell, Widget};
use iced::advanced::Renderer as AdvancedRenderer;
use iced::Point;
use std::sync::atomic::{AtomicU64, Ordering};


pub struct SashH;

impl SashH {
    /// Creates a horizontal sash widget.
    ///
    /// - `children` — panel contents; one per entry in `initial_sizes`.
    /// - `initial_sizes` — starting widths used on the first render only.
    /// - `height` — shared panel height in pixels.
    /// - `sash_size` — thickness of each vertical drag handle in pixels.
    pub fn new<'a, Message, Theme>(
        children: Vec<Element<'a, Message, Theme>>,
        initial_sizes: Vec<f32>,
        height: f32,
        sash_size: f32,
    ) -> SashWidget<'a, Message, Theme>
    where
        Message: Clone + 'a,
        Theme: Catalog + 'a,
    {
        SashWidget {
            children,
            initial_sizes,
            cross_size: height,
            sash_size,
            axis: Axis::Horizontal,
            id: Id::unique(),
            max_size: None,
            min_size: 0.0,
            on_resize: None,
            on_release: None,
            sync_sizes: None,
            class: Theme::default(),
            outer_handle_size: None,
            outer_resize_mode: OuterResizeMode::LastOnly,
            on_outer_resize: None,
            cross_handle_size: None,
            min_cross_size: 0.0,
            max_cross_size: None,
            on_cross_resize: None,
            sync_cross_size: None,
            clip: false,
        }
    }
}


pub struct SashV;

impl SashV {
    /// Creates a vertical sash widget.
    ///
    /// - `children` — panel contents; one per entry in `initial_sizes`.
    /// - `initial_sizes` — starting heights used on the first render only.
    /// - `width` — shared panel width in pixels.
    /// - `sash_size` — thickness of each horizontal drag handle in pixels.
    pub fn new<'a, Message, Theme>(
        children: Vec<Element<'a, Message, Theme>>,
        initial_sizes: Vec<f32>,
        width: f32,
        sash_size: f32,
    ) -> SashWidget<'a, Message, Theme>
    where
        Message: Clone + 'a,
        Theme: Catalog + 'a,
    {
        SashWidget {
            children,
            initial_sizes,
            cross_size: width,
            sash_size,
            axis: Axis::Vertical,
            id: Id::unique(),
            max_size: None,
            min_size: 0.0,
            on_resize: None,
            on_release: None,
            sync_sizes: None,
            class: Theme::default(),
            outer_handle_size: None,
            outer_resize_mode: OuterResizeMode::LastOnly,
            on_outer_resize: None,
            cross_handle_size: None,
            min_cross_size: 0.0,
            max_cross_size: None,
            on_cross_resize: None,
            sync_cross_size: None,
            clip: false,
        }
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
    gap: f32,
    direction: Direction,
    ) -> Vec<Rectangle> 
{
    let mut w_h_bounds = vec![];
    let mut start = match direction {
            Direction::Horizontal => bounds.x,
            Direction::Vertical => bounds.y,
        };
        for (i, width_height) in widths_heights.iter().enumerate() {
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
            start += width_height;
            if i + 1 < widths_heights.len() {
                start += gap;
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
        Status::Active => palette.primary.base.color,
        Status::Hovered => palette.primary.weak.color,
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
        Status::Hovered => palette.background.strong.color,
        Status::Dragged => palette.background.strongest.color,
        Status::Disabled => palette.background.base.color,
    };

    Style {
        background: color.into(),
        border_color: Color::TRANSPARENT,
        border_width: 0.0,
        border_radius: 0.0.into()
    }
}


static NEXT_SASH_ID: AtomicU64 = AtomicU64::new(0);

/// A unique identifier for a [`SashH`] or [`SashV`] widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    /// Generates a new unique [`Id`] using an atomic counter.
    pub fn unique() -> Self {
        Id(NEXT_SASH_ID.fetch_add(1, Ordering::Relaxed))
    }

    /// Creates an [`Id`] from a specific number.
    pub fn new(n: u64) -> Self {
        Id(n)
    }
}


/// Applies resize math to a panel sizes vector.
///
/// The panel at `index` is set to `value`; the adjacent panel absorbs the
/// difference. `min_size` clamps both panels — pass `0.0` for no minimum.
pub fn resize(sizes: &mut Vec<f32>, index: usize, value: f32, min_size: f32) {
    if index >= sizes.len() {
        return;
    }
    let panel_count = sizes.len();
    let value = value.max(min_size);
    let diff = sizes[index] - value;
    if index + 1 < panel_count {
        let next_ideal = sizes[index + 1] + diff;
        let next_actual = next_ideal.max(min_size);
        let excess = (next_actual - next_ideal).max(0.0);
        sizes[index] = (value - excess).max(min_size);
        sizes[index + 1] = next_actual;
    } else {
        sizes[index] = value;
    }
}


/// Applies an outer resize to `sizes` according to `mode`.
/// `new_total` is the desired sum of all panel sizes after the drag.
/// Call this in your `update()` to mirror outer-handle drags on a stored
/// sizes vector, then pass that vector to `sync_sashes()` on the next frame.
pub fn apply_outer_resize(sizes: &mut Vec<f32>, new_total: f32, mode: OuterResizeMode, min_size: f32) {
    if sizes.is_empty() { return; }
    let new_total = new_total.max(sizes.len() as f32 * min_size);
    let old_total: f32 = sizes.iter().sum();
    let delta = new_total - old_total;
    if delta == 0.0 { return; }
    match mode {
        OuterResizeMode::LastOnly => {
            let last = sizes.len() - 1;
            sizes[last] = (sizes[last] + delta).max(min_size);
        }
        OuterResizeMode::Uniform => {
            let per = delta / sizes.len() as f32;
            for s in sizes.iter_mut() {
                *s = (*s + per).max(min_size);
            }
        }
        OuterResizeMode::Proportional => {
            if old_total <= 0.0 { return; }
            let scale = new_total / old_total;
            for s in sizes.iter_mut() {
                *s = (*s * scale).max(min_size);
            }
        }
    }
}

// Applies max size
fn apply_max_size(sizes: &[f32], max_size: Option<f32>) -> Vec<f32> {
    let total: f32 = sizes.iter().sum();
    match max_size {
        Some(max) if total > max && total > 0.0 => {
            sizes.iter().map(|s| s * max / total).collect()
        }
        _ => sizes.to_vec(),
    }
}

fn max_size_scale(sizes: &[f32], max_size: Option<f32>) -> f32 {
    let total: f32 = sizes.iter().sum();
    match max_size {
        Some(max) if total > max && total > 0.0 => total / max,
        _ => 1.0,
    }
}


// Axis
#[derive(Debug, Clone, Copy, PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    fn cursor_coord(self, p: Point) -> f32 {
        match self { Axis::Horizontal => p.x, Axis::Vertical => p.y }
    }
    fn bounds_end(self, b: Rectangle) -> f32 {
        match self { Axis::Horizontal => b.x + b.width, Axis::Vertical => b.y + b.height }
    }
    fn bounds_start(self, b: Rectangle) -> f32 {
        match self { Axis::Horizontal => b.x, Axis::Vertical => b.y }
    }
    fn main_start(self, r: Rectangle) -> f32 {
        match self { Axis::Horizontal => r.x, Axis::Vertical => r.y }
    }
    fn handle_main_size(self, r: Rectangle) -> f32 {
        match self { Axis::Horizontal => r.width, Axis::Vertical => r.height }
    }
    // Returns (handle_width, handle_height) for get_handle_bounds / get_width_height_bounds.
    fn handle_dims(self, sash_size: f32, cross_size: f32) -> (f32, f32) {
        match self {
            Axis::Horizontal => (sash_size, cross_size),
            Axis::Vertical   => (cross_size, sash_size),
        }
    }
    fn child_limit(self, panel_size: f32, cross_size: f32) -> Size {
        match self {
            Axis::Horizontal => Size::new(panel_size, cross_size),
            Axis::Vertical   => Size::new(cross_size, panel_size),
        }
    }
    fn child_offset(self, main: f32) -> Point {
        match self {
            Axis::Horizontal => Point::new(main, 0.0),
            Axis::Vertical   => Point::new(0.0, main),
        }
    }
    fn total_size(self, main: f32, cross_size: f32) -> Size {
        match self {
            Axis::Horizontal => Size::new(main, cross_size),
            Axis::Vertical   => Size::new(cross_size, main),
        }
    }
    fn direction(self) -> Direction {
        match self {
            Axis::Horizontal => Direction::Horizontal,
            Axis::Vertical   => Direction::Vertical,
        }
    }
    fn resize_interaction(self) -> mouse::Interaction {
        match self {
            Axis::Horizontal => mouse::Interaction::ResizingHorizontally,
            Axis::Vertical   => mouse::Interaction::ResizingVertically,
        }
    }
    fn cross_interaction(self) -> mouse::Interaction {
        match self {
            Axis::Horizontal => mouse::Interaction::ResizingVertically,
            Axis::Vertical   => mouse::Interaction::ResizingHorizontally,
        }
    }
    fn cross_coord(self, p: Point) -> f32 {
        match self { Axis::Horizontal => p.y, Axis::Vertical => p.x }
    }

    fn cross_handle_rect(self, bounds: Rectangle, cross_size: f32, handle_size: f32, total_main: f32) -> Rectangle {
        match self {
            Axis::Horizontal => Rectangle {
                x: bounds.x,
                y: bounds.y + cross_size,
                width: total_main,
                height: handle_size,
            },
            Axis::Vertical => Rectangle {
                x: bounds.x + cross_size,
                y: bounds.y,
                width: handle_size,
                height: total_main,
            },
        }
    }
    fn outer_handle_rect(self, bounds: Rectangle, panel_total: f32, handle_size: f32, cross_size: f32) -> Rectangle {
        match self {
            Axis::Horizontal => Rectangle {
                x: bounds.x + panel_total,
                y: bounds.y,
                width: handle_size,
                height: cross_size,
            },
            Axis::Vertical => Rectangle {
                x: bounds.x,
                y: bounds.y + panel_total,
                width: cross_size,
                height: handle_size,
            },
        }
    }
}


/// Controls how panels are resized when the outer trailing handle is dragged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OuterResizeMode {
    /// Only the last panel absorbs the change.
    #[default]
    LastOnly,
    /// Every panel grows or shrinks by the same amount.
    Uniform,
    /// Every panel scales proportionally to its current size.
    Proportional,
}

// State
struct SashState {
    id: Id,
    sizes: Vec<f32>,
    is_dragging: bool,
    drag_index: usize,
    hovered: Option<usize>,
    is_outer_dragging: bool,
    outer_hovered: bool,
    cross_size: f32,
    is_cross_dragging: bool,
    cross_hovered: bool,
    cross_drag_start_size: f32,
    cross_drag_start_cursor: f32,
    limits_max_main: f32,
    limits_max_cross: f32,
}

/// A resizable panel widget. Construct with [`SashH`] or [`SashV`].
pub struct SashWidget<'a, Message, Theme = iced::Theme>
where
    Theme: Catalog,
{
    children: Vec<Element<'a, Message, Theme>>,
    initial_sizes: Vec<f32>,
    cross_size: f32,
    sash_size: f32,
    axis: Axis,
    id: Id,
    max_size: Option<f32>,
    min_size: f32,
    on_resize: Option<Box<dyn Fn(Id, usize, f32) -> Message + 'a>>,
    on_release: Option<Box<dyn Fn(Id, usize) -> Message + 'a>>,
    sync_sizes: Option<Vec<f32>>,
    class: Theme::Class<'a>,
    outer_handle_size: Option<f32>,
    outer_resize_mode: OuterResizeMode,
    on_outer_resize: Option<Box<dyn Fn(Id, f32) -> Message + 'a>>,
    cross_handle_size: Option<f32>,
    min_cross_size: f32,
    max_cross_size: Option<f32>,
    on_cross_resize: Option<Box<dyn Fn(Id, f32) -> Message + 'a>>,
    sync_cross_size: Option<f32>,
    clip: bool,
}

impl<'a, Message, Theme> SashWidget<'a, Message, Theme>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
{
    /// Overrides the auto-generated [`Id`]. Only needed for multi-sash routing.
    pub fn id(mut self, id: Id) -> Self { self.id = id; self }

    /// Maximum total size; panels scale proportionally when exceeded.
    pub fn max_size(mut self, max: f32) -> Self { self.max_size = Some(max); self }

    /// Maximum total size; panels scale proportionally when exceeded.
    pub fn max_size_maybe(mut self, max: Option<f32>) -> Self {
        if let Some(v) = max {
            self.max_size = Some(v);
        }
        self
    }

    /// Minimum panel size enforced while dragging. Default: `0.0`.
    pub fn min_size(mut self, min: f32) -> Self { self.min_size = min; self }

    /// Optional notification fired on every drag tick: `(id, handle_index, new_size)`.
    pub fn on_resize(mut self, f: impl Fn(Id, usize, f32) -> Message + 'a) -> Self {
        self.on_resize = Some(Box::new(f)); self
    }

    /// Optional notification fired on mouse release: `(id, handle_index)`.
    pub fn on_release(mut self, f: impl Fn(Id, usize) -> Message + 'a) -> Self {
        self.on_release = Some(Box::new(f)); self
    }

    /// Sets the visual style of the sash handles.
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into(); self
    }

    /// Pushes external sizes into tree state each layout pass.
    /// Use this to synchronise two or more sashes from `on_resize` callbacks.
    pub fn sync_sashes(mut self, sizes: Vec<f32>) -> Self {
        self.sync_sizes = Some(sizes); self
    }

    /// Enables an outer resize handle at the trailing edge (right for `SashH`, bottom for `SashV`).
    /// `size` sets the handle thickness in pixels.
    pub fn outer_handle(mut self, size: f32) -> Self {
        self.outer_handle_size = Some(size); self
    }

    /// Sets how panels are resized when the outer handle is dragged. Default: [`OuterResizeMode::LastOnly`].
    pub fn outer_resize_mode(mut self, mode: OuterResizeMode) -> Self {
        self.outer_resize_mode = mode; self
    }

    /// Callback fired on every outer-handle drag tick: `(id, new_total_main_size)`.
    pub fn on_outer_resize(mut self, f: impl Fn(Id, f32) -> Message + 'a) -> Self {
        self.on_outer_resize = Some(Box::new(f)); self
    }

    /// Enables a cross-size handle at the far edge of the cross axis
    /// (bottom for `SashH`, right for `SashV`), spanning the full main-axis width.
    /// `size` sets the handle thickness in pixels.
    pub fn cross_handle(mut self, size: f32) -> Self {
        self.cross_handle_size = Some(size); self
    }

    /// Minimum cross size enforced while dragging the cross handle. Default: `0.0`.
    pub fn min_cross_size(mut self, min: f32) -> Self {
        self.min_cross_size = min; self
    }

    /// Maximum cross size enforced while dragging the cross handle.
    pub fn max_cross_size(mut self, max: f32) -> Self {
        self.max_cross_size = Some(max); self
    }

    /// Maximum cross size enforced while dragging the cross handle.
    pub fn max_cross_size_maybe(mut self, max: Option<f32>) -> Self {
        if let Some(v) = max {
            self.max_cross_size = Some(v);
        }
        self
    }

    /// Callback fired on every cross-handle drag tick: `(id, new_cross_size)`.
    pub fn on_cross_resize(mut self, f: impl Fn(Id, f32) -> Message + 'a) -> Self {
        self.on_cross_resize = Some(Box::new(f)); self
    }

    /// Pushes an external cross size into tree state each layout pass.
    /// Use this to synchronise cross sizes across sashes from `on_cross_resize` callbacks.
    pub fn sync_cross_sashes(mut self, size: f32) -> Self {
        self.sync_cross_size = Some(size); self
    }

    /// Sets whether the contents of each panel should be clipped to its bounds.
    /// Prevents child content from overrunning adjacent panels when resized. Default: `false`.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip; self
    }
}

impl<Message, Theme> Widget<Message, Theme, iced::Renderer>
    for SashWidget<'_, Message, Theme>
where
    Message: Clone,
    Theme: Catalog,
{
    fn tag(&self) -> tree::Tag { tree::Tag::of::<SashState>() }

    fn state(&self) -> tree::State {
        tree::State::new(SashState {
            id: self.id,
            sizes: self.initial_sizes.clone(),
            is_dragging: false,
            drag_index: 0,
            hovered: None,
            is_outer_dragging: false,
            outer_hovered: false,
            cross_size: self.cross_size,
            is_cross_dragging: false,
            cross_hovered: false,
            cross_drag_start_size: self.cross_size,
            cross_drag_start_cursor: 0.0,
            limits_max_main: f32::INFINITY,
            limits_max_cross: f32::INFINITY,
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(&mut self.children);
    }

    fn size(&self) -> Size<Length> {
        Size { width: Length::Shrink, height: Length::Shrink }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        if let Some(new) = &self.sync_sizes {
            let st = tree.state.downcast_mut::<SashState>();
            if !st.is_dragging && !st.is_outer_dragging && &st.sizes != new {
                st.sizes = new.clone();
            }
        }
        if let Some(new) = self.sync_cross_size {
            let st = tree.state.downcast_mut::<SashState>();
            if !st.is_cross_dragging && st.cross_size != new {
                st.cross_size = new;
            }
        }
        let ax = self.axis;
        // Clamp stored sizes to whatever space the parent is offering.
        {
            let max = limits.max();
            let max_main  = match ax { Axis::Horizontal => max.width,  Axis::Vertical => max.height };
            let max_cross = match ax { Axis::Horizontal => max.height, Axis::Vertical => max.width  };
            let st = tree.state.downcast_mut::<SashState>();
            st.limits_max_main = max_main;
            st.limits_max_cross = max_cross;
            if max_cross.is_finite() {
                let avail = (max_cross - self.cross_handle_size.unwrap_or(0.0)).max(0.0);
                st.cross_size = st.cross_size.min(avail);
            }
            if max_main.is_finite() {
                let n_sashes = st.sizes.len().saturating_sub(1);
                let avail = (max_main - self.outer_handle_size.unwrap_or(0.0) - n_sashes as f32 * self.sash_size).max(0.0);
                let total: f32 = st.sizes.iter().sum();
                if total > avail && total > 0.0 {
                    let scale = avail / total;
                    for s in &mut st.sizes {
                        *s *= scale;
                    }
                }
            }
        }
        let (display, cross_size) = {
            let st = tree.state.downcast_ref::<SashState>();
            let s = if st.sizes.is_empty() { &self.initial_sizes } else { &st.sizes };
            (apply_max_size(s, self.max_size), st.cross_size)
        };

        let mut child_nodes = Vec::with_capacity(self.children.len());
        let mut main = 0.0_f32;
        let n_panels = self.children.len();
        for (i, child) in self.children.iter_mut().enumerate() {
            let panel_size = display.get(i).copied().unwrap_or(0.0);
            let lim = layout::Limits::new(Size::ZERO, ax.child_limit(panel_size, cross_size));
            let node = child
                .as_widget_mut()
                .layout(&mut tree.children[i], renderer, &lim)
                .move_to(ax.child_offset(main));
            child_nodes.push(node);
            main += panel_size;
            if i + 1 < n_panels {
                main += self.sash_size;
            }
        }
        let total_main = main + self.outer_handle_size.unwrap_or(0.0);
        let total_cross = cross_size + self.cross_handle_size.unwrap_or(0.0);
        layout::Node::with_children(ax.total_size(total_main, total_cross), child_nodes)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, child_layout), child_tree) in self
            .children.iter()
            .zip(layout.children())
            .zip(tree.children.iter())
        {
            if self.clip {
                let child_bounds = child_layout.bounds();
                if let Some(clipped_viewport) = child_bounds.intersection(viewport) {
                    child.as_widget().draw(child_tree, renderer, theme, style, child_layout, cursor, &clipped_viewport);
                }
            } else {
                child.as_widget().draw(child_tree, renderer, theme, style, child_layout, cursor, viewport);
            }
        }

        let ax = self.axis;
        let st = tree.state.downcast_ref::<SashState>();
        let display = apply_max_size(&st.sizes, self.max_size);
        let cross_size = st.cross_size;
        let bounds = layout.bounds();
        let offsets: Vec<f32> = (0..display.len()).map(|i| i as f32 * self.sash_size).collect();
        let (hw, hh) = ax.handle_dims(self.sash_size, cross_size);
        let hbs = get_handle_bounds(bounds, &display, hw, hh, &offsets, false, ax.direction());
        let hover = st.hovered;
        for (i, hb) in hbs.iter().enumerate() {
            let status = if st.is_dragging && i == st.drag_index { Status::Dragged }
                else if Some(i) == hover { Status::Hovered }
                else { Status::Active };
            let sty = theme.style(&self.class, status);
            renderer.fill_quad(
                renderer::Quad {
                    bounds: *hb,
                    border: Border { radius: sty.border_radius, width: sty.border_width, color: sty.border_color },
                    ..renderer::Quad::default()
                },
                sty.background,
            );
        }

        if let Some(ohs) = self.outer_handle_size {
            let n_sashes = display.len().saturating_sub(1);
            let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
            let outer_rect = ax.outer_handle_rect(bounds, panel_total, ohs, cross_size);
            let outer_status = if st.is_outer_dragging { Status::Dragged }
                else if st.outer_hovered { Status::Hovered }
                else { Status::Active };
            let sty = theme.style(&self.class, outer_status);
            renderer.fill_quad(
                renderer::Quad {
                    bounds: outer_rect,
                    border: Border { radius: sty.border_radius, width: sty.border_width, color: sty.border_color },
                    ..renderer::Quad::default()
                },
                sty.background,
            );
        }

        if let Some(chs) = self.cross_handle_size {
            let n_sashes = display.len().saturating_sub(1);
            let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
            let total_main = panel_total + self.outer_handle_size.unwrap_or(0.0);
            let cross_rect = ax.cross_handle_rect(bounds, cross_size, chs, total_main);
            let cross_status = if st.is_cross_dragging { Status::Dragged }
                else if st.cross_hovered { Status::Hovered }
                else { Status::Active };
            let sty = theme.style(&self.class, cross_status);
            renderer.fill_quad(
                renderer::Quad {
                    bounds: cross_rect,
                    border: Border { radius: sty.border_radius, width: sty.border_width, color: sty.border_color },
                    ..renderer::Quad::default()
                },
                sty.background,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((child, child_layout), child_tree) in self
            .children.iter_mut()
            .zip(layout.children())
            .zip(tree.children.iter_mut())
        {
            child.as_widget_mut().update(child_tree, event, child_layout, cursor, renderer, shell, viewport);
        }

        let ax = self.axis;
        let bounds = layout.bounds();
        let st = tree.state.downcast_mut::<SashState>();
        let is_dragging = st.is_dragging;
        let cross_size = st.cross_size;

        let display = apply_max_size(&st.sizes, self.max_size);
        let scale = max_size_scale(&st.sizes, self.max_size);
        let end = ax.bounds_end(bounds);
        let offsets: Vec<f32> = (0..display.len()).map(|i| i as f32 * self.sash_size).collect();
        let (hw, hh) = ax.handle_dims(self.sash_size, cross_size);
        let hbs = get_handle_bounds(bounds, &display, hw, hh, &offsets, false, ax.direction());
        let pbs = get_width_height_bounds(bounds, &display, hw, hh, self.sash_size, ax.direction());

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(idx) = find_mouse_over_handle_bounds(&hbs, cursor) {
                    st.is_dragging = true;
                    st.drag_index = idx;
                    shell.capture_event();
                } else if let Some(ohs) = self.outer_handle_size {
                    let n_sashes = display.len().saturating_sub(1);
                    let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
                    let outer_rect = ax.outer_handle_rect(bounds, panel_total, ohs, cross_size);
                    if cursor.is_over(outer_rect) {
                        st.is_outer_dragging = true;
                        shell.capture_event();
                    } else if let Some(chs) = self.cross_handle_size {
                        let total_main = panel_total + ohs;
                        let cross_rect = ax.cross_handle_rect(bounds, cross_size, chs, total_main);
                        if cursor.is_over(cross_rect) {
                            st.is_cross_dragging = true;
                            st.cross_drag_start_size = st.cross_size;
                            st.cross_drag_start_cursor = ax.cross_coord(cursor.position().unwrap_or_default());
                            shell.capture_event();
                        }
                    }
                } else if let Some(chs) = self.cross_handle_size {
                    let n_sashes = display.len().saturating_sub(1);
                    let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
                    let total_main = panel_total;
                    let cross_rect = ax.cross_handle_rect(bounds, cross_size, chs, total_main);
                    if cursor.is_over(cross_rect) {
                        st.is_cross_dragging = true;
                        st.cross_drag_start_size = st.cross_size;
                        st.cross_drag_start_cursor = ax.cross_coord(cursor.position().unwrap_or_default());
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if is_dragging {
                    let id = st.id;
                    if let Some(f) = &self.on_release { shell.publish(f(id, st.drag_index)); }
                    st.is_dragging = false;
                    st.drag_index = 0;
                    shell.invalidate_layout();
                    shell.request_redraw();
                    shell.capture_event();
                } else if st.is_outer_dragging {
                    st.is_outer_dragging = false;
                    shell.invalidate_layout();
                    shell.request_redraw();
                    shell.capture_event();
                } else if st.is_cross_dragging {
                    st.is_cross_dragging = false;
                    shell.invalidate_layout();
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { id: _, position }) => {
                if is_dragging {
                    let idx = st.drag_index;
                    let id = st.id;
                    let hb = hbs[idx];
                    let pb = pbs[idx];
                    let hc = hbs.len();
                    let pc = pbs.len();
                    let pos = ax.cursor_coord(*position);
                    let pb_start = ax.main_start(pb);
                    let hb_main = ax.handle_main_size(hb);
                    let ohs = self.outer_handle_size.unwrap_or(0.0);
                    let v = if pos < pb_start && idx == 0 {
                        0.0_f32
                    } else if idx > 0 && pos < ax.main_start(hbs[idx - 1]) {
                        0.0_f32
                    } else if idx < hc - 1 && pos > ax.main_start(hbs[idx + 1]) {
                        (ax.main_start(hbs[idx + 1]) - hb_main - pb_start).round()
                    } else if hc < pc && pos > end - ohs - hb_main {
                        (end - ohs - hb_main - pb_start).round()
                    } else {
                        (pos - pb_start).round()
                    };
                    resize(&mut st.sizes, idx, v * scale, self.min_size);
                    if let Some(f) = &self.on_resize { shell.publish(f(id, idx, v * scale)); }
                    shell.capture_event();
                    shell.invalidate_layout();
                    shell.request_redraw();
                } else if st.is_outer_dragging {
                    let id = st.id;
                    let pos = ax.cursor_coord(*position);
                    let n_sashes = st.sizes.len().saturating_sub(1);
                    let sash_total = n_sashes as f32 * self.sash_size;
                    let container_max = (st.limits_max_main - self.outer_handle_size.unwrap_or(0.0) - sash_total).max(0.0);
                    let new_total = (pos - ax.bounds_start(bounds) - sash_total).round()
                        .max(0.0)
                        .min(self.max_size.unwrap_or(f32::MAX))
                        .min(container_max);
                    apply_outer_resize(&mut st.sizes, new_total, self.outer_resize_mode, self.min_size);
                    if let Some(f) = &self.on_outer_resize {
                        let total: f32 = st.sizes.iter().sum();
                        shell.publish(f(id, total));
                    }
                    shell.capture_event();
                    shell.invalidate_layout();
                    shell.request_redraw();
                } else if st.is_cross_dragging {
                    let id = st.id;
                    let pos = ax.cross_coord(*position);
                    let container_max_cross = (st.limits_max_cross - self.cross_handle_size.unwrap_or(0.0)).max(0.0);
                    let new_cross = (st.cross_drag_start_size + pos - st.cross_drag_start_cursor)
                        .round()
                        .max(self.min_cross_size)
                        .min(self.max_cross_size.unwrap_or(f32::MAX))
                        .min(container_max_cross);
                    st.cross_size = new_cross;
                    if let Some(f) = &self.on_cross_resize { shell.publish(f(id, new_cross)); }
                    shell.capture_event();
                    shell.invalidate_layout();
                    shell.request_redraw();
                } else {
                    let new_hover = find_mouse_over_handle_bounds(&hbs, cursor);
                    if new_hover != st.hovered {
                        st.hovered = new_hover;
                        shell.request_redraw();
                    }
                    if let Some(ohs) = self.outer_handle_size {
                        let n_sashes = display.len().saturating_sub(1);
                        let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
                        let outer_rect = ax.outer_handle_rect(bounds, panel_total, ohs, cross_size);
                        let new_outer_hovered = cursor.is_over(outer_rect);
                        if new_outer_hovered != st.outer_hovered {
                            st.outer_hovered = new_outer_hovered;
                            shell.request_redraw();
                        }
                    }
                    if let Some(chs) = self.cross_handle_size {
                        let n_sashes = display.len().saturating_sub(1);
                        let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
                        let total_main = panel_total + self.outer_handle_size.unwrap_or(0.0);
                        let cross_rect = ax.cross_handle_rect(bounds, cross_size, chs, total_main);
                        let new_cross_hovered = cursor.is_over(cross_rect);
                        if new_cross_hovered != st.cross_hovered {
                            st.cross_hovered = new_cross_hovered;
                            shell.request_redraw();
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let ax = self.axis;
        let st = tree.state.downcast_ref::<SashState>();
        if st.is_dragging || st.is_outer_dragging { return ax.resize_interaction(); }
        if st.is_cross_dragging { return ax.cross_interaction(); }
        let display = apply_max_size(&st.sizes, self.max_size);
        let cross_size = st.cross_size;
        let bounds = layout.bounds();
        let offsets: Vec<f32> = (0..display.len()).map(|i| i as f32 * self.sash_size).collect();
        let (hw, hh) = ax.handle_dims(self.sash_size, cross_size);
        let hbs = get_handle_bounds(bounds, &display, hw, hh, &offsets, false, ax.direction());
        if find_mouse_over_handle_bounds(&hbs, cursor).is_some() {
            return ax.resize_interaction();
        }
        if let Some(ohs) = self.outer_handle_size {
            let n_sashes = display.len().saturating_sub(1);
            let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
            let outer_rect = ax.outer_handle_rect(bounds, panel_total, ohs, cross_size);
            if cursor.is_over(outer_rect) {
                return ax.resize_interaction();
            }
        }
        if let Some(chs) = self.cross_handle_size {
            let n_sashes = display.len().saturating_sub(1);
            let panel_total: f32 = display.iter().sum::<f32>() + n_sashes as f32 * self.sash_size;
            let total_main = panel_total + self.outer_handle_size.unwrap_or(0.0);
            let cross_rect = ax.cross_handle_rect(bounds, cross_size, chs, total_main);
            if cursor.is_over(cross_rect) {
                return ax.cross_interaction();
            }
        }
        self.children.iter().zip(layout.children()).zip(tree.children.iter())
            .map(|((c, l), t)| c.as_widget().mouse_interaction(t, l, cursor, viewport, renderer))
            .max()
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: iced::Vector,
    ) -> Vec<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        overlay::from_children(&mut self.children, tree, layout, renderer, viewport, translation)
    }
}

impl<'a, Message, Theme> From<SashWidget<'a, Message, Theme>>
    for Element<'a, Message, Theme>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
{
    fn from(w: SashWidget<'a, Message, Theme>) -> Self {
        Element::new(w)
    }
}

// Type aliases preserve the existing public API.
// pub type SashHWidget<'a, Message, Theme = iced::Theme> = SashWidget<'a, Message, Theme>;
// pub type SashVWidget<'a, Message, Theme = iced::Theme> = SashWidget<'a, Message, Theme>;

#[cfg(test)]
mod tests {
    use super::*;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn rect(x: f32, y: f32, w: f32, h: f32) -> Rectangle {
        Rectangle { x, y, width: w, height: h }
    }

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.001
    }

    fn sizes_approx_eq(a: &[f32], b: &[f32]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| approx_eq(*x, *y))
    }

    // ── resize ───────────────────────────────────────────────────────────────

    #[test]
    fn resize_basic_shrink() {
        let mut s = vec![200.0, 200.0];
        resize(&mut s, 0, 150.0, 0.0);
        assert!(approx_eq(s[0], 150.0));
        assert!(approx_eq(s[1], 250.0)); // neighbour absorbs the 50px
    }

    #[test]
    fn resize_basic_grow() {
        let mut s = vec![100.0, 300.0];
        resize(&mut s, 0, 200.0, 0.0);
        assert!(approx_eq(s[0], 200.0));
        assert!(approx_eq(s[1], 200.0));
    }

    #[test]
    fn resize_respects_min_size_on_target() {
        let mut s = vec![200.0, 200.0];
        resize(&mut s, 0, 10.0, 50.0); // requested 10, min is 50
        assert!(s[0] >= 50.0);
    }

    #[test]
    fn resize_respects_min_size_on_neighbour() {
        // Growing panel 0 so much that panel 1 would go below min
        let mut s = vec![100.0, 100.0];
        resize(&mut s, 0, 180.0, 50.0); // panel 1 min = 50
        assert!(s[1] >= 50.0);
        assert!(s[0] >= 50.0);
    }

    #[test]
    fn resize_last_panel_no_neighbour() {
        let mut s = vec![100.0, 200.0];
        resize(&mut s, 1, 300.0, 0.0); // last panel, no neighbour
        assert!(approx_eq(s[1], 300.0));
        assert!(approx_eq(s[0], 100.0)); // first panel untouched
    }

    #[test]
    fn resize_out_of_bounds_index_is_noop() {
        let mut s = vec![100.0, 200.0];
        resize(&mut s, 5, 50.0, 0.0);
        assert!(sizes_approx_eq(&s, &[100.0, 200.0]));
    }

    #[test]
    fn resize_total_preserved_when_no_min_clash() {
        let mut s = vec![150.0, 250.0, 100.0];
        let total_before: f32 = s.iter().sum();
        resize(&mut s, 1, 200.0, 0.0);
        // Only the dragged panel and its right neighbour change; total of those two is preserved.
        let total_after: f32 = s.iter().sum();
        assert!(approx_eq(total_before, total_after));
    }

    // ── apply_outer_resize ───────────────────────────────────────────────────

    #[test]
    fn outer_resize_last_only_grow() {
        let mut s = vec![100.0, 100.0];
        apply_outer_resize(&mut s, 300.0, OuterResizeMode::LastOnly, 0.0);
        assert!(approx_eq(s[0], 100.0));
        assert!(approx_eq(s[1], 200.0));
    }

    #[test]
    fn outer_resize_last_only_shrink() {
        let mut s = vec![100.0, 200.0];
        apply_outer_resize(&mut s, 250.0, OuterResizeMode::LastOnly, 0.0);
        assert!(approx_eq(s[0], 100.0));
        assert!(approx_eq(s[1], 150.0));
    }

    #[test]
    fn outer_resize_last_only_respects_min() {
        let mut s = vec![100.0, 100.0];
        apply_outer_resize(&mut s, 10.0, OuterResizeMode::LastOnly, 50.0); // min=50, so total >= 100
        assert!(s[1] >= 50.0);
    }

    #[test]
    fn outer_resize_uniform_grow() {
        let mut s = vec![100.0, 100.0];
        apply_outer_resize(&mut s, 300.0, OuterResizeMode::Uniform, 0.0);
        assert!(sizes_approx_eq(&s, &[150.0, 150.0]));
    }

    #[test]
    fn outer_resize_proportional_grow() {
        let mut s = vec![100.0, 300.0]; // 25% / 75%
        apply_outer_resize(&mut s, 800.0, OuterResizeMode::Proportional, 0.0);
        assert!(approx_eq(s[0], 200.0)); // 25% of 800
        assert!(approx_eq(s[1], 600.0)); // 75% of 800
    }

    #[test]
    fn outer_resize_no_delta_is_noop() {
        let mut s = vec![100.0, 200.0];
        apply_outer_resize(&mut s, 300.0, OuterResizeMode::LastOnly, 0.0);
        assert!(sizes_approx_eq(&s, &[100.0, 200.0]));
    }

    #[test]
    fn outer_resize_empty_is_noop() {
        let mut s: Vec<f32> = vec![];
        apply_outer_resize(&mut s, 300.0, OuterResizeMode::LastOnly, 0.0);
        assert!(s.is_empty());
    }

    // ── apply_max_size ───────────────────────────────────────────────────────

    #[test]
    fn apply_max_size_scales_down() {
        let result = apply_max_size(&[200.0, 200.0], Some(200.0)); // total 400, max 200
        assert!(approx_eq(result[0], 100.0));
        assert!(approx_eq(result[1], 100.0));
    }

    #[test]
    fn apply_max_size_noop_when_under() {
        let s = vec![50.0, 50.0];
        let result = apply_max_size(&s, Some(200.0));
        assert!(sizes_approx_eq(&result, &s));
    }

    #[test]
    fn apply_max_size_noop_when_none() {
        let s = vec![500.0, 500.0];
        let result = apply_max_size(&s, None);
        assert!(sizes_approx_eq(&result, &s));
    }

    #[test]
    fn apply_max_size_preserves_proportions() {
        let result = apply_max_size(&[100.0, 300.0], Some(200.0)); // total 400, max 200
        // 100/400 * 200 = 50, 300/400 * 200 = 150
        assert!(approx_eq(result[0], 50.0));
        assert!(approx_eq(result[1], 150.0));
    }

    // ── max_size_scale ───────────────────────────────────────────────────────

    #[test]
    fn max_size_scale_over_limit() {
        let scale = max_size_scale(&[300.0, 300.0], Some(300.0)); // total 600, max 300
        assert!(approx_eq(scale, 2.0));
    }

    #[test]
    fn max_size_scale_at_or_under_limit() {
        assert!(approx_eq(max_size_scale(&[100.0, 100.0], Some(300.0)), 1.0));
        assert!(approx_eq(max_size_scale(&[100.0, 200.0], None), 1.0));
    }

    // ── get_handle_bounds ────────────────────────────────────────────────────

    #[test]
    fn handle_bounds_horizontal_positions() {
        let bounds = rect(0.0, 0.0, 500.0, 100.0);
        let panels = [150.0, 200.0, 150.0];
        // New model: offsets = [i * sash_size] — handle starts flush at panel right edge.
        let offsets = [0.0, 4.0]; // two inner handles, sash_size = 4
        let hbs = get_handle_bounds(bounds, &panels, 4.0, 100.0, &offsets, false, Direction::Horizontal);
        assert_eq!(hbs.len(), 2);
        // first handle starts at x=150 (right edge of panel 0)
        assert!(approx_eq(hbs[0].x, 150.0));
        // second handle starts at x=354 (right edge of panel 1 + one sash gap)
        assert!(approx_eq(hbs[1].x, 354.0));
    }

    #[test]
    fn handle_bounds_vertical_positions() {
        let bounds = rect(0.0, 0.0, 100.0, 400.0);
        let panels = [100.0, 100.0, 100.0];
        // New model: offsets = [i * sash_size] — handle starts flush at panel bottom edge.
        let offsets = [0.0, 4.0]; // sash_size = 4
        let hbs = get_handle_bounds(bounds, &panels, 100.0, 4.0, &offsets, false, Direction::Vertical);
        assert_eq!(hbs.len(), 2);
        // first handle at y=100 (bottom of panel 0)
        assert!(approx_eq(hbs[0].y, 100.0));
        // second handle at y=204 (bottom of panel 1 + one sash gap)
        assert!(approx_eq(hbs[1].y, 204.0));
    }

    // ── get_width_height_bounds ──────────────────────────────────────────────

    #[test]
    fn width_height_bounds_count_and_positions() {
        let bounds = rect(10.0, 5.0, 400.0, 100.0);
        let panels = [100.0, 150.0, 100.0];
        // gap = 4.0 (sash_size); each panel origin is offset by gap after every preceding panel.
        let wbs = get_width_height_bounds(bounds, &panels, 4.0, 100.0, 4.0, Direction::Horizontal);
        assert_eq!(wbs.len(), 3);
        assert!(approx_eq(wbs[0].x, 10.0));          // bounds.x
        assert!(approx_eq(wbs[1].x, 114.0));         // 10 + 100 + 4
        assert!(approx_eq(wbs[2].x, 268.0));         // 10 + 100 + 4 + 150 + 4
    }
}

