//! UI rendering for Sacred Geometry.

extern crate alloc;
use alloc::format;
use alloc::string::String;

use gam::{Gam, Point, Line, TextView, TextBounds, GlyphStyle};

use crate::app::{SacredApp, AppState};
use crate::geometry::{self, Primitive};

const SCREEN_W: i16 = 336;
const HEADER_H: i16 = 30;
const FOOTER_Y: i16 = 490;

fn draw_header(gam: &Gam, title: &str) {
    gam.draw_rectangle(Point::new(0, 0), Point::new(SCREEN_W, HEADER_H)).ok();
    let mut tv = TextView::new(
        TextBounds::BoundingBox(Point::new(8, 2), Point::new(SCREEN_W - 8, HEADER_H)),
    );
    tv.style = GlyphStyle::Bold;
    tv.invert = true;
    tv.text = String::from(title);
    gam.post_textview(&mut tv).ok();
}

fn draw_footer(gam: &Gam, text: &str) {
    gam.draw_line(Line::new(Point::new(0, FOOTER_Y), Point::new(SCREEN_W, FOOTER_Y))).ok();
    let mut tv = TextView::new(
        TextBounds::BoundingBox(Point::new(8, FOOTER_Y + 4), Point::new(SCREEN_W - 8, FOOTER_Y + 40)),
    );
    tv.style = GlyphStyle::Small;
    tv.text = String::from(text);
    gam.post_textview(&mut tv).ok();
}

pub fn draw(gam: &Gam, app: &SacredApp) {
    gam.clear().ok();

    match app.state {
        AppState::Selector => draw_selector(gam, app),
        AppState::Viewing => draw_viewing(gam, app),
    }

    gam.redraw().ok();
}

fn draw_selector(gam: &Gam, app: &SacredApp) {
    draw_header(gam, "Sacred Geometry");

    let names = geometry::figure_names();
    let start_y: i16 = 45;
    let line_h: i16 = 36;
    let visible = 11;

    let scroll = if app.cursor >= visible {
        app.cursor - visible + 1
    } else {
        0
    };

    for i in 0..names.len().min(visible) {
        let idx = scroll + i;
        if idx >= names.len() { break; }

        let y = start_y + (i as i16) * line_h;
        let selected = idx == app.cursor;

        if selected {
            gam.draw_rectangle(
                Point::new(4, y - 2),
                Point::new(SCREEN_W - 4, y + line_h - 6),
            ).ok();
        }

        let mut tv = TextView::new(
            TextBounds::BoundingBox(
                Point::new(12, y),
                Point::new(SCREEN_W - 12, y + line_h),
            ),
        );
        tv.style = GlyphStyle::Regular;
        if selected {
            tv.invert = true;
        }
        tv.text = format!("{}. {}", idx + 1, names[idx]);
        gam.post_textview(&mut tv).ok();
    }

    draw_footer(gam, "Up/Down: browse  Enter: view  Menu: quit");
}

fn draw_viewing(gam: &Gam, app: &SacredApp) {
    let names = geometry::figure_names();
    let title = if app.cursor < names.len() {
        names[app.cursor]
    } else {
        "Unknown"
    };
    draw_header(gam, title);

    // Draw all primitives
    for prim in &app.primitives {
        match *prim {
            Primitive::Line { x0, y0, x1, y1 } => {
                // Clip to content area
                if y0 >= HEADER_H && y1 >= HEADER_H && y0 < FOOTER_Y && y1 < FOOTER_Y
                   && x0 >= 0 && x1 >= 0 && x0 < SCREEN_W && x1 < SCREEN_W
                {
                    gam.draw_line(Line::new(
                        Point::new(x0, y0),
                        Point::new(x1, y1),
                    )).ok();
                }
            }
            Primitive::Circle { cx, cy, r } => {
                draw_circle(gam, cx, cy, r);
            }
        }
    }

    draw_footer(gam, "Up/Down: resize  Enter/Left: back");
}

/// Draw a circle using Bresenham's midpoint algorithm.
fn draw_circle(gam: &Gam, cx: i16, cy: i16, r: i16) {
    if r <= 0 { return; }

    // Use line segments to approximate the circle (32 segments)
    let segments = 32i32;
    for i in 0..segments {
        let a0 = i * 1024 / segments;
        let a1 = (i + 1) * 1024 / segments;

        let (x0, y0) = circle_point(cx, cy, r, a0);
        let (x1, y1) = circle_point(cx, cy, r, a1);

        if y0 >= HEADER_H && y1 >= HEADER_H && y0 < FOOTER_Y && y1 < FOOTER_Y
           && x0 >= 0 && x1 >= 0 && x0 < SCREEN_W && x1 < SCREEN_W
        {
            gam.draw_line(Line::new(
                Point::new(x0, y0),
                Point::new(x1, y1),
            )).ok();
        }
    }
}

fn circle_point(cx: i16, cy: i16, r: i16, angle_1024: i32) -> (i16, i16) {
    let a = ((angle_1024 % 1024) + 1024) % 1024;
    let (idx, sign) = if a < 256 {
        (a, 1i32)
    } else if a < 512 {
        (512 - a, 1)
    } else if a < 768 {
        (a - 512, -1)
    } else {
        (1024 - a, -1)
    };
    let y_raw = (4 * idx * (256 - idx)) as i64;
    let sin_val = (y_raw * 1024 / (256 * 256) as i64) as i32 * sign;
    let cos_val = {
        let ca = ((a + 256) % 1024) as i32;
        let (cidx, csign) = if ca < 256 {
            (ca, 1i32)
        } else if ca < 512 {
            (512 - ca, 1)
        } else if ca < 768 {
            (ca - 512, -1)
        } else {
            (1024 - ca, -1)
        };
        let cy_raw = (4 * cidx * (256 - cidx)) as i64;
        (cy_raw * 1024 / (256 * 256) as i64) as i32 * csign
    };

    let px = cx as i32 + (r as i32 * cos_val / 1024);
    let py = cy as i32 + (r as i32 * sin_val / 1024);
    (px as i16, py as i16)
}
