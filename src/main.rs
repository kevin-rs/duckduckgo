use clap::Parser;
use duckduckgo::browser::{Browser, ResultFormat};
use duckduckgo::cli::Cli;
use urlencoding::encode;

/// The main entry point of the DuckDuckGo search CLI application.
///
/// It parses command-line arguments using the `clap` crate, configures a `reqwest::Client` based on
/// the provided command-line options, and performs a DuckDuckGo search using the specified query
/// and optional operators.
///
/// # Arguments
/// * `--user-agent` - Specify a custom User-Agent for the HTTP requests. Default is the reqwest default User-Agent.
/// * `--cookie` - Enable cookie storage for HTTP requests.
/// * `--proxy` - Specify an HTTP proxy for requests. Must be a valid proxy URL.
/// * `--format` - Enable detailed result format. Default is a list format.
/// * `--limit` - Specify the limit for the number of search results.
/// * `--query` - The search query to be used in the DuckDuckGo search.
/// * `--operators` - Optional search operators to refine the search.
/// * `--safe` - Enable safe search mode.
///
/// # Examples
/// ```
/// // Run the DuckDuckGo search CLI with a query and display results in list format.
/// duckduckgo --query "Rust programming"
///
/// // Run the DuckDuckGo search CLI with a query and operators, limiting results to 5.
/// duckduckgo --query "Rust programming" --operators "site:github.com" --limit 5
/// ```
///
/// # Panics
/// The function panics if the DuckDuckGo search with operators fails or if the query is missing.
///
#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args = Cli::parse();

    // Configure reqwest::Client based on command-line options
    let mut client_builder = reqwest::Client::builder();
    if !args.user_agent.is_empty() {
        client_builder = client_builder.user_agent(args.user_agent);
    }
    if args.cookie {
        client_builder = client_builder.cookie_store(true);
    }
    if !args.proxy.is_empty() {
        client_builder = client_builder.proxy(reqwest::Proxy::all(args.proxy).unwrap());
    }
    let client = client_builder.build().unwrap();

    // Initialize DuckDuckGo browser with the configured client
    let browser = Browser::new(client);

    // Determine the result format based on the command-line option
    let result_format = if args.format {
        ResultFormat::Detailed
    } else {
        ResultFormat::List
    };

    // Set the limit for the number of search results
    let limit = args.limit;

    // Perform DuckDuckGo search based on the provided query and optional operators
    if !args.query.is_empty() {
        if !args.operators.is_empty() {
            browser
                .search_operators(
                    &encode(&args.query),
                    &args.operators,
                    args.safe,
                    result_format,
                    Some(limit),
                )
                .await
                .expect("Search with operators failed");
        } else {
            browser
                .search(&encode(&args.query), args.safe, result_format, Some(limit))
                .await
                .expect("Search failed");
        }
    } else {
        // Print an error message if the query is missing
        println!("Error: Query is required");
    }
}
