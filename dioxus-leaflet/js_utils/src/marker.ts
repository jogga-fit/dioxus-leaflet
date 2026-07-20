import type { L, Id } from "./types";
import { get_map } from "./map";
import { setup } from "./util";
import { get_popup } from "./popup";

const _markers = new Map<Id, L.Marker>();

export function get_marker(marker_id: Id): L.Marker | undefined {
    return _markers.get(marker_id);
}

export async function update_marker(map_id: Id, marker_id: Id, coordinate: L.LatLngExpression, icon?: L.IconOptions) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating marker ${marker_id}`);
    }

    // A cached marker can belong to a previous (destroyed) map instance when
    // Dioxus reuses scope ids across remounts — re-create it on this map.
    let marker = _markers.get(marker_id);
    if (!marker || !map.hasLayer(marker)) {
        marker?.remove();
        marker = l.marker([0, 0]).addTo(map);
    }
    _markers.set(marker_id, marker);

    marker.setLatLng(coordinate);
    if (icon) {
        marker.setIcon(l.icon(icon));
    }

    const popup = get_popup(marker_id);
    if (popup) {
        marker.unbindPopup();
        marker.bindPopup(popup.body, popup.options);
    }
}

export async function delete_marker(_map_id: Id, marker_id: Id) {
    // Never wait for the map here: on unmount delete_map may have already
    // unregistered it, and awaiting get_map would park this delete until a
    // future map reuses the id — then remove the *new* marker.
    _markers.get(marker_id)?.remove();
    _markers.delete(marker_id);
}