//! Keys -> core, and drawing. Lines go through `Screen::lines` (clipped to the
//! content area); circles are drawn as outlines by GAM and clipped to the
//! canvas, then the header and footer bands are repainted over any spill.

use gam::menu::{Circle, DrawStyle, PixelColor, Point};
use gam::GlyphStyle;
use precursor_appkit::keys::Key;
use precursor_appkit::screen::Screen;
use sacred_core::model::{Model, Outcome, Screen as View};
use sacred_core::Prim;

#[derive(PartialEq, Eq)]
pub enum Action {
    Stay,
    Leave,
}

pub struct App {
    screen: Screen,
    model: Model,
}

impl App {
    pub fn new(screen: Screen) -> Self { Self { screen, model: Model::new() } }

    pub fn screen(&self) -> &Screen { &self.screen }

    pub fn on_key(&mut self, key: Key) -> Action {
        match self.model.on_key(key) {
            Outcome::Leave => Action::Leave,
            Outcome::Stay => Action::Stay,
        }
    }

    pub fn on_foreground(&mut self) {
        self.screen.refresh_size();
        self.draw();
    }

    pub fn draw(&mut self) {
        self.screen.clear();
        match self.model.screen {
            View::Selector => self.draw_selector(),
            View::Viewing => self.draw_viewing(),
        }
        self.screen.flush();
    }

    fn draw_selector(&mut self) {
        let top = self.screen.header("Sacred Geometry");
        let bottom = self.screen.footer("↑↓ choose · ⏎ view · ⌫ back");
        let note_h = 3 * self.screen.line_height(GlyphStyle::Small) + 6;
        let labels = self.model.labels();
        let rows = self.screen.list(top, bottom - note_h, &labels, &self.model.list);
        self.model.set_visible(rows);
        self.screen.para(bottom - note_h + 4, bottom, GlyphStyle::Small, self.model.figure().note());
    }

    fn draw_viewing(&mut self) {
        let size = self.screen.size();
        let w = size.x as i32;
        // Same geometry as Screen::header / Screen::footer.
        let top = self.screen.line_height(GlyphStyle::Bold) + 8;
        let bottom = size.y - self.screen.line_height(GlyphStyle::Small) - 6;
        let h = (bottom - top).max(1) as i32;
        let dy = top as i32;
        self.model.set_area(w, h);
        let title = self.model.title();

        let mut segs = Vec::new();
        let mut circles = Vec::new();
        for p in self.model.prims() {
            match *p {
                Prim::Line((x0, y0), (x1, y1)) => segs.push(((x0, y0 + dy), (x1, y1 + dy))),
                Prim::Circle((cx, cy), r) => circles.push(((cx, cy + dy), r)),
            }
        }
        self.screen.lines(segs, ((0, dy), (w - 1, dy + h - 1)), 1);
        self.outline_circles(&circles, w, size.y as i32);

        // Repaint the bands over any circle that spilled into them.
        self.screen.fill(Point::new(0, 0), Point::new(size.x - 1, top - 5), PixelColor::Light);
        self.screen.fill(Point::new(0, bottom + 1), Point::new(size.x - 1, size.y - 1), PixelColor::Light);
        self.screen.header(&title);
        self.screen.footer("↑↓ size · ←→ figure · ⌫ back");
    }

    /// Circle outlines with no fill. (`Screen::circles` fills with white, which
    /// would erase the arcs of circles drawn earlier in an overlapping figure.)
    fn outline_circles(&self, circles: &[((i32, i32), i32)], w: i32, h: i32) {
        let style = DrawStyle { fill_color: None, stroke_color: Some(PixelColor::Dark), stroke_width: 1 };
        let mut list = gam::GamObjectList::new(self.screen.gid());
        for &((cx, cy), r) in circles {
            // skip circles wholly off the canvas, and absurd sizes (cost grows with r squared)
            if r <= 0 || r > 700 || cx + r < 0 || cy + r < 0 || cx - r >= w || cy - r >= h {
                continue;
            }
            let circ = Circle::new_with_style(Point::new(cx as isize, cy as isize), r as isize, style);
            if let Err(obj) = list.push(gam::GamObjectType::Circ(circ)) {
                self.screen.gam().draw_list(list).ok();
                list = gam::GamObjectList::new(self.screen.gid());
                list.push(obj).ok();
            }
        }
        self.screen.gam().draw_list(list).ok();
    }
}
