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
