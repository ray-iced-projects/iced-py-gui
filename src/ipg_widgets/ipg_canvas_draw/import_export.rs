//! Imports and exports drawings to a JSON file.
//!
//! The JSON format uses a `"type"` field to discriminate between shape kinds.
//! Colors are stored as `[r, g, b, a]` float arrays.
//! Each shape only carries the fields that are relevant to it.

use iced::{Color, Font, Pixels, Point, Radians, Vector, alignment};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::Id;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::canvas_draw::{
    Arc, Bezier, DrawWidget, Circle, DrawMode, DrawStatus,
    Ellipse, FreeHand, Line, PolyLine, Polygon, RightTriangle, Text,
};

// ── Point ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExportPoint {
    pub x: f32,
    pub y: f32,
}

impl ExportPoint {
    fn from_point(p: &Point) -> Self {
        Self { x: p.x, y: p.y }
    }
    fn to_point(self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

// ── Color  [r, g, b, a] ──────────────────────────────────────────────────────

type ExportColor = [f32; 4];

fn color_to_arr(c: Color) -> ExportColor {
    [c.r, c.g, c.b, c.a]
}

fn arr_to_color(c: &ExportColor) -> Color {
    Color::from_rgba(c[0], c[1], c[2], c[3])
}

// ── Alignment ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExportHorizontal {
    Default,
    Left,
    Center,
    Right,
    Justified,
}

impl ExportHorizontal {
    fn to_iced(self) -> iced::advanced::text::Alignment {
        match self {
            Self::Default   => iced::advanced::text::Alignment::Default,
            Self::Left      => iced::advanced::text::Alignment::Left,
            Self::Center    => iced::advanced::text::Alignment::Center,
            Self::Right     => iced::advanced::text::Alignment::Right,
            Self::Justified => iced::advanced::text::Alignment::Justified,
        }
    }
    fn from_iced(h: iced::advanced::text::Alignment) -> Self {
        match h {
            iced::advanced::text::Alignment::Default   => Self::Default,
            iced::advanced::text::Alignment::Left      => Self::Left,
            iced::advanced::text::Alignment::Center    => Self::Center,
            iced::advanced::text::Alignment::Right     => Self::Right,
            iced::advanced::text::Alignment::Justified => Self::Justified,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExportVertical {
    Top,
    Center,
    Bottom,
}

impl ExportVertical {
    fn to_iced(self) -> alignment::Vertical {
        match self {
            Self::Top    => alignment::Vertical::Top,
            Self::Center => alignment::Vertical::Center,
            Self::Bottom => alignment::Vertical::Bottom,
        }
    }
    fn from_iced(v: alignment::Vertical) -> Self {
        match v {
            alignment::Vertical::Top    => Self::Top,
            alignment::Vertical::Center => Self::Center,
            alignment::Vertical::Bottom => Self::Bottom,
        }
    }
}

// ── Tagged export enum ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExportWidget {
    Arc {
        mid_point:   ExportPoint,
        points:      Vec<ExportPoint>,
        radius:      f32,
        start_angle: f32,
        end_angle:   f32,
        color:       ExportColor,
        width:       f32,
    },
    Bezier {
        points:    Vec<ExportPoint>,
        mid_point: ExportPoint,
        degrees:   f32,
        color:     ExportColor,
        width:     f32,
    },
    Circle {
        center:       ExportPoint,
        circle_point: ExportPoint,
        radius:       f32,
        color:        ExportColor,
        width:        f32,
    },
    Ellipse {
        points:   Vec<ExportPoint>,
        center:   ExportPoint,
        rotation: f32,
        color:    ExportColor,
        width:    f32,
    },
    Line {
        points:    Vec<ExportPoint>,
        mid_point: ExportPoint,
        degrees:   f32,
        color:     ExportColor,
        width:     f32,
    },
    PolyLine {
        points:      Vec<ExportPoint>,
        poly_points: usize,
        mid_point:   ExportPoint,
        pl_point:    ExportPoint,
        degrees:     f32,
        color:       ExportColor,
        width:       f32,
    },
    Polygon {
        points:      Vec<ExportPoint>,
        poly_points: usize,
        mid_point:   ExportPoint,
        pg_point:    ExportPoint,
        degrees:     f32,
        color:       ExportColor,
        width:       f32,
    },
    RightTriangle {
        points:    Vec<ExportPoint>,
        mid_point: ExportPoint,
        tr_point:  ExportPoint,
        degrees:   f32,
        color:     ExportColor,
        width:     f32,
    },
    FreeHand {
        points: Vec<ExportPoint>,
        color:  ExportColor,
        width:  f32,
    },
    Text {
        content:  String,
        position: ExportPoint,
        degrees:  f32,
        size:     f32,
        color:    ExportColor,
        align_x:  ExportHorizontal,
        align_y:  ExportVertical,
    },
}

// ── Import ────────────────────────────────────────────────────────────────────

pub fn import_widgets(
    widgets: Vec<ExportWidget>,
) -> (HashMap<Id, DrawWidget>, HashMap<Id, DrawWidget>) {
    let mut curves: HashMap<Id, DrawWidget> = HashMap::new();
    let mut text_curves: HashMap<Id, DrawWidget> = HashMap::new();

    for widget in widgets {
        match widget {
            ExportWidget::Arc { mid_point, points, radius, start_angle, end_angle, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::Arc(Arc {
                    id,
                    points:      points.iter().map(|p| p.to_point()).collect(),
                    mid_point:   mid_point.to_point(),
                    radius,
                    color:       arr_to_color(&color),
                    width,
                    start_angle: Radians(start_angle),
                    end_angle:   Radians(end_angle),
                    draw_mode:   DrawMode::Display,
                    status:      DrawStatus::Completed,
                }));
            },
            ExportWidget::Bezier { points, mid_point, degrees, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::Bezier(Bezier {
                    id,
                    points:    points.iter().map(|p| p.to_point()).collect(),
                    mid_point: mid_point.to_point(),
                    color:     arr_to_color(&color),
                    width,
                    degrees,
                    draw_mode: DrawMode::Display,
                    status:    DrawStatus::Completed,
                }));
            },
            ExportWidget::Circle { center, circle_point, radius, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::Circle(Circle {
                    id,
                    center:       center.to_point(),
                    circle_point: circle_point.to_point(),
                    radius,
                    color:        arr_to_color(&color),
                    width,
                    draw_mode:    DrawMode::Display,
                    status:       DrawStatus::Completed,
                }));
            },
            ExportWidget::Ellipse { points, center, rotation, color, width } => {
                let id = Id::unique();
                let pts: Vec<Point> = points.iter().map(|p| p.to_point()).collect();
                let vx = pts[1].distance(pts[0]);
                let vy = pts[2].distance(pts[0]);
                curves.insert(id.clone(), DrawWidget::Ellipse(Ellipse {
                    id,
                    center:    center.to_point(),
                    radii:     Vector { x: vx, y: vy },
                    rotation:  Radians(rotation),
                    color:     arr_to_color(&color),
                    width,
                    points:    pts,
                    draw_mode: DrawMode::Display,
                    status:    DrawStatus::Completed,
                }));
            },
            ExportWidget::Line { points, mid_point, degrees, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::Line(Line {
                    id,
                    points:    points.iter().map(|p| p.to_point()).collect(),
                    mid_point: mid_point.to_point(),
                    color:     arr_to_color(&color),
                    width,
                    degrees,
                    draw_mode: DrawMode::Display,
                    status:    DrawStatus::Completed,
                }));
            },
            ExportWidget::PolyLine { points, poly_points, mid_point, pl_point, degrees, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::PolyLine(PolyLine {
                    id,
                    points:      points.iter().map(|p| p.to_point()).collect(),
                    poly_points,
                    mid_point:   mid_point.to_point(),
                    pl_point:    pl_point.to_point(),
                    color:       arr_to_color(&color),
                    width,
                    degrees,
                    draw_mode:   DrawMode::Display,
                    status:      DrawStatus::Completed,
                }));
            },
            ExportWidget::Polygon { points, poly_points, mid_point, pg_point, degrees, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::Polygon(Polygon {
                    id,
                    points:      points.iter().map(|p| p.to_point()).collect(),
                    poly_points,
                    mid_point:   mid_point.to_point(),
                    pg_point:    pg_point.to_point(),
                    color:       arr_to_color(&color),
                    width,
                    degrees,
                    draw_mode:   DrawMode::Display,
                    status:      DrawStatus::Completed,
                }));
            },
            ExportWidget::RightTriangle { points, mid_point, tr_point, degrees, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::RightTriangle(RightTriangle {
                    id,
                    points:    points.iter().map(|p| p.to_point()).collect(),
                    mid_point: mid_point.to_point(),
                    tr_point:  tr_point.to_point(),
                    color:     arr_to_color(&color),
                    width,
                    degrees,
                    draw_mode: DrawMode::Display,
                    status:    DrawStatus::Completed,
                }));
            },
            ExportWidget::FreeHand { points, color, width } => {
                let id = Id::unique();
                curves.insert(id.clone(), DrawWidget::FreeHand(FreeHand {
                    id,
                    points:    points.iter().map(|p| p.to_point()).collect(),
                    color:     arr_to_color(&color),
                    width,
                    draw_mode: DrawMode::Display,
                    status:    DrawStatus::Completed,
                    completed: true,
                }));
            },
            ExportWidget::Text { content, position, degrees, size, color, align_x, align_y } => {
                let id = Id::unique();
                text_curves.insert(id.clone(), DrawWidget::Text(Text {
                    id,
                    content,
                    position:    position.to_point(),
                    color:       arr_to_color(&color),
                    size:        Pixels(size),
                    line_height: LineHeight::Relative(1.2),
                    font:        Font::default(),
                    align_x:     align_x.to_iced(),
                    align_y:     align_y.to_iced(),
                    shaping:     Shaping::Basic,
                    degrees,
                    draw_mode:   DrawMode::Display,
                    status:      DrawStatus::Completed,
                }));
            },
        }
    }

    (curves, text_curves)
}

