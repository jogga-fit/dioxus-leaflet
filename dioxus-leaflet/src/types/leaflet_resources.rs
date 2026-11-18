use serde::{Deserialize, Serialize};

/// The location of the Leaflet CSS and JavaScript files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LeafletResources {
    /// Get the specified version from a CDN.
    Cdn {
        version: String,
        /// The start of the URL of a different CDN. With `None`, the CDN is `unpkg.com`.
        base_url: Option<String>,
    },
    /// Get local files.
    Local { css_path: String, js_path: String },
}

impl LeafletResources {
    pub fn cdn(version: impl Into<String>) -> Self {
        Self::Cdn {
            version: version.into(),
            base_url: None,
        }
    }

    pub fn cdn_with_base_url(version: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::Cdn {
            version: version.into(),
            base_url: Some(base_url.into()),
        }
    }

    pub fn local(css_path: impl Into<String>, js_path: impl Into<String>) -> Self {
        Self::Local {
            css_path: css_path.into(),
            js_path: js_path.into(),
        }
    }

    pub fn css_url(&self) -> String {
        match self {
            Self::Cdn { version, base_url } => {
                let base = base_url.as_deref().unwrap_or("https://unpkg.com");
                format!("{}/leaflet@{}/dist/leaflet.css", base, version)
            }
            Self::Local { css_path, .. } => css_path.clone(),
        }
    }

    pub fn js_url(&self) -> String {
        match self {
            Self::Cdn { version, base_url } => {
                let base = base_url.as_deref().unwrap_or("https://unpkg.com");
                format!("{}/leaflet@{}/dist/leaflet.js", base, version)
            }
            Self::Local { js_path, .. } => js_path.clone(),
        }
    }

    /// The integrity hash of the CSS file. Only known versions from `unpkg.com` have a hash.
    pub fn css_integrity(&self) -> Option<String> {
        match self {
            Self::Cdn { version, base_url } if base_url.is_none() => match version.as_str() {
                "1.9.4" => Some("sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=".to_string()),
                "1.9.3" => Some("sha256-kLaT2GOSpHechhsozzB+flnD+zUyjE2LlfWPgU04xyI=".to_string()),
                "1.9.2" => Some("sha256-sA+zWATbFveLLNqWO2gtiw3HL/lh1giY/Inf1BJ0z14=".to_string()),
                _ => None,
            },
            _ => None,
        }
    }

    /// The integrity hash of the JavaScript file. Only known versions from `unpkg.com` have a hash.
    pub fn js_integrity(&self) -> Option<String> {
        match self {
            Self::Cdn { version, base_url } if base_url.is_none() => match version.as_str() {
                "1.9.4" => Some("sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=".to_string()),
                "1.9.3" => Some("sha256-WBkoXOwTeyKclOHuWtc+i2uENFpDZ9YPdf5Hf+D7ewM=".to_string()),
                "1.9.2" => Some("sha256-o9N4PsYA2zOcVD5OHEHviWpTGQ4Q1jEzU7oJiE+zRCE=".to_string()),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Default for LeafletResources {
    fn default() -> Self {
        Self::cdn("1.9.4")
    }
}
