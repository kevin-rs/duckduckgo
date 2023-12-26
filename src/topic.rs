use crate::icon::Icon;
use serde::Deserialize;

/// A struct representing a topic in a DuckDuckGo search result.
#[derive(Debug, Deserialize, Default)]
pub struct Topic {
    /// The URL associated with the topic.
    #[serde(rename = "FirstURL")]
    pub first_url: Option<String>,

    /// The icon associated with the topic.
    #[serde(rename = "Icon")]
    pub icon: Option<Icon>,

    /// The result string associated with the topic.
    #[serde(rename = "Result")]
    pub result: Option<String>,

    /// The text description of the topic.
    #[serde(rename = "Text")]
    pub text: Option<String>,

    /// The URL associated with the topic.
    #[serde(rename = "URL")]
    pub url: Option<String>,
}
