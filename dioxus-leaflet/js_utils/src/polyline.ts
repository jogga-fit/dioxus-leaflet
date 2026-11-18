import { setup } from "./util";
import { get_map } from "./map";
import type { L, Id } from "./types";
import { get_popup } from "./popup";

const _lines = new Map<Id, L.Polyline>();

export function get_polyline(polyline_id: Id): L.Polyline | undefined {
    return _lines.get(polyline_id);
}

export async function update_polyline(map_id: Id, polyline_id: Id, coordinates: L.LatLngLiteral[][], options: L.PathOptions) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating polyline ${polyline_id}`);
    }

    // Dioxus can use a scope id again after a remount. Then the cached line can belong to
    // a previous map instance that Leaflet removed. In this condition, make it again on this map.
    let line = _lines.get(polyline_id);
    if (!line || !map.hasLayer(line)) {
        line?.remove();
        line = l.polyline([]).addTo(map);
    }
    _lines.set(polyline_id, line);

    line.setLatLngs(coordinates);
    line.setStyle(options);

    const popup = get_popup(polyline_id);
    if (popup) {
        line.unbindPopup();
        line.bindPopup(popup.body, popup.options);
    }
}

export async function delete_polyline(_map_id: Id, polyline_id: Id) {
    // Do not wait for the map here. On unmount, `delete_map` can remove the map first.
    // Then `await get_map` holds this call until a new map gets the same id.
    // At that time, it removes the *new* polyline. `Layer.remove()` works without the map.
    _lines.get(polyline_id)?.remove();
    _lines.delete(polyline_id);
}
