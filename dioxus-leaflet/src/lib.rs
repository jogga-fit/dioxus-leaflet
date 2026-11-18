//! # Dioxus Leaflet
//!
//! A general-purpose Leaflet map component for Dioxus apps.
//!
//! ## Functions
//!
//! - A map component with markers that you can change.
//! - Popups and your own styles.
//! - A marker system that you can extend.
//! - Props that are almost the same as TypeScript props.
//! - Leaflet from a CDN, at a version that you select, or from local files.
//! - Leaflet resources with an integrity check.
//!
//! ## Basic use
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_leaflet::{LatLng, Map, MapPosition, Marker, Popup};
//!
//! fn App() -> Element {
//!     rsx! {
//!         Map {
//!             initial_position: MapPosition::new(51.505, -0.09, 13.0),
//!             height: "400px",
//!             width: "100%",
//!             Marker {
//!                 coordinate: LatLng::new(51.505, -0.09),
//!                 Popup {
//!                     b { "London" }
//!                     br { }
//!                     "Capital of England"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Other configurations
//!
//! ### A specified Leaflet version from a CDN
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_leaflet::{Map, MapPosition, MapOptions, LeafletResources};
//!
//! fn App() -> Element {
//!     let options = MapOptions::default()
//!         .with_leaflet_resources(LeafletResources::cdn("1.9.3"));
//!
//!     rsx! {
//!         Map {
//!             initial_position: MapPosition::default(),
//!             options: options,
//!             height: "400px",
//!             width: "100%"
//!         }
//!     }
//! }
//! ```
//!
//! ### Local Leaflet files
//!
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_leaflet::{Map, MapPosition, MapOptions, LeafletResources};
//!
//! fn App() -> Element {
//!     let options = MapOptions::default()
//!         .with_leaflet_resources(LeafletResources::local(
//!             "/static/css/leaflet.css",
//!             "/static/js/leaflet.js"
//!         ));
//!
//!     rsx! {
//!         Map {
//!             initial_position: MapPosition::default(),
//!             options: options,
//!             height: "400px",
//!             width: "100%"
//!         }
//!     }
//! }
//! ```
mod components;
mod interop;
mod types;

pub use components::{Map, Marker, Polygon, Polyline, Popup};
pub use types::{
    Color, FitBoundsOptions, LatLng, LatLngBounds, LeafletResources, LineCap, LineJoin,
    MapFitBounds, MapOptions, MapPosition, MarkerIcon, PathOptions, Point, PopupOptions, TileLayer,
};
