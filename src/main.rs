extern crate idf_im_lib;
use idf_im_lib::ensure_path;
use indicatif::ProgressBar;

use std::path::Path;
use std::{env, vec};

const VERSION: &str = env!("CARGO_PKG_VERSION");
use dialoguer::{MultiSelect, Select};
mod cli_args;
mod wizard;

#[tokio::main]
async fn main() {
    let args = cli_args::get_cli().get_matches();
    let config = cli_args::parse_cli(&args);

    let final_config = wizard::run_wizzard_run(config).await;

    match final_config {
        Ok(_) => println!("Successfully installed IDF"),
        Err(err) => println!("Error: {}", err),
    }

    // next step is source the env vars
    // activate venvironment
    // or at least spit user instruction how to do this
}
