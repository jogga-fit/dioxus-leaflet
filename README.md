# Dioxus Leaflet

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE-APACHE)
[![Crates.io](https://img.shields.io/crates/v/dioxus-leaflet.svg)](https://crates.io/crates/dioxus-leaflet)
[![Documentation](https://docs.rs/dioxus-leaflet/badge.svg)](https://docs.rs/dioxus-leaflet)

A general-purpose [Leaflet](https://leafletjs.com/) map component for [Dioxus](https://dioxuslabs.com/). This crate gives a reactive map component that is easy to use in a Dioxus app.

## Features

- **Map objects as components**: Dioxus components for markers, polygons, and popups.
- **Interactive markers**, with popups and your own styles.
- **Reactive map components**: they update automatically when your data changes.
- **Leaflet from a CDN or from local files**: you can select the version on the CDN.
- **Leaflet resources with an integrity check**, so that a changed file from the CDN does not load.
- **Responsive design**: you can set the dimensions.
- **Tile layers that you can change**, for example OpenStreetMap and satellite images.
- **`MapOptions`** for the zoom, pan, and interaction controls.
- **Events** for map clicks and map movements.

## Screenshots

![Dioxus Leaflet Example 1](images/image1.png)

![Dioxus Leaflet Example 2](images/image2.png)

### It also works on a phone

![Dioxus Leaflet Mobile Example 1](images/image3.png)

![Dioxus Leaflet Mobile Example 2](images/image4.png)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dioxus-leaflet = "0.3.0"
dioxus = "0.7.0-rc.2"
```

## Quick Start

This is an example to start with:

```rust
use dioxus::prelude::*;
use dioxus_leaflet::{Map, MapPosition, Marker, Popup};

fn App() -> Element {
    rsx! {
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            height: "500px",
            width: "100%",
            Marker {
                coordinate: LatLng::new(51.505, -0.09),
                Popup {
                    b { "London" }
                    br { }
                    "Capital of England"
                }
            }
            Marker {
                coordinate: LatLng::new(48.8566, 2.3522),
                Popup {
                    b { "Paris" }
                    br { }
                    "Capital of France"
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
```

## Core Components

### Map Component

The primary `Map` component gives a Leaflet map with all functions. It can contain child components, for example `Marker` and `Polygon`:

```rust
rsx! {
    Map {
        initial_position: MapPosition::new(51.505, -0.09, 13.0),
        height: "400px",
        width: "100%",
        class: "my-custom-map",
        style: "border: 1px solid #ccc;",
        on_click: move |pos| {
            println!("Map clicked at: {}", pos);
        },
        // Add markers as child components
        Marker {
            coordinate: LatLng::new(51.505, -0.09),
            // Add popup as child of marker
            Popup {
                b { "London" }
                br {}
                "Capital of England"
            }
        }
        // Add polygons as child components
        Polygon {
            coordinates: vec![
                LatLng::new(51.5, -0.1),
                LatLng::new(51.5, 0.0),
                LatLng::new(51.4, 0.0),
            ],
            options: PathOptions {
                color: Color::new([1., 0., 0.]),
                fill: true,
                ..Default::default()
            },
            // Add popup to polygon too
            Popup {
                "A red triangle"
            }
        }
    }
}
```

### Map Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `initial_position` | `MapPosition` | London coordinates | The map center and zoom at the start |
| `markers` | `Vec<MapMarker>` | Empty | The markers on the map |
| `height` | `String` | `"500px"` | The height of the map container |
| `width` | `String` | `"100%"` | The width of the map container |
| `options` | `MapOptions` | Default | The map configuration |
| `class` | `String` | `""` | Added CSS classes |
| `style` | `String` | `""` | Added CSS styles |
| `on_marker_click` | `EventHandler<MapMarker>` | None | The callback after a click on a marker |
| `on_map_click` | `EventHandler<MapPosition>` | None | The callback after a click on the map |
| `on_map_move` | `EventHandler<MapPosition>` | None | The callback after the map moves |

## Leaflet Resources Configuration

You can configure how the app loads the Leaflet CSS and JavaScript files. Use a specified version from a CDN, or give local files.

### A CDN with the default version (1.9.4)

```rust
// Default configuration uses Leaflet 1.9.4 from unpkg.com
let options = MapOptions::default();
```

### A CDN with a specified version

```rust
use dioxus_leaflet::{MapOptions, LeafletResources};

let options = MapOptions::default()
    .with_leaflet_resources(LeafletResources::cdn("1.9.3"));
```

### A different CDN base URL

```rust
let options = MapOptions::default()
    .with_leaflet_resources(
        LeafletResources::cdn_with_base_url("1.9.4", "https://cdn.jsdelivr.net/npm")
    );
```

### Local files

```rust
let options = MapOptions::default()
    .with_leaflet_resources(LeafletResources::local(
        "/static/css/leaflet.css",
        "/static/js/leaflet.js"
    ));
```

### Leaflet Resource Options

| Method | Description | Security |
|--------|-------------|----------|
| `LeafletResources::cdn(version)` | Gets the specified version from the `unpkg.com` CDN | An integrity check for known versions |
| `LeafletResources::cdn_with_base_url(version, base_url)` | Gets the files from a different CDN base URL | No integrity check |
| `LeafletResources::local(css_path, js_path)` | Gets local files from the specified paths | No integrity check |

The integrity check is automatic for the known Leaflet versions (1.9.2, 1.9.3, 1.9.4) from the default `unpkg.com` CDN.

## Markers

### Basic Markers

```rust
let marker = MapMarker::new(51.505, -0.09, "London");
```

### Markers with more options

```rust
let marker = MapMarker::new(51.505, -0.09, "London")
    .with_description("The capital city of England")
    .with_custom_data("country", "UK")
    .with_custom_data("population", "8900000")
    .with_popup_options(PopupOptions {
        max_width: Some(250),
        close_button: Some(true),
        auto_close: Some(false),
        ..Default::default()
    });
```

### Your own marker icons

```rust
use dioxus_leaflet::MarkerIcon;

let custom_icon = MarkerIcon::new("https://example.com/custom-icon.png")
    .with_size(32, 32)
    .with_anchor(16, 32);

let marker = MapMarker::new(51.505, -0.09, "Custom Marker")
    .with_icon(custom_icon);
```

## Map Configuration

### Map Options

Use `MapOptions` to change how the map behaves. Each value has a good default:

```rust
use dioxus_leaflet::{MapOptions, TileLayer};

// Full configuration with all options specified
let options = MapOptions {
    zoom_control: true,
    scroll_wheel_zoom: true,
    double_click_zoom: false,
    touch_zoom: true,
    dragging: true,
    keyboard: true,
    attribution_control: true,
    tile_layer: TileLayer::satellite(), // Use satellite imagery
};

// Minimal configuration using builder pattern
let minimal_options = MapOptions::default()
    .with_double_click_zoom(false)
    .with_tile_layer(TileLayer::satellite());

// All controls disabled
let disabled_options = MapOptions::minimal()
    .with_tile_layer(TileLayer::satellite());

// Use default configuration
let default_options = MapOptions::default(); // All options set to sensible defaults

rsx! {
    Map {
        options: options,
        // ... other props
    }
}
```

### Available Map Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `zoom_control` | `bool` | `true` | Show or hide the zoom buttons |
| `scroll_wheel_zoom` | `bool` | `true` | Turn the scroll wheel zoom on or off |
| `double_click_zoom` | `bool` | `true` | Turn the double-click zoom on or off |
| `touch_zoom` | `bool` | `true` | Turn the touch zoom and pinch zoom on or off |
| `dragging` | `bool` | `true` | Turn the map drag on or off |
| `keyboard` | `bool` | `true` | Turn the keyboard controls on or off |
| `attribution_control` | `bool` | `true` | Show or hide the attribution control |
| `tile_layer` | `TileLayer` | OpenStreetMap | The tile layer configuration |
| `leaflet_resources` | `LeafletResources` | CDN v1.9.4 | The location of the Leaflet CSS and JavaScript files |

### Tile Layers

Select a tile layer provider:

```rust
use dioxus_leaflet::TileLayer;

// OpenStreetMap (default)
let osm_tiles = TileLayer::openstreetmap();

// Satellite imagery
let satellite_tiles = TileLayer::satellite();

// Custom tile layer
let custom_tiles = TileLayer {
    url: "https://{s}.tile.custom-provider.com/{z}/{x}/{y}.png".to_string(),
    attribution: "&copy; Custom Provider".to_string(),
    max_zoom: 18,
    subdomains: vec!["a".to_string(), "b".to_string()],
};
```

## Events

The map and the markers send these events:

```rust
fn App() -> Element {
    let mut selected_marker = use_signal(|| None::<MapMarker>);
    let mut map_center = use_signal(|| MapPosition::default());

    rsx! {
        Map {
            on_marker_click: move |marker| {
                selected_marker.set(Some(marker));
            },
            on_map_click: move |position| {
                println!("Clicked at: {}, {}", position.lat, position.lng);
            },
            on_map_move: move |position| {
                map_center.set(position);
            },
            // ... other props
        }
        
        // Display selected marker info
        if let Some(marker) = selected_marker.read().as_ref() {
            div {
                "Selected: {marker.title}"
                if let Some(desc) = &marker.description {
                    p { "{desc}" }
                }
            }
        }
    }
}
```

## Styles

### CSS classes

The component has these CSS classes. You can add styles to them:

- `.dioxus-leaflet-container`: the primary container.
- `.dioxus-leaflet-map`: the map element.

### Your own styles

```css
.dioxus-leaflet-container {
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.my-custom-map {
    border: 2px solid #007cba;
}
```

## Examples

### A basic map with a specified Leaflet version

```rust
use dioxus::prelude::*;
use dioxus_leaflet::{Map, MapPosition, MapMarker, MapOptions, LeafletResources};

fn App() -> Element {
    let markers = vec![
        MapMarker::new(51.505, -0.09, "London")
            .with_description("Capital of England"),
    ];

    let options = MapOptions::default()
        .with_leaflet_resources(LeafletResources::cdn("1.9.3"))
        .with_zoom_control(true);

    rsx! {
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 13.0),
            markers: markers,
            options: options,
            height: "400px",
            width: "100%"
        }
    }
}
```

### A map with local Leaflet files

```rust
use dioxus::prelude::*;
use dioxus_leaflet::{Map, MapPosition, MapMarker, MapOptions, LeafletResources};

fn OfflineMap() -> Element {
    let markers = vec![
        MapMarker::new(48.8566, 2.3522, "Paris")
            .with_description("Capital of France"),
    ];

    let options = MapOptions::default()
        .with_leaflet_resources(LeafletResources::local(
            "/assets/leaflet/leaflet.css",
            "/assets/leaflet/leaflet.js"
        ));

    rsx! {
        Map {
            initial_position: MapPosition::new(48.8566, 2.3522, 12.0),
            markers: markers,
            options: options,
            height: "500px",
            width: "100%"
        }
    }
}
```

### Tourist Map

```rust
use dioxus::prelude::*;
use dioxus_leaflet::{Map, MapPosition, MapMarker, PopupOptions};

fn TouristMap() -> Element {
    let attractions = vec![
        MapMarker::new(51.5074, -0.1278, "Big Ben")
            .with_description("Famous clock tower in London")
            .with_popup_options(PopupOptions {
                max_width: Some(200),
                ..Default::default()
            }),
        MapMarker::new(51.5033, -0.1195, "London Eye")
            .with_description("Giant Ferris wheel on the Thames"),
        MapMarker::new(51.5194, -0.1270, "British Museum")
            .with_description("World-famous museum with artifacts from around the globe"),
    ];

    rsx! {
        div { class: "tourist-map-container",
            h1 { "London Attractions" }
            Map {
                initial_position: MapPosition::new(51.5074, -0.1278, 12.0),
                markers: attractions,
                height: "100vh",
                width: "100vw",
                class: "attraction-map"
            }
        }
    }
}
```

### Real-time location track

```rust
fn LocationTracker() -> Element {
    let mut current_position = use_signal(|| MapPosition::default());
    let mut path_markers = use_signal(|| Vec::<MapMarker>::new());

    // Simulate location updates
    use_future(move || async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let new_pos = MapPosition::new(
                current_position.read().lat + (fastrand::f64() - 0.5) * 0.01,
                current_position.read().lng + (fastrand::f64() - 0.5) * 0.01,
                current_position.read().zoom,
            );
            current_position.set(new_pos.clone());
            
            let marker = MapMarker::new(new_pos.lat, new_pos.lng, "Current Location");
            path_markers.with_mut(|markers| markers.push(marker));
        }
    });

    rsx! {
        Map {
            initial_position: current_position.read().clone(),
            markers: path_markers.read().clone(),
            height: "500px",
            on_map_click: move |pos| {
                current_position.set(pos);
            }
        }
    }
}
```

## Contributing

We accept contributions. Send a pull request. For a large change, first open an issue to discuss the change.

## License

You can use this project with one of these licenses:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT) or [MIT](http://opensource.org/licenses/MIT))

You select the license.

## Acknowledgments

- [Leaflet](https://leafletjs.com/): the map library in this component.
- [Dioxus](https://dioxuslabs.com/): the reactive UI library for Rust.
- [OpenStreetMap](https://www.openstreetmap.org/): the free geographic data in the examples.

---

### Internet connection

- **CDN mode**: The app must have an internet connection to get Leaflet from the CDN. This is the default.
- **Local mode**: The app works offline with local Leaflet files.

### Local files

To use local Leaflet files:

1. Download Leaflet from [leafletjs.com](https://leafletjs.com/download.html).
2. Put the CSS and JavaScript files in your static assets directory.
3. Configure the paths with `LeafletResources::local()`.

An example directory structure:

```text
static/
├── css/
│   └── leaflet.css
├── js/
│   └── leaflet.js
```

Use:

```rust
let options = MapOptions::default()
    .with_leaflet_resources(LeafletResources::local(
        "/static/css/leaflet.css",
        "/static/js/leaflet.js"
    ));
```
