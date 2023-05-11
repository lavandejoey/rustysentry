# RustySentry

RustySentry is a Rust application that scans Git repositories for sensitive information and prevents leakage.

## Getting Started

To get started with RustySentry:

1. Install Rust and Cargo. Follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone the RustySentry repository: `git clone https://github.com/your-username/RustySentry.git`
3. Build the project: `cargo build --release`
4. Run the scanner on your Git repo: `./target/release/rustysentry scan /path/to/repo`

RustySentry will scan your entire Git history for sensitive strings, file types, or other information you want to prevent from leaking.

## Features

- Scan Git commit history and diffs
- Search for sensitive strings (credit card numbers, social security numbers, etc.)
- Detect sensitive file types (PDFs, images, key files, etc.)
- Allow custom regex patterns and strings to search for
- Flag and optionally remove detected sensitive information
- Provide reports on what was detected for auditing purposes

## Configuration

RustySentry is configured via a TOML config file. You can specify:

- Strings and regex patterns to scan for
- File extensions and MIME types to detect
- Whether to remove detected information or just report findings
- Reporting and logging options

A sample config file is provided in `config.toml`. Modify it as needed for your use case.

## Contributing

Contributions to RustySentry are welcome! Here are some ways you can contribute:

- Report or fix any bugs
- Add new features like support for more file types, string patterns, etc.
- Improve documentation and examples
- Refactor code to be more efficient or idiomatic Rust
- Add more tests to improve coverage

Please open an issue to discuss any contributions before submitting a pull request.