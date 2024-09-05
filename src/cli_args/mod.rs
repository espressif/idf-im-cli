use clap::builder::styling::{AnsiColor, Color, Style, Styles};
use clap::{arg, command, ColorChoice, Parser};
use config::{Config, ConfigError, File};
use idf_im_lib::get_log_directory;
use log::{debug, info, LevelFilter};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Settings {
    pub path: Option<PathBuf>,
    pub idf_path: Option<PathBuf>,
    pub tool_download_folder_name: Option<String>,
    pub tool_install_folder_name: Option<String>,
    pub target: Option<Vec<String>>,
    pub idf_versions: Option<Vec<String>>,
    pub tools_json_file: Option<String>,
    pub idf_tools_path: Option<String>,
    pub config_file: Option<PathBuf>,
    pub non_interactive: Option<bool>,
    pub wizard_all_questions: Option<bool>,
    pub mirror: Option<String>,
    pub idf_mirror: Option<String>,
    pub recurse_submodules: Option<bool>,
}

fn custom_styles() -> Styles {
    Styles::styled()
        .header(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan))))
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))))
}

#[derive(Parser, Debug)]
#[command(
    author,
    version = VERSION,
    about = "ESP-IDF Install Manager",
    long_about = "All you need to manage your ESP-IDF installations",
    color = ColorChoice::Always,
    styles = custom_styles()
)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    target: Option<String>,

    #[arg(short, long)]
    idf_versions: Option<String>,

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
        help = "url for download mirror to use instead of github.com for downloading esp-idf"
    )]
    idf_mirror: Option<String>,

    #[arg(
        short,
        long,
        action = clap::ArgAction::Count,
        help = "Increase verbosity level (can be used multiple times)"
    )]
    verbose: u8,

    #[arg(short, long, help = "Set the language for the wizard (en, cn)")]
    locale: Option<String>,

    #[arg(long, help = "file in which logs will be stored (default: eim.log)")]
    log_file: Option<String>,

    #[arg(
        short,
        long,
        help = "Should the installer recurse into submodules of the ESP-IDF repository?"
    )]
    recurse_submodules: Option<bool>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let cli = Cli::parse();

        Self::setup_logging(&cli)?;
        Self::set_locale(&cli.locale);

        let mut builder = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/development").required(false));

        if let Some(config_path) = cli.config.clone() {
            debug!("Using config file: {}", config_path.display());
            builder = builder.add_source(File::from(config_path));
        }

        builder = builder.add_source(config::Environment::with_prefix("ESP").separator("_"));

        let mut cfg = builder.build()?;

        for (key, value) in cli.into_iter() {
            if let Some(v) = value {
                if key != "config" {
                    debug!("Setting {} to {:?}", key, v);
                    cfg.set(&key, v)?;
                }
            }
        }

        cfg.try_deserialize()
    }

    fn setup_logging(cli: &Cli) -> Result<(), ConfigError> {
        let log_file_name = cli.log_file.clone().map_or_else(
            || {
                get_log_directory()
                    .map(|dir| dir.join("eim.log"))
                    .unwrap_or_else(|| {
                        eprintln!("Failed to get log directory, using default eim.log");
                        PathBuf::from("eim.log")
                    })
            },
            PathBuf::from,
        );

        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
            .build(log_file_name)
            .map_err(|e| ConfigError::Message(format!("Failed to build file appender: {}", e)))?;

        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n")))
            .build();

        let log_level = match cli.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

        let config = log4rs::Config::builder()
            .appender(Appender::builder().build("file", Box::new(logfile)))
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(
                Root::builder()
                    .appender("stdout")
                    .appender("file")
                    .build(log_level),
            )
            .map_err(|e| ConfigError::Message(format!("Failed to build log4rs config: {}", e)))?;

        log4rs::init_config(config)
            .map_err(|e| ConfigError::Message(format!("Failed to initialize logger: {}", e)))?;

        Ok(())
    }

    fn set_locale(locale: &Option<String>) {
        match locale {
            Some(l) => {
                rust_i18n::set_locale(l);
                info!("Set locale to: {}", l);
            }
            None => debug!("No locale specified, defaulting to en"),
        }
    }

    pub fn save(&self, file_path: &str) -> Result<(), ConfigError> {
        let toml_value = toml::to_string(self).map_err(|e| ConfigError::Message(e.to_string()))?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)
            .map_err(|e| ConfigError::Message(e.to_string()))?;
        file.write_all(toml_value.as_bytes())
            .map_err(|e| ConfigError::Message(e.to_string()))?;

        Ok(())
    }
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
                self.non_interactive.map(Into::into),
            ),
            (
                "target".to_string(),
                self.target
                    .map(|s| s.split(',').collect::<Vec<&str>>().into()),
            ),
            (
                "idf_version".to_string(),
                self.idf_versions
                    .map(|s| s.split(',').collect::<Vec<&str>>().into()),
            ),
            (
                "tool_download_folder_name".to_string(),
                self.tool_download_folder_name.map(Into::into),
            ),
            (
                "tool_install_folder_name".to_string(),
                self.tool_install_folder_name.map(Into::into),
            ),
            (
                "tools_json_file".to_string(),
                self.tools_json_file.map(Into::into),
            ),
            (
                "idf_tools_path".to_string(),
                self.idf_tools_path.map(Into::into),
            ),
            ("mirror".to_string(), self.mirror.map(Into::into)),
            ("idf_mirror".to_string(), self.idf_mirror.map(Into::into)),
            (
                "recurse_submodules".to_string(),
                self.recurse_submodules.map(Into::into),
            ),
        ]
        .into_iter()
    }
}
