//! The ten figures as lines and circles in pixel coordinates.
//!
//! Every figure is built from `fixed::polar` (real sine and cosine) around a
//! centre with a base radius `r`. [`Figure::extent`] says how far the figure
//! reaches from the centre in units of `r`, so [`Figure::fit_radius`] can pick
//! a size that fits the screen (the old fixed default of 120 put the Seed of
//! Life at twice the screen width).

use precursor_appkit_core::fixed::{cos, isqrt, polar, sin, step_angle, QUARTER};

use crate::spiral;

/// A pixel point.
pub type Pt = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    Line(Pt, Pt),
    /// Circle outline: centre and radius.
    Circle(Pt, i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Figure {
    SeedOfLife,
    FlowerOfLife,
    MetatronsCube,
    SriYantra,
    VesicaPiscis,
    TorusOfLife,
    Hexagram,
    Pentagram,
    GoldenSpiral,
    Enneagram,
}

pub const ALL: [Figure; 10] = [
    Figure::SeedOfLife,
    Figure::FlowerOfLife,
    Figure::MetatronsCube,
    Figure::SriYantra,
    Figure::VesicaPiscis,
    Figure::TorusOfLife,
    Figure::Hexagram,
    Figure::Pentagram,
    Figure::GoldenSpiral,
    Figure::Enneagram,
];

/// Fibonacci squares used by the spiral.
const SPIRAL_SQUARES: usize = 10;

/// Up is angle 270 degrees on screen (y grows downward).
const UP: i32 = 3 * QUARTER;
const DOWN: i32 = QUARTER;

impl Figure {
    /// Name as shown; figures that are simplified say so.
    pub fn label(self) -> &'static str {
        match self {
            Figure::SeedOfLife => "Seed of Life",
            Figure::FlowerOfLife => "Flower of Life",
            Figure::MetatronsCube => "Metatron's Cube",
            Figure::SriYantra => "Sri Yantra (simplified)",
            Figure::VesicaPiscis => "Vesica Piscis",
            Figure::TorusOfLife => "Torus of Life (simplified)",
            Figure::Hexagram => "Hexagram",
            Figure::Pentagram => "Pentagram",
            Figure::GoldenSpiral => "Golden Spiral",
            Figure::Enneagram => "Enneagram {9/4}",
        }
    }

    /// One line on how the figure is built (and what is simplified).
    pub fn note(self) -> &'static str {
        match self {
            Figure::SeedOfLife => "Seven circles of equal radius: one at the centre, six on its circumference.",
            Figure::FlowerOfLife => "Nineteen circles on a hexagonal lattice with spacing equal to the radius.",
            Figure::MetatronsCube => "Thirteen tangent circles (one, six, six) with all 78 lines between their centres.",
            Figure::SriYantra => "Simplified: nine nested equilateral triangles in a circle; they do not interlock as the traditional yantra does.",
            Figure::VesicaPiscis => "Two circles, each centred on the other's circumference, and the chord between their crossings.",
            Figure::TorusOfLife => "Simplified: a Seed of Life inside its bounding circle; the traditional torus has more arcs.",
            Figure::Hexagram => "Two equilateral triangles inscribed in a circle.",
            Figure::Pentagram => "A pentagon and its {5/2} star inscribed in a circle.",
            Figure::GoldenSpiral => "Ten Fibonacci squares with a quarter circle in each, joined into a spiral.",
            Figure::Enneagram => "The regular {9/4} star in a circle; not the Gurdjieff enneagram.",
        }
    }

    /// Greatest distance from the centre, as `(numerator, denominator)` times `r`.
    pub fn extent(self) -> (i32, i32) {
        match self {
            Figure::SeedOfLife | Figure::TorusOfLife => (2, 1),
            Figure::FlowerOfLife => (3, 1),
            Figure::MetatronsCube => (5, 4),
            Figure::VesicaPiscis => (3, 2),
            Figure::SriYantra | Figure::Hexagram | Figure::Pentagram | Figure::GoldenSpiral | Figure::Enneagram => (1, 1),
        }
    }

    /// Radius that makes the whole figure fit a `w` x `h` area with a margin.
    pub fn fit_radius(self, w: i32, h: i32) -> i32 {
        let (n, d) = self.extent();
        ((w.min(h) / 2 - 4) * d / n).max(8)
    }

    /// The figure's lines and circles around `c` at base radius `r`.
    pub fn build(self, c: Pt, r: i32) -> Vec<Prim> {
        match self {
            Figure::SeedOfLife => seed_of_life(c, r),
            Figure::FlowerOfLife => flower_of_life(c, r),
            Figure::MetatronsCube => metatrons_cube(c, r),
            Figure::SriYantra => sri_yantra(c, r),
            Figure::VesicaPiscis => vesica_piscis(c, r),
            Figure::TorusOfLife => {
                let mut v = seed_of_life(c, r);
                v.push(Prim::Circle(c, 2 * r));
                v
            }
            Figure::Hexagram => {
                let mut v = vec![Prim::Circle(c, r)];
                v.extend(polygon(c, r, 3, UP));
                v.extend(polygon(c, r, 3, DOWN));
                v
            }
            Figure::Pentagram => {
                let mut v = vec![Prim::Circle(c, r)];
                v.extend(polygon(c, r, 5, UP));
                v.extend(star_polygon(c, r, 5, 2, UP));
                v
            }
            Figure::GoldenSpiral => golden_spiral(c, r),
            Figure::Enneagram => {
                let mut v = vec![Prim::Circle(c, r)];
                v.extend(polygon(c, r, 9, UP));
                v.extend(star_polygon(c, r, 9, 4, UP));
                v
            }
        }
    }
}

