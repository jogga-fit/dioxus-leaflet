use super::LatLng;
use serde::{Deserialize, Serialize};

/// Represents a rectangular geographical area.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatLngBounds {
    pub south_west: LatLng,
    pub north_east: LatLng,
}

impl LatLngBounds {
    pub const fn new(south_west: LatLng, north_east: LatLng) -> Self {
        Self {
            south_west,
            north_east,
        }
    }
}