// ── Export ────────────────────────────────────────────────────────────────────

pub fn convert_to_export(
    curves: &HashMap<Id, DrawWidget>,
    text:   &HashMap<Id, DrawWidget>,
) -> Vec<ExportWidget> {
    let mut export = vec![];

    for widget in curves.values().chain(text.values()) {
        let ew = match widget {
            DrawWidget::None => continue,
            DrawWidget::Arc(arc) => ExportWidget::Arc {
                mid_point:   ExportPoint::from_point(&arc.mid_point),
                points:      arc.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                radius:      arc.radius,
                start_angle: arc.start_angle.0,
                end_angle:   arc.end_angle.0,
                color:       color_to_arr(arc.color),
                width:       arc.width,
            },
            DrawWidget::Bezier(bz) => ExportWidget::Bezier {
                points:    bz.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                mid_point: ExportPoint::from_point(&bz.mid_point),
                degrees:   bz.degrees,
                color:     color_to_arr(bz.color),
                width:     bz.width,
            },
            DrawWidget::Circle(cir) => ExportWidget::Circle {
                center:       ExportPoint::from_point(&cir.center),
                circle_point: ExportPoint::from_point(&cir.circle_point),
                radius:       cir.radius,
                color:        color_to_arr(cir.color),
                width:        cir.width,
            },
            DrawWidget::Ellipse(ell) => ExportWidget::Ellipse {
                points:   ell.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                center:   ExportPoint::from_point(&ell.center),
                rotation: ell.rotation.0,
                color:    color_to_arr(ell.color),
                width:    ell.width,
            },
            DrawWidget::Line(ln) => ExportWidget::Line {
                points:    ln.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                mid_point: ExportPoint::from_point(&ln.mid_point),
                degrees:   ln.degrees,
                color:     color_to_arr(ln.color),
                width:     ln.width,
            },
            DrawWidget::PolyLine(pl) => ExportWidget::PolyLine {
                points:      pl.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                poly_points: pl.poly_points,
                mid_point:   ExportPoint::from_point(&pl.mid_point),
                pl_point:    ExportPoint::from_point(&pl.pl_point),
                degrees:     pl.degrees,
                color:       color_to_arr(pl.color),
                width:       pl.width,
            },
            DrawWidget::Polygon(pg) => ExportWidget::Polygon {
                points:      pg.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                poly_points: pg.poly_points,
                mid_point:   ExportPoint::from_point(&pg.mid_point),
                pg_point:    ExportPoint::from_point(&pg.pg_point),
                degrees:     pg.degrees,
                color:       color_to_arr(pg.color),
                width:       pg.width,
            },
            DrawWidget::RightTriangle(tr) => ExportWidget::RightTriangle {
                points:    tr.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                mid_point: ExportPoint::from_point(&tr.mid_point),
                tr_point:  ExportPoint::from_point(&tr.tr_point),
                degrees:   tr.degrees,
                color:     color_to_arr(tr.color),
                width:     tr.width,
            },
            DrawWidget::FreeHand(fh) => ExportWidget::FreeHand {
                points: fh.points.iter().map(|p| ExportPoint::from_point(p)).collect(),
                color:  color_to_arr(fh.color),
                width:  fh.width,
            },
            DrawWidget::Text(txt) => ExportWidget::Text {
                content:  txt.content.clone(),
                position: ExportPoint::from_point(&txt.position),
                degrees:  txt.degrees,
                size:     txt.size.0,
                color:    color_to_arr(txt.color),
                align_x:  ExportHorizontal::from_iced(txt.align_x),
                align_y:  ExportVertical::from_iced(txt.align_y),
            },
        };
        export.push(ew);
    }

    export
}
