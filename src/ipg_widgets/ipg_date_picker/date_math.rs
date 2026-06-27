//! Color math
use iced::{Color, Point, Size};

/// Convert HSV (hue 0-255, sat 0-1, val 0-1) to RGB.
pub fn hsv_to_rgb(hue: u8, s: f32, v: f32) -> Color {
    let h = (hue as f32 / 255.0) * 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as u32 {
        0..=59    => (c, x, 0.0),
        60..=119  => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _         => (c, 0.0, x),
    };
    Color::from_rgb(r + m, g + m, b + m)
}

/// Returns (saturation, value) in 0..=1 for the given RGB.
pub fn rgb_to_sv(r: u8, g: u8, b: u8) -> (f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let s = if max > 0.0 { (max - min) / max } else { 0.0 };
    (s, max)
}

pub fn rgb_to_hue(r: u8, g: u8, b: u8) -> u8 {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    if delta < 1e-6 {
        return 0; // achromatic
    }
    let hue_deg = if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let hue_deg = if hue_deg < 0.0 { hue_deg + 360.0 } else { hue_deg };
    ((hue_deg / 360.0) * 255.0).round() as u8
}

pub fn hue_to_rgb(hue: u8) -> Color {
    let h = (hue as f32 / 255.0) * 360.0;
    let c = 1.0_f32;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let (r, g, b) = match h as u32 {
        0..=59    => (c, x, 0.0),
        60..=119  => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _         => (c, 0.0, x),
    };
    Color::from_rgb(r, g, b)
}

/// Compute the HSV-space color at canvas position `pos` given `hue`.
pub fn color_at(pos: Point, size: Size, hue: u8) -> (u8, u8, u8) {
    let s = (pos.x / size.width).clamp(0.0, 1.0);
    let v = (pos.y / size.height).clamp(0.0, 1.0);
    let c = hsv_to_rgb(hue, s, v);
    (
        (c.r * 255.0).round() as u8,
        (c.g * 255.0).round() as u8,
        (c.b * 255.0).round() as u8,
    )
}
