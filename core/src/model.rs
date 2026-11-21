//! Selector / viewer state machine. The shell maps GAM keys to [`Key`] and
//! draws the primitives from [`Model::prims`].

use precursor_appkit_core::keys::Key;
use precursor_appkit_core::layout::ListState;

use crate::figures::{Figure, Prim, ALL};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Selector,
    Viewing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Stay,
    Leave,
}

/// Smallest radius offered.
pub const MIN_RADIUS: i32 = 8;

type CacheKey = (Figure, i32, i32, i32);

pub struct Model {
    pub screen: Screen,
    pub list: ListState,
    area: (i32, i32),
    visible: usize,
    /// `None` means "the default for this figure and area".
    radius: Option<i32>,
    cache: Option<(CacheKey, Vec<Prim>)>,
}

impl Default for Model {
    fn default() -> Self { Self::new() }
}

impl Model {
    pub fn new() -> Self {
        Self {
            screen: Screen::Selector,
            list: ListState::default(),
            area: (0, 0),
            visible: 8,
            radius: None,
            cache: None,
        }
    }

    pub fn figure(&self) -> Figure { ALL[self.list.selected] }

    pub fn labels(&self) -> Vec<String> { ALL.iter().map(|f| f.label().to_string()).collect() }

    pub fn set_visible(&mut self, rows: usize) { self.visible = rows.max(1); }

    /// Drawing area (below the header, above the footer).
    pub fn set_area(&mut self, w: i32, h: i32) { self.area = (w, h); }

    pub fn default_radius(&self) -> i32 { self.figure().fit_radius(self.area.0, self.area.1) }

    pub fn radius(&self) -> i32 { self.radius.unwrap_or_else(|| self.default_radius()) }

    pub fn title(&self) -> String { format!("{} r={}", self.figure().label(), self.radius()) }

    fn select(&mut self, delta: isize) {
        self.list.step_wrapping(delta, ALL.len(), self.visible);
        self.radius = None;
    }

    fn resize(&mut self, grow: bool) {
        let r = self.radius();
        let step = (r / 8).max(2);
        // up to twice the fitting size (the figure is then clipped, not dropped)
        let max = self.default_radius() * 2;
        self.radius = Some(if grow { (r + step).min(max) } else { (r - step).max(MIN_RADIUS) });
    }

    pub fn on_key(&mut self, key: Key) -> Outcome {
        match self.screen {
            Screen::Selector => match key {
                Key::Up => self.list.step(-1, ALL.len(), self.visible),
                Key::Down => self.list.step(1, ALL.len(), self.visible),
                Key::Enter => {
                    self.radius = None;
                    self.screen = Screen::Viewing;
                }
                k if k.is_back() => return Outcome::Leave,
                _ => {}
            },
            Screen::Viewing => match key {
                Key::Up => self.resize(true),
                Key::Down => self.resize(false),
                Key::Left => self.select(-1),
                Key::Right => self.select(1),
                Key::Enter => self.screen = Screen::Selector,
                k if k.is_back() => self.screen = Screen::Selector,
                _ => {}
            },
        }
        Outcome::Stay
    }

    /// Primitives for the current figure centred in the area, cached.
    pub fn prims(&mut self) -> &[Prim] {
        let key = (self.figure(), self.radius(), self.area.0, self.area.1);
        if self.cache.as_ref().map(|c| c.0) != Some(key) {
            let c = (self.area.0 / 2, self.area.1 / 2);
            self.cache = Some((key, key.0.build(c, key.1)));
        }
        &self.cache.as_ref().expect("just filled").1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_radius_depends_on_the_figure() {
        let mut m = Model::new();
        m.set_area(336, 440);
        let seed = m.default_radius();
        m.list.selected = 1; // Flower of Life
        let flower = m.default_radius();
        assert!(flower < seed, "flower {flower} should be smaller than seed {seed}");
        assert_eq!(seed, 82);
        m.list.selected = 6; // Hexagram
        assert_eq!(m.default_radius(), 164);
    }

    #[test]
    fn viewing_resizes_within_bounds_and_arrows_change_figure() {
        let mut m = Model::new();
        m.set_area(336, 440);
        m.on_key(Key::Enter);
        assert_eq!(m.screen, Screen::Viewing);
        let r0 = m.radius();
        m.on_key(Key::Up);
        assert!(m.radius() > r0);
        for _ in 0..100 {
            m.on_key(Key::Up);
        }
        assert_eq!(m.radius(), m.default_radius() * 2);
        for _ in 0..200 {
            m.on_key(Key::Down);
        }
        assert_eq!(m.radius(), MIN_RADIUS);
        m.on_key(Key::Right);
        assert_eq!(m.figure(), Figure::FlowerOfLife);
        assert_eq!(m.radius(), m.default_radius(), "a new figure starts at its own default size");
        m.on_key(Key::Left);
        m.on_key(Key::Left);
        assert_eq!(m.figure(), Figure::Enneagram, "wraps");
    }

    #[test]
    fn back_leaves_only_from_the_selector() {
        let mut m = Model::new();
        m.on_key(Key::Enter);
        assert_eq!(m.on_key(Key::Backspace), Outcome::Stay);
        assert_eq!(m.screen, Screen::Selector);
        assert_eq!(m.on_key(Key::F(4)), Outcome::Leave);
    }

    /// Audit: segments with an endpoint off screen were rejected; here oversize
    /// figures are still produced whole for the shell to clip.
    #[test]
    fn oversize_figures_are_kept_whole() {
        let mut m = Model::new();
        m.set_area(336, 440);
        m.on_key(Key::Enter);
        let n = m.prims().len();
        for _ in 0..100 {
            m.on_key(Key::Up);
        }
        assert_eq!(m.prims().len(), n);
    }

    #[test]
    fn prims_are_centred_and_cached_per_setting() {
        let mut m = Model::new();
        m.set_area(300, 300);
        let first = m.prims().to_vec();
        assert!(first.contains(&Prim::Circle((150, 150), m.radius())));
        m.on_key(Key::Enter);
        m.on_key(Key::Down);
        assert_ne!(first, m.prims());
    }
}
