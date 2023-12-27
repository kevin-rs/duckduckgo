//! # DuckDuckGo
//!
//! A CLI tool for performing DuckDuckGo searches with additional customization options.
//!
//! ## Quick Start
//!
//! Get started with the `duckduckgo` CLI by following these simple steps:
//!
//! 1. Install the `duckduckgo` tool using Cargo:
//!
//! ```bash
//! cargo install --locked duckduckgo
//! ```
//!
//! 2. Use the following options to perform searches and customize your search experience:
//!
//! ```bash
//! duckduckgo -q "your search query" -o "search operators" -s -f -l 10 -u "custom user agent" -c -p "proxy"
//! ```
//!
//! ## Options
//!
//! | Option                  | Description                                               |
//! |-------------------------|-----------------------------------------------------------|
//! | `--query`               | Sets the search query.                                    |
//! | `--operators`           | Sets the search operators.                                |
//! | `--safe`                | Enable safe search.                                       |
//! | `--format`              | Set the output format (`false` for list or `true` for detailed). |
//! | `--limit`               | Limit the number of results (default is 10).              |
//! | `--user-agent`          | Set a custom user agent for the search.                   |
//! | `--cookie`              | Set a cookie for the search.                               |
//! | `--proxy`               | Set a proxy for the search.                                |
//!
//! ## GitHub Repository
//!
//! You can access the source code for this CLI tool on [GitHub](https://github.com/wiseaidev/duckduckgo).
//!
//! ## Contributing
//!
//! Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement,
//! please engage with the project on [GitHub](https://github.com/wiseaidev/duckduckgo).
//! Your contributions help improve this CLI tool for the community.

pub mod browser;
pub mod cli;
pub mod colors;
pub mod icon;
pub mod response;
pub mod topic;
pub mod user_agents;
