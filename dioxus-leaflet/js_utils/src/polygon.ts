import { setup } from "./util";
import { get_map } from "./map";
import type { L, Id, Json } from "./types";
import { get_popup } from "./popup";

const _gons = new Map<Id, L.Polygon>();

export function get_polygon(polygon_id: Id): L.Polygon | undefined {
    return _gons.get(polygon_id);
}

export async function update_polygon(map_id: Id, polygon_id: Id, coordinates: L.LatLngLiteral[][][], options: L.PathOptions) {
    const l = await setup();
    const map = await get_map(map_id);
    if (!map) {
        throw new Error(`Map with id ${map_id} not found when updating polygon ${polygon_id}`);
    }

    // A cached polygon can belong to a previous (destroyed) map instance when
    // Dioxus reuses scope ids across remounts — re-create it on this map.
    let gon = _gons.get(polygon_id);
    if (!gon || !map.hasLayer(gon)) {
        gon?.remove();
        gon = l.polygon([]).addTo(map);
    }
    _gons.set(polygon_id, gon);

    gon.setLatLngs(coordinates);
    gon.setStyle(options);

    const popup = get_popup(polygon_id);
    if (popup) {
        gon.unbindPopup();
        gon.bindPopup(popup.body, popup.options);
    }
}

export async function delete_polygon(_map_id: Id, polygon_id: Id) {
    // Never wait for the map here: on unmount delete_map may have already
    // unregistered it, and awaiting get_map would park this delete until a
    // future map reuses the id. Layer.remove() works without the map.
    _gons.get(polygon_id)?.remove();
    _gons.delete(polygon_id);
}