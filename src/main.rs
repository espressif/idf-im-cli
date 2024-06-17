extern crate idf_im_lib;
use idf_im_lib::ensure_path;
use indicatif::ProgressBar;

use clap::{arg, Command};
use std::path::Path;
use std::{env, result, vec};

const VERSION: &str = env!("CARGO_PKG_VERSION");
use dialoguer::{MultiSelect, Select};

use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() {
    // let _matches = Command::new("ESP-IDF Installation Manager")
    //     .version(VERSION)
    //     .about("All you need to manage your ESP-IDF installations")
    //     .arg(arg!(
    //         -n --name <NAME> "state your name"
    //     ))
    //     .get_matches();

    let items = vec![
        "check prerequisities",
        "get avalible idf versions",
        "download and patch idf",
        // "run idf install",
        " * run donwloaded idf python script using rustpython",
        " * test_download",
        "parse tools.json and download tools",
        "python sanity check",
        "install python environment",
    ];

    let selection = MultiSelect::new()
        .with_prompt("What do you choose?")
        .items(&items)
        .interact()
        .unwrap();

    let base_install_path = "/Users/petrgadorek/tmp/test-e"; // the path should be sufixed with the tag so user can install multuiple idf versions

    let mut custom_path = "".to_string(); // the path should be sufixed with the tag so user can install multuiple idf versions
    let mut selected_target = "".to_string(); // the IDF version
    let mut selected_chip = "".to_string();

    for i in selection {
        match i {
            0 => {
                let unsatisfied_prerequisities =
                    idf_im_lib::system_dependencies::check_prerequisites();
                match unsatisfied_prerequisities {
                    Ok(prerequisities) => {
                        if prerequisities.is_empty() {
                            println!("All prerequisities are satisfied!");
                        } else {
                            println!("The following prerequisities are not satisfied:");
                            println!("{:?}", prerequisities);
                        }
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            1 => {
                let avalible_targets_result =
                    idf_im_lib::idf_versions::get_avalible_targets().await;
                match avalible_targets_result {
                    Ok(avalible_targets) => {
                        let selection = Select::new()
                            .with_prompt("What target do you choose?")
                            .items(&avalible_targets)
                            .interact()
                            .unwrap();

                        selected_chip = avalible_targets[selection].clone();
                        let avalible_versions = idf_im_lib::idf_versions::get_idf_name_by_target(
                            &avalible_targets[selection],
                        )
                        .await;

                        let selected_version = Select::new()
                            .with_prompt("What varsion do you choose?")
                            .items(&avalible_versions)
                            .interact()
                            .unwrap();
                        selected_target = avalible_versions[selected_version].clone();
                        custom_path = format!("{}/{}", base_install_path, selected_target).clone();
                        println!("Selected IDF version {:?}", selected_target.to_string());
                    }
                    Err(err) => println!("JELEN {:?}", err),
                }
            }
            2 => {
                // let tag_name = "v5.2.2";
                if selected_target.is_empty() {
                    println!("Please select IDF version first!");
                } else {
                    idf_im_lib::ensure_path(&custom_path);
                    let output =
                        idf_im_lib::get_esp_idf_by_tag_name(&custom_path, &selected_target);
                    println!("{:?}", output);
                    // let output2 = idf_im_lib::apply_patchset(&custom_path);
                    // match output2 {
                    //     Ok(output) => println!("{}", output),
                    //     Err(err) => panic!("Failed to apply patchset: {:?}", err),
                    // }
                }
            }
            // 3 => println!("{:?}", idf_im_lib::python_utils::run_idf_tools()),
            3 => {
                if selected_target.is_empty() {
                    println!("Please select IDF version first!");
                } else {
                    // idf_im_lib::ensure_path(&custom_path);
                    // let output = idf_im_lib::get_rustpython_fork(&custom_path);
                    // println!("{:?}", output);

                    env::set_var("IDF_TOOLS_PATH", &custom_path);
                    let output2 = idf_im_lib::run_idf_tools_using_rustpython(&custom_path);
                    match output2 {
                        Ok(output) => println!("{}", output),
                        Err(err) => panic!("Failed to run idf tools: {:?}", err),
                    }
                    // add the installed tools to path
                    idf_im_lib::add_path_to_path(&format!("{}/tools", base_install_path));
                }
            }
            4 => {
                let progress_bar = ProgressBar::new(100);
                let update_progress = move |amount_downloaded: u64, total_size: u64| {
                    let current_progress =
                        ((amount_downloaded as f64) / (total_size as f64)) * 100.0;
                    progress_bar.set_position(current_progress as u64);
                };
                idf_im_lib::download_file(
                    &"https://bit.ly/1GB-testfile".to_string(),
                    "/tmp",
                    &update_progress,
                )
                .await
                .unwrap();
            }
            5 => {
                if selected_target.is_empty() {
                    println!("Please select IDF version first!");
                } else {
                    let tool_download_directory = &format!("{}/dist", custom_path);
                    idf_im_lib::ensure_path(&tool_download_directory);
                    let tool_install_directory = &format!("{}/tools", custom_path);
                    idf_im_lib::ensure_path(&tool_install_directory);
                    let tool_file_path = &format!("{}/esp-idf/tools/tools.json", custom_path);
                    match idf_im_lib::idf_tools::read_and_parse_tools_file(tool_file_path) {
                        Ok(tools) => {
                            let list = idf_im_lib::idf_tools::filter_tools_by_target(
                                tools.tools,
                                &selected_chip,
                            );

                            // println!("{:?}", list);
                            let platform =
                                match idf_im_lib::idf_tools::get_platform_identification() {
                                    Ok(platform) => platform,
                                    Err(err) => {
                                        println!("{:?}", err);
                                        return;
                                    }
                                };
                            let download_links =
                                idf_im_lib::idf_tools::change_links_donwanload_mirror(
                                    idf_im_lib::idf_tools::get_download_link_by_platform(
                                        list, &platform,
                                    ),
                                    // Some("https://dl.espressif.com/github_assets"), // this switches mirror, should be parametrized
                                    None,
                                );
                            println!("{:#?}", download_links);
                            for (tool_name, download_link) in download_links.iter() {
                                println!("Downloading tool: {}", tool_name);
                                let progress_bar = ProgressBar::new(100);
                                let update_progress =
                                    move |amount_downloaded: u64, total_size: u64| {
                                        let current_progress = ((amount_downloaded as f64)
                                            / (total_size as f64))
                                            * 100.0;
                                        progress_bar.set_position(current_progress as u64);
                                    };

                                idf_im_lib::download_file(
                                    download_link,
                                    tool_download_directory,
                                    &update_progress,
                                )
                                .await
                                .unwrap();
                                // TODO: check sha256 of downloaded files
                                let file_path = Path::new(download_link);
                                let filename: &str =
                                    file_path.file_name().unwrap().to_str().unwrap();
                                let out = idf_im_lib::decompress_archive(
                                    &format!("{}/{}", tool_download_directory, filename),
                                    tool_install_directory,
                                );
                                match out {
                                    Ok(_) => {
                                        println!("extracted tool: {}", tool_name);
                                    }
                                    Err(err) => println!("{:?}", err),
                                }
                                // TODO: add folder with extracted tools to path
                                idf_im_lib::add_path_to_path(tool_install_directory);
                                env::set_var("IDF_TOOLS_PATH", &tool_install_directory);
                                // TODO: delete the archives (??)
                            }
                        }
                        Err(err) => println!("{:?}", err),
                    }
                }
            }
            6 => {
                let outpusts = idf_im_lib::python_utils::python_sanity_check(None);
                let mut all_ok = true;
                for output in outpusts {
                    match output {
                        Ok(output) => println!("{}", output),
                        Err(err) => {
                            all_ok = false;
                            println!("{:?}", err)
                        }
                    }
                }
                if all_ok {
                    println!("All good!")
                } else {
                    panic!(" Your python does not meets the requirements!")
                }
            }
            7 => {
                // let install_utils_script =
                //     &format!("{}/esp-idf/tools/install_util.py", custom_path);
                // let output = idf_im_lib::python_utils::run_python_script_from_file(
                //     install_utils_script,
                //     Some("extract features"),
                //     None,
                // );
                // match output {
                //     Ok(output) => println!("{}", output),
                //     Err(err) => panic!("Failed to run idf tools: {:?}", err),
                // }
                let install_utils_script = &format!("{}/esp-idf/tools/idf_tools.py", custom_path);
                let output2 = idf_im_lib::python_utils::run_python_script_from_file(
                    install_utils_script,
                    Some("install-python-env"),
                    None,
                );
                match output2 {
                    Ok(output) => println!("{}", output),
                    Err(err) => panic!("Failed to run idf tools: {:?}", err),
                }
            }
            _ => panic!("Invalid selection"),
        }
    }
    // call idf_tools.py export (to export the env variables

    // next step is source the env vars
}
