use color::{OpaqueColor, Srgb};
use serde::{Deserialize, Serialize};

pub type Color = OpaqueColor<Srgb>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LineJoin {
    Arcs,
    Bevel,
    Miter,
    #[serde(rename = "miter-clip")]
    MiterClip,
    Round,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathOptions {
    /// If `true`, Leaflet shows a line along the path. Set it to `false` to remove the edge of a polygon or a `Circle`.
    pub stroke: bool,

    #[serde(with = "color_format")]
    pub color: Color,

    /// The width of the line, in pixels.
    pub weight: u32,

    pub opacity: f32,

    /// The shape at the ends of the line.
    pub line_cap: LineCap,

    /// The shape at the corners of the line.
    pub line_join: LineJoin,

    /// If `true`, Leaflet fills the path with a color.
    pub fill: bool,

    /// The fill color. The default is the value of `color`.
    #[serde(with = "color_format")]
    pub fill_color: Color,

    pub fill_opacity: f32,
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            stroke: true,
            color: Color::from_rgb8(0x33, 0x88, 0xff),
            weight: 3,
            opacity: 1.0,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            fill: true,
            fill_color: Color::from_rgb8(0x33, 0x88, 0xff),
            fill_opacity: 0.2,
        }
    }
}

mod color_format {
    use super::Color;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let color = color.to_rgba8();
        serializer.serialize_str(&format!(
            "rgb({r}, {g}, {b})",
            r = color.r,
            g = color.g,
            b = color.b
        ))
    }

    pub fn deserialize<'de, D>(_: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        panic!("Deserializing colors not (yet) supported")
    }
}
