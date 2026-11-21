use serde::{Deserialize, Serialize};

/// The configuration of a marker icon: an image (`L.icon`) or HTML (`L.divIcon`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all_fields = "camelCase")]
pub enum MarkerIcon {
    Image {
        icon_url: String,
        icon_size: Option<(u32, u32)>,
        icon_anchor: Option<(u32, u32)>,
        popup_anchor: Option<(i32, i32)>,
        shadow_url: Option<String>,
        shadow_size: Option<(u32, u32)>,
    },
    /// Leaflet puts `html` into a `div`. With no `class_name`, the `div` has no class of its own.
    Html {
        html: String,
        class_name: Option<String>,
        icon_size: Option<(u32, u32)>,
        icon_anchor: Option<(u32, u32)>,
    },
}

impl MarkerIcon {
    /// An image icon with only a URL. All other fields are `None`.
    pub fn new(icon_url: impl Into<String>) -> Self {
        Self::Image {
            icon_url: icon_url.into(),
            icon_size: None,
            icon_anchor: None,
            popup_anchor: None,
            shadow_url: None,
            shadow_size: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MarkerIcon;
    use serde_json::json;

    #[test]
    fn wire_shape() {
        let image = serde_json::to_value(MarkerIcon::new("a.png")).unwrap();
        assert_eq!(image["iconUrl"], "a.png");
        assert!(image.get("html").is_none());
        let html = MarkerIcon::Html {
            html: "<b></b>".into(),
            class_name: None,
            icon_size: Some((20, 20)),
            icon_anchor: Some((10, 10)),
        };
        let value = serde_json::to_value(&html).unwrap();
        assert_eq!(value["iconSize"], json!([20, 20]));
        assert_eq!(serde_json::from_value::<MarkerIcon>(value).unwrap(), html);
    }
}
