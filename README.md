# 🦆 DuckDuckGo

[![Crates.io](https://img.shields.io/crates/v/duckduckgo.svg)](https://crates.io/crates/duckduckgo)
[![docs](https://docs.rs/duckduckgo/badge.svg)](https://docs.rs/duckduckgo/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

```sh
duckduckgo 0.1.0

▓█████▄  █    ██  ▄████▄   ██ ▄█▀▓█████▄  █    ██  ▄████▄   ██ ▄█▀  ▄████  ▒█████  
▒██▀ ██▌ ██  ▓██▒▒██▀ ▀█   ██▄█▒ ▒██▀ ██▌ ██  ▓██▒▒██▀ ▀█   ██▄█▒  ██▒ ▀█▒▒██▒  ██▒
░██   █▌▓██  ▒██░▒▓█    ▄ ▓███▄░ ░██   █▌▓██  ▒██░▒▓█    ▄ ▓███▄░ ▒██░▄▄▄░▒██░  ██▒
░▓█▄   ▌▓▓█  ░██░▒▓▓▄ ▄██▒▓██ █▄ ░▓█▄   ▌▓▓█  ░██░▒▓▓▄ ▄██▒▓██ █▄ ░▓█  ██▓▒██   ██░
░▒████▓ ▒▒█████▓ ▒ ▓███▀ ░▒██▒ █▄░▒████▓ ▒▒█████▓ ▒ ▓███▀ ░▒██▒ █▄░▒▓███▀▒░ ████▓▒░
 ▒▒▓  ▒ ░▒▓▒ ▒ ▒ ░ ░▒ ▒  ░▒ ▒▒ ▓▒ ▒▒▓  ▒ ░▒▓▒ ▒ ▒ ░ ░▒ ▒  ░▒ ▒▒ ▓▒ ░▒   ▒ ░ ▒░▒░▒░ 
 ░ ▒  ▒ ░░▒░ ░ ░   ░  ▒   ░ ░▒ ▒░ ░ ▒  ▒ ░░▒░ ░ ░   ░  ▒   ░ ░▒ ▒░  ░   ░   ░ ▒ ▒░ 
 ░ ░  ░  ░░░ ░ ░ ░        ░ ░░ ░  ░ ░  ░  ░░░ ░ ░ ░        ░ ░░ ░ ░ ░   ░ ░ ░ ░ ▒  
   ░       ░     ░ ░      ░  ░      ░       ░     ░ ░      ░  ░         ░     ░ ░  
 ░               ░                ░               ░
  Search and advanced search in DuckDuckGo 
  ========================================

  Perform searches and advanced searches on DuckDuckGo from the command line.
```

> 🚀 **duckduckgo**: A powerful Rust-based command-line tool for seamless DuckDuckGo searches.

## 📖 Table of Contents

- [Installation](#-installation)
- [Feature](#-feature)
- [Usage](#-usage)
- [Options](#-options)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Installation

To install `duckduckgo`, use the following Cargo command:

```bash
cargo install --locked duckduckgo
```

## 🚀 Features

- Perform DuckDuckGo searches with or without operators.
- Customizable user agent, proxy, and cookie support.
- Beautiful ANSI-colored output.

## 🚗 Usage

Learn how to use the duckduckgo and explore its features with the following examples:

### Perform a basic search:

```bash
duckduckgo --query "rust lang"
```

### Use search operators:

```bash
duckduckgo --query "rust lang" --operators "+tutorial"
```

### Enable safe search:

```bash
duckduckgo --query "rust lang" --safe
```

### Set the output format to detailed:

```bash
duckduckgo --query "rust lang" --format
```

### Limit the number of results to 3:

```bash
duckduckgo --query "rust lang" --limit 3
```

### Set user agent:

```bash
duckduckgo --query "rust lang" --user-agent "MyCustomAgent"
```

### Set cookie for subsequent requests:

```bash
duckduckgo --query "rust lang" --cookie
```

### Set proxy:

```bash
duckduckgo --query "rust lang" --proxy "socks5://192.168.1.1:9000"
```

## 🎨 Options

| Option                   | Default Value | Description                                              |
|--------------------------|---------------|----------------------------------------------------------|
| `--safe`                 | `false`       | Enable safe search.                                      |
| `--proxy`                | `""`          | Set a proxy for the search (e.g., "socks5://192.168.1.1:9000"). |
| `--user-agent`           | `Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3` | Set a custom user agent for the search.                   |
| `--cookie`               | `true`        | Set a cookie for the search.                              |
| `--format`               | `false`       | Set the output format (`false` for list or `true` for detailed). |
| `--limit`                | `10`          | Limit the number of results (default is 10).             |

## 🤝 Contributing

Contributions and feedback are welcome! If you'd like to contribute, report an issue, or suggest an enhancement, please engage with the project on [GitHub](https://github.com/wiseaidev/duckduckgo).
Your contributions help improve this CLI for the community.

## 📄 License

This project is licensed under the [MIT License](LICENSE).
