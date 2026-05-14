export * as L from "./leaflet";

export type Json = string | number | boolean | null | { [key: string]: Json } | Json[];
export type RustCallback<A, R> = (arg: A) => Promise<R>;

export type Id = number;

export interface LatLng {
    lat: number,
    lng: number,
    alt?: number | null,
}

export interface MapPosition {
    coordinates: LatLng,
    zoom: number,
}

export interface Point {
    x: number,
    y: number,
}

export interface LatLngBounds {
    south_west: LatLng,
    north_east: LatLng,
}

export interface FitBoundsOptions {
    padding_top_left?: Point | null,
    padding_bottom_right?: Point | null,
    padding?: Point | null,
    max_zoom?: number | null,
    animate?: boolean | null,
    duration?: number | null,
    ease_linearity?: number | null,
    no_move_start?: boolean | null,
}

export interface MapFitBounds {
    bounds: LatLngBounds,
    options: FitBoundsOptions,
}

export interface MapOptions {
    zoom_control?: boolean,
    scroll_wheel_zoom?: L.Zoom,
    double_click_zoom?: L.Zoom,
    touch_zoom?: L.Zoom,
    dragging?: boolean,
    keyboard?: boolean,
    attribution_control?: boolean,
    tile_layer: TileLayer,
    fit_bounds?: MapFitBounds | null,
}

export interface TileLayer {
    url: string,
    attribution: string,
    max_zoom: number,
    subdomains: string[],
}
