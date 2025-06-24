use anyhow::Result;
#[cfg(feature = "cli")]
use {
    clap::Parser,
    duckduckgo::browser::Browser,
    duckduckgo::cli::{Backend, Cli},
    duckduckgo::colors::{AnsiColor, AnsiStyle},
    duckduckgo::response::ResultFormat,
    duckduckgo::user_agents::get,
    urlencoding::encode,
};

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
/// * `--backend` - Set backend to use.
///
/// # Examples
/// ```
/// // Run the DuckDuckGo search CLI with a query and display results in list format.
/// ddg --query "Rust programming"
///
/// // Run the DuckDuckGo search CLI with a query and operators, limiting results to 5.
/// ddg --query "Rust programming" --operators "site:github.com" --limit 5
/// ```
///
/// # Errors
/// The function handles errors gracefully and prints out error messages if the DuckDuckGo search
/// with operators fails, if the query is missing, etc.
#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let style = AnsiStyle {
        bold: true,
        color: Some(AnsiColor::Red),
    };

    let mut client_builder = reqwest::Client::builder();
    let mut usr_agent = "";
    if !args.user_agent.is_empty() {
        if let Some(agent) = get(&args.user_agent[..]) {
            client_builder = client_builder.user_agent(agent);
            usr_agent = agent;
        } else {
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
        let proxy = reqwest::Proxy::all(&args.proxy)?;
        client_builder = client_builder.proxy(proxy);
    }

    let client = client_builder.build()?;
    let browser = Browser::new(client);

    let result_format = if args.format {
        ResultFormat::Detailed
    } else {
        ResultFormat::List
    };

    let limit = Some(args.limit);

    if args.query.is_empty() {
        eprintln!(
            "{}Error: Query is required!{}",
            style.escape_code(),
            AnsiStyle::reset_code()
        );
        std::process::exit(1);
    }

    match args.backend {
        Backend::Auto => {
            if !args.operators.is_empty() {
                browser
                    .search_operators(
                        &encode(&args.query),
                        &encode(&args.operators),
                        args.safe,
                        result_format,
                        limit,
                    )
                    .await?;
            } else {
                browser
                    .search(&encode(&args.query), args.safe, result_format, limit)
                    .await?;
            }
        }
        Backend::Lite => {
            let results = browser
                .lite_search(&args.query, "wt-wt", limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.title, r.url, r.snippet);
            }
        }
        Backend::Images => {
            let results = browser
                .images(&args.query, "wt-wt", args.safe, limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.title, r.url, r.image);
            }
        }
        Backend::News => {
            let results = browser
                .news(&args.query, "wt-wt", args.safe, limit, usr_agent)
                .await?;
            for r in results {
                println!("{}\n{}\n{}", r.date, r.title, r.url);
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() -> Result<()> {
    Ok(())
}
