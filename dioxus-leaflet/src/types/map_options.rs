use super::{LatLngBounds, LeafletResources, Point, TileLayer};
use serde::{Deserialize, Serialize};

/// The configuration for the `fitBounds` method of a Leaflet map.
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FitBoundsOptions {
    pub padding_top_left: Option<Point>,
    pub padding_bottom_right: Option<Point>,
    pub padding: Option<Point>,
    pub max_zoom: Option<f64>,
    pub animate: Option<bool>,
    pub duration: Option<f64>,
    pub ease_linearity: Option<f64>,
    pub no_move_start: Option<bool>,
}

/// After the map starts, Leaflet fits the map to this area, with this configuration.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapFitBounds {
    pub bounds: LatLngBounds,
    pub options: FitBoundsOptions,
}

impl MapFitBounds {
    pub const fn new(bounds: LatLngBounds) -> Self {
        Self {
            bounds,
            options: FitBoundsOptions {
                padding_top_left: None,
                padding_bottom_right: None,
                padding: None,
                max_zoom: None,
                animate: None,
                duration: None,
                ease_linearity: None,
                no_move_start: None,
            },
        }
    }

    pub const fn with_options(mut self, options: FitBoundsOptions) -> Self {
        self.options = options;
        self
    }
}

/// The configuration of the map.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapOptions {
    pub zoom_control: bool,
    pub scroll_wheel_zoom: bool,
    pub double_click_zoom: bool,
    pub touch_zoom: bool,
    pub dragging: bool,
    pub keyboard: bool,
    pub attribution_control: bool,
    pub tile_layer: TileLayer,
    pub leaflet_resources: LeafletResources,
    pub fit_bounds: Option<MapFitBounds>,
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            zoom_control: true,
            scroll_wheel_zoom: true,
            double_click_zoom: true,
            touch_zoom: true,
            dragging: true,
            keyboard: true,
            attribution_control: true,
            tile_layer: TileLayer::default(),
            leaflet_resources: LeafletResources::default(),
            fit_bounds: None,
        }
    }
}

impl MapOptions {
    /// All controls are off. The tile layer is the default.
    pub fn minimal() -> Self {
        Self {
            zoom_control: false,
            scroll_wheel_zoom: false,
            double_click_zoom: false,
            touch_zoom: false,
            dragging: false,
            keyboard: false,
            attribution_control: false,
            tile_layer: TileLayer::default(),
            leaflet_resources: LeafletResources::default(),
            fit_bounds: None,
        }
    }

    pub fn with_zoom_control(mut self, enabled: bool) -> Self {
        self.zoom_control = enabled;
        self
    }

    pub fn with_scroll_wheel_zoom(mut self, enabled: bool) -> Self {
        self.scroll_wheel_zoom = enabled;
        self
    }

    pub fn with_double_click_zoom(mut self, enabled: bool) -> Self {
        self.double_click_zoom = enabled;
        self
    }

    pub fn with_touch_zoom(mut self, enabled: bool) -> Self {
        self.touch_zoom = enabled;
        self
    }

    pub fn with_dragging(mut self, enabled: bool) -> Self {
        self.dragging = enabled;
        self
    }

    pub fn with_keyboard(mut self, enabled: bool) -> Self {
        self.keyboard = enabled;
        self
    }

    pub fn with_attribution_control(mut self, enabled: bool) -> Self {
        self.attribution_control = enabled;
        self
    }

    pub fn with_tile_layer(mut self, tile_layer: TileLayer) -> Self {
        self.tile_layer = tile_layer;
        self
    }

    pub fn with_leaflet_resources(mut self, resources: LeafletResources) -> Self {
        self.leaflet_resources = resources;
        self
    }

    /// Leaflet fits the map to this area after the map starts.
    pub fn with_fit_bounds(mut self, fit_bounds: MapFitBounds) -> Self {
        self.fit_bounds = Some(fit_bounds);
        self
    }
}
