use std::string::String;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use toml;
use serde::Deserialize;
use clap::{Parser, Subcommand};

#[derive(Parser, Deserialize)]
#[command(name = "RustySentry")]
#[command(author = "Ziyi Joshua LIU <lavandejoey@outlook.com>")]
#[command(version = "0.1.0")]
#[command(about = "A tool for detecting sensitive information in Git repositories by Rust.", long_about = None)]
pub(crate) struct Configs {
    /// Set log level. info:info(default), debug:debug, warn:warn, error:error.
    #[arg(short, long, value_name = "LOG LEVEL")]
    pub(crate) log: Option<String>,
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,
    /// The path of the Git repository to be checked
    #[arg(short, long)]
    pub(crate) repo_path: Option<String>,
    /// The pattern of sensitive information to be detected
    #[arg(short, long, name = "PATTERN")]
    pub(crate) pattern: Option<String>,
    /// The output file path for the result
    #[arg(short, long, value_name = "OUTPUT FILE")]
    pub(crate) output_file: Option<PathBuf>,
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
pub(crate) enum Commands {
    ///TODO:Scan for sensitive information
    Scan,
}

impl Configs {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut configs = Self::parse();

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
                    configs.log = configs.log.clone().or(file_config.log);
                    configs.repo_path = configs.repo_path.clone().or(file_config.repo_path);
                    configs.pattern = configs.pattern.clone().or(file_config.pattern);
                    configs.output_file = configs.output_file.clone().or(file_config.output_file);
                    // configs.ignore_case = configs.ignore_case || file_config.ignore_case;
                    // configs.verbose = configs.verbose || file_config.verbose;
                }
                None => return Ok(configs)
            }
        }
        Ok(configs)
    }
}
//
// impl Configs {
//     fn parse(f_path: &PathBuf) -> Result<Configs, String> {
//         let f = match std::fs::File::open(f_path) {
//             Ok(f) => f,
//             Err(e) => return Err(format!("Failed to open config file: {}", e)),
//         };
//         let yaml_config: Configs = match serde_yaml::from_reader(f) {
//             Ok(conf) => conf,
//             Err(e) => return Err(format!("Failed to parse config file: {}", e)),
//         };
//         Ok(yaml_config)
//     }
//
//     pub(crate) fn combine(cli_config: &mut Configs) -> Configs {
//         match &cli_config.config {
//             None => panic!("None"),
//             Some(file_path) => {
//                 let mut yaml_config = match Configs::parse(file_path) {
//                     Ok(conf) => conf,
//                     Err(e) => panic!("{}", format!("{}", e))
//                 };
//                 // check replaceable var in cli_config, if its empty in cli_config, replace by value in yaml_config
//                 let configs: Configs = Configs {
//                     log: { if cli_config.log.is_none() { yaml_config.log.take() } else { cli_config.log.take() } },
//                     repo_path: { if cli_config.repo_path.is_none() { yaml_config.repo_path.take() } else { cli_config.repo_path.take() } },
//                     pattern: { if cli_config.pattern.is_none() { yaml_config.pattern.take() } else { cli_config.pattern.take() } },
//                     output: { if cli_config.output.is_none() { yaml_config.output.take() } else { cli_config.output.take() } },
//                     ignore_case: { if !cli_config.ignore_case { yaml_config.ignore_case } else { cli_config.ignore_case } },
//                     verbose: { if !cli_config.verbose { yaml_config.verbose } else { cli_config.verbose } },
//                 };
//                 configs
//             }
//         }
//     }
// }