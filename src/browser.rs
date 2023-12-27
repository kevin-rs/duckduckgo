use crate::colors::AnsiColor;
use crate::colors::AnsiStyle;
use crate::response::Response;
use crate::topic::Topic;
use reqwest;

const BASE_URL: &str = "https://api.duckduckgo.com/";

/// Enum representing different result formats for DuckDuckGo searches.
pub enum ResultFormat {
    /// Display search results in a list format.
    List,
    /// Display search results in a detailed format.
    Detailed,
}

/// A struct representing a browser for interacting with the DuckDuckGo API.
pub struct Browser {
    /// The underlying HTTP client used for making requests.
    pub client: reqwest::Client,
}

impl Browser {
    /// Creates a new instance of `Browser` with the specified HTTP client.
    ///
    /// # Arguments
    /// * `client` - The reqwest HTTP client to be used for making requests.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::browser::Browser;
    /// use reqwest::Client;
    ///
    /// let client = Client::new();
    /// let browser = Browser::new(client);
    /// ```
    pub fn new(client: reqwest::Client) -> Self {
        Browser { client }
    }

    /// Performs a DuckDuckGo search based on the provided path, result format, and optional result limit.
    ///
    /// # Arguments
    /// * `path` - The path to be appended to the DuckDuckGo API base URL.
    /// * `result_format` - The format in which the search results should be displayed (List or Detailed).
    /// * `limit` - Optional limit for the number of search results to be displayed.
    ///
    /// # Returns
    /// `Result<(), reqwest::Error>` - Result indicating success or failure of the search operation.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::browser::{Browser, ResultFormat};
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let browser = Browser::new(client);
    ///     browser.browse("?q=Rust", ResultFormat::List, Some(5)).await.unwrap();
    /// }
    /// ```
    pub async fn browse(
        &self,
        path: &str,
        result_format: ResultFormat,
        limit: Option<usize>,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}{}&format=json", BASE_URL, path);
        let response = self.client.get(&url).send().await?.text().await?;

        let api_response: Response = serde_json::from_str(&response).unwrap();

        match result_format {
            ResultFormat::List => self.print_results_list(api_response, limit),
            ResultFormat::Detailed => self.print_results_detailed(api_response, limit),
        }

