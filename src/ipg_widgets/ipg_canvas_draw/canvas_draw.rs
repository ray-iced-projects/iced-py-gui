//! draw_canvas
use std::collections::HashMap;

use iced::keyboard::Key;
use iced::widget::Id;
use iced::widget::text::{LineHeight, Shaping};
use iced::{Color, Font, Pixels, Radians, Vector, mouse};
use iced::widget::canvas::Event;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Element, Fill, Point, Renderer, Theme};
use serde::{Deserialize, Serialize};

use pyo3::pyclass;


use super::helpers::{build_polygon, get_angle_of_vectors, get_horizontal_angle_of_vector, get_line_from_slope_intercept, get_linear_regression, get_mid_point, iced_h_text_alignment, iced_v_text_alignment, rotate_geometry, to_degrees, to_radians, translate_geometry};
use super::path_builds::{build_arc_path, build_bezier_path, build_circle_path, 
    build_ellipse_path, build_free_hand_path, build_line_path, 
    build_polygon_path, build_polyline_path, build_right_triangle_path, build_text_path};



#[derive(Debug, Clone, Default)]
pub enum CanvasWidget {
    #[default]
    None,
    Arc(Arc),
    Bezier(Bezier),
    Circle(Circle),
    Ellipse(Ellipse),
    Line(Line),
    PolyLine(PolyLine),
    Polygon(Polygon),
    RightTriangle(RightTriangle),
    Text(Text),
    FreeHand(FreeHand),
}

impl CanvasWidget {
    // ── simple field accessors ──────────────────────────────────────────
    pub fn id(&self) -> Id {
        match self {
            CanvasWidget::None => Id::new("None"),
            CanvasWidget::Arc(w) => w.id.clone(),
            CanvasWidget::Bezier(w) => w.id.clone(),
            CanvasWidget::Circle(w) => w.id.clone(),
            CanvasWidget::Ellipse(w) => w.id.clone(),
            CanvasWidget::Line(w) => w.id.clone(),
            CanvasWidget::PolyLine(w) => w.id.clone(),
            CanvasWidget::Polygon(w) => w.id.clone(),
            CanvasWidget::RightTriangle(w) => w.id.clone(),
            CanvasWidget::FreeHand(w) => w.id.clone(),
            CanvasWidget::Text(w) => w.id.clone(),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            CanvasWidget::None => Color::TRANSPARENT,
            CanvasWidget::Arc(w) => w.color,
            CanvasWidget::Bezier(w) => w.color,
            CanvasWidget::Circle(w) => w.color,
            CanvasWidget::Ellipse(w) => w.color,
            CanvasWidget::Line(w) => w.color,
            CanvasWidget::PolyLine(w) => w.color,
            CanvasWidget::Polygon(w) => w.color,
            CanvasWidget::RightTriangle(w) => w.color,
            CanvasWidget::FreeHand(w) => w.color,
            CanvasWidget::Text(w) => w.color,
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            CanvasWidget::None => 0.0,
            CanvasWidget::Arc(w) => w.width,
            CanvasWidget::Bezier(w) => w.width,
            CanvasWidget::Circle(w) => w.width,
            CanvasWidget::Ellipse(w) => w.width,
            CanvasWidget::Line(w) => w.width,
            CanvasWidget::PolyLine(w) => w.width,
            CanvasWidget::Polygon(w) => w.width,
            CanvasWidget::RightTriangle(w) => w.width,
            CanvasWidget::FreeHand(w) => w.width,
            CanvasWidget::Text(_) => 1.0,
        }
    }

    pub fn draw_mode(&self) -> DrawMode {
        match self {
            CanvasWidget::None => DrawMode::Display,
            CanvasWidget::Arc(w) => w.draw_mode,
            CanvasWidget::Bezier(w) => w.draw_mode,
            CanvasWidget::Circle(w) => w.draw_mode,
            CanvasWidget::Ellipse(w) => w.draw_mode,
            CanvasWidget::Line(w) => w.draw_mode,
            CanvasWidget::PolyLine(w) => w.draw_mode,
            CanvasWidget::Polygon(w) => w.draw_mode,
            CanvasWidget::RightTriangle(w) => w.draw_mode,
            CanvasWidget::FreeHand(w) => w.draw_mode,
            CanvasWidget::Text(w) => w.draw_mode,
        }
    }

    pub fn status(&self) -> DrawStatus {
        match self {
            CanvasWidget::None => DrawStatus::Completed,
            CanvasWidget::Arc(w) => w.status,
            CanvasWidget::Bezier(w) => w.status,
            CanvasWidget::Circle(w) => w.status,
            CanvasWidget::Ellipse(w) => w.status,
            CanvasWidget::Line(w) => w.status,
            CanvasWidget::PolyLine(w) => w.status,
            CanvasWidget::Polygon(w) => w.status,
            CanvasWidget::RightTriangle(w) => w.status,
            CanvasWidget::FreeHand(w) => w.status,
            CanvasWidget::Text(w) => w.status,
        }
    }

    // ── field mutators ──────────────────────────────────────────────────
    pub fn set_mode(&mut self, mode: DrawMode) {
        match self {
            CanvasWidget::None => {},
            CanvasWidget::Arc(w) => w.draw_mode = mode,
            CanvasWidget::Bezier(w) => w.draw_mode = mode,
            CanvasWidget::Circle(w) => w.draw_mode = mode,
            CanvasWidget::Ellipse(w) => w.draw_mode = mode,
            CanvasWidget::Line(w) => w.draw_mode = mode,
            CanvasWidget::PolyLine(w) => w.draw_mode = mode,
            CanvasWidget::Polygon(w) => w.draw_mode = mode,
            CanvasWidget::RightTriangle(w) => w.draw_mode = mode,
            CanvasWidget::FreeHand(w) => w.draw_mode = mode,
            CanvasWidget::Text(w) => w.draw_mode = mode,
        }
    }

    pub fn set_status(&mut self, status: DrawStatus) {
        match self {
            CanvasWidget::None => {},
            CanvasWidget::Arc(w) => w.status = status,
            CanvasWidget::Bezier(w) => w.status = status,
            CanvasWidget::Circle(w) => w.status = status,
            CanvasWidget::Ellipse(w) => w.status = status,
            CanvasWidget::Line(w) => w.status = status,
            CanvasWidget::PolyLine(w) => w.status = status,
            CanvasWidget::Polygon(w) => w.status = status,
            CanvasWidget::RightTriangle(w) => w.status = status,
            CanvasWidget::FreeHand(w) => w.status = status,
            CanvasWidget::Text(w) => w.status = status,
        }
    }

    // ── geometry helpers ────────────────────────────────────────────────
    /// Geometric center/anchor used for find-closest and distance comparisons.
    pub fn mid_point(&self) -> Point {
        match self {
            CanvasWidget::None => Point::default(),
            CanvasWidget::Arc(w) => w.mid_point,
            CanvasWidget::Bezier(w) => w.mid_point,
            CanvasWidget::Circle(w) => w.center,
            CanvasWidget::Ellipse(w) => w.center,
            CanvasWidget::Line(w) => w.mid_point,
            CanvasWidget::PolyLine(w) => w.mid_point,
            CanvasWidget::Polygon(w) => w.mid_point,
            CanvasWidget::RightTriangle(w) => w.mid_point,
            CanvasWidget::FreeHand(w) => w.points.first().copied().unwrap_or_default(),
            CanvasWidget::Text(w) => w.position,
        }
    }

    /// Distance from the widget's center/anchor to the given cursor point.
    pub fn distance_to(&self, cursor: Point) -> f32 {
        cursor.distance(self.mid_point())
    }

    /// Current rotation degrees, used when entering Rotate mode.
    pub fn degrees(&self) -> Option<f32> {
        match self {
            CanvasWidget::None => Some(0.0),
            CanvasWidget::Arc(w) => Some(Radians::into(w.start_angle)),
            CanvasWidget::Bezier(w) => Some(w.degrees),
            CanvasWidget::Circle(_) => Some(0.0),
            CanvasWidget::Ellipse(_) => Some(0.0),
            CanvasWidget::Line(w) => Some(w.degrees),
            CanvasWidget::PolyLine(w) => Some(w.degrees),
            CanvasWidget::Polygon(w) => Some(w.degrees),
            CanvasWidget::RightTriangle(w) => Some(w.degrees),
            CanvasWidget::FreeHand(_) => None,
            CanvasWidget::Text(w) => Some(w.degrees),
        }
    }

