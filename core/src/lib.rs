//! Sacred geometry constructions with no Xous dependencies.
//!
//! * [`figures`]: the ten figures as lines and circles in pixel coordinates.
//! * [`spiral`]: the Fibonacci square tiling and its quarter-circle arcs.
//! * [`model`]: selector / viewer state machine driven by decoded keys.

pub mod figures;
pub mod model;
pub mod spiral;

pub use figures::{Figure, Prim};
pub use model::{Model, Outcome, Screen};
