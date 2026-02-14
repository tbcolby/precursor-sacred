//! State machine for Sacred Geometry.
//!
//! States:
//!   Selector  — choose a sacred geometry figure
//!   Viewing   — display the figure with size adjustment

extern crate alloc;
use alloc::vec::Vec;

use crate::geometry::{self, Primitive};

const KEY_UP: char = '\u{F700}';
const KEY_DOWN: char = '\u{F701}';
const KEY_LEFT: char = '\u{F702}';
const KEY_RIGHT: char = '\u{F703}';
const KEY_ENTER: char = '\u{000D}';
const KEY_MENU: char = '\u{2234}';

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    Selector,
    Viewing,
}

pub struct SacredApp {
    pub state: AppState,
    pub needs_redraw: bool,

    pub cursor: usize,
    pub radius: i16,

    // Precomputed drawing primitives
    pub primitives: Vec<Primitive>,
}

impl SacredApp {
    pub fn new() -> Self {
        Self {
            state: AppState::Selector,
            needs_redraw: true,
            cursor: 0,
            radius: 120,
            primitives: Vec::new(),
        }
    }

    fn compute_figure(&mut self) {
        let cx: i16 = 168;
        let cy: i16 = 260;
        self.primitives = geometry::generate(self.cursor, cx, cy, self.radius);
    }

    pub fn handle_key(&mut self, key: char) -> bool {
        self.needs_redraw = true;
        match self.state {
            AppState::Selector => self.handle_selector(key),
            AppState::Viewing => self.handle_viewing(key),
        }
    }

    fn handle_selector(&mut self, key: char) -> bool {
        let names = geometry::figure_names();
        match key {
            KEY_MENU => return false,
            KEY_UP => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            KEY_DOWN => {
                if self.cursor < names.len() - 1 {
                    self.cursor += 1;
                }
            }
            KEY_ENTER => {
                self.compute_figure();
                self.state = AppState::Viewing;
            }
            _ => {}
        }
        true
    }

    fn handle_viewing(&mut self, key: char) -> bool {
        match key {
            KEY_MENU | KEY_LEFT | KEY_ENTER => {
                self.state = AppState::Selector;
            }
            KEY_UP => {
                // Increase size
                self.radius = (self.radius + 10).min(200);
                self.compute_figure();
            }
            KEY_DOWN => {
                // Decrease size
                self.radius = (self.radius - 10).max(40);
                self.compute_figure();
            }
            _ => {}
        }
        true
    }
}