    // ── path builders ───────────────────────────────────────────────────
    /// Build a (path, color, width) for the completed cached layer.
    /// Returns None when Inprogress, or for Text / None variants.
    pub fn display_path(&self) -> Option<(Path, Color, f32)> {
        if self.status() == DrawStatus::Inprogress {
            return None;
        }
        let color = self.color();
        let width = self.width();
        let mode  = self.draw_mode();
        match self {
            CanvasWidget::None | CanvasWidget::Text(_) => None,
            CanvasWidget::Arc(arc) => {
                let (path, ..) = build_arc_path(arc, mode, None, None, false);
                Some((path, color, width))
            },
            CanvasWidget::Bezier(bz) => {
                let (path, ..) = build_bezier_path(bz, mode, None, None, false, None);
                Some((path, color, width))
            },
            CanvasWidget::Circle(cir) => {
                let path = build_circle_path(cir, mode, None, None, false);
                Some((path, color, width))
            },
            CanvasWidget::Ellipse(ell) => {
                let path = build_ellipse_path(ell, mode, None, None, false);
                Some((path, color, width))
            },
            CanvasWidget::Line(line) => {
                let (path, ..) = build_line_path(line, mode, None, None, false, None);
                Some((path, color, width))
            },
            CanvasWidget::Polygon(pg) => {
                let (path, ..) = build_polygon_path(pg, mode, None, false, false, None);
                Some((path, color, width))
            },
            CanvasWidget::PolyLine(pl) => {
                let (path, ..) = build_polyline_path(pl, mode, None, None, false, false, None);
                Some((path, color, width))
            },
            CanvasWidget::RightTriangle(tr) => {
                let (path, ..) = build_right_triangle_path(tr, mode, None, None, false, false, None);
                Some((path, color, width))
            },
            CanvasWidget::FreeHand(fh) => {
                let path = build_free_hand_path(fh, mode, None, None);
                Some((path, color, width))
            },
        }
    }