/// Regular `n`-gon inscribed in radius `r`, first vertex at `start`.
pub fn polygon_points(c: Pt, r: i32, n: i32, start: i32) -> Vec<Pt> {
    (0..n).map(|k| polar(c, r, start + step_angle(k, n))).collect()
}

fn polygon(c: Pt, r: i32, n: i32, start: i32) -> Vec<Prim> {
    let p = polygon_points(c, r, n, start);
    (0..p.len()).map(|i| Prim::Line(p[i], p[(i + 1) % p.len()])).collect()
}

/// Star polygon {n/k}: every k-th vertex joined.
fn star_polygon(c: Pt, r: i32, n: i32, k: i32, start: i32) -> Vec<Prim> {
    let p = polygon_points(c, r, n, start);
    (0..p.len()).map(|i| Prim::Line(p[i], p[(i + k as usize) % p.len()])).collect()
}

/// Centres of the Seed of Life: the middle and six at distance `r`.
pub fn seed_centers(c: Pt, r: i32) -> Vec<Pt> {
    let mut v = vec![c];
    v.extend((0..6).map(|i| polar(c, r, step_angle(i, 6))));
    v
}

fn seed_of_life(c: Pt, r: i32) -> Vec<Prim> { seed_centers(c, r).into_iter().map(|p| Prim::Circle(p, r)).collect() }

/// Centres of the Flower of Life: hexagonal lattice, spacing `r`, two rings.
pub fn flower_centers(c: Pt, r: i32) -> Vec<Pt> {
    let mut v = seed_centers(c, r);
    // six at 2r (on the lattice axes), six at r * sqrt(3) (between them)
    v.extend((0..6).map(|i| polar(c, 2 * r, step_angle(i, 6))));
    let r3 = isqrt(3 * (r as u64) * (r as u64)) as i32;
    v.extend((0..6).map(|i| polar(c, r3, step_angle(2 * i + 1, 12))));
    v
}

fn flower_of_life(c: Pt, r: i32) -> Vec<Prim> { flower_centers(c, r).into_iter().map(|p| Prim::Circle(p, r)).collect() }

