//! Sacred geometry constructions.
//!
//! All constructions are compass-and-straightedge based, computed with
//! fixed-point integer math. Output is a list of drawing primitives.

extern crate alloc;
use alloc::vec::Vec;

/// Drawing primitives for the renderer.
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Line { x0: i16, y0: i16, x1: i16, y1: i16 },
    Circle { cx: i16, cy: i16, r: i16 },
}

/// Fixed-point sine (1024 units per revolution, output scaled by 1024).
fn sin_1k(angle: i32) -> i32 {
    let a = ((angle % 1024) + 1024) % 1024;
    let (idx, sign) = if a < 256 {
        (a, 1i32)
    } else if a < 512 {
        (512 - a, 1)
    } else if a < 768 {
        (a - 512, -1)
    } else {
        (1024 - a, -1)
    };
    let y = (4 * idx * (256 - idx)) as i64;
    let raw = (y * 1024 / (256 * 256) as i64) as i32;
    raw * sign
}

fn cos_1k(angle: i32) -> i32 {
    sin_1k(angle + 256)
}

/// Point on circle: (cx + r*cos(a)/1024, cy + r*sin(a)/1024)
fn point_on_circle(cx: i16, cy: i16, r: i16, angle_1024: i32) -> (i16, i16) {
    let x = cx as i32 + (r as i32 * cos_1k(angle_1024) / 1024);
    let y = cy as i32 + (r as i32 * sin_1k(angle_1024) / 1024);
    (x as i16, y as i16)
}

/// Generate a regular polygon (N sides) inscribed in a circle.
fn polygon(cx: i16, cy: i16, r: i16, n: i32, rotation: i32) -> Vec<Primitive> {
    let mut prims = Vec::new();
    for i in 0..n {
        let a0 = rotation + i * 1024 / n;
        let a1 = rotation + (i + 1) * 1024 / n;
        let (x0, y0) = point_on_circle(cx, cy, r, a0);
        let (x1, y1) = point_on_circle(cx, cy, r, a1);
        prims.push(Primitive::Line { x0, y0, x1, y1 });
    }
    prims
}

/// Generate a star polygon {n/k} (n points, connect every k-th).
fn star_polygon(cx: i16, cy: i16, r: i16, n: i32, k: i32, rotation: i32) -> Vec<Primitive> {
    let mut prims = Vec::new();
    for i in 0..n {
        let a0 = rotation + i * 1024 / n;
        let a1 = rotation + ((i + k) % n) * 1024 / n;
        let (x0, y0) = point_on_circle(cx, cy, r, a0);
        let (x1, y1) = point_on_circle(cx, cy, r, a1);
        prims.push(Primitive::Line { x0, y0, x1, y1 });
    }
    prims
}

// ─── Sacred Geometry Figures ───

/// Seed of Life: 7 overlapping circles (center + 6 at 60° intervals).
pub fn seed_of_life(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = Vec::new();
    prims.push(Primitive::Circle { cx, cy, r });
    for i in 0..6 {
        let angle = i * 1024 / 6;
        let (px, py) = point_on_circle(cx, cy, r, angle);
        prims.push(Primitive::Circle { cx: px, cy: py, r });
    }
    prims
}

/// Flower of Life: Seed of Life + outer ring of 12 more circles.
pub fn flower_of_life(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = seed_of_life(cx, cy, r);
    // Second ring: 12 circles at 2r distance, offset by 30°
    for i in 0..6 {
        let angle = i * 1024 / 6;
        let (px, py) = point_on_circle(cx, cy, 2 * r, angle);
        prims.push(Primitive::Circle { cx: px, cy: py, r });
    }
    for i in 0..6 {
        let angle = i * 1024 / 6 + 1024 / 12; // offset 30°
        let (px, py) = point_on_circle(cx, cy, 2 * r, angle);
        prims.push(Primitive::Circle { cx: px, cy: py, r });
    }
    prims
}

/// Metatron's Cube: 13 circles + all connecting lines between centers.
pub fn metatrons_cube(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = Vec::new();
    let mut centers: Vec<(i16, i16)> = Vec::new();

    // Center circle
    centers.push((cx, cy));
    prims.push(Primitive::Circle { cx, cy, r: r / 3 });

    // Inner ring (6 circles)
    for i in 0..6 {
        let angle = i * 1024 / 6;
        let (px, py) = point_on_circle(cx, cy, r / 2, angle);
        centers.push((px, py));
        prims.push(Primitive::Circle { cx: px, cy: py, r: r / 3 });
    }

    // Outer ring (6 circles)
    for i in 0..6 {
        let angle = i * 1024 / 6;
        let (px, py) = point_on_circle(cx, cy, r, angle);
        centers.push((px, py));
        prims.push(Primitive::Circle { cx: px, cy: py, r: r / 3 });
    }

    // Connect all 13 centers with lines
    for i in 0..centers.len() {
        for j in (i + 1)..centers.len() {
            prims.push(Primitive::Line {
                x0: centers[i].0, y0: centers[i].1,
                x1: centers[j].0, y1: centers[j].1,
            });
        }
    }

    prims
}

/// Sri Yantra (simplified): 9 interlocking triangles.
pub fn sri_yantra(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = Vec::new();

    // Outer circle
    prims.push(Primitive::Circle { cx, cy, r });

    // 4 upward triangles at decreasing sizes
    let up_sizes = [r, r * 4 / 5, r * 3 / 5, r * 2 / 5];
    for &s in &up_sizes {
        // Upward triangle (point at top)
        let rot = 768; // -90° = top
        prims.extend_from_slice(&polygon(cx, cy, s, 3, rot));
    }

    // 5 downward triangles at decreasing sizes
    let down_sizes = [r * 9 / 10, r * 7 / 10, r / 2, r * 3 / 10, r / 5];
    for &s in &down_sizes {
        // Downward triangle (point at bottom)
        let rot = 256; // 90° = bottom
        prims.extend_from_slice(&polygon(cx, cy, s, 3, rot));
    }

    prims
}

