use std::{fmt, path::PathBuf, str::FromStr};

// use clap::{command, Parser, ValueEnum};
use clap::{arg, command, value_parser, Arg, Command, Parser, ValueEnum};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// pub struct Args {
//     #[arg(short, long, default_value = "shuttle")]
//     pub name: String,
// }
#[derive(Debug, Default)]
pub struct Config {
    pub path: Option<PathBuf>,
    pub target: Option<ChipId>,
    pub idf_version: Option<String>,
    pub config_file: Option<PathBuf>,
    pub non_interactive: Option<bool>,
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
        match s {
            "ESP32" => Ok(ChipId::Esp32),
            "ESP32-S2" => Ok(ChipId::Esp32s2),
            "ESP32-S3" => Ok(ChipId::Esp32s3),
            "ESP32-C2" => Ok(ChipId::Esp32c2),
            "ESP32-C3" => Ok(ChipId::Esp32c3),
            "ESP32-C6" => Ok(ChipId::Esp32c6),
            "ESP32-H2" => Ok(ChipId::Esp32h2),
            "ESP32-P4" => Ok(ChipId::Esp32p4),
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

pub fn get_cli() -> clap::Command {
    Command::new("ESP-IDF Installation Manager")
        .version(VERSION)
        .about("All you need to manage your ESP-IDF installations")
        .arg(
            arg!(
                -p --path <PATH> "base instalation path"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -t --target <VALUE> "which chip you are using"
            )
            .required(false)
            .value_parser(value_parser!(ChipId)),
        )
        .arg(
            arg!(
              --"idf-version" <VALUE> "which version of idf we want to install"
            )
            .required(false)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
              -c --"config-file" <VALUE> "path to file with instalator configuration"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
              -n --"non-interactive" <BOOL> "show the wizard"
            )
            .required(false)
            .value_parser(value_parser!(bool))
            .default_value(std::ffi::OsStr::new("false")),
        )
}

pub fn parse_cli(arg_matches: &clap::ArgMatches) -> Config {
    let mut config = Config::default();

    // we need to parse config file first, because cli params have higher priority
    // TODO: we shoud parse env even before
    if let Some(config_file) = arg_matches.get_one::<PathBuf>("config-file") {
        println!("Value for config_file: {:?}", config_file);
        println!("Parsing config file not implemented yet");
    }

    if let Some(path) = arg_matches.get_one::<PathBuf>("path") {
        config.path = Some(path.to_owned());
    }
    if let Some(target) = arg_matches.get_one::<ChipId>("target") {
        config.target = Some(target.to_owned());
    }
    if let Some(idf_version) = arg_matches.get_one::<String>("idf-version") {
        config.idf_version = Some(idf_version.to_owned());
    }
    if let Some(non_interactive) = arg_matches.get_one::<bool>("non-interactive") {
        config.non_interactive = Some(*non_interactive);
    }
    config
}
