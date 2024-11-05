use clap::builder::styling::{AnsiColor, Color, Style, Styles};
use clap::{arg, command, ColorChoice, Parser};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    #[arg(
        short,
        long,
        help = "Base Path to which all the files and folder will be installed"
    )]
    path: Option<String>,

    #[arg(
        long,
        help = "Absolute path to save esp_idf.json file. Default is $HOME/.esp_installation_manager/esp_idf.json"
    )]
    esp_idf_json_path: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "You can provide multiple targets separated by comma"
    )]
    target: Option<String>,

    #[arg(
        short,
        long,
        help = "you can provide multiple versions of ESP-IDF separated by comma"
    )]
    idf_versions: Option<String>,

    #[arg(long)]
    pub tool_download_folder_name: Option<String>,

    #[arg(long)]
    pub tool_install_folder_name: Option<String>,

    #[arg(
        long,
        help = "Path to tools.json file relative from ESP-IDF installation folder"
    )]
    pub idf_tools_path: Option<String>,

    #[arg(
        long,
        help = "Path to idf_tools.py file relative from ESP-IDF installation folder"
    )]
    pub tools_json_file: Option<String>,

    #[arg(short, long)]
    pub non_interactive: Option<bool>,

    #[arg(
        short,
        long,
        help = "url for download mirror to use instead of github.com"
    )]
    pub mirror: Option<String>,

    #[arg(
        long,
        help = "url for download mirror to use instead of github.com for downloading esp-idf"
    )]
    pub idf_mirror: Option<String>,

    #[arg(
        short,
        long,
        action = clap::ArgAction::Count,
        help = "Increase verbosity level (can be used multiple times)"
    )]
    pub verbose: u8,

    #[arg(short, long, help = "Set the language for the wizard (en, cn)")]
    pub locale: Option<String>,

    #[arg(long, help = "file in which logs will be stored (default: eim.log)")]
    pub log_file: Option<String>,

    #[arg(
        short,
        long,
        help = "Should the installer recurse into submodules of the ESP-IDF repository (default true) "
    )]
    pub recurse_submodules: Option<bool>,
}

impl IntoIterator for Cli {
    type Item = (String, Option<config::Value>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("path".to_string(), self.path.map(Into::into)),
            (
                "esp_idf_json_path".to_string(),
                self.esp_idf_json_path.map(Into::into),
            ),
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
                "idf_versions".to_string(),
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
