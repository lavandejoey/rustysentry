use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RustySentry")]
#[command(author = "Liu, Ziyi <lavandejoey@outlook.com>")]
#[command(version = "0.1.0")]
#[command(about = "A tool for detecting sensitive information in Git repositories by Rust.", long_about = None)]
pub(crate) struct Cli {
    /// Optional name to operate on
    pub(crate) name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub(crate) debug: u8,

    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    ///Scan for sensitive information
    Scan {
        /// The path of the Git repository to be checked
        #[arg(short, long)]
        repo_path: String,
        /// The pattern of sensitive information to be detected
        #[arg(short, long, name = "PATTERN")]
        pattern: String,
        /// The output file path for the result
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}
