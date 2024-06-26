use log::{error, info};
use simple_logger::SimpleLogger;
extern crate idf_im_lib;

mod cli_args;
mod wizard;

#[tokio::main]
async fn main() {
    let settings = cli_args::Settings::new();
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

    // next step is source the env vars
    // activate venvironment
    // or at least spit user instruction how to do this
}
