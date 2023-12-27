use clap::Parser;
use duckduckgo::browser::{Browser, ResultFormat};
use duckduckgo::cli::Cli;
use duckduckgo::colors::{AnsiColor, AnsiStyle};
use duckduckgo::user_agents::USER_AGENTS;
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
/// # Errors
/// The function handles errors gracefully and prints out error messages if the DuckDuckGo search
/// with operators fails, if the query is missing, etc.
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args = Cli::parse();

    let style = AnsiStyle {
        bold: true,
        color: Some(AnsiColor::Red),
    };

    // Configure reqwest::Client based on command-line options
    let mut client_builder = reqwest::Client::builder();

    // Check if the user selected a valid user agent
    if !args.user_agent.is_empty() {
        if let Some(user_agent) = USER_AGENTS.get(&args.user_agent[..]) {
            client_builder = client_builder.user_agent(*user_agent);
        } else {
            // Print an error message and exit if the user agent is not found
            eprintln!(
                "{}Error: Invalid user agent selected!{}",
                style.escape_code(),
                AnsiStyle::reset_code()
            );
            std::process::exit(1);
        }
    }
    if args.cookie {
        client_builder = client_builder.cookie_store(true);
    }
    if !args.proxy.is_empty() {
        // Attempt to create a proxy, and handle errors
        let proxy = reqwest::Proxy::all(&args.proxy)?;
        client_builder = client_builder.proxy(proxy);
    }
    let client = client_builder.build()?;

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
                    &encode(&args.operators),
                    args.safe,
                    result_format,
                    Some(limit),
                )
                .await?;
        } else {
            browser
                .search(&encode(&args.query), args.safe, result_format, Some(limit))
                .await?;
        }
    } else {
        // Print an error message and exit if the query is missing
        eprintln!(
            "{}Error: Query is required!{}",
            style.escape_code(),
            AnsiStyle::reset_code()
        );
        std::process::exit(1);
    }

    Ok(())
}
