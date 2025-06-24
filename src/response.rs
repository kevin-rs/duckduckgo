use crate::topic::Topic;
use serde::Deserialize;
use serde_json::Value;

/// A struct representing the response received from the DuckDuckGo API.
#[derive(Debug, Deserialize)]
pub struct Response {
    /// The abstract text associated with the search result.
    #[serde(rename = "Abstract")]
    pub r#abstract: Option<String>,

    /// The source of the abstract, if available.
    #[serde(rename = "AbstractSource")]
    pub abstract_source: Option<String>,

    /// The detailed abstract text associated with the search result.
    #[serde(rename = "AbstractText")]
    pub abstract_text: Option<String>,

    /// The URL associated with the abstract, if available.
    #[serde(rename = "AbstractURL")]
    pub abstract_url: Option<String>,

    /// The answer to the query, if available.
    #[serde(rename = "Answer")]
    pub answer: Option<String>,

    /// The type of answer provided.
    #[serde(rename = "AnswerType")]
    pub answer_type: Option<String>,

    /// The definition associated with the search result.
    #[serde(rename = "Definition")]
    pub definition: Option<String>,

    /// The source of the definition, if available.
    #[serde(rename = "DefinitionSource")]
    pub definition_source: Option<String>,

    /// The URL associated with the definition, if available.
    #[serde(rename = "DefinitionURL")]
    pub definition_url: Option<String>,

    /// The entity associated with the search result.
    #[serde(rename = "Entity")]
    pub entity: Option<String>,

    /// The heading or title of the search result.
    #[serde(rename = "Heading")]
    pub heading: Option<String>,

    /// The URL of the image associated with the search result.
    #[serde(rename = "Image")]
    pub image: Option<String>,

    /// The height of the image.
    #[serde(rename = "ImageHeight")]
    pub image_height: Value,

    /// Indicates whether the image is a logo.
    #[serde(rename = "ImageIsLogo")]
    pub image_is_logo: Value,

    /// The width of the image.
    #[serde(rename = "ImageWidth")]
    pub image_width: Value,

    /// The infobox associated with the search result.
    #[serde(rename = "Infobox")]
    pub info_box: Option<Value>,

    /// The redirect URL, if the result is a redirect.
    #[serde(rename = "Redirect")]
    pub redirect: Option<String>,

    /// The list of related topics.
    #[serde(rename = "RelatedTopics")]
    pub related_topics: Vec<Topic>,

    /// The list of generic results.
    #[serde(rename = "Results")]
    pub results: Vec<Value>,

    /// The type of the response.
    #[serde(rename = "Type")]
    pub r#type: String,

    /// An example query associated with the response.
    #[serde(rename = "ExampleQuery")]
    pub example_query: Option<String>,

    /// The date when the response was created.
    #[serde(rename = "CreatedDate")]
    pub created_date: Option<String>,
}

/// Enum representing different result formats for DuckDuckGo searches.
pub enum ResultFormat {
    /// Display search results in a list format.
    List,
    /// Display search results in a detailed format.
    Detailed,
}

/// Represents a single image search result from DuckDuckGo.
pub struct ImageResult {
    /// The title or description of the image.
    pub title: String,
    /// The direct URL to the full-sized image.
    pub image: String,
    /// The URL to the image thumbnail (smaller preview).
    pub thumbnail: String,
    /// The URL of the page hosting the image.
    pub url: String,
    /// The height of the image in pixels.
    pub height: u32,
    /// The width of the image in pixels.
    pub width: u32,
    /// The source or provider of the image.
    pub source: String,
}

/// Represents a single news article result from DuckDuckGo.
pub struct NewsResult {
    /// The publication date of the news article in ISO-8601 format.
    pub date: String,
    /// The headline or title of the news article.
    pub title: String,
    /// A short excerpt or summary of the news article.
    pub body: String,
    /// The URL linking to the full news article.
    pub url: String,
    /// Optional URL of an image associated with the news article.
    pub image: Option<String>,
    /// The source or publisher of the news article.
    pub source: String,
}

/// Represents a single search result from DuckDuckGo Lite search.
#[derive(Debug, Clone)]
pub struct LiteSearchResult {
    /// The title or headline of the search result.
    pub title: String,
    /// The URL linked by the search result.
    pub url: String,
    /// A short snippet or preview text from the search result.
    pub snippet: String,
}
