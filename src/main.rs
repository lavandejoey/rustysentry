/*
 *         Copyright (C) 2023. Ziyi Joshua LIU
 *         This program is free software: you can redistribute it and/or modify
 *         it under the terms of the GNU General Public License as published by
 *         the Free Software Foundation, either version 3 of the License, or
 *         (at your option) any later version.
 *
 *         This program is distributed in the hope that it will be useful,
 *         but WITHOUT ANY WARRANTY; without even the implied warranty of
 *         MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *         GNU General Public License for more details.
 *
 *         You should have received a copy of the GNU General Public License
 *         along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
// local module
mod git_actor;
mod utils;

// local use
use git_actor::{path2repo};
// use git_actor::{GitActor};
use utils::{Configs, Commands};
// packages
use std::io::{Write};
use std::string::String;
use env_logger::{Builder, Env};
use log::{debug, error, info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configs: Configs = Configs::new()?;

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
        None => verbose_level.clone(),
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

    // if log level is warn, warn
    if log_level.eq_ignore_ascii_case("warn") {
        warn!("Log level is set to warn, which is not recommended. Instead, use info or debug.");
    }

    match configs.command {
        Some(Commands::Scan { repo_path }) => {
            debug!("Scan command is called.");
            // Here you can use repo_path
            if let Some(path) = repo_path {
                debug!("Scanning repository at {}", path.clone());
                let repo = path2repo(
                    path.clone(),
                    configs.output.clone().unwrap(),
                )?;
                debug!("Repository at {} is scanned.", path.clone());
                git_actor::traverse_head(&repo)?;
            }
        }
        None => todo!(),
    }

    Ok(())
}