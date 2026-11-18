use crate::{LatLng, MapOptions, MapPosition, interop, types::Id};
use dioxus::{
    core::{spawn_forever, use_drop},
    prelude::*,
};
use std::rc::Rc;

const MAP_CSS: Asset = asset!("/assets/dioxus_leaflet.scss");
const PROTOMAPS_LEAFLET_JS: &str =
    "https://unpkg.com/protomaps-leaflet@5.1.0/dist/protomaps-leaflet.js";

/// The primary map component. Leaflet shows it.
#[component]
pub fn Map(
    /// The position of the map at the start.
    #[props(default = MapPosition::default())]
    initial_position: MapPosition,

    /// The height of the map container.
    #[props(into)]
    height: Option<String>,

    /// The width of the map container.
    #[props(into)]
    width: Option<String>,

    /// The configuration of the map.
    options: Option<MapOptions>,

    /// An added CSS class for the map container.
    #[props(into)]
    class: Option<String>,

    /// Added CSS styles for the map container.
    #[props(into)]
    style: Option<String>,

    /// The callback after a click on the map.
    on_click: Option<EventHandler<LatLng>>,

    /// The callback after the map moves.
    on_move: Option<EventHandler<MapPosition>>,

    children: Element,
) -> Element {
    let id = use_context_provider(|| Rc::new(Id::map(dioxus_core::current_scope_id().0)));
    let options = options.unwrap_or(MapOptions::default());
    let leaflet_css = options.leaflet_resources.css_url();
    let leaflet_js = options.leaflet_resources.js_url();
    let vector = options.tile_layer.flavor.is_some();
    let height = height.unwrap_or_else(|| "400px".to_string());
    let width = width.unwrap_or_else(|| "100%".to_string());
    let style = style.unwrap_or_default();
    let class = class.unwrap_or_default();

    let id2 = id.clone();
    let load_error = use_resource(move || {
        let id = id2.clone();
        let pos = initial_position;
        let opts = options.clone();
        async move {
            interop::update_map(&id, &pos, &opts)
                .await
                .map_err(|e| e.to_string())
        }
    });

    let id2 = id.clone();
    let _click_handle = use_resource(move || {
        let id = id2.clone();
        async move {
            if let Some(on_click) = on_click {
                interop::on_map_click(&id, on_click).await
            } else {
                Ok(())
            }
        }
    });

    let id2 = id.clone();
    let _move_handle = use_resource(move || {
        let id = id2.clone();
        async move {
            if let Some(on_move) = on_move {
                interop::on_map_move(&id, on_move).await
            } else {
                Ok(())
            }
        }
    });

    let id2 = id.clone();
    use_drop(move || {
        let id = id2.clone();
        spawn_forever(async move {
            _ = interop::delete_map(&id).await;
        });
    });

    rsx! {
        document::Style { href: leaflet_css }

        document::Style { href: MAP_CSS }

        document::Script { src: leaflet_js }

        // protomaps-leaflet is a `GridLayer` that shows the vector tiles on a canvas.
        if vector {
            document::Script { src: PROTOMAPS_LEAFLET_JS }
        }

        // This script starts the map.
        document::Script { src: interop::DL_JS, r#type: "module" }

        if let Some(Err(err)) = load_error() {
            p { "{err}" }
        } else {
            // Keep the dimensions and the style on the outer container. Leaflet must find them there when it puts the map into the page.
            div {
                class: "dioxus-leaflet-container {class}",
                style: "height: {height}; width: {width}; {style}",

                // Leaflet controls this element.
                div {
                    id: "dioxus-leaflet-map-{id.id()}",
                    class: "dioxus-leaflet-map",
                    {children}
                }
            }
        }
    }
}
