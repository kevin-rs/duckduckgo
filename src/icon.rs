use serde::Deserialize;
use serde_json::Value;

/// A struct representing an icon associated with a DuckDuckGo search result.
#[derive(Debug, Deserialize)]
pub struct Icon {
    /// The height of the icon.
    #[serde(rename = "Height")]
    pub height: Value,

    /// The URL pointing to the icon image.
    #[serde(rename = "URL")]
    pub url: String,

    /// The width of the icon.
    #[serde(rename = "Width")]
    pub width: Value,
}

impl Icon {
    /// Creates a new instance of `Icon` with the specified height, URL, and width.
    ///
    /// # Arguments
    /// * `height` - The height of the icon.
    /// * `url` - The URL pointing to the icon image.
    /// * `width` - The width of the icon.
    ///
    /// # Examples
    /// ```
    /// use serde_json::Value;
    /// use duckduckgo::icon::Icon;
    ///
    /// let icon = Icon {
    ///     height: Value::Number(10.into()),
    ///     url: String::from("https://example.com/icon.png"),
    ///     width: Value::Number(20.into()),
    /// };
    /// ```
    pub fn new(height: Value, url: String, width: Value) -> Self {
        Icon { height, url, width }
    }
}
