use std::io::{Write};
use std::string::String;
use env_logger::{Builder, Env};
use git2::{Repository};
use log::{debug, error, info, warn};
use config::Configs;

mod config;
mod traverse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configs: Configs = config::Configs::new()?;

    // Initial settings
    // Set log level
    let verbose_level: String = match configs.verbose {
        true => String::from("debug"),
        false => String::from("info"),
    };
    let log_level: String = match configs.log {
        Some(l)if l.eq_ignore_ascii_case("info") => String::from("info"),
        Some(l)if l.eq_ignore_ascii_case("debug") => String::from("debug"),
        Some(l)if l.eq_ignore_ascii_case("warn") => String::from("warn"),
        Some(l)if l.eq_ignore_ascii_case("error") => String::from("error"),
        None => verbose_level,
        _ => panic!("DO NOT BE CRAZY! (Check if your log mode config is correct.)"),
    };

    let env = Env::default()
        .filter_or("RUST_LOG", &log_level)
        .write_style_or("RUST_LOG_STYLE", "always");
    Builder::from_env(env).format(|buf, record| {
        let now = chrono::Local::now();
        writeln!(buf, "{} [{}] {}",
                 now.format("[%Y-%m-%d %H:%M:%S%.3f]"), record.level(), record.args())
    }).init();

    // Open the Git repository in the current directory
    let repo = Repository::open(".").unwrap();
    // Traverse the commit that the HEAD reference points to
    traverse::traverse_head(&repo).unwrap();
    Ok(())
}