//! path_builds

use std::f32::consts::PI;

use iced::{widget::canvas::{self, Frame, Path, Stroke}, Color, Point, Radians, Vector};

use charts_rs_mod::{format_string, GuiArrow, GuiAxis, GuiBubble, GuiCircle, GuiGrid, GuiLegend, GuiLine, GuiPie, GuiPolygon, GuiPolyline, GuiRect, GuiSmoothLine, GuiSmoothLineFill, GuiStraightLine, GuiStraightLineFill, GuiText};
use charts_rs_mod::Position as ChPosition;
use charts_rs_mod as ch;

pub fn build_arrow(
        ar: &GuiArrow,
        frame: &mut Frame,
        )
{
    let x_offset = ar.width / 2.0;
        let y_offset = ar.width / 2.0;
        let points = vec![
            Point {
                x: ar.x,
                y: ar.y,
            },
            Point {
                x: ar.x - x_offset,
                y: ar.y - y_offset,
            },
            Point {
                x: ar.x + ar.width,
                y: ar.y,
            },
            Point {
                x: ar.x - x_offset,
                y: ar.y + y_offset,
            },
            // repeat first to close
            Point {
                x: ar.x,
                y: ar.y,
            },
        ];

        let path = Path::new(|p| {
            points.iter().enumerate().for_each(|(index, point)| {
                if index == 0 {
                    p.move_to(*point);
                } else {
                    p.line_to(*point);
                }
            });

        let stroke = 
        Stroke {
            style: canvas::stroke::Style::Solid(convert_color(ar.stroke_color)),
            width: 1.0,
            ..Stroke::default()
        };

        frame.stroke(&path, stroke)

});
        
}

pub fn build_axis(
        ax: &GuiAxis,
        frame: &mut Frame,
        )
{
let mut ic = vec![];

    let stroke_color = match ax.stroke_color {
        Some(c) => convert_color(c),
        None => Color::TRANSPARENT,
    };

    let stroke_width = 1.0;

    let mut line_data = vec![];
    if stroke_color!= Color::TRANSPARENT {
        let values = match ax.position {
            ChPosition::Top => {
                let y = ax.top + ax.height;
                (ax.left, y, ax.left + ax.width, y)
            }
            ChPosition::Right => {
                let y = ax.top + ax.height;
                (ax.left, ax.top, ax.left, y)
            }
            ChPosition::Bottom => (ax.left, ax.top, ax.left + ax.width, ax.top),
            _ => {
                let x = ax.left + ax.width;
                (x, ax.top, x, ax.top + ax.height)
            }
        };
    }
    
    let is_horizontal = ax.position == ChPosition::Bottom || 
                                ax.position == ChPosition::Top;

    let axis_length = if is_horizontal {
        ax.width
    } else {
        ax.height
    };
    let font_size = ax.font_size;
    let formatter = &ax.formatter.clone().unwrap_or_default();

    let mut text_list = vec![];
    let mut text_unit_count: usize = 1;
    if font_size > 0.0 && !ax.data.is_empty() {
        text_list = ax
            .data
            .iter()
            .map(|item| format_string(item, formatter))
            .collect();
        if ax.position == ChPosition::Top || ax.position == ChPosition::Bottom {
            let f = ch::font::get_font(&ax.font_family).context(ch::GetFontSnafu)?;
            let total_measure = ch::font::measure_text(f, font_size, &text_list.join(" "));
            // Not enough space
            if total_measure.width() > axis_length {
                text_unit_count += (total_measure.width() / axis_length).ceil() as usize;
            }
        }
    }

    let mut split_number = ax.split_number;
    if split_number == 0 {
        split_number = ax.data.len();
    }
    if stroke_color != Color::transparent().to_vec() {
        let unit = axis_length / split_number as f32;
        let tick_interval = ax.tick_interval.max(text_unit_count);
        let tick_start = ax.tick_start;
        for i in 0..=split_number {
            if i < tick_start {
                continue;
            }
            let index = if i > tick_start { i - tick_start } else { i };
            if i != tick_start && (tick_interval != 0 && index % tick_interval != 0) {
                continue;
            }

            let values = match ax.position {
                Position::Top => {
                    let x = left + unit * i as f32;
                    let y = top + height;
                    (x, y - tick_length, x, y)
                }
                Position::Right => {
                    let y = top + unit * i as f32;
                    (left, y, left + tick_length, y)
                }
                Position::Bottom => {
                    let x = left + unit * i as f32;
                    (x, top, x, top + tick_length)
                }
                _ => {
                    let y = top + unit * i as f32;
                    let x = left + width;
                    (x, y, x - tick_length, y)
                }
            };

            line_data.push(
                IcedComponent::Line(IcedLine {
                    move_to: (values.0, values.1),
                    line_to: (values.2, values.3),
                    stroke_color,
                    stroke_width,
                }));
        }
    }
    ic.extend(line_data);

    let mut text_data = vec![];
    let name_rotate = ax.name_rotate / std::f32::consts::PI * 180.0;
    if !text_list.is_empty() {
        let name_gap = ax.name_gap;
        let f = font::get_font(&ax.font_family).context(GetFontSnafu).unwrap();
        let mut data_len = ax.data.len();
        let is_name_align_start = ax.name_align == Align::Left;
        if is_name_align_start {
            data_len -= 1;
        }
        let unit = axis_length / data_len as f32;

        for (index, text) in text_list.iter().enumerate() {
            if index % text_unit_count != 0 {
                continue;
            }
            let b = font::measure_text(f, font_size, text);
            let mut unit_offset = unit * index as f32 + unit / 2.0;
            if is_name_align_start {
                unit_offset -= unit / 2.0;
            }
            let text_width = b.width();

            let values = match ax.position {
                Position::Top => {
                    let y = top + height - name_gap;
                    let x = left + unit_offset - text_width / 2.0;
                    (x, y)
                }
                Position::Right => {
                    let x = left + name_gap;
                    let y = top + unit_offset + font_size / 2.0;
                    (x, y)
                }
                Position::Bottom => {
                    let y = top + font_size + name_gap;
                    let x = left + unit_offset - text_width / 2.0;
                    (x, y)
                }
                _ => {
                    let x = left + width - text_width - name_gap;
                    let y = top + unit_offset + font_size / 2.0 - 2.0;
                    (x, y)
                }
            };
            let mut transform = None;
            let mut x = Some(values.0);
            let mut y = Some(values.1);
            let mut text_anchor = None;
            if name_rotate != 0.0 {
                let w = ax.name_rotate.sin().abs() * b.width();
                let translate_x = (values.0 + b.width() / 2.0) as i32;
                let translate_y = (values.1 + w / 2.0) as i32;
                text_anchor = Some("middle".to_string());

                let a = name_rotate as i32;
                transform = Some(format!(
                    "translate({translate_x},{translate_y}) rotate({a})"
                ));
                x = None;
                y = None;
            }

            let font_color = match ax.font_color {
                Some(c) => Some(c.to_vec()),
                None => None,
            };

            text_data.push(
                IcedComponent::Text(IcedText {
                    text: text.to_string(),
                    font_family: Some(ax.font_family.clone()),
                    font_size: Some(ax.font_size),
                    font_color,
                    font_weight: ax.font_weight.clone(),
                    x,
                    y,
                    transform,
                    text_anchor,
                    line_height: None,
                    dx: None,
                    dy: None,
                    dominant_baseline: None,
                    alignment_baseline: None,
                }));
        }
    };

    ic.extend(text_data);
    


}

