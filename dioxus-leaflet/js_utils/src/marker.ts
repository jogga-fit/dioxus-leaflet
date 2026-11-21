import type { L, Id, MarkerIcon } from "./types";
import { get_map } from "./map";
import { setup } from "./util";
import { get_popup } from "./popup";

const _markers = new Map<Id, L.Marker>();

export function get_marker(marker_id: Id): L.Marker | undefined {
    return _markers.get(marker_id);
}

export async function update_marker(map_id: Id, marker_id: Id, coordinate: L.LatLngExpression, icon?: MarkerIcon) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating marker ${marker_id}`);
    }

    // Dioxus can use a scope id again after a remount. Then the cached marker can belong to
    // a previous map instance that Leaflet removed. In this condition, make it again on this map.
    let marker = _markers.get(marker_id);
    if (!marker || !map.hasLayer(marker)) {
        marker?.remove();
        marker = l.marker([0, 0]).addTo(map);
    }
    _markers.set(marker_id, marker);

    marker.setLatLng(coordinate);
    if (icon) {
        // Leaflet gives a div icon the class `leaflet-div-icon`, which paints a white box.
        marker.setIcon("html" in icon ? l.divIcon({ ...icon, className: icon.className ?? "" }) : l.icon(icon));
    }

    const popup = get_popup(marker_id);
    if (popup) {
        marker.unbindPopup();
        marker.bindPopup(popup.body, popup.options);
    }
}

export async function delete_marker(_map_id: Id, marker_id: Id) {
    // Do not wait for the map here. On unmount, `delete_map` can remove the map first.
    // Then `await get_map` holds this call until a new map gets the same id.
    // At that time, it removes the *new* marker.
    _markers.get(marker_id)?.remove();
    _markers.delete(marker_id);
}