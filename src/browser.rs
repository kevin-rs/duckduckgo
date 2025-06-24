use crate::colors::AnsiColor;
use crate::colors::AnsiStyle;
use crate::response::*;
use crate::topic::Topic;
use anyhow::{Context, Result};
use chrono::TimeZone;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use serde_json::Value;

const BASE_URL: &str = "https://api.duckduckgo.com/";

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

    /// Sends an HTTP request to the given URL using the specified method and query parameters.
    ///
    /// # Arguments
    /// * `method` - The HTTP method to use (GET, POST, etc.).
    /// * `url` - The target URL.
    /// * `params` - A slice of key-value string pairs to be included as query parameters.
    ///
    /// # Returns
    /// A `Result` containing the HTTP response or an error.
    ///
    /// # Example
    /// ```rust
    /// use reqwest::Method;
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::user_agents::get;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let browser = Browser::new(reqwest::Client::new());
    ///     let user_agent = get("firefox").unwrap();
    ///     let response = browser.request(Method::GET, "https://api.duckduckgo.com", user_agent, &[("test", "123")]).await?;
    ///     assert!(response.status().is_success());
    ///     Ok(())
    /// }
    /// ```
    pub async fn request(
        &self,
        method: reqwest::Method,
        url: &str,
        user_agent: &str,
        params: &[(&str, &str)],
    ) -> Result<reqwest::Response> {
        let req = self
            .client
            .request(method, url)
            .query(params)
            .header("User-Agent", user_agent)
            .header("Accept", "application/json")
            .header("Referer", "https://duckduckgo.com/")
            .header("Accept-Language", "en-US,en;q=0.9");

        let resp = req.send().await?.error_for_status()?;
        Ok(resp)
    }

    /// Retrieves the `vqd` token required for JavaScript-based DuckDuckGo API endpoints.
    ///
    /// # Arguments
    /// * `query` - The search query string.
    ///
    /// # Returns
    /// A `Result` containing the extracted `vqd` string or an error if not found.
    ///
    /// # Example
    /// ```rust
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::user_agents::get;
    ///
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let browser = Browser::new(reqwest::Client::new());
    ///     let user_agent = get("firefox").unwrap();
    ///     let vqd = browser.get_vqd("rust programming", user_agent).await?;
    ///     assert!(!vqd.is_empty());
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_vqd(&self, query: &str, user_agent: &str) -> Result<String> {
        let resp = self
            .request(
                reqwest::Method::GET,
                "https://duckduckgo.com/",
                user_agent,
                &[("q", query)],
            )
            .await?;

        let text = resp.text().await?;

        let re = Regex::new(r#"vqd=.?['"]?([\d-]+)['"]?"#)?;

        let vqd = re
            .captures(&text)
            .and_then(|c| c.get(1).map(|m| m.as_str().to_string()))
            .context("Missing vqd in response")?;

        Ok(vqd)
    }

    /// Performs a search using DuckDuckGo Lite, a text-only HTML interface.
    ///
    /// # Arguments
    /// * `query` - The search query.
    /// * `region` - The region code (e.g., `"wt-wt"` for worldwide).
    /// * `limit` - Optional maximum number of results to return.
    ///
    /// # Returns
    /// A list of `LiteSearchResult` items.
    ///
    /// # Example
    /// ```rust
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::user_agents::get;
    ///
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let browser = Browser::new(reqwest::Client::new());
    ///     let user_agent = get("firefox").unwrap();
    ///     let results = browser.lite_search("rust language", "wt-wt", Some(3), user_agent).await?;
    ///     assert!(results.len() <= 3);
    ///     Ok(())
    /// }
    /// ```
    pub async fn lite_search(
        &self,
        query: &str,
        region: &str,
        limit: Option<usize>,
        user_agent: &str,
    ) -> anyhow::Result<Vec<LiteSearchResult>> {
        let resp = self
            .request(
                reqwest::Method::POST,
                "https://lite.duckduckgo.com/lite/",
                user_agent,
                &[("q", query), ("kl", region)],
            )
            .await
            .context("Failed to send request to DuckDuckGo Lite")?;

        let body = resp.text().await.context("Failed to read response body")?;
        let doc = Html::parse_document(&body);
        let sel = Selector::parse("table tr").map_err(|e| anyhow::anyhow!("{e}"))?;

        let mut results = Vec::new();
        let a_sel = Selector::parse("a").map_err(|e| anyhow::anyhow!("{e}"))?;
        let snippet_sel =
            Selector::parse("td.result-snippet").map_err(|e| anyhow::anyhow!("{e}"))?;

        for tr in doc.select(&sel) {
            if let Some(a) = tr.select(&a_sel).next() {
                let title = a.text().collect::<String>();
                if let Some(href) = a.value().attr("href") {
                    let snippet = tr
                        .select(&snippet_sel)
                        .next()
                        .map(|n| n.text().collect())
                        .unwrap_or_default();

                    results.push(LiteSearchResult {
                        title,
                        url: href.to_string(),
                        snippet,
                    });

                    if limit.is_some_and(|l| results.len() >= l) {
                        break;
                    }
                }
            }
        }

        Ok(results)
    }

    /// Performs an image search on DuckDuckGo.
    ///
    /// # Arguments
    /// * `query` - The search query.
    /// * `region` - The region code (e.g., `"wt-wt"`).
    /// * `safesearch` - Whether to enable safe search.
    /// * `limit` - Optional maximum number of image results.
    ///
    /// # Returns
    /// A list of `ImageResult` items.
    ///
    /// # Example
    /// ```rust
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::user_agents::get;
    ///
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let browser = Browser::new(reqwest::Client::new());
    ///     let user_agent = get("firefox").unwrap();
    ///     let images = browser.images("rustacean", "wt-wt", true, Some(5), user_agent).await?;
    ///     assert!(!images.is_empty());
    ///     Ok(())
    /// }
    /// ```
    pub async fn images(
        &self,
        query: &str,
        region: &str,
        safesearch: bool,
        limit: Option<usize>,
        user_agent: &str,
    ) -> Result<Vec<ImageResult>> {
        let vqd = self.get_vqd(query, user_agent).await?;
        let mut page_params = vec![
            ("q", query.to_string()),
            ("l", region.to_string()),
            ("vqd", vqd),
            ("o", "json".into()),
            ("p", if safesearch { "1" } else { "-1" }.into()),
        ];

        let mut results = Vec::new();

        loop {
            let params_ref: Vec<(&str, &str)> =
                page_params.iter().map(|(k, v)| (*k, v.as_ref())).collect();

            let resp = self
                .request(
                    reqwest::Method::GET,
                    "https://duckduckgo.com/i.js",
                    user_agent,
                    &params_ref,
                )
                .await?;

            let j: Value = resp.json().await?;
            if let Some(array) = j.get("results").and_then(|r| r.as_array()) {
                for item in array.iter() {
                    results.push(ImageResult {
                        title: item["title"].as_str().unwrap_or("").to_string(),
                        image: item["image"].as_str().unwrap_or("").to_string(),
                        thumbnail: item["thumbnail"].as_str().unwrap_or("").to_string(),
                        url: item["url"].as_str().unwrap_or("").to_string(),
                        height: item["height"].as_u64().unwrap_or(0) as u32,
                        width: item["width"].as_u64().unwrap_or(0) as u32,
                        source: item["source"].as_str().unwrap_or("").to_string(),
                    });

                    if limit.is_some_and(|l| results.len() >= l) {
                        return Ok(results);
                    }
                }
            }

            if let Some(next) = j.get("next").and_then(|n| n.as_str()) {
                let s = next.split("s=").nth(1).unwrap_or("").to_string();
                page_params.push(("s", s));
            } else {
                break;
            }
        }

        Ok(results)
    }

    /// Performs a news search using DuckDuckGo's `news.js` API.
    ///
    /// # Arguments
    /// * `query` - The search query.
    /// * `region` - Region/language code (e.g., `"wt-wt"`).
    /// * `safesearch` - Enables/disables safe search.
    /// * `limit` - Optional limit for number of news results.
    ///
    /// # Returns
    /// A list of `NewsResult` entries, including title, source, URL, and date.
    ///
    /// # Example
    /// ```rust
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::user_agents::get;
    ///
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let user_agent = get("firefox").unwrap();
    ///     let browser = Browser::new(reqwest::Client::new());
    ///     let news = browser.news("AI", "wt-wt", true, Some(5), user_agent).await?;
    ///     assert!(news.iter().any(|n| n.title.contains("AI")));
    ///     Ok(())
    /// }
    /// ```
    pub async fn news(
        &self,
        query: &str,
        region: &str,
        safesearch: bool,
        limit: Option<usize>,
        user_agent: &str,
    ) -> Result<Vec<NewsResult>> {
        let vqd = self.get_vqd(query, user_agent).await?;
        let mut page_params = vec![
            ("q", query.to_string()),
            ("l", region.to_string()),
            ("vqd", vqd),
            ("o", "json".into()),
            ("p", if safesearch { "1" } else { "-1" }.into()),
            ("noamp", "1".into()),
        ];

        let mut results = Vec::new();

        loop {
            let params_ref: Vec<(&str, &str)> =
                page_params.iter().map(|(k, v)| (*k, v.as_ref())).collect();

            let resp = self
                .request(
                    reqwest::Method::GET,
                    "https://duckduckgo.com/news.js",
                    user_agent,
                    &params_ref,
                )
                .await?;

            let j: Value = resp.json().await?;
            if let Some(array) = j.get("results").and_then(|r| r.as_array()) {
                for item in array.iter() {
                    let date = item["date"]
                        .as_i64()
                        .map(|ts| {
                            chrono::Utc
                                .timestamp_opt(ts, 0)
                                .single()
                                .unwrap_or_else(chrono::Utc::now)
                        })
                        .unwrap_or_else(chrono::Utc::now);

                    results.push(NewsResult {
                        date: date.to_rfc3339(),
                        title: item["title"].as_str().unwrap_or("").to_string(),
                        body: item["excerpt"].as_str().unwrap_or("").to_string(),
                        url: item["url"].as_str().unwrap_or("").to_string(),
                        image: item
                            .get("image")
                            .and_then(|v| v.as_str())
                            .map(str::to_string),
                        source: item["source"].as_str().unwrap_or("").to_string(),
                    });

                    if limit.is_some_and(|l| results.len() >= l) {
                        return Ok(results);
                    }
                }
            }

            if let Some(next) = j.get("next").and_then(|n| n.as_str()) {
                let s = next.split("s=").nth(1).unwrap_or("").to_string();
                page_params.push(("s", s));
            } else {
                break;
            }
        }

        Ok(results)
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
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::response::ResultFormat;
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
    ) -> Result<()> {
        let separator = if path.contains('?') { '&' } else { '?' };
        let url = format!("{}{}{}format=json", BASE_URL, path, separator);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to send request to {}", url))?;

        let status = response.status();
        let text = response
            .text()
            .await
            .with_context(|| "Failed to read response body")?;

        if !status.is_success() {
            anyhow::bail!("Request failed with status {}: {}", status, text);
        }

        let api_response: Response = serde_json::from_str(&text)
            .with_context(|| format!("Failed to parse JSON response: {}", text))?;

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
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::response::ResultFormat;
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
    ) -> Result<()> {
        let safe_param = if safe_search { "&kp=1" } else { "&kp=-2" };
        let path = format!("?q={}{}", query, safe_param);

        self.browse(&path, result_format, limit)
            .await
            .with_context(|| format!("Failed to perform search for query '{}'", query))
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
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::response::ResultFormat;
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
    ) -> Result<()> {
        let safe_param = if safe_search { "&kp=1" } else { "&kp=-2" };
        let path = format!("?q={}&kl={}{}", query, params, safe_param);

        self.browse(&path, result_format, limit)
            .await
            .with_context(|| format!("Failed to perform advanced search for query '{}'", query))
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
    /// use duckduckgo::browser::Browser;
    /// use duckduckgo::response::ResultFormat;
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
    ) -> Result<()> {
        let safe_param = if safe_search { "&kp=1" } else { "&kp=-2" };
        let path = format!("?q={}&{}{}", query, operators, safe_param);

        self.browse(&path, result_format, limit)
            .await
            .with_context(|| format!("Failed to perform operator search for query '{}'", query))
    }
}
