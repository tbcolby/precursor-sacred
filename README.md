# Sacred Geometry

Ten compass-and-straightedge figures for the [Precursor](https://precursor.dev) handheld, drawn as real circles and lines on the 1-bit display.

Version 0.2. Not yet run on Renode or hardware: `capture.py` is written but unrun.

## Figures

| Figure | What is drawn |
|--------|---------------|
| Seed of Life | seven circles: one in the middle, six on its circumference |
| Flower of Life | nineteen circles on a hexagonal lattice (spacing = radius) |
| Metatron's Cube | thirteen tangent circles (1, 6, 6) with all 78 lines between centres |
| Sri Yantra (simplified) | nine nested equilateral triangles in a circle; they do not interlock as the traditional yantra does |
| Vesica Piscis | two circles, each through the other's centre, and the chord between their crossings |
| Torus of Life (simplified) | a Seed of Life in its bounding circle |
| Hexagram | two equilateral triangles in a circle |
| Pentagram | a pentagon and its {5/2} star in a circle |
| Golden Spiral | ten Fibonacci squares with a quarter circle in each |
| Enneagram {9/4} | the regular {9/4} star in a circle (not the Gurdjieff enneagram) |

## Features

- Each figure starts at a radius chosen from its own extent so the whole figure fits the canvas (the Flower of Life is a third of the screen's half-width per radius, a hexagram is the full half-width).
- Up/Down resize (to twice the fitting size). Oversize figures are clipped at the edge, not dropped: lines are clipped with `fixed::clip_line`, circles are clipped by the canvas and the header and footer bands are repainted over any spill.
- Left/Right step to the previous or next figure while viewing.
- All trigonometry is `precursor_appkit_core::fixed` (Q16.16 sine and cosine, tested against f64); there is no float maths at runtime.
- Selector shows a one-line description of what each figure is, including what is simplified.

## Keys

Every screen leaves with Backspace or F4 (the `∴` menu key belongs to the OS).

| Screen | Keys |
|--------|------|
| Selector | Up/Down choose, Enter view, Backspace/F4 leave the app |
| Figure | Up/Down size, Left/Right previous/next figure, Enter/Backspace/F4 back to the selector |

## Storage

Nothing is saved.

## Layout

```
core/          sacred-core: figures (lines + circles), Fibonacci spiral, selector state (18 tests)
src/main.rs    registration and message routing (no pump: nothing animates)
src/app.rs     keys -> core, drawing
```

Building: `bin/integrate --xous-root <xous-core> --grade <this repo>` from [xous-dev-toolkit](https://github.com/tbcolby/xous-dev-toolkit). Host tests: `cd core && cargo test`. Manifest: `manifest-fragment.json`, key `sacred` (GAM app name `gam::APP_NAME_SACRED`).

## Fixed from the 0.1 audit

- The shared sine was 0 at 90 degrees (present twice in this app), putting three of the seven Seed of Life circles on the centre and turning every circle into an X. Tests measure the seven Seed of Life centres, the nineteen lattice centres of the Flower of Life, polygon and star vertices on their circle, and the Metatron and vesica geometry.
- Segments with an endpoint off screen were silently rejected, and the default radius of 120 put the Seed of Life at twice the screen width: figures are clipped, and each has a default radius derived from its extent (test: every figure fits, and fills at least 55% of, the half-width).
- Golden spiral: squares overlapped and the scale was `r / 55`, a single pixel at the smaller radii. It is now a real Fibonacci tiling with joined quarter circles; tests check the squares tile their bounding rectangle with no overlap, every arc lies in its square with endpoints on corners, arcs join end to start with a continuous tangent, and the figure has size at every radius.
- Metatron's Cube used circles of r/3 on rings of r/2 and r (overlapping): circles are r/4, tangent (tested).
- The Flower of Life's outer ring put six circles at 30 degrees offset at distance 2r, which is not a lattice point: they are now at r times the square root of 3 (tested against the lattice).
- Circles were 32-segment polygons (with a comment claiming Bresenham): they are real GAM circle outlines.
- Sri Yantra and Torus of Life are labelled simplified, in the name and in the description.
- Redundant field names (clippy), Apple PUA arrows, `∴` as quit, `terminate_process`, `UxType::Chat`, hard-coded 336x536 layout: replaced by appkit keys, `relinquish_focus`, a framebuffer app sized from the canvas.

## Limitations

- GAM draws a circle by visiting every pixel of its bounding square, so large circles at the maximum size (2x fit) are slow; circles beyond 700 px radius are skipped.
- `Screen::circles` in appkit fills with white and would erase overlapping circles, so this app builds its own unfilled circle list (an appkit fix is pending).
- The Sri Yantra, Torus and Enneagram are simplified or plain geometric versions, as labelled.
- Figures are drawn at 1 px line width.

## Author

Made by Tyler Colby, [Colby's Data Movers, LLC](https://colbysdatamovers.com). Apache-2.0, see [LICENSE](LICENSE).
