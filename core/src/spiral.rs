//! Golden spiral: a Fibonacci square tiling with a quarter circle in each
//! square. Pure integer geometry in "unit" coordinates (the first square has
//! side 1); [`crate::figures`] scales it to pixels.
//!
//! Construction: squares 1, 1, 2, 3, 5, ... are attached in turn to the right,
//! top, left and bottom of the bounding rectangle, so each new square's side
//! equals the rectangle's side it joins. The arc in each square starts where
//! the previous one ended with the same tangent, and turns the same way.
//! (The old version stepped by each square's own side and overlapped squares.)

/// An axis-aligned square: top-left `(x, y)` and side `s`, unit coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub x: i64,
    pub y: i64,
    pub s: i64,
}

/// A quarter circle: from `start` to `end` around `center`, radius `s`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Arc {
    pub start: (i64, i64),
    pub end: (i64, i64),
    pub center: (i64, i64),
    pub s: i64,
}

/// Fibonacci numbers 1, 1, 2, 3, ...
pub fn fib(n: usize) -> i64 {
    let (mut a, mut b) = (1i64, 1i64);
    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    a
}

/// The first `n` squares and their arcs.
pub fn layout(n: usize) -> (Vec<Square>, Vec<Arc>) {
    let mut squares = Vec::with_capacity(n);
    let mut arcs = Vec::with_capacity(n);
    // bounding rectangle: x0..x1, y0..y1
    let (mut x0, mut y0, mut x1, mut y1) = (0i64, 0i64, 0i64, 0i64);
    // arc cursor: point and heading
    let mut p = (0i64, 0i64);
    let mut h = (0i64, 1i64);
    for k in 0..n {
        let s = fib(k + 1);
        let sq = if k == 0 {
            (x1, y1) = (1, 1);
            Square { x: 0, y: 0, s: 1 }
        } else {
            match (k - 1) % 4 {
                0 => {
                    let q = Square { x: x1, y: y0, s };
                    x1 += s;
                    q
                }
                1 => {
                    let q = Square { x: x0, y: y0 - s, s };
                    y0 -= s;
                    q
                }
                2 => {
                    let q = Square { x: x0 - s, y: y0, s };
                    x0 -= s;
                    q
                }
                _ => {
                    let q = Square { x: x0, y: y1, s };
                    y1 += s;
                    q
                }
            }
        };
        squares.push(sq);
        // turn toward n = (h.y, -h.x): centre at p + s n, end at centre + s h
        let n = (h.1, -h.0);
        let center = (p.0 + s * n.0, p.1 + s * n.1);
        let end = (center.0 + s * h.0, center.1 + s * h.1);
        arcs.push(Arc { start: p, end, center, s });
        p = end;
        h = n;
    }
    (squares, arcs)
}

/// Bounding box `(min_x, min_y, max_x, max_y)` of the squares.
pub fn bounds(squares: &[Square]) -> (i64, i64, i64, i64) {
    let mut b = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for q in squares {
        b = (b.0.min(q.x), b.1.min(q.y), b.2.max(q.x + q.s), b.3.max(q.y + q.s));
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corners(q: &Square) -> [(i64, i64); 4] { [(q.x, q.y), (q.x + q.s, q.y), (q.x, q.y + q.s), (q.x + q.s, q.y + q.s)] }

    #[test]
    fn fibonacci_sides() {
        let got: Vec<i64> = (1..=10).map(fib).collect();
        assert_eq!(got, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }

    /// Audit: the old squares overlapped (the 2-square sat on top of the second 1-square).
    #[test]
    fn squares_tile_the_bounding_rectangle_without_overlap() {
        for n in 2..=10 {
            let (sq, _) = layout(n);
            let (x0, y0, x1, y1) = bounds(&sq);
            let area: i64 = sq.iter().map(|q| q.s * q.s).sum();
            assert_eq!(area, (x1 - x0) * (y1 - y0), "n={n}: gaps or overlaps");
            for (i, a) in sq.iter().enumerate() {
                for b in &sq[i + 1..] {
                    let disjoint = a.x + a.s <= b.x || b.x + b.s <= a.x || a.y + a.s <= b.y || b.y + b.s <= a.y;
                    assert!(disjoint, "n={n}: {a:?} overlaps {b:?}");
                }
            }
            // a Fibonacci rectangle: sides are consecutive Fibonacci numbers
            let (w, h) = (x1 - x0, y1 - y0);
            assert_eq!((w.min(h), w.max(h)), (fib(n), fib(n + 1)), "n={n}");
        }
    }

    #[test]
    fn each_square_side_equals_the_side_it_joins() {
        let (sq, _) = layout(10);
        for (k, q) in sq.iter().enumerate() {
            assert_eq!(q.s, fib(k + 1));
        }
    }

    #[test]
    fn arcs_join_end_to_start_and_sit_in_their_squares() {
        let (sq, arcs) = layout(10);
        for k in 0..10 {
            let (q, a) = (&sq[k], &arcs[k]);
            let c = corners(q);
            for p in [a.start, a.end, a.center] {
                assert!(c.contains(&p), "arc {k}: {p:?} is not a corner of {q:?}");
            }
            // endpoints are diagonal corners; the centre is a third corner
            assert_eq!(((a.end.0 - a.start.0).abs(), (a.end.1 - a.start.1).abs()), (q.s, q.s), "arc {k}");
            assert_eq!(a.s, q.s);
            if k + 1 < 10 {
                assert_eq!(a.end, arcs[k + 1].start, "arc {k} does not join arc {}", k + 1);
            }
        }
    }

    /// The tangent must be continuous where arcs join: the radius at the join
    /// of arc k and arc k+1 lies along one line through both centres.
    #[test]
    fn arcs_join_with_a_continuous_tangent() {
        let (_, arcs) = layout(10);
        for w in arcs.windows(2) {
            let j = w[0].end;
            let r0 = (j.0 - w[0].center.0, j.1 - w[0].center.1);
            let r1 = (j.0 - w[1].center.0, j.1 - w[1].center.1);
            assert_eq!(r0.0 * r1.1 - r0.1 * r1.0, 0, "radii at {j:?} are not collinear");
        }
    }
}