        Ok(())
    }

    /// Prints search results in list format.
    ///
    /// # Arguments
    /// * `api_response` - The response from the DuckDuckGo API.
    /// * `limit` - Optional limit for the number of search results to be displayed.
    pub fn print_results_list(&self, api_response: Response, limit: Option<usize>) {
        if let Some(heading) = api_response.heading {
            let style = AnsiStyle {
                bold: true,
                color: Some(AnsiColor::Gold),
            };
            println!(
                "{}{}{}",
                style.escape_code(),
                heading,
                AnsiStyle::reset_code()
            );
        }

        let topics = &api_response.related_topics;

        for (index, topic) in topics
            .iter()
            .enumerate()
            .take(limit.unwrap_or(topics.len()))
        {
            self.print_related_topic(index + 1, topic);
        }
    }

    /// Prints a related topic in a detailed format.
    ///
    /// # Arguments
    /// * `index` - The index of the related topic.
    /// * `topic` - The related topic to be printed.
    pub fn print_related_topic(&self, index: usize, topic: &Topic) {
        let style = AnsiStyle {
            bold: false,
            color: Some(AnsiColor::BrightGreen),
        };

        // Access fields directly instead of using `get`
        let text = match &topic.text {
            Some(t) => t,
            None => {
                return;
            }
        };

        let first_url = match &topic.first_url {
            Some(url) => url,
            None => {
                return;
            }
        };

        println!("{}. {} {}", index, text, style.escape_code());
        println!("URL: {}{}", first_url, style.escape_code());
        if let Some(icon) = &topic.icon {
            let style = AnsiStyle {
                bold: false,
                color: Some(AnsiColor::BrightBlue),
            };
            if !icon.url.is_empty() {
                let full_url = format!("https://duckduckgo.com{}", icon.url);
                println!("Image URL: {}{}", full_url, style.escape_code());
            }
        }
        println!("--------------------------------------------");
    }

    /// Prints search results in detailed format.
    ///
    /// # Arguments
    /// * `api_response` - The response from the DuckDuckGo API.
    /// * `limit` - Optional limit for the number of search results to be displayed.
    pub fn print_results_detailed(&self, api_response: Response, limit: Option<usize>) {
        if let Some(heading) = api_response.heading {
            let style = AnsiStyle {
                bold: true,
                color: None,
            };
            println!(
                "{}{}{}",
                style.escape_code(),
                heading,
                AnsiStyle::reset_code()
            );
        }

        if let Some(abstract_text) = api_response.abstract_text {
            let style = AnsiStyle {
                bold: false,
                color: Some(AnsiColor::LightGray),
            };
            println!("Abstract: {}{}", abstract_text, style.escape_code());
        }

        if let Some(abstract_source) = api_response.abstract_source {
            let style = AnsiStyle {
                bold: false,
                color: Some(AnsiColor::Purple),
            };
            println!(
                "Abstract Source: {}{}",
                abstract_source,
                style.escape_code()
            );
        }

        if let Some(abstract_url) = api_response.abstract_url {
            let style = AnsiStyle {
                bold: false,
                color: Some(AnsiColor::Silver),
            };
            println!("Abstract URL: {}{}", abstract_url, style.escape_code());
        }

        if let Some(image) = api_response.image {
            let style = AnsiStyle {
                bold: false,
                color: Some(AnsiColor::SkyBlue),
            };
            if !image.is_empty() {
                let full_url = format!("https://duckduckgo.com{}", image);
                println!("Image URL: {}{}", full_url, style.escape_code());
            }
        }

        let topics = &api_response.related_topics;

        for (index, topic) in topics
            .iter()
            .enumerate()
            .take(limit.unwrap_or(topics.len()))
        {
            self.print_related_topic(index + 1, topic);
        }
    }

    /// Performs a basic DuckDuckGo search with the provided parameters.
    ///
    /// # Arguments
    /// * `query` - The search query.
    /// * `safe_search` - A boolean indicating whether safe search is enabled.
    /// * `result_format` - The format in which the search results should be displayed (List or Detailed).
    /// * `limit` - Optional limit for the number of search results to be displayed.
    ///
    /// # Returns
    /// `Result<(), reqwest::Error>` - Result indicating success or failure of the search operation.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::browser::{Browser, ResultFormat};
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let browser = Browser::new(client);
    ///     browser.search("Rust", true, ResultFormat::Detailed, Some(5)).await.unwrap();
    /// }
    /// ```
    pub async fn search(
        &self,
        query: &str,
        safe_search: bool,
        result_format: ResultFormat,
        limit: Option<usize>,
    ) -> Result<(), reqwest::Error> {
        let safe_param = if safe_search { "&kp=1" } else { "&kp=-2" }; // Enable or disable safe search
        let path = format!("?q={}{}", query, safe_param);
        self.browse(&path, result_format, limit).await
    }

    /// Performs an advanced DuckDuckGo search with additional parameters.
    ///
    /// # Arguments
    /// * `query` - The search query.
    /// * `params` - Additional search parameters.
    /// * `safe_search` - A boolean indicating whether safe search is enabled.
    /// * `result_format` - The format in which the search results should be displayed (List or Detailed).
    /// * `limit` - Optional limit for the number of search results to be displayed.
    ///
    /// # Returns
    /// `Result<(), reqwest::Error>` - Result indicating success or failure of the search operation.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::browser::{Browser, ResultFormat};
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let browser = Browser::new(client);
    ///     browser.advanced_search("Rust", "lang:en", true, ResultFormat::Detailed, Some(5)).await.unwrap();
    /// }
    /// ```
    pub async fn advanced_search(
        &self,
        query: &str,
        params: &str,
        safe_search: bool,
        result_format: ResultFormat,
        limit: Option<usize>,
    ) -> Result<(), reqwest::Error> {
        let safe_param = if safe_search { "&kp=1" } else { "&kp=-2" }; // Enable or disable safe search
        let path = format!("?q={}&kl={}{}", query, params, safe_param);
        self.browse(&path, result_format, limit).await
    }

    /// Performs a DuckDuckGo search with custom search operators.
    ///
    /// # Arguments
    /// * `query` - The search query.
    /// * `operators` - Custom search operators.
    /// * `safe_search` - A boolean indicating whether safe search is enabled.
    /// * `result_format` - The format in which the search results should be displayed (List or Detailed).
    /// * `limit` - Optional limit for the number of search results to be displayed.
    ///
    /// # Returns
    /// `Result<(), reqwest::Error>` - Result indicating success or failure of the search operation.
    ///
    /// # Examples
    /// ```
    /// use duckduckgo::browser::{Browser, ResultFormat};
    /// use reqwest::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    ///     let browser = Browser::new(client);
    ///     browser.search_operators("Rust", "site:github.com", true, ResultFormat::List, Some(5)).await.unwrap();
    /// }
    /// ```
    pub async fn search_operators(
        &self,
        query: &str,
        operators: &str,
        safe_search: bool,
        result_format: ResultFormat,
        limit: Option<usize>,
    ) -> Result<(), reqwest::Error> {
        let safe_param = if safe_search { "&kp=1" } else { "&kp=-2" }; // Enable or disable safe search
        let path = format!("?q={}&{}{}", query, operators, safe_param);
        self.browse(&path, result_format, limit).await
    }
}
