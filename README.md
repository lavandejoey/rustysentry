# RustySentry

---
RustySentry is a **[Rust](https://www.rust-lang.org/)** application that scans Git repositories for **sensitive information and prevents leakage**.

RustySentry was inspired by [Gitleaks](https://github.com/gitleaks/gitleaks), a similar tool written in Go. RustySentry is written in Rust and is designed to be secured, fast, efficient, and easy to use.

>**_THE PROGRAMME IS STILL IN DEVELOPMENT AND IS NOT READY FOR PRODUCTION USE.
>I AM TRYING MY BEST TO FINISH THE VERSION 1.0.0 AS SOON AS POSSIBLE._**

## Getting Started

---
RustySentry will scan your entire Git history for sensitive strings, file types, or other information you want to prevent from leaking.

## Features

---
- Search for sensitive strings (credit card numbers, social security numbers, etc.)
- Detect sensitive file types (PDFs, images, key files, etc.)
- Allow custom regex patterns and strings to search for
- Flag and optionally remove detected sensitive information
- Provide reports on what was detected for auditing purposes

## Configuration

---
RustySentry supports various options for configuration. You can set by:
1. Command line arguments (Superior)
2. Toml config file or YAML config file (Inferior and Alternative)
### Command line arguments
```text
A tool for detecting sensitive information in Git repositories by Rust.

Usage: rustysentry [OPTIONS] [COMMAND]

Commands:
  scan  TODO:Scan for sensitive information
  help  Print this message or the help of the given subcommand(s)

Options:
  -l, --log <LOG LEVEL>            Set log level. info:info(default), debug:debug, warn:warn, error:error
  -c, --config <FILE>              Sets a custom config file
  -r, --repo-path <REPO_PATH>      The path of the Git repository to be checked
  -p, --pattern <PATTERN>          The pattern of sensitive information to be detected
  -o, --output-file <OUTPUT FILE>  The output file path for the result
  -i, --ignore-case                Case-insensitive search
  -v, --verbose                    Verbose output
  -h, --help                       Print help information
  -V, --version                    Print version information
```
### Config file(toml or yaml)
```toml
```
```yaml
```
A sample config file is provided in `config.toml`. Modify it as needed for your use case.

## Contributing

---
Contributions to RustySentry are welcome! Here are some ways you can contribute:

- Report or fix any bugs
- Add new features like support for more file types, string patterns, etc.
- Improve documentation and examples
- Refactor code to be more efficient or idiomatic Rust
- Add more tests to improve coverage

**Please open an issue to discuss any contributions before submitting a pull request.**