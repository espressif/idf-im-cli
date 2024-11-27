use std::path::PathBuf;

use clap::Parser;
use cli_args::{Cli, Commands};
use config::ConfigError;
use log::{debug, error, info, LevelFilter};
extern crate idf_im_lib;
use idf_im_lib::get_log_directory;
use idf_im_lib::settings::Settings;
mod cli_args;
mod wizard;

rust_i18n::i18n!("locales", fallback = "en");

use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};

fn setup_logging(cli: &cli_args::Cli) -> Result<(), config::ConfigError> {
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

    // ... (rest of the logging setup code)
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
        .appender(
            Appender::builder()
                .filter(Box::new(log4rs::filter::threshold::ThresholdFilter::new(
                    LevelFilter::Trace,
                )))
                .build("file", Box::new(logfile)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(log4rs::filter::threshold::ThresholdFilter::new(
                    log_level,
                )))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Trace),
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    setup_logging(&cli).unwrap();
    set_locale(&cli.locale);

    match &cli.command {
        Commands::Install(install_args) => {
            let settings = Settings::new(
                install_args.config.clone(),
                install_args.clone().into_iter(),
            );
            match settings {
                Ok(settings) => {
                    let result = wizard::run_wizzard_run(settings).await;
                    match result {
                        Ok(r) => {
                            info!("Wizard result: {:?}", r);
                            println!("Successfully installed IDF");
                            println!("Now you can start using IDF tools");
                        }
                        Err(err) => error!("Error: {}", err),
                    }
                }
                Err(err) => error!("Error: {}", err),
            }
        }
        Commands::List => {
            // Implement listing installed versions
            println!("Listing installed versions...");
            match idf_im_lib::version_manager::get_esp_ide_config() {
                Ok(config) => {
                    println!("Installed versions:");
                    for version in config.idf_installed {
                        if version.id == config.idf_selected_id {
                            println!("- {} (selected)", version.name);
                        } else {
                            println!("- {}", version.name);
                        }
                    }
                }
                Err(err) => error!("Error: {}", err),
            }
        }
        Commands::Select { version } => {
            // Implement version selection
            println!("Selecting version: {}", version);
        }
        Commands::Discover => {
            // Implement version discovery
            println!("Discovering available versions...");
        }
        Commands::Remove { version } => {
            // Implement version removal
            println!("Removing version: {}", version);
        }
        Commands::Purge => {
            // Implement complete purge
            println!("Purging all installations...");
        }
    }

    // let settings = Settings::new(cli.config.clone(), cli.into_iter());
    // // let settings = cli_args::Settings::new();
    // match settings {
    //     Ok(settings) => {
    //         let result = wizard::run_wizzard_run(settings).await;
    //         match result {
    //             Ok(r) => {
    //                 info!("Wizard result: {:?}", r);
    //                 println!("Successfully installed IDF");
    //                 println!("Now you can start using IDF tools");
    //             }
    //             Err(err) => error!("Error: {}", err),
    //         }
    //     }
    //     Err(err) => error!("Error: {}", err),
    // }
}