    /// Build a path for the pending overlay (New / Edit / Rotate modes).
    /// Returns (path, color, width, label_anchor, degrees_left, degrees_center).
    /// Returns None for Text and None variants — those need separate frame handling.
    pub fn pending_path(
        &self,
        mode: DrawMode,
        cursor: Option<Point>,
        edit_index: Option<usize>,
        edit_mid: bool,
        edit_other: bool,
        degrees_override: Option<f32>,
    ) -> Option<(Path, Color, f32, Point, Option<f32>, Option<f32>)> {
        let color = self.color();
        let width = self.width();
        match self {
            CanvasWidget::None | CanvasWidget::Text(_) => None,
            CanvasWidget::Arc(arc) => {
                let (path, mid_point, _, _, deg_l, deg_c) =
                    build_arc_path(arc, mode, cursor, edit_index, edit_mid);
                Some((path, color, width, mid_point, deg_l, deg_c))
            },
            CanvasWidget::Bezier(bz) => {
                let (path, degrees, builder_mid) =
                    build_bezier_path(bz, mode, cursor, edit_index, edit_mid, degrees_override);
                // In New mode the label appears near the first anchor point;
                // for Edit/Rotate use the (potentially-translated) mid from the builder.
                let anchor = if mode == DrawMode::New {
                    bz.points.first().copied().unwrap_or(bz.mid_point)
                } else {
                    builder_mid
                };
                Some((path, color, width, anchor, None, Some(degrees)))
            },
            CanvasWidget::Circle(cir) => {
                let path = build_circle_path(cir, mode, cursor, edit_index, edit_mid);
                Some((path, color, width, cir.center, None, None))
            },
            CanvasWidget::Ellipse(ell) => {
                let path = build_ellipse_path(ell, mode, cursor, edit_index, edit_mid);
                let deg_c = if mode == DrawMode::Rotate {
                    Some(to_degrees(&ell.rotation.0))
                } else {
                    None
                };
                Some((path, color, width, ell.center, None, deg_c))
            },
            CanvasWidget::Line(line) => {
                let (path, degrees, builder_mid) =
                    build_line_path(line, mode, cursor, edit_index, edit_mid, degrees_override);
                // New mode shows the angle as a left label near the first click point;
                // Edit/Rotate show it as a center label near the mid-point.
                let (anchor, deg_l, deg_c) = if mode == DrawMode::New {
                    (line.points.first().copied().unwrap_or(builder_mid), Some(degrees), None)
                } else {
                    (builder_mid, None, Some(degrees))
                };
                Some((path, color, width, anchor, deg_l, deg_c))
            },
            CanvasWidget::Polygon(pg) => {
                let (path, degrees, mid_point) =
                    build_polygon_path(pg, mode, cursor, edit_mid, edit_other, degrees_override);
                Some((path, color, width, mid_point, None, Some(degrees)))
            },
            CanvasWidget::PolyLine(pl) => {
                let (path, degrees, mid_point) =
                    build_polyline_path(pl, mode, cursor, edit_index, edit_mid, edit_other, degrees_override);
                Some((path, color, width, mid_point, None, Some(degrees)))
            },
            CanvasWidget::RightTriangle(tr) => {
                let (path, degrees, mid_point, _) =
                    build_right_triangle_path(tr, mode, cursor, edit_index, edit_mid, edit_other, degrees_override);
                Some((path, color, width, mid_point, None, Some(degrees)))
            },
            CanvasWidget::FreeHand(fh) => {
                let path = build_free_hand_path(fh, mode, cursor, edit_index);
                Some((path, color, width, Point::default(), None, None))
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Serialize, Deserialize, PartialEq, Eq,)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DrawWidget {
    None,
    Arc,
    Bezier,
    Circle,
    Ellipse,
    Line,
    PolyLine,
    Polygon,
    RightTriangle,
    Text,
    FreeHand,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq,)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DrawMode {
    #[default]
    Display,
    Edit,
    New,
    Rotate,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq,)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DrawStatus {
    Inprogress,
    Completed,
    Delete,
}

// used to display text widget
impl DrawMode {
    pub fn string(&self) -> Option<String> {
        match &self {
            DrawMode::Display => Some("Display".to_string()),
            DrawMode::New => Some("New".to_string()),
            DrawMode::Edit => Some("Edit".to_string()),
            DrawMode::Rotate => Some("Rotate".to_string()),
        }
    }

    pub fn to_enum(s: String) -> Self {
        match s.as_str() {
            "Display" => DrawMode::Display,
            "Edit" => DrawMode::Edit,
            "New" => DrawMode::New,
            "Rotate" => DrawMode::Rotate,
            _ => DrawMode::Display,
        }
    }
    pub fn options() -> Vec<String> {
        vec!["Display".to_string(), "New".to_string(), "Edit".to_string(), "Rotate".to_string(),]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HTextAlignment {
    Left,
    Center,
    Right,
}

impl HTextAlignment {
    pub fn string(&self) -> Option<String> {
        match &self {
            HTextAlignment::Left => Some("H_Left".to_string()),
            HTextAlignment::Center => Some("H_Center".to_string()),
            HTextAlignment::Right => Some("H_Right".to_string()),
        }
    }

    pub fn to_enum(s: String) -> Self {
        match s.as_str() {
            "H_Left" => HTextAlignment::Left,
            "H_Center" => HTextAlignment::Center,
            "H_Right" => HTextAlignment::Right,
            _ => HTextAlignment::Center,
        }
    }
    pub fn options() -> Vec<String> {
        vec!["H_Left".to_string(), "H_Center".to_string(), "H_Right".to_string()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VTextAlignment {
    Top,
    Center,
    Bottom,
}

impl VTextAlignment {
    pub fn string(&self) -> Option<String> {
        match &self {
            VTextAlignment::Top => Some("V_Top".to_string()),
            VTextAlignment::Center => Some("V_Center".to_string()),
            VTextAlignment::Bottom => Some("V_Bottom".to_string()),
        }
    }

    pub fn to_enum(s: String) -> Self {
        match s.as_str() {
            "V_Top" => VTextAlignment::Top,
            "V_Center" => VTextAlignment::Center,
            "V_Bottom" => VTextAlignment::Bottom,
            _ => VTextAlignment::Center,
        }
    }
    pub fn options() -> Vec<String> {
        vec!["V_Top".to_string(), "V_Center".to_string(), "V_Bottom".to_string()]
    }
}


#[derive(Debug)]
pub struct DrawState {
    cache: canvas::Cache,
    text_cache: HashMap<Id, canvas::Cache>,
    pub curves: HashMap<Id, CanvasWidget>,
    pub text_curves: HashMap<Id, CanvasWidget>,
    pub edit_widget_id: Option<Id>,
    pub escape_pressed: bool,
    // user inputs
    pub radio_widget: Option<DrawWidget>,
    pub draw_mode: DrawMode,
    pub draw_color: Color,
    pub canvas_color: Color,
    pub poly_points: usize,
    pub step_degrees: f32,
    pub draw_width: f32,
    pub h_text_alignment: HTextAlignment,
    pub v_text_alignment: VTextAlignment,

    pub timer_event_enabled: bool,
    pub timer_duration: u64,
    pub elapsed_time: u64,
    pub blink: bool,
    pub cursor_blink_ms: u64,
    /// Text submitted from the external TextInput, waiting to be placed.
    pub pending_text: Option<String>,
    /// Canvas position clicked while no text was pending.
    pub pending_anchor: Option<Point>,
}

impl Default for DrawState {
    fn default() -> Self {
        Self { 
            cache: canvas::Cache::new(),
            text_cache: HashMap::new(),
            curves: HashMap::new(),
            text_curves: HashMap::new(),
            edit_widget_id: None,
            escape_pressed: false,
            // user inputs
            draw_mode: DrawMode::Display,
            radio_widget: None,
            draw_color: Color::from_rgb(0.961, 0.871, 0.702),
            canvas_color: Color::from_rgb(0.0, 0.502, 0.502),
            poly_points: 3,
            step_degrees: 6.0,
            draw_width: 2.0,
            h_text_alignment: HTextAlignment::Center,
            v_text_alignment: VTextAlignment::Center,

            timer_event_enabled: false,
            timer_duration: 750,
            elapsed_time: 0,
            blink: false,
            cursor_blink_ms: 250,
            pending_text: None,
            pending_anchor: None,
        }
    }
}

impl DrawState {
    pub fn view<'a>(
        &'a self, 
        curves: &'a HashMap<Id, CanvasWidget>, 
        text_curves: &'a HashMap<Id, CanvasWidget>,
    ) -> Element<'a, CanvasWidget> {

        Canvas::new(DrawPending {
            state: self,
            curves,
            text_curves,
        })
        .width(Fill)
        .height(Fill)
        .into()
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }

    pub fn request_text_redraw(&mut self) {
        for cache in self.text_cache.values_mut() {
            cache.clear();
        }
    }

    /// Ensure a cache entry exists for this text Id, then clear it.
    pub fn request_text_redraw_for(&mut self, id: &Id) {
        self.text_cache.entry(id.clone()).or_insert_with(canvas::Cache::new).clear();
    }

    /// Remove the cache entry for a deleted text widget.
    pub fn remove_text_cache(&mut self, id: &Id) {
        self.text_cache.remove(id);
    }

    /// Seed cache entries for all text_curves currently loaded (e.g. at startup).
    pub fn init_text_caches(&mut self) {
        for id in self.text_curves.keys() {
            self.text_cache.entry(id.clone()).or_insert_with(canvas::Cache::new);
        }
    }
}

struct DrawPending<'a> {
    state: &'a DrawState,
    curves: &'a HashMap<Id, CanvasWidget>,
    text_curves: &'a HashMap<Id, CanvasWidget>,
}

impl<'a> canvas::Program<CanvasWidget> for DrawPending<'a> {
    type State = Option<Pending>;

    fn update(
        &self,
        program_state: &mut Self::State,
        event: &Event,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<CanvasWidget>>{
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return None;
        };
        
        match event {
            Event::Mouse(mouse_event) => {
                if self.state.escape_pressed {
                    *program_state = None;
                    return None
                }
                
                let message = match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        match self.state.draw_mode {
                            DrawMode::Display => {
                                None
                            },
                            DrawMode::New => {
                                match program_state {
                                    // First click in New mode.
                                    // For Text: if pending_text is ready, place immediately;
                                    // otherwise store the anchor and wait for text submit.
                                    // For all other widgets: start the normal multi-click flow.
                                    None => {
                                        if self.state.radio_widget.is_none() {
                                            return None
                                        }

                                        // ── Text widget special path ──────────────────
                                        if self.state.radio_widget == Some(DrawWidget::Text) {
                                            match &self.state.pending_text {
                                                Some(text_content) => {
                                                    // Both text and position are ready — place it.
                                                    let mut selected_widget = add_new_widget(
                                                        DrawWidget::Text,
                                                        self.state.poly_points,
                                                        self.state.draw_color,
                                                        self.state.draw_width,
                                                        self.state.draw_mode,
                                                        self.state.h_text_alignment,
                                                        self.state.v_text_alignment,
                                                    );
                                                    if let CanvasWidget::Text(ref mut txt) = selected_widget {
                                                        txt.content = text_content.clone();
                                                        txt.position = cursor_position;
                                                        txt.status = DrawStatus::Completed;
                                                        txt.draw_mode = DrawMode::Display;
                                                    }
                                                    return complete_new_widget(selected_widget, cursor_position)
                                                        .map(canvas::Action::publish);
                                                },
                                                None => {
                                                    // No text yet — store anchor and wait.
                                                    return Some(canvas::Action::publish(
                                                        CanvasWidget::Text({
                                                            let mut dummy = Text::default();
                                                            dummy.draw_mode = DrawMode::New;
                                                            dummy.status = DrawStatus::Inprogress;
                                                            dummy.position = cursor_position;
                                                            dummy
                                                        })
                                                    ));
                                                }
                                            }
                                        }

                                        // ── All other widgets ─────────────────────────
                                        let selected_widget =
                                            add_new_widget(
                                                self.state.radio_widget.unwrap(),
                                                self.state.poly_points,
                                                self.state.draw_color,
                                                self.state.draw_width,
                                                self.state.draw_mode,
                                                self.state.h_text_alignment,
                                                self.state.v_text_alignment,
                                            );

                                        let (widget, _) =
                                            set_widget_point(
                                                &selected_widget,
                                                cursor_position,
                                            );
                                        *program_state = Some(Pending::New {
                                            widget: widget.clone(),
                                        });
                                        Some(canvas::Action::request_redraw())
                                    },
                                    // Subsequent clicks for non-text widgets.
                                    Some(Pending::New { widget, .. }) => {
                                        let widget_clone = widget.clone();
                                        let (new_widget, completed) =
                                            set_widget_point(&widget_clone, cursor_position);

                                        if completed {
                                            *program_state = None;
                                            complete_new_widget(new_widget, cursor_position).map(canvas::Action::publish)
                                        } else {
                                            *program_state = Some(Pending::New {
                                                widget: new_widget.clone(),
                                            });
                                            Some(canvas::Action::request_redraw())
                                        }
                                    },
                                    _ => None,
                                }
                            },
                            DrawMode::Edit => {
                                match program_state {
                                    // edit consists of 3 clicks
                                    // 1 - find closest widget
                                    // 2 - find closest point
                                    // 3 - finish
                                    None => {
                                        let widget_opt = 
                                            find_closest_widget(self.curves, self.text_curves, cursor_position);
                                        
                                        let selected_widget = 
                                            match widget_opt {
                                                Some(w) => w,
                                                None => return None,
                                            };

                                        // set draw_mode to indicate being edited
                                        let widget = 
                                            set_widget_mode_or_status(
                                                selected_widget, 
                                                Some(DrawMode::Edit),
                                                Some(DrawStatus::Inprogress),
                                            );
                                        // Publish the Inprogress widget so draw_callback
                                        // marks it Inprogress in canvas_states; draw_all
                                        // will then skip it and only the pending overlay
                                        // renders it until editing is finished.
                                        *program_state = Some(Pending::EditSecond {
                                            widget: widget.clone(),
                                        });
                                        Some(canvas::Action::publish(widget))
                                    },
                                    // The second click is a Some() since it was created above
                                    // The pending is carrying the previous info
                                    // This second click will find the point
                                    // and replace with cursor
                                    Some(Pending::EditSecond { 
                                        widget,
                                    }) => {
                                        // Find for closest point to edit in selected widget
                                        // which might be either a mid point(translate) or 
                                        // curve point (move point).
                                        let (point_index, mid_point, other_point) = 
                                            find_closest_point_index(widget, cursor_position);
                                        let widget = widget.clone();
                                        *program_state = Some(Pending::EditThird {
                                            widget: widget.clone(),
                                            edit_point_index: point_index,
                                            edit_mid_point: mid_point,
                                            edit_other_point: other_point,
                                        });
                                        
                                        Some(canvas::Action::request_redraw())
                                    },
                                    // The third click will send back the DrawCurve
                                    // with the finally updated curve
                                    Some(Pending::EditThird { .. }) => {
                                        if let Some(Pending::EditThird { 
                                            widget,
                                            edit_point_index,
                                            edit_mid_point,
                                            edit_other_point, 
                                        }) = program_state.take() {
                                            Some(update_edited_widget(
                                                widget.clone(), 
                                                cursor_position, 
                                                edit_point_index, 
                                                edit_mid_point,
                                                edit_other_point,
                                                DrawStatus::Completed,
                                            )).map(canvas::Action::publish)
                                        } else {
                                            None
                                        }
                                    },
                                    _ => None,
                                }
                            },
                            DrawMode::Rotate => {
                                match program_state {
                                    // rotation consists of 2 clicks
                                    // 1 - find closest widget
                                    //  - move mouse wheel
                                    // 2 - click to finish
                                    None => {
                                        let widget_opt = 
                                            find_closest_widget(self.curves, self.text_curves, cursor_position);
                                        
                                        let selected_widget = 
                                            match widget_opt {
                                                Some(w) => w,
                                                None => return None,
                                            };
                                        
                                        // The widget needs to be in Display initially, 
                                        // in order to display it in pending
                                        // However, the below return of the draw curve 
                                        // the widget need to be in the rotate method.
                                        let widget = 
                                            set_widget_mode_or_status(
                                                selected_widget, 
                                                Some(DrawMode::Rotate),
                                                Some(DrawStatus::Inprogress),
                                            );
                                        
                                        // Publish the Inprogress widget so draw_callback
                                        // marks it Inprogress in canvas_states; draw_all
                                        // will then skip it and only the pending overlay
                                        // renders it until rotation is finished.
                                        *program_state = Some(Pending::Rotate {
                                            widget: widget.clone(),
                                            step_degrees: self.state.step_degrees,
                                            degrees: widget.degrees(),
                                        });

                                        Some(canvas::Action::publish(widget))
                                    },
                                    // After the final rotation completed
                                    Some(Pending::Rotate { .. }) => {
                                        if let Some(Pending::Rotate {
                                            mut widget,
                                            step_degrees: _,
                                            degrees: _,
                                        }) = program_state.take() {
                                            let (rotated_widget, _) = 
                                                update_rotated_widget(
                                                    &mut widget,
                                                    0.0,
                                                    Some(DrawStatus::Completed),
                                                );

                                            Some(rotated_widget).map(canvas::Action::publish)
                                        } else {
                                            None
                                        }
                                    },
                                    _ => None,
                                }
                            },  
                        }
                    },
                    mouse::Event::WheelScrolled { delta} => {
                        match self.state.draw_mode {
                            DrawMode::Rotate => {
                                match program_state {
                                    None => None,
                                    Some(Pending::Rotate { 
                                        widget,
                                        step_degrees,
                                        degrees: _,  
                                    }) => {
                                        let delta = match delta {
                                            mouse::ScrollDelta::Lines { x:_, y } => y,
                                            mouse::ScrollDelta::Pixels { x:_, y } => y,
                                        };

                                        // Setting the widget draw_mode at each mouse wheel
                                        // since it was set to Display initially.
                                        // Otherwise needed to have another pending type
                                        // and duplicate a lot of code.  Had to clone anyway.
                                        let (widget, degrees) = 
                                            update_rotated_widget(
                                                widget, 
                                                *step_degrees*delta,
                                                None, 
                                            );
                                        
                                        *program_state = Some(Pending::Rotate{
                                            widget,
                                            step_degrees: *step_degrees,
                                            degrees: Some(degrees),
                                        });
                                        Some(canvas::Action::request_redraw())
                                    },
                                    _ => None,
                                }
                            },
                            _ => None,
                        }
                    },
                    mouse::Event::CursorMoved { .. } if program_state.is_some() => {
                        Some(canvas::Action::request_redraw())
                    },
                    _ => None,
                };
                
                message
            },
            Event::Keyboard(key_event) => {
                let message = match key_event {
                    iced::keyboard::Event::KeyPressed { 
                        key:_, 
                        modified_key, 
                        physical_key:_, 
                        location:_, 
                        modifiers:_, 
                        text:_,
                        repeat:_, } => {
                            match program_state {
                                None => None,
                                Some(Pending::New { .. }) => {
                                    if let Some(Pending::New { mut widget }) = program_state.take() {
                                        let (widget, completed) = 
                                            add_keypress(&mut widget, modified_key);
                                        match widget {
                                            Some(widget) => {
                                                if completed {
                                                    // FreeHand Enter: publish the finished widget.
                                                    Some(widget)
                                                } else {
                                                    *program_state = Some(Pending::New { widget });
                                                    return Some(canvas::Action::request_redraw());
                                                }
                                            },
                                            None => None,
                                        }
                                    } else {
                                        None
                                    }
                                },
                                Some(Pending::EditSecond { .. }) => {
                                    if let Some(Pending::EditSecond { widget }) = program_state.take() {
                                        let del_key = get_del_key(modified_key);
                                        let del_widget = if del_key {
                                            set_widget_mode_or_status(
                                                widget.clone(), 
                                                None, 
                                                Some(DrawStatus::Delete),
                                            )
                                        } else {
                                            widget.clone()
                                        };
                                        Some(del_widget)
                                    } else {
                                        None
                                    }
                                },
                                _ => None,
                            }
                        },
                    iced::keyboard::Event::KeyReleased {key: _, location:_, modifiers:_, modified_key: _, physical_key: _ } => None,
                    iced::keyboard::Event::ModifiersChanged(_) => None,
                };

                message.map(canvas::Action::publish)
            },
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        
        let content =
            self.state.cache.draw(renderer, bounds.size(), 
                            |frame| {

                let background = Path::rectangle(Point::ORIGIN, frame.size());
                frame.fill(&background, self.state.canvas_color);

                DrawCurve::draw_all(self.curves, frame, theme);

                frame.stroke(
                    &Path::rectangle(Point::ORIGIN, frame.size()),
                    Stroke::default()
                        .with_width(2.0)
                        .with_color(theme.palette().background.base.text),
                );
            });

        let mut text_content = vec![];
        for (id, text_curve) in self.text_curves.iter() {
            // Retrieve the cache for this text widget (may be absent if just loaded).
            if let Some(cache) = self.state.text_cache.get(id) {
                text_content.push(cache.draw(renderer, bounds.size(), |frame| {
                    DrawCurve::draw_text(text_curve, self.state.blink, frame, theme, renderer);
                }));
            }
        }
            

        if let Some(pending) = state {
            let mut content = vec![content, pending.draw(renderer, theme, bounds, cursor)];
            content.append(&mut text_content);
            content
        } else if let Some(preview_text) = &self.state.pending_text {
            // Text content submitted, waiting for an anchor click — follow cursor.
            let mut frame = Frame::new(renderer, bounds.size());
            if let Some(cursor_pos) = cursor.position_in(bounds) {
                let txt = Text {
                    id: Id::unique(),
                    content: preview_text.clone(),
                    position: cursor_pos,
                    color: self.state.draw_color,
                    size: Pixels(16.0),
                    line_height: LineHeight::Relative(1.2),
                    font: Default::default(),
                    align_x: iced_h_text_alignment(self.state.h_text_alignment).into(),
                    align_y: iced_v_text_alignment(self.state.v_text_alignment),
                    shaping: Shaping::Basic,
                    degrees: 0.0,
                    draw_mode: DrawMode::New,
                    status: DrawStatus::Inprogress,
                };
                draw_text_in_frame(&txt, &mut frame, DrawMode::New, cursor_pos, 0.0, false, renderer);
            }
            let mut content = vec![content, frame.into_geometry()];
            content.append(&mut text_content);
            content
        } else {
            let mut content = vec![content];
            content.append(&mut text_content);
            content
        }

    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}


#[derive(Debug, Clone, Default)]
pub struct DrawCurve {
}

impl DrawCurve {
    fn draw_all(curves: &HashMap<Id, CanvasWidget>, frame: &mut Frame, _theme: &Theme) {
        // Redraws only when the cache is cleared (i.e. a shape is completed/deleted).
        // Inprogress widgets are skipped — the pending overlay renders them instead.
        for (_id, widget) in curves.iter() {
            if let Some((path, color, width)) = widget.display_path() {
                frame.stroke(&path, Stroke::default().with_width(width).with_color(color));
            }
        }
    }

    fn draw_text(text_curve: &CanvasWidget, mut blink: bool, frame: &mut Frame, _theme: &Theme, renderer: &Renderer) {

        let (path, color, width) = 
            match &text_curve {
                CanvasWidget::Text(txt) => {
                    // During edit or rotate, pending draws the text,
                    // so skip drawing here.
                    if txt.draw_mode == DrawMode::Display || 
                        txt.draw_mode == DrawMode::New {
                        if txt.draw_mode == DrawMode::Display {
                            blink = false;
                        }
                        frame.translate(Vector::new(txt.position.x, txt.position.y));
                        let (text, path) = 
                            build_text_path (
                                txt,
                                txt.draw_mode,
                                blink,
                                renderer,
                            );
                        frame.rotate(to_radians(&txt.degrees));
                        frame.fill_text(text);
                        
                        (path, Some(txt.color), Some(1.0))
                    } else {
                        (None, None, None)
                    }
                },
                _ => (None, None, None)
            };

            if let Some(path) = path { frame.stroke(
                &path,
                Stroke::default()
                .with_width(width.unwrap())
                .with_color(color.unwrap()),
                ) }
        
    }
}



/// Draw degree-angle labels near `anchor` on the pending overlay frame.
fn draw_degrees_labels(
    frame: &mut Frame,
    anchor: Point,
    degrees_left: Option<f32>,
    degrees_center: Option<f32>,
) {
    if let Some(d) = degrees_left {
        frame.fill_text(canvas::Text {
            position: Point::new(anchor.x - 30.0, anchor.y - 10.0),
            color: Color::WHITE,
            size: 10.0.into(),
            content: format!("{:.1}", d),
            align_x: iced::advanced::text::Alignment::Center,
            align_y: iced::alignment::Vertical::Center,
            ..canvas::Text::default()
        });
    }
    if let Some(d) = degrees_center {
        frame.fill_text(canvas::Text {
            position: Point::new(anchor.x - 10.0, anchor.y - 20.0),
            color: Color::WHITE,
            size: 10.0.into(),
            content: format!("{:.1}", d),
            align_x: iced::advanced::text::Alignment::Center,
            align_y: iced::alignment::Vertical::Center,
            ..canvas::Text::default()
        });
    }
}

/// Render a text widget directly onto `frame` (used by the pending overlay).
fn draw_text_in_frame(
    txt: &Text,
    frame: &mut Frame,
    mode: DrawMode,
    position: Point,
    degrees: f32,
    blink: bool,
    renderer: &Renderer,
) {
    frame.translate(Vector::new(position.x, position.y));
    let (text, path) = build_text_path(txt, mode, blink, renderer);
    frame.rotate(to_radians(&degrees));
    frame.fill_text(text.clone());
    if let Some(p) = path {
        frame.stroke(&p, Stroke::default().with_width(2.0).with_color(text.color));
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Pending {
    New {
        widget: CanvasWidget,
    },
    EditSecond {
        widget: CanvasWidget, 
        },
    EditThird {
        widget: CanvasWidget, 
        edit_point_index: Option<usize>,
        edit_mid_point: bool,
        edit_other_point: bool,
        },
    Rotate {
        widget: CanvasWidget,
        step_degrees: f32,
        degrees: Option<f32>,
    },
}

impl Pending {
    fn draw(
        &self,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Geometry {
        let _ = theme;
        let mut frame = Frame::new(renderer, bounds.size());

        if let Some(cursor) = cursor.position_in(bounds) {
            match self {
                Pending::New { widget } => {
                    match widget {
                        CanvasWidget::Text(txt) => {
                            // Show preview: if anchor already set, render there;
                            // otherwise follow cursor so user sees where text will land.
                            let pos = if txt.position == Point::default() { cursor } else { txt.position };
                            draw_text_in_frame(txt, &mut frame, DrawMode::New, pos, txt.degrees, false, renderer);
                        },
                        _ => {
                            if let Some((path, color, width, anchor, deg_l, deg_c)) =
                                widget.pending_path(DrawMode::New, Some(cursor), None, false, false, None)
                            {
                                draw_degrees_labels(&mut frame, anchor, deg_l, deg_c);
                                frame.stroke(&path, Stroke::default().with_width(width).with_color(color));
                            }
                        }
                    }
                },
                Pending::EditSecond { widget } => {
                    match widget {
                        CanvasWidget::Text(txt) => {
                            draw_text_in_frame(txt, &mut frame, DrawMode::Edit, txt.position, txt.degrees, false, renderer);
                        },
                        _ => {
                            if let Some((path, color, width, ..)) =
                                widget.pending_path(DrawMode::Edit, Some(cursor), None, false, false, None)
                            {
                                frame.stroke(&path, Stroke::default().with_width(width).with_color(color));
                            }
                        }
                    }
                },
                Pending::EditThird {
                    widget,
                    edit_point_index,
                    edit_mid_point,
                    edit_other_point,
                } => {
                    match widget {
                        CanvasWidget::Text(txt) => {
                            draw_text_in_frame(txt, &mut frame, DrawMode::Edit, cursor, txt.degrees, false, renderer);
                        },
                        _ => {
                            if let Some((path, color, width, anchor, deg_l, deg_c)) =
                                widget.pending_path(DrawMode::Edit, Some(cursor), *edit_point_index, *edit_mid_point, *edit_other_point, None)
                            {
                                draw_degrees_labels(&mut frame, anchor, deg_l, deg_c);
                                frame.stroke(&path, Stroke::default().with_width(width).with_color(color));
                            }
                        }
                    }
                },
                Pending::Rotate { widget, step_degrees: _, degrees } => {
                    match widget {
                        CanvasWidget::Text(txt) => {
                            draw_text_in_frame(txt, &mut frame, DrawMode::Rotate, txt.position, degrees.unwrap_or(txt.degrees), false, renderer);
                        },
                        _ => {
                            if let Some((path, color, width, anchor, deg_l, deg_c)) =
                                widget.pending_path(widget.draw_mode(), None, None, false, false, *degrees)
                            {
                                draw_degrees_labels(&mut frame, anchor, deg_l, deg_c);
                                frame.stroke(&path, Stroke::default().with_width(width).with_color(color));
                            }
                        }
                    }
                },
            };
        }

        frame.into_geometry()
    }
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub radius: f32,
    pub color: Color,
    pub width: f32,
    pub start_angle: Radians,
    pub end_angle: Radians,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct Bezier {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub id: Id,
    pub center: Point,
    pub circle_point: Point,
    pub radius: f32,
    pub color: Color,
    pub width: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub id: Id,
    pub points: Vec<Point>,
    pub center: Point,
    pub radii: Vector,
    pub rotation: Radians,
    pub color: Color,
    pub width: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct PolyLine {
    pub id: Id,
    pub points: Vec<Point>,
    pub poly_points: usize,
    pub mid_point: Point,
    pub pl_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub id: Id,
    pub points: Vec<Point>,
    pub poly_points: usize,
    pub mid_point: Point,
    pub pg_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct RightTriangle {
    pub id: Id,
    pub points: Vec<Point>,
    pub mid_point: Point,
    pub tr_point: Point,
    pub color: Color,
    pub width: f32,
    pub degrees: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub id: Id,
    pub content: String,
    pub position: Point,
    pub color: Color,
    pub size: Pixels,
    pub line_height: LineHeight,
    pub font: Font,
    pub align_x: iced::advanced::text::Alignment,
    pub align_y: iced::alignment::Vertical,
    pub shaping: Shaping,
    pub degrees: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            id: Id::unique(),
            content: String::new(),
            position: Point::default(),
            color: Color::BLACK,
            size: Pixels(16.0),
            line_height: LineHeight::Relative(1.2),
            font: Font::default(),
            align_x: iced::advanced::text::Alignment::Left,
            align_y: iced::alignment::Vertical::Top,
            shaping: Shaping::Basic,
            degrees: 0.0,
            draw_mode: DrawMode::New,
            status: DrawStatus::Inprogress,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FreeHand {
    pub id: Id,
    pub points: Vec<Point>,
     pub color: Color,
    pub width: f32,
    pub draw_mode: DrawMode,
    pub status: DrawStatus,
    pub completed: bool,
}


fn add_new_widget(widget: DrawWidget, 
                    poly_points: usize, 
                    color: Color,
                    width: f32,
                    draw_mode: DrawMode,
                    h_alignment: HTextAlignment,
                    v_alignment: VTextAlignment,
                    ) 
                    -> CanvasWidget {
    match widget {
        DrawWidget::None => {
            CanvasWidget::None
        },
        DrawWidget::Arc => {
            CanvasWidget::Arc(
                Arc {
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    radius: 0.0,
                    color,
                    width,
                    start_angle: Radians::PI,
                    end_angle: Radians::PI,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                })
        
        },
        DrawWidget::Bezier => {
            CanvasWidget::Bezier(
                Bezier { 
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    color, 
                    width, 
                    degrees: 0.0, 
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
        DrawWidget::Circle => {
            CanvasWidget::Circle(
                Circle {
                    id: Id::unique(),
                    center: Point::default(),
                    circle_point: Point::default(),
                    radius: 0.0,
                    color,
                    width,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
        DrawWidget::Ellipse => {
            CanvasWidget::Ellipse(
                Ellipse {
                    id: Id::unique(),
                    points: vec![],
                    center: Point::default(),
                    radii: Vector{x: 0.0, y: 0.0},
                    rotation: Radians(0.0),
                    color,
                    width,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
        DrawWidget::Line => {
            CanvasWidget::Line(
                Line {
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
        DrawWidget::PolyLine => {
            CanvasWidget::PolyLine(
                PolyLine {
                    id: Id::unique(),
                    points: vec![],
                    poly_points,
                    mid_point: Point::default(),
                    pl_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
        DrawWidget::Polygon => {
            CanvasWidget::Polygon(
                Polygon {
                    id: Id::unique(),
                    points: vec![],
                    poly_points,
                    mid_point: Point::default(),
                    pg_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
        DrawWidget::RightTriangle => {
            CanvasWidget::RightTriangle(
                RightTriangle {
                    id: Id::unique(),
                    points: vec![],
                    mid_point: Point::default(),
                    tr_point: Point::default(),
                    color,
                    width,
                    degrees: 0.0,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
        DrawWidget::FreeHand => {
            CanvasWidget::FreeHand(
                FreeHand {
                    id: Id::unique(),
                    points: vec![],
                    color,
                    width,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                    completed: false,
                }
            )
        }
        DrawWidget::Text => {
            let h_align = iced_h_text_alignment(h_alignment);
            let v_align = iced_v_text_alignment(v_alignment);
            CanvasWidget::Text(
                Text {
                    id: Id::unique(),
                    content: String::new(),
                    position: Point::default(),
                    color,
                    size: Pixels(16.0),
                    line_height: LineHeight::Relative(1.2),
                    font: Default::default(),
                    align_x: h_align.into(),
                    align_y: v_align.into(),
                    shaping: Shaping::Basic,
                    degrees: 0.0,
                    draw_mode,
                    status: DrawStatus::Inprogress,
                }
            )
        },
    }
}

fn complete_new_widget(widget: CanvasWidget, cursor: Point) -> Option<CanvasWidget> {
    match widget {
        CanvasWidget::None => {
            None
        },
        CanvasWidget::Arc(arc) => {
            Some(CanvasWidget::Arc(arc))
        },
        CanvasWidget::Bezier(mut bz) => {
            bz.mid_point = 
                get_mid_point(
                    bz.points[0], 
                    bz.points[1]
                );

            Some(CanvasWidget::Bezier(bz))
        },
        CanvasWidget::Circle(cir) => { 
            Some(CanvasWidget::Circle(cir))
        },
        CanvasWidget::Ellipse(mut ell) => {
            ell.center = ell.points[0];
            let vx = ell.points[1].distance(ell.center);
            let vy = cursor.distance(ell.center);
            ell.radii = Vector{ x: vx, y: vy };
            
            // Calculate rotation based on cursor position
            let rotation_degrees = get_horizontal_angle_of_vector(ell.center, cursor);
            ell.rotation = Radians(to_radians(&rotation_degrees));
            
            Some(CanvasWidget::Ellipse(ell))
        },
        CanvasWidget::Line(mut ln) => {
            // degree is angle rotation around mid point 
            let degrees = 
                get_horizontal_angle_of_vector(
                    ln.points[0],
                    ln.points[1], 
                );
            ln.degrees = degrees;

            Some(CanvasWidget::Line(ln))
        },
        CanvasWidget::Polygon(mut pg) => {
            pg.pg_point = cursor;
            let degrees = 
                get_horizontal_angle_of_vector(
                    pg.mid_point, 
                    cursor, 
                    );

            pg.degrees = degrees;
            pg.points = 
                build_polygon(
                    pg.mid_point, 
                    pg.pg_point, 
                    pg.poly_points,
                    pg.degrees,
                );
            
            Some(CanvasWidget::Polygon(pg))
        },
        CanvasWidget::PolyLine(mut pl) => {
            let (slope, intercept) =
                get_linear_regression(&pl.points);
            
            let line = 
                get_line_from_slope_intercept(
                    &pl.points, 
                    slope, 
                    intercept
                );
            pl.mid_point = 
                get_mid_point(
                    line.0, 
                    line.1);
            pl.pl_point = 
                Point::new(
                    pl.mid_point.x + 100.0, 
                    pl.mid_point.y
                );
            pl.degrees = 
                get_horizontal_angle_of_vector(
                    pl.mid_point,
                    pl.pl_point,
                );
            
            Some(CanvasWidget::PolyLine(pl))
        },
        CanvasWidget::RightTriangle(mut tr) => {
            tr.mid_point = tr.points[1];
            let trans_pts = translate_geometry(&tr.points, Point::default(), tr.points[1]);
            let opp = Point::new(-trans_pts[2].x, -trans_pts[2].y);
            tr.tr_point = Point::new(opp.x+tr.points[1].x, opp.y+tr.points[1].y);
            if tr.points[1].x > tr.points[2].x {
                tr.degrees = 180.0;
            } else {
                tr.degrees = 0.0;
            }
            
            Some(CanvasWidget::RightTriangle(tr))
        },
        CanvasWidget::FreeHand(mut fh) => {
            fh.points.push(cursor);
            Some(CanvasWidget::FreeHand(fh))
        }
        CanvasWidget::Text(mut txt) => {
            txt.degrees = 0.0;
            txt.status = DrawStatus::Completed;
            Some(CanvasWidget::Text(txt))
        }
    }
}

/// Build a completed Text widget from external inputs and place it at `position`.
/// Called by `process_draw_updates` when both text content and anchor are ready.
pub fn build_placed_text_widget(
    content: String,
    position: Point,
    color: Color,
    h_alignment: HTextAlignment,
    v_alignment: VTextAlignment,
) -> CanvasWidget {
    let h_align = iced_h_text_alignment(h_alignment);
    let v_align = iced_v_text_alignment(v_alignment);
    CanvasWidget::Text(Text {
        id: Id::unique(),
        content,
        position,
        color,
        size: Pixels(16.0),
        line_height: LineHeight::Relative(1.2),
        font: Default::default(),
        align_x: h_align.into(),
        align_y: v_align.into(),
        shaping: Shaping::Basic,
        degrees: 0.0,
        draw_mode: DrawMode::Display,
        status: DrawStatus::Completed,
    })
}

fn update_edited_widget(widget: CanvasWidget,                        cursor: Point, 
                        index: Option<usize>, 
                        mid_point: bool,
                        other_point: bool,
                        status: DrawStatus,
                    ) -> CanvasWidget {
    match widget {
        CanvasWidget::None => {
            CanvasWidget::None
        },
        CanvasWidget::Arc(mut arc) => {
            if index.is_some() {
                arc.points[index.unwrap()] = cursor;
                if index.unwrap() == 1 {
                    arc.radius = arc.mid_point.distance(cursor);
                    arc.start_angle = get_angle_of_vectors(
                            arc.mid_point, 
                            Point::new(-arc.mid_point.x, arc.mid_point.y), 
                            cursor) + Radians::PI;
                    arc.end_angle = 
                            get_angle_of_vectors(
                                arc.mid_point, 
                                cursor, 
                                arc.points[2]) + arc.start_angle;
                }
                if index.unwrap() == 2 {
                    arc.end_angle = 
                        get_angle_of_vectors(
                            arc.points[0], 
                            arc.points[1], 
                            cursor
                        ) + arc.start_angle;
                }
                // calc the end_angle point        
                let r = arc.radius;
                let b = arc.end_angle.0;
                let point_b = Point::new(r*b.cos(), r*b.sin());
                arc.points[2] = translate_geometry(&[point_b], arc.mid_point, Point::default())[0];

            } else if mid_point {
                arc.points = 
                    translate_geometry(
                        &arc.points, 
                        cursor,
                        arc.mid_point, 
                        );
                arc.mid_point = cursor;
            }
            arc.status = status;
            CanvasWidget::Arc(arc)
        },
        CanvasWidget::Bezier(mut bz) => {
            if index.is_some() {
                bz.points[index.unwrap()] = cursor;
                bz.mid_point = get_mid_point(bz.points[0], bz.points[1]);
            } else if mid_point {
                bz.points = 
                    translate_geometry(
                        &bz.points, 
                        cursor,
                        bz.mid_point, 
                        );
                bz.mid_point = cursor;
            }
            let degrees = 
                get_horizontal_angle_of_vector(
                    bz.points[0],
                    bz.points[1], 
                );
            bz.degrees = degrees;
            bz.status = status;
            CanvasWidget::Bezier(bz)
        },
        CanvasWidget::Circle(mut cir) => {
            if index.is_some() {
                cir.circle_point = cursor;
                cir.radius = cir.center.distance(cursor);
            } else if mid_point {
                let mut points = vec![cir.circle_point];
                points = 
                    translate_geometry(
                        &points, 
                        cursor,
                        cir.center,
                    );
                cir.center = cursor;
                cir.circle_point = points[0];
            }
            cir.status = status;
            CanvasWidget::Circle(cir)
        },
        CanvasWidget::Ellipse(mut ell) => {
           if mid_point {
                let points = 
                    translate_geometry(
                        &ell.points, 
                        cursor,
                        ell.center,
                    );
                ell.center = cursor;
                ell.points = points;
            }
            if index == Some(1) {
                let p1 = Point::new(cursor.x, ell.center.y);
                let vx = p1.distance(ell.center);
                let vy = ell.points[2].distance(ell.center);
                ell.points[1] = p1;
                ell.radii = Vector{ x: vx, y: vy };
                // Recalculate rotation based on new point position
                let rotation_degrees = get_horizontal_angle_of_vector(ell.center, cursor);
                ell.rotation = Radians(to_radians(&rotation_degrees));
            } else if index == Some(2) {
                let p2 = Point::new(ell.center.x, cursor.y);
                let vx = ell.points[1].distance(ell.center);
                let vy = p2.distance(ell.center);
                ell.points[2] = p2;
                ell.radii = Vector{ x: vx, y: vy };
                // Recalculate rotation based on new point position
                let rotation_degrees = get_horizontal_angle_of_vector(ell.center, cursor);
                ell.rotation = Radians(to_radians(&rotation_degrees));
            }

            ell.status = status;
            CanvasWidget::Ellipse(ell)
        },
        CanvasWidget::Line(mut line) => {
            if index.is_some() {
                line.points[index.unwrap()] = cursor;
                line.mid_point = get_mid_point(line.points[0], line.points[1]);
            } else if mid_point {
                line.points = 
                    translate_geometry(
                        &line.points, 
                        cursor,
                        line.mid_point, 
                        );
                line.mid_point = cursor;
            }

            let degrees = 
                get_horizontal_angle_of_vector(
                    line.points[0],  
                    line.points[1], 
                );
            line.degrees = degrees;
            line.status = status;
            CanvasWidget::Line(line)
        },
        CanvasWidget::Polygon(mut pg) => {
            if other_point {
                pg.pg_point = cursor;
                pg.degrees = get_horizontal_angle_of_vector(pg.mid_point, cursor);
                pg.points = 
                    build_polygon(
                        pg.mid_point, 
                        pg.pg_point, 
                        pg.poly_points,
                        pg.degrees,
                );
            } else if mid_point {
                let trans_pts = 
                    translate_geometry(
                        &[pg.pg_point], 
                        cursor,
                        pg.mid_point, 
                    );
                pg.points = 
                    build_polygon(
                        cursor, 
                        trans_pts[0], 
                        pg.poly_points,
                        pg.degrees,
                    );
                pg.mid_point = cursor;
                pg.pg_point = trans_pts[0];
            }
            pg.status = status;
            CanvasWidget::Polygon(pg)
        },
        CanvasWidget::PolyLine(mut pl) => {
            if index.is_some() {
                pl.points[index.unwrap()] = cursor;
                let mid_point = 
                    get_mid_geometry(
                        &pl.points, 
                        DrawWidget::PolyLine
                    );
                pl.pl_point = 
                    translate_geometry(
                        &[pl.pl_point], 
                        mid_point, 
                        pl.mid_point
                    )[0];
                pl.mid_point = mid_point;
                pl.degrees = 
                    get_horizontal_angle_of_vector(
                        pl.mid_point, 
                        pl.pl_point
                    );
            }  else if mid_point {
                let mut pts = pl.points.clone();
                pts.push(pl.pl_point);
                pts = 
                    translate_geometry(
                        &pts, 
                        cursor,
                        pl.mid_point, 
                    );
                pl.mid_point = cursor;
                pl.pl_point = pts.pop().unwrap();
                pl.points = pts;
            } else if other_point {
                let degrees = get_horizontal_angle_of_vector(pl.mid_point, cursor);
                let step_degrees = degrees-pl.degrees;
                pl.points = rotate_geometry(&pl.points, &pl.mid_point, &step_degrees, DrawWidget::PolyLine);
                pl.pl_point = cursor;
                pl.degrees = degrees;
            }
            pl.status = status;
            CanvasWidget::PolyLine(pl)
        },
        CanvasWidget::RightTriangle(mut tr) => {
            if index.is_some() {
                let index = index.unwrap();
                if index == 0 {
                    tr.points[index].y = cursor.y;
                }
                if index == 1 {
                    tr.points[1].y = cursor.y;
                    tr.points[2].y = cursor.y;
                }
                if index == 2 {
                    tr.points[2].x = cursor.x;
                }
                let mid = get_mid_point(tr.points[1], tr.points[2]);
                let dist_b_mid = Point::new(mid.x-tr.points[2].x, mid.y-tr.points[2].y);
                tr.tr_point = Point::new(tr.points[2].x+dist_b_mid.x, tr.points[2].y+dist_b_mid.y);
            } else if mid_point {
                let mut pts = tr.points.clone();
                pts.push(tr.tr_point);
                pts = 
                    translate_geometry(
                        &pts, 
                        cursor,
                        tr.mid_point, 
                    );
                tr.mid_point = cursor;
                tr.tr_point = pts.pop().unwrap();
                tr.points = pts;
            } else if other_point {
                let degrees = get_horizontal_angle_of_vector(tr.mid_point, cursor);
                let step_degrees = degrees-tr.degrees;
                tr.points = rotate_geometry(&tr.points, &tr.mid_point, &step_degrees, DrawWidget::RightTriangle);
                tr.tr_point = cursor;
                tr.degrees = degrees;
            }
            tr.status = status;
            CanvasWidget::RightTriangle(tr)
        },
        CanvasWidget::FreeHand(mut fh) => {
            if index.is_some() {
                fh.points[index.unwrap()] = cursor;
            }
            fh.status = status;
            CanvasWidget::FreeHand(fh)
        },
        CanvasWidget::Text(mut txt) => {
            txt.position = cursor;
            txt.status = status;
            txt.draw_mode = DrawMode::Display;
            CanvasWidget::Text(txt)
        }
    }
}

fn update_rotated_widget(widget: &mut CanvasWidget, 
                        step_degrees: f32,
                        status: Option<DrawStatus>,
                    ) -> (CanvasWidget, f32) {
    match widget {
        CanvasWidget::None => (CanvasWidget::None, 0.0),
        CanvasWidget::Arc(arc) => {
            arc.points = rotate_geometry(&arc.points, &arc.mid_point, &step_degrees, DrawWidget::Arc);
            arc.start_angle = 
                get_angle_of_vectors(
                    arc.points[0], 
                    Point::new(-arc.points[0].x, arc.points[0].y), 
                    arc.points[1]) + Radians::PI;
            arc.end_angle = 
                get_angle_of_vectors(
                    arc.points[0], 
                    arc.points[1], 
                    arc.points[2]) + arc.start_angle;

            // calc the end_angle point        
            let r = arc.radius;
            let b = arc.end_angle.0;
            let point_b = Point::new(r*b.cos(), r*b.sin());

            arc.points[2] = translate_geometry(&[point_b], arc.mid_point, Point::default())[0];
            
            if status.is_some() {
                arc.status = status.unwrap();
            }
            (CanvasWidget::Arc(arc.clone()), Radians::into(arc.start_angle))
        },
        CanvasWidget::Bezier(bz) => {
            bz.points = rotate_geometry(&bz.points, &bz.mid_point, &step_degrees, DrawWidget::Bezier);
            bz.degrees = get_horizontal_angle_of_vector(bz.mid_point, bz.points[1]);
            if status.is_some() {
                bz.status = status.unwrap();
            }
            (CanvasWidget::Bezier(bz.clone()), bz.degrees)
        },
        CanvasWidget::Circle(cir) => {
            (CanvasWidget::Circle(cir.clone()), 0.0)
        },
        CanvasWidget::Ellipse(ell) => {
            let rads = to_radians(&step_degrees) + ell.rotation.0;
            ell.rotation = Radians(rads);
            if status.is_some() {
                ell.status = status.unwrap();
            }
            (CanvasWidget::Ellipse(ell.clone()), to_degrees(&rads))
        },
        CanvasWidget::Line(ln) => {
            ln.points = rotate_geometry(&ln.points, &ln.mid_point, &step_degrees, DrawWidget::Line);
            ln.degrees = get_horizontal_angle_of_vector(ln.mid_point, ln.points[1]);
            if status.is_some() {
                ln.status = status.unwrap();
            }
            (CanvasWidget::Line(ln.clone()), ln.degrees)
        },
        CanvasWidget::Polygon(pg) => {
            pg.points = rotate_geometry(&pg.points, &pg.mid_point, &step_degrees, DrawWidget::Polygon);
            pg.pg_point = rotate_geometry(&[pg.pg_point], &pg.mid_point, &step_degrees, DrawWidget::Line)[0];
            pg.degrees = get_horizontal_angle_of_vector(pg.mid_point, pg.pg_point);
            if status.is_some() {
                pg.status = status.unwrap();
            }
            (CanvasWidget::Polygon(pg.clone()), pg.degrees)
        },
        CanvasWidget::PolyLine(pl) => {
            let mut pts = pl.points.clone();
            pts.push(pl.pl_point);
            pts = rotate_geometry(&pts, &pl.mid_point, &step_degrees, DrawWidget::PolyLine);
            pl.pl_point = pts.pop().unwrap();
            pl.points = pts;
            pl.degrees = get_horizontal_angle_of_vector(pl.mid_point, pl.pl_point);
            if status.is_some() {
                pl.status = status.unwrap();
            }
            (CanvasWidget::PolyLine(pl.clone()), pl.degrees)
        },
        CanvasWidget::RightTriangle(tr) => {
            let mut pts = tr.points.clone();
            pts.push(tr.tr_point);
            pts = rotate_geometry(&pts, &tr.mid_point, &step_degrees, DrawWidget::RightTriangle);
            tr.tr_point = pts.pop().unwrap();
            tr.points = pts;
            tr.degrees = get_horizontal_angle_of_vector(tr.mid_point, tr.tr_point);
            if status.is_some() {
                tr.status = status.unwrap();
            }
            (CanvasWidget::RightTriangle(tr.clone()), tr.degrees)
        },
        CanvasWidget::FreeHand(fh) => {
            (CanvasWidget::FreeHand(fh.clone()), 0.0)
        },
        CanvasWidget::Text(txt) => {
            txt.degrees += step_degrees;
            if status.is_some() {
                txt.status = status.unwrap();
            }
            (CanvasWidget::Text(txt.clone()), txt.degrees)
        }
    }
}

fn add_keypress(widget: &mut CanvasWidget, modified: &Key) -> (Option<CanvasWidget>, bool) {
    match widget {
        CanvasWidget::FreeHand(fh) => {
            if let Key::Named(named) = modified.as_ref() {
                if named == iced::keyboard::key::Named::Enter {
                    fh.completed = true;
                }
            }
            
           (Some(CanvasWidget::FreeHand(fh.clone())), fh.completed)
            
        }
        _ => (None, false)
    }
}

fn get_del_key(modified: &Key) -> bool {
    match modified.as_ref() {
        Key::Named(named) => {
            matches!(named, iced::keyboard::key::Named::Delete)
        },
        _ => false,
    }
}

pub fn set_widget_mode_or_status(
    mut widget: CanvasWidget,
    mode: Option<DrawMode>,
    status: Option<DrawStatus>,
) -> CanvasWidget {
    if let Some(m) = mode   { widget.set_mode(m); }
    if let Some(s) = status { widget.set_status(s); }
    widget
}

// Adds a cursor position to the points then determines 
// if finish by returning the widget and the boolean
fn set_widget_point(widget: &CanvasWidget, cursor: Point) -> (CanvasWidget, bool) {
    match widget {
        CanvasWidget::None => (CanvasWidget::None, true),
        CanvasWidget::Arc(arc) => {
            let mut arc = arc.clone();
            arc.points.push(cursor);

            let finished = match arc.points.len() {
                1 => {
                    arc.mid_point = arc.points[0];
                    false
                },
                2 => {
                    arc.radius = arc.points[0].distance(arc.points[1]);
                    arc.start_angle = 
                        get_angle_of_vectors(
                            arc.points[0], 
                            Point::new(-arc.points[0].x, arc.points[0].y), 
                            arc.points[1]) + Radians::PI;
                    false
                },
                3 => {
                    arc.end_angle = 
                        get_angle_of_vectors(
                            arc.points[0], 
                            arc.points[1], 
                            cursor) + arc.start_angle;
                    // calc the end_angle point        
                    let r = arc.radius;
                    let b = arc.end_angle.0;
                    let point_b = Point::new(r*b.cos(), r*b.sin());
                    arc.points[2] = translate_geometry(&[point_b], arc.mid_point, Point::default())[0];
                    true
                },
                _ => false
            };

            (CanvasWidget::Arc(arc), finished)
        },
        CanvasWidget::Bezier(bezier) => {
            let mut bz = bezier.clone();
            let mut finished = false;
            bz.points.push(cursor);

            if bz.points.len() == 2 {
                bz.degrees = get_horizontal_angle_of_vector(bz.points[0], bz.points[1]);
            }
            if bz.points.len() == 3 {
                finished = true;
            }
            
            (CanvasWidget::Bezier(bz), finished)
        },
        CanvasWidget::Circle(circle) => {
            let mut cir = circle.clone();
            let finished = if cir.center == Point::default() {
                cir.center = cursor;
                false
            } else {
                cir.radius = cir.center.distance(cursor);
                cir.circle_point = cursor;
                true
            };
            
            (CanvasWidget::Circle(cir), finished)
        },
        CanvasWidget::Ellipse(ell) => {
            let mut ell = ell.clone();
            let finished = if ell.points.len() < 2 {
                ell.points.push(cursor);
                false
            } else if ell.points.len() == 2 {
                ell.points.push(cursor);
                true
            } else {
                false
            };
            
            (CanvasWidget::Ellipse(ell), finished)
        },
        CanvasWidget::Line(line) => {
            let mut ln = line.clone();
            ln.points.push(cursor);

            let finished = if ln.points.len() == 2 {
                ln.mid_point = get_mid_point(ln.points[0], ln.points[1]);
                true
            } else {
                false
            };
            
            (CanvasWidget::Line(ln), finished)
        },
        CanvasWidget::PolyLine(poly_line) => {
            let mut pl = poly_line.clone();
            pl.points.push(cursor);
            let finished = if pl.points.len() == pl.poly_points {
                pl.mid_point = get_mid_geometry(&pl.points, DrawWidget::PolyLine);
                true
            } else {
                false
            };
            
            (CanvasWidget::PolyLine(pl), finished)
        },
        CanvasWidget::Polygon(polygon) => {
            let mut pg = polygon.clone();
            let finished = if pg.mid_point == Point::default() {
                pg.mid_point = cursor;
                false
            } else {
                pg.pg_point = cursor;
                true
            };
            if finished {
                pg.degrees = get_horizontal_angle_of_vector(pg.mid_point, pg.pg_point)
            }
            (CanvasWidget::Polygon(pg), finished)
        },
        CanvasWidget::RightTriangle(right_triangle) => {
            let mut rt = right_triangle.clone();
            rt.points.push(cursor);
            if rt.points.len() > 1 {
            rt.points[1].x = rt.points[0].x;
            }
            if rt.points.len() > 2 {
                rt.points[2].y = rt.points[1].y;
            }
            let finished = if rt.points.len() == 3 {
                // close the triangle
                rt.points.push(right_triangle.points[0]);
                rt.mid_point = get_mid_geometry(&rt.points, DrawWidget::RightTriangle);
                true
            } else {
                false
            };
            
            (CanvasWidget::RightTriangle(rt), finished)
        },
        CanvasWidget::FreeHand(fh) => {
            let mut fh = fh.clone();
            fh.points.push(cursor);
            let finished = fh.completed;
            
            (CanvasWidget::FreeHand(fh), finished)
        },
        CanvasWidget::Text(text) => {
            let mut txt = text.clone();
            
            let finished = if txt.position == Point::default() {
                txt.position = cursor;
                false
            } else {
                txt.status = DrawStatus::Completed;
                txt.draw_mode = DrawMode::Display;
                true
            };

            (CanvasWidget::Text(txt), finished)
        }
    }
}

fn find_closest_widget(
    curves: &HashMap<Id, CanvasWidget>,
    text_curves: &HashMap<Id, CanvasWidget>,
    cursor: Point,
) -> Option<CanvasWidget> {
    curves.iter()
        .chain(text_curves.iter())
        .min_by(|(_, a), (_, b)| {
            a.distance_to(cursor)
                .partial_cmp(&b.distance_to(cursor))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(_, w)| w.clone())
}

// returns a bool if mid_point and an optional usize 
// if a point in points.
fn find_closest_point_index(widget: &CanvasWidget,
                            cursor: Point, 
                            ) -> (Option<usize>, bool, bool) {

    let mut point_dist: f32 = f32::INFINITY;
    let mut point_index = 0;

    match widget {
        CanvasWidget::None => (None, false, false),
        CanvasWidget::Arc(arc) => {
            for (idx, point) in arc.points.iter().enumerate() {
                // skip first point since its a mid_point too.
                if idx == 0 {
                    continue;
                } else {
                    let dist = cursor.distance(*point);
                    if  dist < point_dist {
                        point_index = idx;
                        point_dist = dist;
                    }
                }
                
            };
            
            let mid_dist = arc.mid_point.distance(cursor);

            if mid_dist < point_dist {
                (None, true, false)
            } else {
                (Some(point_index), false, false)
            }
        },
        CanvasWidget::Bezier(bezier) => {
            for (idx, point) in bezier.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = bezier.mid_point.distance(cursor);

            if mid_dist < point_dist {
                (None, true, false)
            } else {
                (Some(point_index), false, false)
            }
        },
        CanvasWidget::Circle(cir) => {
            let center_dist = cursor.distance(cir.center);
            let point_dist = cursor.distance(cir.circle_point);
            if center_dist < point_dist {
                (None, true, false)
            } else {
                (Some(1), false, false)
            }
        }
        CanvasWidget::Ellipse(ell) => {
            let center_dist = cursor.distance(ell.center);
            let point_1_dist = cursor.distance(ell.points[1]);
            let point_2_dist = cursor.distance(ell.points[2]);
            if center_dist < point_1_dist && center_dist < point_2_dist {
                (None, true, false)
            } else if point_1_dist < point_2_dist {
                (Some(1), false, false)
            } else {
                (Some(2), false, false)
            }
        }
        CanvasWidget::Line(line) => {
            for (idx, point) in line.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = cursor.distance(line.mid_point);

            if mid_dist < point_dist {
                (None, true, false)
            } else {
                (Some(point_index), false, false)
            }
        },
        CanvasWidget::Polygon(pg) => {
            let pg_center = cursor.distance(pg.mid_point);
            let pg_point = cursor.distance(pg.pg_point);
            if pg_center <= pg_point {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        CanvasWidget::PolyLine(pl) => {
            for (idx, point) in pl.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = pl.mid_point.distance(cursor);
            let pl_pt_dist = pl.pl_point.distance(cursor);

            if point_dist < mid_dist && point_dist < pl_pt_dist {
                (Some(point_index), false, false)
            } else if mid_dist < pl_pt_dist {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        CanvasWidget::RightTriangle(tr) => {
            for (idx, point) in tr.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            
            let mid_dist = tr.mid_point.distance(cursor);
            let tr_pt_dist = tr.tr_point.distance(cursor);

            if point_dist < mid_dist && point_dist < tr_pt_dist {
                (Some(point_index), false, false)
            } else if mid_dist < tr_pt_dist {
                (None, true, false)
            } else {
                (None, false, true)
            }
        },
        CanvasWidget::FreeHand(fh) => {
            for (idx, point) in fh.points.iter().enumerate() {
                let dist = cursor.distance(*point);
                if  dist < point_dist {
                    point_index = idx;
                    point_dist = dist;
                }
            };
            (Some(point_index), false, false)
        },
        CanvasWidget::Text(_) => {
            // just using the edit_other_point to indicate the position point
            (None, false, true)
        }
    }
    
}


/// Thin wrapper kept for external callers; prefer `widget.id()` in new code.
pub fn get_widget_id(widget: &CanvasWidget) -> Id {
    widget.id()
}

/// Thin wrapper kept for external callers; prefer `(w.draw_mode(), w.status())` in new code.
pub fn get_draw_mode_and_status(widget: &CanvasWidget) -> (DrawMode, DrawStatus) {
    (widget.draw_mode(), widget.status())
}

pub fn get_mid_geometry(pts: &[Point], curve_type: DrawWidget) -> Point {
    match curve_type {
        DrawWidget::Arc => {
            get_mid_point(pts[0], pts[1])
        }
        DrawWidget::Bezier => {
            get_mid_point(pts[0], pts[1])
        },
        DrawWidget::Circle => {
            // return the center point
            pts[0]
        },
        DrawWidget::Ellipse => {
            // return the center point
            pts[0]
        }
        DrawWidget::Line => {
            get_mid_point(pts[0], pts[1])
        },
        DrawWidget::PolyLine => {

            let (slope, intercept) = get_linear_regression(pts);

            let (p1, p2) = get_line_from_slope_intercept(pts, slope, intercept);

            get_mid_point(p1, p2)

        },
        DrawWidget::Polygon => {
            // return the center point
            pts[0]
        },
        DrawWidget::RightTriangle => {
            let x = (pts[0].x + pts[1].x + pts[2].x)/3.0;
            let y = (pts[0].y + pts[1].y + pts[2].y)/3.0;
            Point {x, y}
        },
        DrawWidget::FreeHand => {
            pts[0]
        }
        DrawWidget::Text => {
            pts[0]
        }
        DrawWidget::None => Point::default(),
    }
    
}


// #[macro_export]
// macro_rules! mydbg {
//     // NOTE: We cannot use `concat!` to make a static string as a format argument
//     // of `eprintln!` because `file!` could contain a `{` or
//     // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
//     // will be malformed.
//     () => {
//         $crate::eprintln!("[{}:{}:{}]", $crate::file!(), $crate::line!(), $crate::column!())
//     };
//     ($val:expr $(,)?) => {
//         // Use of `match` here is intentional because it affects the lifetimes
//         // of temporaries - https://stackoverflow.com/a/48732525/1063961
//         match $val {
//             tmp => {
//                 $crate::eprintln!("[{}:{}:{}] {} = {:#?}",
//                     $crate::file!(), $crate::line!(), $crate::column!(), $crate::stringify!($val), &tmp);
//                 tmp
//             }
//         }
//     };
//     ($($val:expr),+ $(,)?) => {
//         ($($crate::mydbg!($val)),+,)
//     };
// }