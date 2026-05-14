import type { L, Id, FitBoundsOptions, MapOptions, MapPosition, Point, RustCallback, Json } from "./types";
import { setup, wait } from "./util";

const _maps = new Map<Id, L.Map>();
const _callbacks = new Map<Id, (map: L.Map) => void>();
const _promises = new Map<Id, Promise<L.Map>>();

function toPointExpression(point?: Point | null): L.PointTuple | undefined {
    return point ? [point.x, point.y] : undefined;
}

function toFitBoundsOptions(options?: FitBoundsOptions | null): L.FitBoundsOptions {
    const fitOptions: L.FitBoundsOptions = {};

    const paddingTopLeft = toPointExpression(options?.padding_top_left);
    const paddingBottomRight = toPointExpression(options?.padding_bottom_right);
    const padding = toPointExpression(options?.padding);

    if (paddingTopLeft) {
        fitOptions.paddingTopLeft = paddingTopLeft;
    }
    if (paddingBottomRight) {
        fitOptions.paddingBottomRight = paddingBottomRight;
    }
    if (padding) {
        fitOptions.padding = padding;
    }
    if (options?.max_zoom != null) {
        fitOptions.maxZoom = options.max_zoom;
    }
    if (options?.animate != null) {
        fitOptions.animate = options.animate;
    }
    if (options?.duration != null) {
        fitOptions.duration = options.duration;
    }
    if (options?.ease_linearity != null) {
        fitOptions.easeLinearity = options.ease_linearity;
    }
    if (options?.no_move_start != null) {
        fitOptions.noMoveStart = options.no_move_start;
    }

    return fitOptions;
}

export async function get_map(map_id: Id): Promise<L.Map> {
    let map = _maps.get(map_id);
    if (!map) {
        let p = _promises.get(map_id);
        if (!p) {
            p = new Promise<L.Map>((resolve) => {
                _callbacks.set(map_id, resolve);
            });
            _promises.set(map_id, p);
        }
        map = await p;
    }
    return map;
}

export async function update_map(map_id: Id, initial_position: MapPosition, options: MapOptions): Promise<void> {
    const l = await setup();

    // Initialize the map with options
    const map = _maps.get(map_id) ?? l.map(`dioxus-leaflet-map-${map_id}`, {
        zoomControl: options.zoom_control,
        scrollWheelZoom: options.scroll_wheel_zoom,
        doubleClickZoom: options.double_click_zoom,
        touchZoom: options.touch_zoom,
        dragging: options.dragging,
        keyboard: options.keyboard,
        attributionControl: options.attribution_control
    });

    map.setView(initial_position.coordinates, initial_position.zoom);

    // Add tile layer
    l.tileLayer(options.tile_layer.url, {
        attribution: options.tile_layer.attribution,
        maxZoom: options.tile_layer.max_zoom,
        subdomains: options.tile_layer.subdomains
    }).addTo(map);

    _maps.set(map_id, map);

    // Resolve any pending promises
    if (_callbacks.has(map_id)) {
        const callback = _callbacks.get(map_id)!;
        callback(map);
        _callbacks.delete(map_id);
        _promises.delete(map_id);
    }

    // Force resize to ensure proper display
    await wait(100);
    map.invalidateSize();

    const fit = options.fit_bounds;
    if (fit) {
        const bounds = l.latLngBounds(fit.bounds.south_west, fit.bounds.north_east);
        const size = map.getSize();
        if (bounds.isValid() && size.x > 0 && size.y > 0) {
            map.fitBounds(bounds, toFitBoundsOptions(fit.options));
        }
    }
}

export function delete_map(map_id: Id) {
    _maps.delete(map_id);
}

export async function on_map_click(map_id: Id, callback: RustCallback<number[], void>): Promise<void> {
    await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when setting onClick handler`);
    }

    map.on("click", async (e: L.LeafletMouseEvent) => {
        try {
            await callback([e.latlng.lat, e.latlng.lng]);
        } catch (error) {
            console.error("Error in on_map_click callback:", error);
        }
    });
}

export async function on_map_move(map_id: Id, callback: RustCallback<number[], void>): Promise<void> {
    await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when setting onMove handler`);
    }

    map.on("move", async () => {
        const center = map.getCenter();
        const zoom = map.getZoom();
        try {
            await callback([center.lat, center.lng, zoom]);
        } catch (error) {
            console.error("Error in map_on_move callback:", error);
        }
    });
}
