//! Imports and exports drawings to a json file

use iced::{Color, Font, Pixels, Point, Radians, Vector, alignment};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::Id;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


use super::draw_canvas::{Arc, Bezier, CanvasWidget, Circle, DrawMode, DrawStatus, Ellipse, FreeHand, Line, PolyLine, Polygon, RightTriangle, Text, Widget};

// iced Point does not derive any serialization 
// so had to use own version for saving data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExportPoint{
    x: f32,
    y: f32,
}

impl ExportPoint {
    fn convert(point: &Point) -> Self {
        ExportPoint {x: point.x, y: point.y}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct ExportColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl ExportColor {
    pub const fn from_rgba(color: &Color) -> ExportColor {
        ExportColor { r: color.r, g: color.g, b: color.b, a: color.a }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExportHorizontal {
    Default,
    Left,
    Center,
    Right,
    Justified,
}


fn convert_to_export_horizontal(h: iced::advanced::text::Alignment) -> ExportHorizontal {
    match h {
        iced::advanced::text::Alignment::Default => ExportHorizontal::Default,
        iced::advanced::text::Alignment::Left => ExportHorizontal::Left,
        iced::advanced::text::Alignment::Center => ExportHorizontal::Center,
        iced::advanced::text::Alignment::Right => ExportHorizontal::Right,
        iced::advanced::text::Alignment::Justified => ExportHorizontal::Justified,
    }
}

fn convert_to_iced_horizontal(h_opt: Option<ExportHorizontal>) -> iced::advanced::text::Alignment {
    let h = if let Some(h) = h_opt {
        h
    } else {
        ExportHorizontal::Default
    };
    match h {
        ExportHorizontal::Default => iced::advanced::text::Alignment::Default,
        ExportHorizontal::Left => iced::advanced::text::Alignment::Left,
        ExportHorizontal::Center => iced::advanced::text::Alignment::Center,
        ExportHorizontal::Right => iced::advanced::text::Alignment::Right,
        ExportHorizontal::Justified => iced::advanced::text::Alignment::Justified,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExportVertical {
   Top,
   Center,
   Bottom,
}

fn convert_to_export_vertical(v: alignment::Vertical) -> ExportVertical {
    match v {
        alignment::Vertical::Top => ExportVertical::Top,
        alignment::Vertical::Center => ExportVertical::Center,
        alignment::Vertical::Bottom => ExportVertical::Bottom,
    }
}

fn convert_to_iced_vertical(v_opt: Option<ExportVertical>) -> alignment::Vertical {
    let v = if let Some(v) = v_opt {
        v
    } else {
        ExportVertical::Top
    };
    match v {
        ExportVertical::Top => alignment::Vertical::Top,
        ExportVertical::Center => alignment::Vertical::Center,
        ExportVertical::Bottom => alignment::Vertical::Bottom,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportWidget {
    pub name: Widget,
    pub content: String,
    pub points: Vec<ExportPoint>,
    pub poly_points: usize,
    pub mid_point: ExportPoint,
    pub other_point: ExportPoint,
    pub rotation: f32,
    pub radius: f32,
    pub color: ExportColor,
    pub width: f32,
    pub align_x: Option<ExportHorizontal>,
    pub align_y: Option<ExportVertical>,
}



#[allow(clippy::redundant_closure)]
pub fn import_widgets(widgets: Vec<ExportWidget>) -> (HashMap<Id, CanvasWidget>, HashMap<Id, CanvasWidget>) {
    
    let mut curves: HashMap<Id, CanvasWidget> = HashMap::new();
    let mut text_curves: HashMap<Id, CanvasWidget> = HashMap::new();

    for widget in widgets.iter() {
        let points: Vec<Point> = widget.points.iter().map(|p| convert_to_point(p)).collect();
        let other_point = convert_to_point(&widget.other_point);
        let color = convert_to_color(&widget.color);
        let width = widget.width;
        let draw_mode = DrawMode::DrawAll;
        let mid_point = convert_to_point(&widget.mid_point);
        
        match widget.name {
            Widget::None => {
            },
            Widget::Arc => {
                let id = Id::unique();
                let arc = Arc {
                    id: id.clone(),
                    points,
                    mid_point,
                    radius: widget.radius,
                    color,
                    width,
                    start_angle: Radians(other_point.x),
                    end_angle: Radians(other_point.y),
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                
                curves.insert(id, CanvasWidget::Arc(arc));
            },
            Widget::Bezier => {
                let id = Id::unique();
                let bz = Bezier {
                    id: id.clone(),
                    points,
                    mid_point,
                    color,
                    width,
                    degrees: widget.rotation,
                    draw_mode,
                    status: DrawStatus::Completed
                };
                
                curves.insert(id, CanvasWidget::Bezier(bz));
            },
            Widget::Circle => {
                let id = Id::unique();
                let cir = Circle {
                    id: id.clone(),
                    center: mid_point,
                    circle_point: convert_to_point(&widget.points[0]),
                    radius: widget.radius,
                    color,
                    width,
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                
                curves.insert(id, CanvasWidget::Circle(cir));
            },
            Widget::Ellipse => {
                let id = Id::unique();
                let vx = points[1].distance(points[0]);
                let vy = points[2].distance(points[0]);
                let ell = Ellipse {
                    id: id.clone(),
                    points,
                    center: convert_to_point(&widget.points[0]),
                    radii: Vector { x: vx, y: vy },
                    rotation: Radians(widget.rotation),
                    color,
                    width,
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                
                curves.insert(id, CanvasWidget::Ellipse(ell));
            },
            Widget::Line => {
                let id = Id::unique();
                let ln = Line {
                    id: id.clone(),
                    points,
                    mid_point,
                    color,
                    width,
                    degrees: widget.rotation,
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                curves.insert(id, CanvasWidget::Line(ln));
            },
            Widget::Polygon => {
                let id = Id::unique();
                let pg = Polygon {
                    id: id.clone(),
                    points,
                    poly_points: widget.poly_points,
                    mid_point,
                    pg_point: other_point,
                    color,
                    width,
                    degrees: widget.rotation,
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                curves.insert(id, CanvasWidget::Polygon(pg));
            },
            Widget::PolyLine => {
                let id = Id::unique();
                let pl = PolyLine {
                    id: id.clone(),
                    points,
                    poly_points: widget.poly_points,
                    mid_point,
                    pl_point: other_point,
                    color,
                    width,
                    degrees: widget.rotation,
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                curves.insert(id, CanvasWidget::PolyLine(pl));
            },
            Widget::RightTriangle => {
                let id = Id::unique();
                let tr = RightTriangle {
                    id: id.clone(),
                    points,
                    mid_point,
                    tr_point: other_point,
                    color,
                    width,
                    degrees: widget.rotation,
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                curves.insert(id, CanvasWidget::RightTriangle(tr));
            },
            Widget::FreeHand => {
                let id = Id::unique();
                let fh = FreeHand {
                    id: id.clone(),
                    points,
                    color,
                    width,
                    draw_mode,
                    status: DrawStatus::Completed,
                    completed: true,
                };
                curves.insert(id, CanvasWidget::FreeHand(fh));
            }
            Widget::Text => {
                let id = Id::unique();
                let txt = Text {
                    id: id.clone(),
                    content: widget.content.clone(),
                    position: other_point,
                    color,
                    size: Pixels(16.0),
                    line_height: LineHeight::Relative(1.2),
                    font: Font::default(),
                    align_x: convert_to_iced_horizontal(widget.align_x),
                    align_y: convert_to_iced_vertical(widget.align_y),
                    shaping: Shaping::Basic,
                    degrees: widget.rotation,
                    draw_mode,
                    status: DrawStatus::Completed,
                };
                text_curves.insert(id, CanvasWidget::Text(txt));
            }
        }
    }

    (curves, text_curves)

}

pub fn convert_to_export(widgets: &HashMap<Id, CanvasWidget>, text: &HashMap<Id, CanvasWidget>) -> Vec<ExportWidget> {
    
    let mut curves = widgets.clone();
    for (k, v) in text.iter() {
        curves.insert(k.clone(), v.clone());
    }

    let mut export = vec![];

    for (_id, widget) in curves.iter() {

        let (name, 
            points, 
            mid_point,
            other_point, 
            poly_points, 
            rotation,
            radius,
            color, 
            width,
            content,
            align_x,
            align_y,
            ) = 
            match widget {
                CanvasWidget::None => {
                    (Widget::None, &vec![], Point::default(), Point::default(), 0, 0.0, 0.0, 
                    Color::TRANSPARENT, 0.0, String::new(), None, None)
                },
                CanvasWidget::Arc(arc) => {
                    let other_point = Point{ x: arc.start_angle.0, y: arc.end_angle.0 };
                    (Widget::Arc, &arc.points, arc.mid_point, other_point, 0, 0.0, arc.radius, 
                        arc.color, arc.width, String::new(), None, None)
                },
                CanvasWidget::Bezier(bz) => {
                    (Widget::Bezier, &bz.points, bz.mid_point, Point::default(), 0, bz.degrees, 0.0, 
                    bz.color, bz.width, String::new(), None, None)
                },
                CanvasWidget::Circle(cir) => {
                    (Widget::Circle, &vec![cir.circle_point], cir.center, cir.circle_point, 0, 0.0, cir.radius, 
                        cir.color, cir.width, String::new(), None, None)
                },
                CanvasWidget::Ellipse(ell) => {
                    (Widget::Ellipse, &ell.points, ell.center, Point::default(), 0, ell.rotation.0, 0.0, 
                    ell.color, ell.width, String::new(), None, None)
                },
                CanvasWidget::Line(ln) => {
                    (Widget::Line, &ln.points, ln.mid_point, Point::default(), 0, ln.degrees, 0.0, 
                    ln.color, ln.width, String::new(), None, None)
                },
                CanvasWidget::Polygon(pg) => {
                    (Widget::Polygon, &pg.points, pg.mid_point, pg.pg_point, pg.poly_points, pg.degrees, 0.0, 
                        pg.color, pg.width, String::new(), None, None)
                },
                CanvasWidget::PolyLine(pl) => {
                    (Widget::PolyLine, &pl.points, pl.mid_point, pl.pl_point, pl.poly_points, pl.degrees, 0.0, 
                        pl.color, pl.width, String::new(), None, None)
                },
                CanvasWidget::RightTriangle(tr) => {
                    (Widget::RightTriangle, &tr.points, tr.mid_point, tr.tr_point, 3, tr.degrees, 0.0, 
                        tr.color, tr.width, String::new(), None, None)
                },
                CanvasWidget::FreeHand(fh) => {
                    (Widget::FreeHand, &fh.points, Point::default(), Point::default(), 0, 0.0, 0.0, 
                    fh.color, fh.width, String::new(), None, None)
                }
                CanvasWidget::Text(txt) => {
                    (Widget::Text, &vec![], Point::default(), txt.position, 0, txt.degrees, 0.0, 
                    txt.color, 0.0, txt.content.clone(), 
                    Some(convert_to_export_horizontal(txt.align_x)), 
                    Some(convert_to_export_vertical(txt.align_y)))
                },
        };

        let x_color = ExportColor::from_rgba(&color);
        let x_mid_pt = ExportPoint::convert(&mid_point);
        let x_other_point = ExportPoint::convert(&other_point);
        let mut x_points = vec![];
        for point in points.iter() {
            x_points.push(ExportPoint::convert(point));
        }
        
        export.push(
            ExportWidget{
                name,
                content,
                points: x_points,
                poly_points, 
                mid_point: x_mid_pt,
                other_point: x_other_point,
                rotation,
                radius, 
                color: x_color, 
                width,
                align_x,
                align_y, 
            })
    }
    
    export

}

fn convert_to_point(point: &ExportPoint) -> Point {
    Point { x: point.x, y: point.y }
}

fn convert_to_color(color: &ExportColor) -> Color {
    Color::from_rgba(color.r, color.g, color.b, color.a)
}
