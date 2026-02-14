# Sacred Geometry — Build Notes

## What This Is
Sacred geometry construction viewer for Precursor. Renders 10 classic figures using compass-and-straightedge constructions.

## Key Decisions
- **Primitive-based**: All figures generate Vec<Primitive> (Line/Circle), rendered by UI layer. Clean separation between geometry and rendering.
- **Circle via polyline**: GAM has no native circle API, so circles drawn as 32-segment polylines. Good enough for 1-bit display.
- **Polygon/star helpers**: `polygon()` and `star_polygon()` are reusable building blocks for many figures.
- **Resizable**: Radius adjustable 40-200px in viewing mode, figures recomputed on resize.
- **sqrt(3)/2 approx**: Vesica Piscis intersection height uses 886/1024 ≈ 0.866.

## Architecture
- 2 states: Selector, Viewing
- 10 figures, all pure functions in geometry.rs
- No storage, no TRNG — pure deterministic geometry