/// Metatron's cube: circle radius `r / 4` so neighbours are tangent; centres
/// at 0, `r / 2` (six) and `r` (six).
pub fn metatron_centers(c: Pt, r: i32) -> Vec<Pt> {
    let mut v = vec![c];
    v.extend((0..6).map(|i| polar(c, r / 2, step_angle(i, 6))));
    v.extend((0..6).map(|i| polar(c, r, step_angle(i, 6))));
    v
}

fn metatrons_cube(c: Pt, r: i32) -> Vec<Prim> {
    let centers = metatron_centers(c, r);
    let mut v: Vec<Prim> = centers.iter().map(|&p| Prim::Circle(p, r / 4)).collect();
    for i in 0..centers.len() {
        for j in i + 1..centers.len() {
            v.push(Prim::Line(centers[i], centers[j]));
        }
    }
    v
}

fn sri_yantra(c: Pt, r: i32) -> Vec<Prim> {
    let mut v = vec![Prim::Circle(c, r)];
    for s in [r * 9 / 10, r * 4 / 5, r * 3 / 5, r * 2 / 5] {
        v.extend(polygon(c, s, 3, UP));
    }
    for s in [r * 7 / 10, r / 2, r * 3 / 10, r / 5, r / 10] {
        v.extend(polygon(c, s, 3, DOWN));
    }
    v
}

/// Chord half-length of the vesica: `r * sqrt(3) / 2`.
pub fn vesica_half_height(r: i32) -> i32 { (isqrt(3 * (r as u64) * (r as u64)) / 2) as i32 }

fn vesica_piscis(c: Pt, r: i32) -> Vec<Prim> {
    let h = vesica_half_height(r);
    vec![
        Prim::Circle((c.0 - r / 2, c.1), r),
        Prim::Circle((c.0 + r / 2, c.1), r),
        Prim::Line((c.0, c.1 - h), (c.0, c.1 + h)),
    ]
}

/// Segments per quarter-circle arc of the spiral.
const ARC_SEGMENTS: i32 = 8;

