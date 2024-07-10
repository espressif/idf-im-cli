use clap::Parser;
use clap::{arg, ValueEnum};
use config::{Config, ConfigError, File};
use log::{error, info};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::{fmt, str::FromStr};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    pub path: Option<PathBuf>,
    pub idf_path: Option<PathBuf>,
    pub tool_download_folder_name: Option<String>,
    pub tool_install_folder_name: Option<String>,
    pub target: Option<String>,
    pub idf_version: Option<String>,
    pub tools_json_file: Option<String>,
    pub idf_tools_path: Option<String>, // relative to idf path
    pub config_file: Option<PathBuf>,
    pub non_interactive: Option<bool>,
    pub mirror: Option<String>,
    pub idf_mirror: Option<String>,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version = VERSION,
    about = "ESP-IDF Install Manager",
    long_about = "All you need to manage your ESP-IDF installations"
)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    target: Option<String>,

    #[arg(short, long)]
    idf_version: Option<String>,
    #[arg(long)]
    tool_download_folder_name: Option<String>,
    #[arg(long)]
    tool_install_folder_name: Option<String>,
    #[arg(
        long,
        help = "Path to tools.json file relative from ESP-IDF installation folder"
    )]
    idf_tools_path: Option<String>,
    #[arg(
        long,
        help = "Path to idf_tools.py file relative from ESP-IDF installation folder"
    )]
    tools_json_file: Option<String>,
    #[arg(short, long)]
    non_interactive: Option<bool>,

    #[arg(
        short,
        long,
        help = "url for download mirror to use instead of github.com"
    )]
    mirror: Option<String>,

    #[arg(
        long,
        help = "url for dowenload mirror to use instead of github.com for downloading esp-idf"
    )]
    idf_mirror: Option<String>,

    #[arg(
      short,
      long,
      action = clap::ArgAction::Count,
      help = "Increase verbosity level (can be used multiple times)"
  )]
    verbose: u8,

    #[arg(short, long, help = "Set the language for the wizard")]
    locale: Option<String>,
}

impl IntoIterator for Cli {
    type Item = (String, Option<config::Value>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (
                "config".to_string(),
                self.config.map(|p| p.to_str().unwrap().into()),
            ),
            (
                "non_interactive".to_string(),
                self.non_interactive.map(|b| b.into()),
            ),
            ("target".to_string(), self.target.map(|p| p.into())),
            (
                "idf_version".to_string(),
                self.idf_version.map(|s| s.into()),
            ),
            (
                "tool_download_folder_name".to_string(),
                self.tool_download_folder_name.map(|s| s.into()),
            ),
            (
                "tool_install_folder_name".to_string(),
                self.tool_install_folder_name.map(|s| s.into()),
            ),
            (
                "tools_json_file".to_string(),
                self.tools_json_file.map(|s| s.into()),
            ),
            (
                "idf_tools_path".to_string(),
                self.idf_tools_path.map(|s| s.into()),
            ),
            ("mirror".to_string(), self.mirror.map(|s| s.into())),
            ("idf_mirror".to_string(), self.idf_mirror.map(|s| s.into())),
        ]
        .into_iter()
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let cli = Cli::parse();
        let log_level = match cli.verbose {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        };

        match SimpleLogger::new().with_level(log_level).init() {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to initialize logger: {}", e);
            }
        }

        let locale = cli.locale.clone();
        match locale {
            Some(l) => {
                rust_i18n::set_locale(l.as_str());
                info!("Set locale to: {}", l);
            }
            None => info!("No locale specified, defaulting to en"),
        };

        let mut builder = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default").required(false))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name("config/development").required(false));

        // If a config file was specified via cli arg, add it here
        if let Some(config_path) = cli.config.clone() {
            builder = builder.add_source(File::from(config_path));
        }

        // Add in settings from the environment (with a prefix of ESP)
        // Eg.. `ESP_TARGET=esp32` would set the `target` key
        builder = builder.add_source(config::Environment::with_prefix("ESP").separator("_"));

        // Now that we've gathered all our config sources, let's merge them
        let mut cfg = builder.build()?;

        for (key, value) in cli.into_iter() {
            if let Some(v) = value {
                cfg.set(&key, v)?;
            }
        }

        // Add in cli-specified values

        // You can deserialize (and thus freeze) the entire configuration
        cfg.try_deserialize()
    }
}

#[derive(Clone, Debug)]
pub enum ChipId {
    Esp32,
    Esp32s2,
    Esp32s3,
    Esp32c2,
    Esp32c3,
    Esp32c6,
    Esp32h2,
    Esp32p4,
}

impl fmt::Display for ChipId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChipId::Esp32 => write!(f, "ESP32"),
            ChipId::Esp32s2 => write!(f, "ESP32-S2"),
            ChipId::Esp32s3 => write!(f, "ESP32-S3"),
            ChipId::Esp32c2 => write!(f, "ESP32-C2"),
            ChipId::Esp32c3 => write!(f, "ESP32-C3"),
            ChipId::Esp32c6 => write!(f, "ESP32-C6"),
            ChipId::Esp32h2 => write!(f, "ESP32-H2"),
            ChipId::Esp32p4 => write!(f, "ESP32-P4"),
        }
    }
}

// this is hardcoded enum but in the future this should be checked on build time agains the idf_versions.json file from idf/idf-versions
// TODO: should we accept any string as argument ang check it on the fly?
impl FromStr for ChipId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ESP32" => Ok(ChipId::Esp32),
            "ESP32-S2" => Ok(ChipId::Esp32s2),
            "ESP32S2" => Ok(ChipId::Esp32s2),
            "ESP32-S3" => Ok(ChipId::Esp32s3),
            "ESP32S3" => Ok(ChipId::Esp32s3),
            "ESP32-C2" => Ok(ChipId::Esp32c2),
            "ESP32C2" => Ok(ChipId::Esp32c2),
            "ESP32-C3" => Ok(ChipId::Esp32c3),
            "ESP32C3" => Ok(ChipId::Esp32c3),
            "ESP32-C6" => Ok(ChipId::Esp32c6),
            "ESP32C6" => Ok(ChipId::Esp32c6),
            "ESP32-H2" => Ok(ChipId::Esp32h2),
            "ESP32H2" => Ok(ChipId::Esp32h2),
            "ESP32-P4" => Ok(ChipId::Esp32p4),
            "ESP32P4" => Ok(ChipId::Esp32p4),
            _ => Err(format!("'{}' is not a valid value", s)),
        }
    }
}

impl ValueEnum for ChipId {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ChipId::Esp32,
            ChipId::Esp32s2,
            ChipId::Esp32s3,
            ChipId::Esp32c2,
            ChipId::Esp32c3,
            ChipId::Esp32c6,
            ChipId::Esp32h2,
            ChipId::Esp32p4,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            ChipId::Esp32 => clap::builder::PossibleValue::new("ESP32"),
            ChipId::Esp32s2 => clap::builder::PossibleValue::new("ESP32-S2"),
            ChipId::Esp32s3 => clap::builder::PossibleValue::new("ESP32-S3"),
            ChipId::Esp32c2 => clap::builder::PossibleValue::new("ESP32-C2"),
            ChipId::Esp32c3 => clap::builder::PossibleValue::new("ESP32-C3"),
            ChipId::Esp32c6 => clap::builder::PossibleValue::new("ESP32-C6"),
            ChipId::Esp32h2 => clap::builder::PossibleValue::new("ESP32-H2"),
            ChipId::Esp32p4 => clap::builder::PossibleValue::new("ESP32-P4"),
        })
    }
}
