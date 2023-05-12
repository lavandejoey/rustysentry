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
//packages
use std::string::String;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use toml;
use serde::Deserialize;
use clap::{Parser, Subcommand};
use rand::random;

#[derive(Parser, Deserialize)]
#[command(propagate_version = true)]
#[command(name = "RustySentry")]
#[command(author = "Ziyi Joshua LIU <lavandejoey@outlook.com>")]
#[command(version = "0.1.0")]
#[command(about = "A tool for detecting sensitive information in Git repositories by Rust.", long_about = None)]
pub(crate) struct Configs {
    /// Set log level. info:info(default), debug:debug, warn:warn, error:error.
    #[arg(short, long, value_name = "LOG LEVEL")]
    pub(crate) log: Option<String>,
    /// Sets a custom config file
    #[arg(default_value = "/media/lavandejoey/Documents/CODE/PROJECTS/rustysentry/config.toml")]
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,
    /// The pattern of sensitive information to be detected
    #[arg(short, long, name = "PATTERN")]
    pub(crate) pattern: Option<String>,
    /// The output file path for the result
    #[arg(short, long, value_name = "LOG AND TEMP FILE PATH")]
    pub(crate) output: Option<PathBuf>,
    /// Case-insensitive search
    #[arg(short, long, value_name = "IGNORE CASE")]
    pub(crate) ignore_case: bool,
    /// Verbose output
    #[arg(short, long)]
    pub(crate) verbose: bool,
    /// Subcommands
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}


#[derive(Subcommand, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Commands {
    Scan{
        repo_path: Option<String>,
    },
}

impl Configs {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut configs = Self::parse();
        let default_config:Configs = Configs {
            log: Some("info".to_string()),
            config: None,
            pattern: None,
            output: {
                // "<os tmp path>/rustysentry/<rand digit string>/"
                Option::from(std::env::temp_dir().join("rustysentry").join(random::<u32>().to_string()))
            },
            ignore_case: false,
            verbose: false,
            command: None,
        };

        if let Some(ref config_path) = configs.config {
            match &config_path.extension() {
                Some(ext) => {
                    let file_config: Configs = match ext.to_str() {
                        Some("toml") => {
                            //toml
                            let config_content = fs::read_to_string(config_path)?;
                            // let file_config: Configs =
                            toml::from_str(&config_content)?
                        }
                        Some("yaml") => {
                            // yaml
                            let f: File = std::fs::File::open(config_path)?;
                            // let file_config: Configs=
                            serde_yaml::from_reader(f)?
                        }
                        _ => return Ok(configs)
                    };
                    configs.log = configs.log.clone().or(file_config.log).or(default_config.log);
                    configs.pattern = configs.pattern.clone().or(file_config.pattern).or(default_config.pattern);
                    configs.output = configs.output.clone().or(file_config.output).or(default_config.output);
                    // configs.ignore_case = configs.ignore_case || file_config.ignore_case;
                    // configs.verbose = configs.verbose || file_config.verbose;
                    configs.command = configs.command.clone().or(file_config.command);
                }
                None => return Ok(configs)
            }
        }
        Ok(configs)
    }
}