fn golden_spiral(c: Pt, r: i32) -> Vec<Prim> {
    let (squares, arcs) = spiral::layout(SPIRAL_SQUARES);
    let (x0, y0, x1, y1) = spiral::bounds(&squares);
    // pixels per unit, 16.16, so the half diagonal of the box is about r
    let (w, h) = ((x1 - x0) as u64, (y1 - y0) as u64);
    let half_diag_units = isqrt(w * w + h * h) as i64 / 2 + 1;
    let u = (r as i64) * 65536 / half_diag_units;
    let to_px = |xq: i64, yq: i64| -> Pt {
        // xq, yq are unit coordinates in 16.16
        let px = ((2 * xq - (x0 + x1) * 65536) * u) >> 33;
        let py = ((2 * yq - (y0 + y1) * 65536) * u) >> 33;
        (c.0 + px as i32, c.1 + py as i32)
    };
    let mut v = Vec::new();
    for q in &squares {
        let (a, b) = (to_px(q.x << 16, q.y << 16), to_px((q.x + q.s) << 16, (q.y + q.s) << 16));
        v.push(Prim::Line(a, (b.0, a.1)));
        v.push(Prim::Line((b.0, a.1), b));
        v.push(Prim::Line(b, (a.0, b.1)));
        v.push(Prim::Line((a.0, b.1), a));
    }
    for arc in &arcs {
        // point(phi) = centre + v0 cos(phi) + v1 sin(phi), v0 = start - centre, v1 = end - centre
        let v0 = (arc.start.0 - arc.center.0, arc.start.1 - arc.center.1);
        let v1 = (arc.end.0 - arc.center.0, arc.end.1 - arc.center.1);
        let pt = |j: i32| {
            let a = j * QUARTER / ARC_SEGMENTS;
            let (cs, sn) = (cos(a) as i64, sin(a) as i64);
            let xq = (arc.center.0 << 16) + v0.0 * cs + v1.0 * sn;
            let yq = (arc.center.1 << 16) + v0.1 * cs + v1.1 * sn;
            to_px(xq, yq)
        };
        for j in 0..ARC_SEGMENTS {
            v.push(Prim::Line(pt(j), pt(j + 1)));
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const C: Pt = (168, 220);

    fn d(p: Pt, q: Pt) -> f64 { (((p.0 - q.0).pow(2) + (p.1 - q.1).pow(2)) as f64).sqrt() }

    fn has(pts: &[Pt], want: (f64, f64), tol: f64) -> bool {
        pts.iter().any(|p| (p.0 as f64 - want.0).abs() <= tol && (p.1 as f64 - want.1).abs() <= tol)
    }

    fn circles(v: &[Prim]) -> Vec<(Pt, i32)> {
        v.iter().filter_map(|p| if let Prim::Circle(c, r) = *p { Some((c, r)) } else { None }).collect()
    }

    fn lines(v: &[Prim]) -> Vec<(Pt, Pt)> {
        v.iter().filter_map(|p| if let Prim::Line(a, b) = *p { Some((a, b)) } else { None }).collect()
    }

    /// Audit: three of the seven Seed of Life centres sat on the centre.
    #[test]
    fn seed_of_life_has_the_seven_exact_centres() {
        let r = 100;
        let pts = seed_centers(C, r);
        assert_eq!(pts.len(), 7);
        let mut want = vec![(168.0, 220.0)];
        for k in 0..6 {
            let a = (k as f64 * 60.0).to_radians();
            want.push((168.0 + 100.0 * a.cos(), 220.0 + 100.0 * a.sin()));
        }
        for w in &want {
            assert!(has(&pts, *w, 1.0), "missing centre {w:?} in {pts:?}");
        }
        let mut uniq = pts.clone();
        uniq.sort_unstable();
        uniq.dedup();
        assert_eq!(uniq.len(), 7, "{pts:?}");
        // every outer circle passes through the middle one's centre
        for &p in &pts[1..] {
            assert!((d(p, C) - r as f64).abs() <= 1.0);
        }
        assert_eq!(circles(&Figure::SeedOfLife.build(C, r)).len(), 7);
    }

    #[test]
    fn flower_of_life_centres_are_the_hex_lattice_within_two_steps() {
        let r = 60.0;
        let pts = flower_centers(C, 60);
        assert_eq!(pts.len(), 19);
        let mut n = 0;
        for i in -2i32..=2 {
            for j in -2i32..=2 {
                if i.abs().max(j.abs()).max((i + j).abs()) <= 2 {
                    n += 1;
                    let want = (168.0 + r * (i as f64 + j as f64 / 2.0), 220.0 + r * j as f64 * 3f64.sqrt() / 2.0);
                    assert!(has(&pts, want, 1.5), "lattice point ({i},{j}) = {want:?} missing from {pts:?}");
                }
            }
        }
        assert_eq!(n, 19);
        let mut uniq = pts.clone();
        uniq.sort_unstable();
        uniq.dedup();
        assert_eq!(uniq.len(), 19);
    }

    /// Audit: circles of r/3 with ring radii r/2 and r overlapped; tangent needs r/4.
    #[test]
    fn metatron_circles_are_tangent_and_all_78_lines_are_present() {
        let r = 120;
        let fig = Figure::MetatronsCube.build(C, r);
        let cs = circles(&fig);
        assert_eq!(cs.len(), 13);
        assert!(cs.iter().all(|c| c.1 == r / 4));
        let pts = metatron_centers(C, r);
        let tangent = 2.0 * (r / 4) as f64;
        for &p in &pts[1..7] {
            assert!((d(p, C) - tangent).abs() <= 1.0, "inner ring not tangent to the middle");
        }
        for k in 0..6 {
            assert!((d(pts[7 + k], pts[1 + k]) - tangent).abs() <= 1.0, "outer not tangent to inner");
        }
        assert_eq!(lines(&fig).len(), 78);
    }

    #[test]
    fn vesica_circles_pass_through_each_others_centres() {
        let r = 100;
        let fig = Figure::VesicaPiscis.build(C, r);
        let cs = circles(&fig);
        assert_eq!(cs.len(), 2);
        assert!((d(cs[0].0, cs[1].0) - r as f64).abs() <= 1.0);
        let (a, b) = lines(&fig)[0];
        for end in [a, b] {
            for (cc, rr) in &cs {
                assert!((d(end, *cc) - *rr as f64).abs() <= 1.5, "{end:?} not on circle {cc:?}");
            }
        }
    }

    #[test]
    fn polygons_and_stars_sit_on_their_circle() {
        for (fig, n) in [(Figure::Hexagram, 6), (Figure::Pentagram, 10), (Figure::Enneagram, 18)] {
            let r = 90;
            let v = fig.build(C, r);
            assert_eq!(lines(&v).len(), n, "{fig:?}");
            for (a, b) in lines(&v) {
                assert!((d(a, C) - r as f64).abs() <= 1.0 && (d(b, C) - r as f64).abs() <= 1.0, "{fig:?}");
            }
        }
        // the top vertex of the pentagram is straight up
        let top = polygon_points(C, 90, 5, UP)[0];
        assert_eq!(top, (168, 130));
    }

    /// Audit: the golden spiral overlapped and was a few pixels at small radii.
    #[test]
    fn golden_spiral_arcs_are_continuous_and_have_size_at_every_radius() {
        for r in [20, 40, 100, 200] {
            let v = Figure::GoldenSpiral.build(C, r);
            let ls = lines(&v);
            assert_eq!(ls.len(), SPIRAL_SQUARES * 4 + SPIRAL_SQUARES * ARC_SEGMENTS as usize);
            let arcs = &ls[SPIRAL_SQUARES * 4..];
            // consecutive arc segments chain end to start (also across arcs)
            for w in arcs.windows(2) {
                assert_eq!(w[0].1, w[1].0, "r={r}: spiral has a gap");
            }
            let (minx, maxx) = (ls.iter().map(|l| l.0 .0.min(l.1 .0)).min().unwrap(), ls.iter().map(|l| l.0 .0.max(l.1 .0)).max().unwrap());
            let (miny, maxy) = (ls.iter().map(|l| l.0 .1.min(l.1 .1)).min().unwrap(), ls.iter().map(|l| l.0 .1.max(l.1 .1)).max().unwrap());
            let span = (maxx - minx).max(maxy - miny);
            assert!(span as f64 >= r as f64 * 1.4 && span <= 2 * r + 4, "r={r}: span {span}");
        }
    }

    #[test]
    fn every_figure_fits_its_default_radius_in_the_area() {
        let (w, h) = (336, 440);
        for fig in ALL {
            let r = fig.fit_radius(w, h);
            let mut worst = 0.0f64;
            for p in fig.build((w / 2, h / 2), r) {
                match p {
                    Prim::Line(a, b) => {
                        for q in [a, b] {
                            worst = worst.max(d(q, (w / 2, h / 2)));
                        }
                    }
                    Prim::Circle(c, rr) => worst = worst.max(d(c, (w / 2, h / 2)) + rr as f64),
                }
            }
            let limit = (w.min(h) / 2) as f64;
            assert!(worst <= limit, "{fig:?}: reaches {worst} > {limit}");
            assert!(worst >= limit * 0.55, "{fig:?}: only reaches {worst} of {limit}");
        }
    }

    #[test]
    fn labels_are_honest_about_simplified_figures() {
        assert!(Figure::SriYantra.label().contains("simplified"));
        assert!(Figure::TorusOfLife.label().contains("simplified"));
        assert!(ALL.iter().all(|f| !f.note().is_empty()));
    }
}
