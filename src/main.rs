use std::path::{Path, PathBuf};

use clap::Parser;
use cli_args::{Cli, Commands};
use config::ConfigError;
use idf_im_lib::version_manager::{remove_single_idf_version, select_idf_version};
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
use wizard::helpers::{generic_input, generic_select};

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
            println!("Listing installed versions...");
            match idf_im_lib::version_manager::get_esp_ide_config() {
                Ok(config) => {
                    if config.idf_installed.len() == 0 {
                        println!(
                            "No versions found. Use eim install to install a new ESP-IDF version."
                        );
                    } else {
                        println!("Installed versions:");
                        for version in config.idf_installed {
                            if version.id == config.idf_selected_id {
                                println!("- {} (selected)", version.name);
                            } else {
                                println!("- {}", version.name);
                            }
                        }
                    }
                }
                Err(err) => error!("Error: {}", err),
            }
        }
        Commands::Select { version } => {
            if version.is_none() {
                match idf_im_lib::version_manager::list_installed_versions() {
                    Ok(versions) => {
                        if versions.len() == 0 {
                            println!("No versions installed");
                        } else {
                            println!("Available versions:");
                            let options = versions.iter().map(|v| v.name.clone()).collect();
                            match generic_select("Which version do you want to select?", &options) {
                                Ok(selected) => match select_idf_version(&selected) {
                                    Ok(_) => {
                                        println!("Selected version: {}", selected);
                                    }
                                    Err(err) => error!("Error: {}", err),
                                },
                                Err(err) => error!("Error: {}", err),
                            }
                        }
                    }
                    Err(err) => error!("Error: {}", err),
                }
            } else {
                match select_idf_version(&version.clone().unwrap()) {
                    Ok(_) => {
                        println!("Selected version: {}", version.clone().unwrap());
                    }
                    Err(err) => error!("Error: {}", err),
                }
            }
        }
        Commands::Rename { version, new_name } => {
            if version.is_none() {
                match idf_im_lib::version_manager::list_installed_versions() {
                    Ok(versions) => {
                        if versions.len() == 0 {
                            println!("No versions installed");
                        } else {
                            let options = versions.iter().map(|v| v.name.clone()).collect();
                            let version = crate::wizard::helpers::generic_select(
                                "Which version do you want to rename?",
                                &options,
                            )
                            .unwrap(); // todo move to function and add error handling
                            let new_name = generic_input(
                                "Enter new name:",
                                "you need to enter a new name",
                                "",
                            )
                            .unwrap(); // todo move to function and add error handling
                            match idf_im_lib::version_manager::rename_idf_version(
                                &version, new_name,
                            ) {
                                Ok(_) => {
                                    println!("Version renamed.");
                                }
                                Err(err) => error!("Error: {}", err),
                            }
                        }
                    }
                    Err(err) => error!("Error: {}", err),
                }
            } else if new_name.is_none() {
                let new_name =
                    generic_input("Enter new name:", "you need to enter a new name", "").unwrap(); // todo move to function and add error handling
                match idf_im_lib::version_manager::rename_idf_version(
                    &version.clone().unwrap(),
                    new_name,
                ) {
                    Ok(_) => {
                        println!("Version renamed.");
                    }
                    Err(err) => error!("Error: {}", err),
                }
            } else {
                match idf_im_lib::version_manager::rename_idf_version(
                    &version.clone().unwrap(),
                    new_name.clone().unwrap(),
                ) {
                    Ok(_) => {
                        println!("Version renamed.");
                    }
                    Err(err) => error!("Error: {}", err),
                }
            }
        }
        Commands::Discover => {
            // Implement version discovery
            println!("Discovering available versions...");
            let idf_dirs = idf_im_lib::version_manager::find_esp_idf_folders("/");
            for dir in idf_dirs {
                println!("Found IDF directory: {}", dir);
            }
        }
        Commands::Remove { version } => {
            // todo: add spinner
            if version.is_none() {
                match idf_im_lib::version_manager::list_installed_versions() {
                    Ok(versions) => {
                        if versions.len() == 0 {
                            println!("No versions installed");
                        } else {
                            println!("Available versions:");
                            let options = versions.iter().map(|v| v.name.clone()).collect();
                            match generic_select("Which version do you want to remove?", &options) {
                                Ok(selected) => match remove_single_idf_version(&selected) {
                                    Ok(_) => {
                                        println!("Removed version: {}", selected);
                                    }
                                    Err(err) => error!("Error: {}", err),
                                },
                                Err(err) => error!("Error: {}", err),
                            }
                        }
                    }
                    Err(err) => error!("Error: {}", err),
                }
            } else {
                match remove_single_idf_version(&version.clone().unwrap()) {
                    Ok(_) => {
                        println!("Removed version: {}", version.clone().unwrap());
                    }
                    Err(err) => error!("Error: {}", err),
                }
            }
        }
        Commands::Purge => {
            // Todo: offer to run discovery first
            println!("Purging all IDF installations...");
            match idf_im_lib::version_manager::list_installed_versions() {
                Ok(versions) => {
                    if versions.len() == 0 {
                        println!("No versions installed");
                    } else {
                        for version in versions {
                            println!("Removing version: {}", version.name);
                            match remove_single_idf_version(&version.name) {
                                Ok(_) => {
                                    println!("Removed version: {}", version.name);
                                }
                                Err(err) => error!("Error: {}", err),
                            }
                        }
                    }
                }
                Err(err) => error!("Error: {}", err),
            }
        }
    }
}