/// Vesica Piscis: two overlapping circles with centers on each other's circumference.
pub fn vesica_piscis(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let half = r / 2;
    let mut prims = Vec::new();
    prims.push(Primitive::Circle { cx: cx - half, cy, r });
    prims.push(Primitive::Circle { cx: cx + half, cy, r });
    // Outline the vesica (connecting intersection points)
    // Intersection height: sqrt(3)/2 * r ≈ r * 886 / 1024
    let h = (r as i32 * 886 / 1024) as i16;
    prims.push(Primitive::Line { x0: cx, y0: cy - h, x1: cx, y1: cy + h });
    prims
}

/// Torus of Life: Seed of Life with connecting arcs (drawn as polylines).
pub fn torus_of_life(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = seed_of_life(cx, cy, r);
    // Add outer bounding circle
    prims.push(Primitive::Circle { cx, cy, r: 2 * r });
    // Add inner circle
    prims.push(Primitive::Circle { cx, cy, r: r / 2 });
    prims
}

/// Hexagram (Star of David): two overlapping triangles.
pub fn hexagram(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = Vec::new();
    prims.push(Primitive::Circle { cx, cy, r });
    prims.extend_from_slice(&polygon(cx, cy, r, 3, 768)); // upward
    prims.extend_from_slice(&polygon(cx, cy, r, 3, 256)); // downward
    prims
}

/// Pentagram: 5-pointed star inside a circle.
pub fn pentagram(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = Vec::new();
    prims.push(Primitive::Circle { cx, cy, r });
    prims.extend_from_slice(&polygon(cx, cy, r, 5, 768)); // pentagon
    prims.extend_from_slice(&star_polygon(cx, cy, r, 5, 2, 768)); // star
    prims
}

/// Golden Spiral: quarter-circle arcs in golden rectangles.
pub fn golden_spiral(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = Vec::new();

    // Draw Fibonacci squares with quarter-circle arcs
    // Fibonacci: 1, 1, 2, 3, 5, 8, 13, 21, 34, 55
    let fib = [1i32, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let scale = r as i32 * 1024 / (55 * 1024); // scale to fit

    let mut x = cx as i32 - r as i32 / 2;
    let mut y = cy as i32 - r as i32 / 2;

    for (i, &f) in fib.iter().enumerate() {
        let s = (f * scale).max(1);
        // Draw the square
        let x0 = x as i16;
        let y0 = y as i16;
        let x1 = (x + s) as i16;
        let y1 = (y + s) as i16;
        prims.push(Primitive::Line { x0, y0, x1: x1, y1: y0 });
        prims.push(Primitive::Line { x0: x1, y0, x1: x1, y1: y1 });
        prims.push(Primitive::Line { x0: x1, y0: y1, x1: x0, y1: y1 });
        prims.push(Primitive::Line { x0: x0, y0: y1, x1: x0, y1: y0 });

        // Quarter-circle arc (approximate with line segments)
        let arc_segments = 8;
        for j in 0..arc_segments {
            let a0 = (i as i32 % 4) * 256 + j * 256 / arc_segments;
            let a1 = (i as i32 % 4) * 256 + (j + 1) * 256 / arc_segments;
            let acx = match i % 4 {
                0 => x + s,
                1 => x + s,
                2 => x,
                _ => x,
            };
            let acy = match i % 4 {
                0 => y + s,
                1 => y,
                2 => y,
                _ => y + s,
            };
            let (px0, py0) = point_on_circle(acx as i16, acy as i16, s as i16, a0);
            let (px1, py1) = point_on_circle(acx as i16, acy as i16, s as i16, a1);
            prims.push(Primitive::Line { x0: px0, y0: py0, x1: px1, y1: py1 });
        }

        // Move to next square position
        match i % 4 {
            0 => x += s,
            1 => y -= s,
            2 => x -= s,
            _ => y += s,
        }
    }

    prims
}

/// Enneagram: 9-pointed star in a circle.
pub fn enneagram(cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    let mut prims = Vec::new();
    prims.push(Primitive::Circle { cx, cy, r });
    prims.extend_from_slice(&polygon(cx, cy, r, 9, 768));
    prims.extend_from_slice(&star_polygon(cx, cy, r, 9, 4, 768));
    prims
}

/// Get all available figure names.
pub fn figure_names() -> &'static [&'static str] {
    &[
        "Seed of Life",
        "Flower of Life",
        "Metatron's Cube",
        "Sri Yantra",
        "Vesica Piscis",
        "Torus of Life",
        "Hexagram",
        "Pentagram",
        "Golden Spiral",
        "Enneagram",
    ]
}

/// Generate primitives for a figure by index.
pub fn generate(index: usize, cx: i16, cy: i16, r: i16) -> Vec<Primitive> {
    match index {
        0 => seed_of_life(cx, cy, r),
        1 => flower_of_life(cx, cy, r),
        2 => metatrons_cube(cx, cy, r),
        3 => sri_yantra(cx, cy, r),
        4 => vesica_piscis(cx, cy, r),
        5 => torus_of_life(cx, cy, r),
        6 => hexagram(cx, cy, r),
        7 => pentagram(cx, cy, r),
        8 => golden_spiral(cx, cy, r),
        9 => enneagram(cx, cy, r),
        _ => Vec::new(),
    }
}
