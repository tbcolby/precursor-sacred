# Sacred Geometry

A compass-and-straightedge construction viewer for the [Precursor](https://precursor.dev) handheld, rendering 10 classic sacred geometry figures on the 336×536 monochrome 1-bit display.

## Features

- **10 sacred geometry figures** — from Seed of Life to Golden Spiral
- **Resizable** — Up/Down to scale figures in viewing mode
- **Integer math only** — All constructions use fixed-point trig, no floating-point
- **Primitive-based rendering** — Lines and circles as drawing primitives

## Figures

1. **Seed of Life** — 7 overlapping circles
2. **Flower of Life** — Seed of Life + 12 outer circles
3. **Metatron's Cube** — 13 circles with all connecting lines
4. **Sri Yantra** — 9 interlocking triangles in a circle
5. **Vesica Piscis** — Two overlapping circles
6. **Torus of Life** — Seed of Life with bounding circles
7. **Hexagram** — Star of David (two overlapping triangles)
8. **Pentagram** — 5-pointed star in a circle
9. **Golden Spiral** — Fibonacci squares with quarter-circle arcs
10. **Enneagram** — 9-pointed star in a circle

## Controls

| Key | Action |
|-----|--------|
| Up/Down | Browse figures / resize in viewer |
| Enter | View selected figure |
| Left/Enter/Menu | Return to selector |
| Menu | Quit from selector |

## Architecture

- `geometry.rs` — 10 construction functions, polygon/star helpers, integer trig, Primitive enum (Line/Circle)
- `app.rs` — 2-state machine (Selector → Viewing), radius adjustment
- `ui.rs` — Selector list, primitive renderer with circle approximation via 32-segment polylines
- `main.rs` — Standard Xous event loop

---

## Development

This app was developed using the methodology described in [xous-dev-toolkit](https://github.com/tbcolby/xous-dev-toolkit) — an LLM-assisted approach to Precursor app development on macOS ARM64.

---

## Author

Made by Tyler Colby — [Colby's Data Movers, LLC](https://colbysdatamovers.com)

Contact: [tyler@colbysdatamovers.com](mailto:tyler@colbysdatamovers.com) | [GitHub Issues](https://github.com/tbcolby/precursor-sacred/issues)

---

## License

Licensed under the Apache License, Version 2.0.

See [LICENSE](LICENSE) for the full text.
