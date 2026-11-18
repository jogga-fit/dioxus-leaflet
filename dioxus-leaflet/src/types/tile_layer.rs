use serde::{Deserialize, Serialize};

/// The configuration of a tile layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileLayer {
    pub url: String,
    pub attribution: String,
    pub max_zoom: u8,
    pub subdomains: Vec<String>,
    /// A protomaps-leaflet flavor: `light`, `dark`, `white`, `grayscale` or `black`.
    /// With a flavor, `url` is a vector tile source and the browser paints the tiles.
    /// The source is a `{z}/{x}/{y}.mvt` template or a `.pmtiles` file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,
}

impl TileLayer {
    /// The OpenStreetMap tile layer. This is the default.
    pub fn openstreetmap() -> Self {
        Self {
            url: "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png".to_string(),
            attribution: "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors".to_string(),
            max_zoom: 19,
            subdomains: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            flavor: None,
        }
    }

    /// A tile layer of satellite images.
    pub fn satellite() -> Self {
        Self {
            url: "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}".to_string(),
            attribution: "Tiles &copy; Esri".to_string(),
            max_zoom: 18,
            subdomains: vec![],
            flavor: None,
        }
    }
}

impl Default for TileLayer {
    fn default() -> Self {
        Self::openstreetmap()
    }
}