pub fn build_bubble(
        bu: &GuiBubble,
        frame: &mut Frame,
        )
{

}

pub fn build_circle(
        cir: &GuiCircle,
        frame: &mut Frame,
        ) 
{
    let path = Path::new(|p| {
        p.circle(Point::new(cir.cx, cir.cy), cir.r);
    });

    let color = match cir.stroke_color {
        Some(c) => convert_color(c),
        None => Color::TRANSPARENT,
    };

    let stroke = 
        Stroke {
            style: canvas::stroke::Style::Solid(color),
            width: cir.stroke_width,
            ..Stroke::default()
        };
    
    if cir.fill.is_some() {
        frame.fill(&path, convert_color(cir.fill.unwrap()))
    }

    frame.stroke(&path, stroke)

}

pub fn build_grid(
        grid: &GuiGrid,
        frame: &mut Frame,
        )
{

}

pub fn build_legend(
        leg: &GuiLegend,
        frame: &mut Frame,
        )
{

}


pub fn build_line(
        line: &GuiLine, 
        frame: &mut Frame
        ) 
{
    let path = Path::new(|p| {
        p.move_to(line.points[0]);
        p.line_to(line.points[1]);
    });
            
}

pub fn build_pie(
        pie: &GuiPie,
        frame: &mut Frame,
        )
{

}

pub fn build_polygon(
        pg: &GuiPolygon, 
        frame: &mut Frame,   
        ) 
{
    let path = Path::new(|p| {
        let points = &pg.points;
        for (index, point) in points.iter().enumerate() {
            if index == 0 {
                p.move_to(*point);
            } else {
                p.line_to(*point);
            }
        }
        p.line_to(points[0]);
    });

}

pub fn build_polyline(
        pl: &GuiPolyline, 
        frame: &mut Frame,   
        )
{
    let path = Path::new(|p| {
        for (index, point) in pl.points.iter().enumerate() {
            if index == 0 {
                p.move_to(*point);
            } else {
                p.line_to(*point);
            }
        }
    });
            
}

pub fn build_rect(
        rect: &GuiRect,
        frame: &mut Frame,
        )
{

}

pub fn build_smooth_line(
        sl: &GuiSmoothLine,
        frame: &mut Frame,
        )
{

}

pub fn build_smooth_line_fill(
        slf: &GuiSmoothLineFill,
        frame: &mut Frame,
        )
{

}

pub fn build_straight_line(
        sl: &GuiStraightLine,
        frame: &mut Frame,
        )
{

}

pub fn build_straight_line_fill(
        slf: &GuiStraightLineFill,
        frame: &mut Frame,
        )
{

}

pub fn build_text (
        txt: &GuiText, 
        frame: &mut Frame,
        ) {

        let mut text = canvas::Text {
                    content: txt.content.clone(),
                    position: Point::ORIGIN,
                    color: txt.color,
                    size: txt.size,
                    line_height: txt.line_height,
                    font: txt.font,
                    horizontal_alignment: txt.horizontal_alignment,
                    vertical_alignment: txt.vertical_alignment,
                    shaping: txt.shaping,
                };
                  
}

fn convert_color(c: charts_rs_mod::Color) -> iced::Color {
    Color::from_rgba8(c.r, c.g, c.b, c.a as f32)
}