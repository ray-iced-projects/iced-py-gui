//! ipg_canvas

use iced::widget::container;
use iced::{Color, Element, Point, Radians};

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_types::{CanvasWidget, DrawMode, DrawStatus};
use canvas::draw_canvas::{CanvasState, get_draw_mode_and_status, get_widget_id, set_widget_mode_or_status};
use ipg_alignment::{get_horizontal_alignment, get_vertical_alignment, try_extract_ipg_horizontal_alignment, try_extract_ipg_vertical_alignment};
use ipg_helpers::{try_extract_f64, try_extract_point, try_extract_string};
use ipg_styling::try_extract_rgba_color;
use ipg_types::{CanvasMessage, Message};



#[derive(Debug, Clone)]
pub struct IpgCanvas {
    pub id: usize,
}

impl IpgCanvas {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

pub fn construct_canvas(canvas_state: &CanvasState) -> Element<'_, Message> {
    let draw: Element<CanvasMessage> = container(
        canvas_state
            .view(
                &canvas_state.curves,
                &canvas_state.text_curves,
            )
            .map(CanvasMessage::WidgetDraw),
    )
    .into();
    draw.map(move |message| Message::Canvas(message))
}

pub fn canvas_callback(canvas_message: CanvasMessage, app_state: &mut IpgState, canvas_state: &mut CanvasState) {
    match canvas_message {
        CanvasMessage::WidgetDraw(mut widget) => {
            // Since the text widget may have a blinking cursor, the only way to use a timer
            // is to use the main subscription one at this time, canvas lacks a time event.
            // Therefore, the pending has to return the curve also at each change so that
            // the curves can be updated.  The subscription clears the text cache at each tick.
            match widget {
                CanvasWidget::Text(_) => {
                    let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                    let id = get_widget_id(&widget);
                    match draw_status {
                        DrawStatus::Completed => {
                            widget = set_widget_mode_or_status(widget, Some(DrawMode::DrawAll), None);
                            canvas_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
                            canvas_state.timer_event_enabled = false;
                            canvas_state.draw_mode = DrawMode::DrawAll;
                        },
                        DrawStatus::Delete => {
                            canvas_state.text_curves.remove(&id);
                            canvas_state.timer_event_enabled = false;
                        },
                        DrawStatus::Inprogress => {
                            // Since the text always returns a new curve or updated curve,
                            // a check for the first return is need to see if a text is present. 
                            let present = canvas_state.text_curves.get(&id);
                            if present.is_none() {
                                canvas_state.text_curves.insert(id, widget.clone());
                            } else {
                                canvas_state.text_curves.entry(id).and_modify(|k| *k= widget.clone());
                            }
                        },
                    }
                    match draw_mode {
                        DrawMode::Edit | DrawMode::Rotate => {
                            let id = get_widget_id(&widget);
                            canvas_state.edit_widget_id = Some(id.clone());
                            canvas_state.text_curves.entry(id).and_modify(|k| *k= widget);
                        },
                        _ => (),
                    }
                    canvas_state.request_text_redraw();
                },
                _ => {
                    let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
                    match draw_status {
                        DrawStatus::Completed => {
                            widget = set_widget_mode_or_status(widget, Some(DrawMode::DrawAll), None);
                        },
                        DrawStatus::Delete => {
                            let id = get_widget_id(&widget);
                            canvas_state.curves.remove(&id);
                        },  
                        _ => (),
                    }
                    if draw_mode == DrawMode::New {
                        let id = get_widget_id(&widget);
                        let widget = set_widget_mode_or_status(widget.clone(), Some(DrawMode::DrawAll), Some(DrawStatus::Completed));
                        canvas_state.curves.insert(id, widget);
                    } else {
                        // if not new must be in edit or rotate mode so modify.
                        let id = get_widget_id(&widget);
                        canvas_state.edit_widget_id = Some(id.clone());
                        canvas_state.curves.entry(id).and_modify(|k| *k= widget);
                    }
                    
                    canvas_state.request_redraw();
                },
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCanvasParam {
    Clear,
    CanvasColor,
    DrawColor,
    FillColor,
    DrawWidth,
    Mode,
    PolyPoints,
    Widget,
    TextAlignment,
}

// update only the canvas, not the propterties of the canvas widgets.
// see canvas_geometry_update
pub fn canvas_item_update(canvas_state: &mut CanvasState, 
                            item: &PyObject, 
                            value: &PyObject,
                            mut last_id: usize,) 
                            -> Option<usize> 
{
    let update = try_extract_canvas_update(item);
    let name = "Canvas".to_string();
    match update {
        IpgCanvasParam::Clear => {
            canvas_state.request_redraw();
            None
        },
        IpgCanvasParam::CanvasColor => {
            let rgba = try_extract_rgba_color(value, name);
            canvas_state.canvas_color = Color::from(rgba);
            canvas_state.request_redraw();
            None
        },
        IpgCanvasParam::DrawColor => {
            let rgba = try_extract_rgba_color(value, name);
            canvas_state.draw_color = Color::from(rgba);
            None
        },
        IpgCanvasParam::FillColor => {
            let rgba = try_extract_rgba_color(value, name);
            canvas_state.fill_color = Some(Color::from(rgba));
            None
        },
        IpgCanvasParam::DrawWidth => {
            let width = try_extract_f64(value, name) as f32;
            canvas_state.draw_width = width;
            None
        },
        IpgCanvasParam::Mode => {
            canvas_state.draw_mode = try_extract_mode(value);
            None
        },
        IpgCanvasParam::PolyPoints => {
            let input = try_extract_string(value, name);
            canvas_state.poly_points = match input.parse::<usize>() {
                Ok(int) => int,
                Err(e) => panic!("PolyPoint input must be an integer, {}", e),
            };
            None
        },
        IpgCanvasParam::TextAlignment => {
            let align = try_extract_ipg_horizontal_alignment(value);
            if align.is_some() {
                canvas_state.h_text_alignment = get_horizontal_alignment(&align.unwrap())
            }
            let align = try_extract_ipg_vertical_alignment(value);
            if align.is_some() {
                canvas_state.v_text_alignment = get_vertical_alignment(&align.unwrap());
            }
            None
        }
        IpgCanvasParam::Widget => {
            let selected_widget = Some(try_extract_widget(value));
            canvas_state.selected_widget = selected_widget;
            canvas_state.timer_event_enabled = selected_widget == Some(CanvasWidget::Text);
            None
        }
    }
}

pub fn try_extract_canvas_update(update_obj: &PyObject) -> IpgCanvasParam {
    Python::attach(|py| {
        let res = update_obj.extract::<IpgCanvasParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas update extraction failed"),
        }
    })
}

fn try_extract_mode(update_obj: &PyObject) -> DrawMode {
    Python::attach(|py| {
        let res = update_obj.extract::<DrawMode>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas mode update extraction failed"),
        }
    })
}

fn try_extract_widget(update_obj: &PyObject) -> CanvasWidget {
    Python::attach(|py| {
        let res = update_obj.extract::<CanvasWidget>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas widget update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCanvasGeometryParam {
    Position,
    Rotation,
}

pub fn match_canvas_widget(widget: &mut CanvasWidget, item: &PyObject, value: &PyObject) {
    let update_item = try_extract_geometry_update(item);
    let name = "CanvasGeometry".to_string();
    match widget {
        CanvasWidget::None => (),
        CanvasWidget::Arc(arc) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                arc.mid_point = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                panic!("Arc has no rotation property")
            }
        },
        CanvasWidget::Bezier(bz) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                bz.mid_point = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                bz.rotation = val;
            }
        },
        CanvasWidget::Circle(cir) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                cir.center = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                panic!("Circle update has no rotation property")
            }
        },
        CanvasWidget::Ellipse(ell) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                ell.center = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                ell.rotation = Radians(val);
            }
        },
        CanvasWidget::Image(img) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                img.position = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                img.rotation = val;
            }
        },
        CanvasWidget::Line(line) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                line.mid_point = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                line.rotation = val;
            }
        },
        CanvasWidget::PolyLine(pl) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                pl.mid_point = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                pl.rotation = val;
            }
        },
        CanvasWidget::Polygon(pg) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                pg.mid_point = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                pg.rotation = val;
            }
        },
        CanvasWidget::Rectangle(rect) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                rect.mid_point = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                panic!("Rectangle has no rotation property use polygon with 4 sides")
            }
        },
        CanvasWidget::RightTriangle(tr) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                tr.mid_point = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                tr.rotation = val;
            }
        },
        CanvasWidget::Text(txt) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                txt.position = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                let val = try_extract_f64(value, name) as f32;
                txt.rotation = val;
            }
        },
        CanvasWidget::FreeHand(fh) => match update_item {
            IpgCanvasGeometryParam::Position => {
                let val = try_extract_point(value, name);
                fh.points[0] = Point::from(val);
            }
            IpgCanvasGeometryParam::Rotation => {
                panic!("Freehand geometry has no rotation property")
            }
        },
    }
}

pub fn try_extract_geometry_update(update_obj: &PyObject) -> IpgCanvasGeometryParam {
    Python::attach(|py| {
        let res = update_obj.extract::<IpgCanvasGeometryParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Canvas update extraction failed"),
        }
    })
}
