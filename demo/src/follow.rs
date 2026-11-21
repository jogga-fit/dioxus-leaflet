use dioxus::prelude::*;
use dioxus_leaflet::{
    Color, LatLng, Map, MapOptions, MapPosition, Marker, MarkerIcon, PathOptions, Polyline,
    TileLayer,
};
use std::f64::consts::TAU;

/// A point that goes around a ring near London, one step each second.
fn at(step: usize) -> LatLng {
    let a = step as f64 * TAU / 60.0;
    LatLng::new(51.505 + 0.004 * a.sin(), -0.09 + 0.006 * a.cos())
}

/// The map keeps a point that moves in its middle. A pull on the map stops this. "Re-centre" starts it again.
#[component]
pub fn Follow() -> Element {
    let mut step = use_signal(|| 0usize);
    let mut following = use_signal(|| true);
    let mut shown = use_signal(MapPosition::default);
    use_future(move || async move {
        loop {
            _ = document::eval("await new Promise(r => setTimeout(r, 1000))").await;
            step += 1;
        }
    });
    let trace = use_memo(move || vec![(0..=step()).map(at).collect::<Vec<_>>()]);
    // The cone points along the path. The heading turns 6 degrees to the left each step.
    let icon = use_memo(move || {
        Some(MarkerIcon::Html {
            html: format!(
                r#"<div class="you" style="transform: rotate(-{}deg)"><i class="cone"></i><i class="dot"></i></div>"#,
                6 * step()
            ),
            class_name: None,
            icon_size: Some((48, 48)),
            icon_anchor: Some((24, 24)),
        })
    });
    let point = at(step());
    let c = shown().coordinates;

    rsx! {
        Map {
            initial_position: MapPosition { coordinates: at(0), zoom: 15.0 },
            height: "70vh",
            options: MapOptions::default().with_tile_layer(TileLayer::openstreetmap()),
            center: following().then_some(point),
            on_drag: move |_| following.set(false),
            on_move: move |pos| shown.set(pos),
            Polyline {
                coordinates: trace,
                options: PathOptions { color: Color::new([0., 0.4, 1.]), weight: 4, ..Default::default() },
            }
            Marker { coordinate: point, icon }
        }
        p { id: "follow-state", "following: {following}, step: {step}" }
        p { id: "point", "{point.lat:.6} {point.lng:.6}" }
        p { id: "center", "{c.lat:.6} {c.lng:.6} {shown().zoom}" }
        button { id: "recentre", onclick: move |_| following.set(true), "Re-centre" }
    }
}
