use serde::{Deserialize, Serialize};

/// A point on the screen, in pixels.
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
