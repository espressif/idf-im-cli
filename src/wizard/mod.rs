use crate::cli_args::{ChipId, Config};
use console::Style;
use dialoguer::theme::ColorfulTheme;
use idf_im_lib::idf_tools::ToolsFile;
use idf_im_lib::idf_versions;
use indicatif::ProgressBar;
use ratatui::prelude::*;
use ratatui_splash_screen::{SplashConfig, SplashError, SplashScreen};
use rfd::FileDialog;
use std::error::Error;
use std::io::{stdout, Stdout};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use std::{env, fs};

// folder select
use dialoguer::{MultiSelect, Select};
use walkdir::{DirEntry, WalkDir};

fn show_splash_screen(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    static SPLASH_CONFIG: SplashConfig = SplashConfig {
        image_data: include_bytes!("../../assets/logo.png"),
        sha256sum: Some("405a91875ca5c1247140c73cd80dbde1962b0f747330058c0989a324bb311d5f"),
        render_steps: 10,
        use_colors: true,
    };

    let mut splash_screen = SplashScreen::new(SPLASH_CONFIG)?;
    while !splash_screen.is_rendered() {
        terminal.draw(|frame| {
            frame.render_widget(&mut splash_screen, frame.size());
        })?;
        std::thread::sleep(Duration::from_millis(120));
    }

    Ok(())
}

fn check_prerequisites(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<Vec<&str>, String> {
    let unsatisfied_prerequisities = idf_im_lib::system_dependencies::check_prerequisites();
    match unsatisfied_prerequisities {
        Ok(prerequisities) => {
            if prerequisities.is_empty() {
                println!("All prerequisities are satisfied!");
                Ok(vec![])
            } else {
                println!("The following prerequisities are not satisfied:");
                println!("{:?}", prerequisities);
                Ok(prerequisities)
            }
        }
        Err(err) => Err(err),
    }
}

async fn select_target() -> Result<ChipId, String> {
    let avalible_targets_result = idf_im_lib::idf_versions::get_avalible_targets().await;
    match avalible_targets_result {
        Ok(avalible_targets) => {
            let selection = Select::new()
                .with_prompt("What target do you choose?")
                .items(&avalible_targets)
                .interact()
                .unwrap();

            return Ok(ChipId::from_str(&avalible_targets[selection].to_uppercase()).unwrap());
        }
        Err(err) => Err(format!("We were unable to get avalible targets {:?}", err)),
    }
}

async fn select_idf_version(target: &str) -> Result<String, String> {
    let avalible_versions =
        idf_im_lib::idf_versions::get_idf_name_by_target(&target.to_string().to_lowercase()).await;

    // TODO: map latest to the latest version
    let selected_version = Select::new()
        .with_prompt("What varsion do you choose?")
        .items(&avalible_versions)
        .interact()
        .unwrap();
    return Ok(avalible_versions[selected_version].clone());
    // custom_path = format!("{}/{}", base_install_path, selected_target).clone();
    // println!("Selected IDF version {:?}", selected_target.to_string());
}

fn download_idf(path: &str, tag: &str) -> Result<String, String> {
    idf_im_lib::ensure_path(&path.to_string());
    let output = idf_im_lib::get_esp_idf_by_tag_name(&path.to_string(), &tag.to_string());
    match output {
        Ok(_) => Ok("ok".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

// TODO: add mirror support
async fn download_tools(
    tools_file: ToolsFile,
    selected_chip: String,
    destination_path: &str,
) -> Vec<String> {
    let list = idf_im_lib::idf_tools::filter_tools_by_target(tools_file.tools, &selected_chip);

    // println!("{:?}", list);
    let platform = match idf_im_lib::idf_tools::get_platform_identification() {
        Ok(platform) => platform,
        Err(err) => {
            panic!("Can not identify platfor for tools instalation.  {:?}", err);
        }
    };
    let download_links = idf_im_lib::idf_tools::change_links_donwanload_mirror(
        idf_im_lib::idf_tools::get_download_link_by_platform(list, &platform),
        // Some("https://dl.espressif.com/github_assets"), // this switches mirror, should be parametrized
        None,
    );
    let mut downloaded_tools: Vec<String> = vec![];
    for (tool_name, download_link) in download_links.iter() {
        println!("Downloading tool: {}", tool_name);
        let progress_bar = ProgressBar::new(100);
        let update_progress = move |amount_downloaded: u64, total_size: u64| {
            let current_progress = ((amount_downloaded as f64) / (total_size as f64)) * 100.0;
            progress_bar.set_position(current_progress as u64);
        };
        println!("Download link: {}", download_link);
        println!("destination: {}", destination_path);

        match idf_im_lib::download_file(download_link, destination_path, &update_progress).await {
            Ok(_) => {
                let file_path = Path::new(download_link);
                let filename: &str = file_path.file_name().unwrap().to_str().unwrap();
                downloaded_tools.push(filename.to_string());
                println!("Downloaded {}", tool_name);
            }
            Err(err) => {
                println!("Failed to download tool: {}", tool_name);
                println!("Error: {:?}", err);
                panic!();
            }
        }
        // TODO: check sha256 of downloaded files
    }
    downloaded_tools
}

fn extract_tools(tools: Vec<String>, source_path: &str, destination_path: &str) {
    for tool in tools.iter() {
        let mut archive_path = PathBuf::from(source_path);
        archive_path.push(tool);
        let out = idf_im_lib::decompress_archive(archive_path.to_str().unwrap(), destination_path);
        match out {
            Ok(_) => {
                println!("extracted tool: {}", tool);
            }
            Err(err) => println!("{:?}", err), // TODO: return error
        }
    }
}

fn python_sanity_check() {
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

fn install_python_environment() {}

fn folder_select(path: &str) -> String {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    let current_folder: Vec<DirEntry> = WalkDir::new(path)
        .min_depth(0)
        .max_depth(0)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();
    let current_path = current_folder[0].path().to_str().unwrap().to_string();
    let mut selects = vec!["."];
    let ancestors = current_folder[0].path().ancestors().collect::<Vec<_>>();
    if !current_folder.is_empty() && ancestors.len() > 1 {
        selects.push("..");
    }
    let directories_in_current_folder: Vec<DirEntry> = WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();
    let filenames = directories_in_current_folder
        .iter()
        .map(|entry| entry.file_name().to_str().unwrap())
        .collect::<Vec<&str>>();
    let select = Select::with_theme(&theme)
        .clear(true)
        .report(false)
        // .with_prompt(format!("Current folder {}", current_path.as_str()))
        .items(&selects)
        .items(&filenames);
    let selection = select.interact().unwrap();
    match selection {
        0 => {
            return current_path;
        } // select current folder
        1 => folder_select(ancestors[1].to_str().unwrap()), // go up
        _ => folder_select(
            directories_in_current_folder[selection - 2]
                .path()
                .to_str()
                .unwrap(),
        ), // open subfolder
    }
}

pub async fn run_wizzard_run(mut config: Config) -> Result<(), String> {
    println!("Config: {:?}", config);

    if let Some(non_interactive) = config.non_interactive {
        if non_interactive {
            panic!("Running Wizard in non-interactive mode is not supported.");
        }
    }
    // unimplemented!()
    // create a terminal
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).map_err(|err| format!("{}", err))?;

    terminal.clear();

    // render splash screen
    // show_splash_screen(&mut terminal);
    // terminal.clear();
    // check prerequisites
    check_prerequisites(&mut terminal);
    terminal.clear();
    python_sanity_check();
    terminal.clear();
    // select target
    if config.target.is_none() {
        let chip_id = match select_target().await {
            Ok(target) => target,
            Err(err) => {
                println!("{:?}", err);
                return Err(err);
            }
        };
        config.target = Some(chip_id);
    }
    let target = config.target.clone().unwrap().to_string();
    terminal.clear();
    // select version
    if config.idf_version.is_none() {
        config.idf_version = Some(select_idf_version(&target).await.unwrap());
    }
    let idf_versions = config.idf_version.unwrap();
    // let version: String = select_idf_version(&config.target).await.unwrap();
    terminal.clear();
    // select folder
    println!("target: {:?}", config.target);
    println!("version: {:?}", idf_versions);
    // instalation path consist from base path and idf version
    let mut instalation_path: PathBuf = PathBuf::new();
    if let Some(path) = config.path {
        instalation_path.push(&path);
    } else {
        instalation_path.push("/tmp/esp-new/"); // default testing folder TODO: remove and move to config
    }
    instalation_path.push(&idf_versions);
    idf_im_lib::ensure_path(&instalation_path.display().to_string());

    println!("Joined path: {}", instalation_path.display());
    // download idf
    let output = download_idf(&instalation_path.to_str().unwrap(), &idf_versions);
    println!("Download output: {:?}", output);
    // pepare paths --> should be somewhat configurable --> config,cmd, env
    let mut tool_download_directory = PathBuf::new();
    tool_download_directory.push(&instalation_path);
    tool_download_directory.push("dist");
    idf_im_lib::ensure_path(&tool_download_directory.display().to_string());
    let mut tool_install_directory = PathBuf::new();
    tool_install_directory.push(&instalation_path);
    tool_install_directory.push("tools");
    idf_im_lib::ensure_path(&tool_install_directory.display().to_string());

    println!(
        "Tool install directory: {}",
        tool_install_directory.display()
    );
    println!(
        "Tool download directory: {}",
        tool_download_directory.display()
    );
    println!("Joined path: {}", instalation_path.display());

    let mut tools_json_file = PathBuf::new();
    tools_json_file.push(&instalation_path);
    tools_json_file.push("esp-idf");
    tools_json_file.push("tools");
    tools_json_file.push("tools.json");

    println!("Tools json file: {}", tools_json_file.display());

    if !fs::metadata(&tools_json_file).is_ok() {
        println!("Tools.json file does not exist. Please select valied tools.json file");
        unimplemented!(); // TODO: select tools.json file using file picker
    }
    let tools =
        match idf_im_lib::idf_tools::read_and_parse_tools_file(tools_json_file.to_str().unwrap()) {
            Ok(tools) => tools,
            Err(err) => {
                panic!("Failed to read tools.json file. Error: {:?}", err);
            }
        };
    let downloaded_tools_list =
        download_tools(tools, target, &tool_download_directory.to_str().unwrap()).await;
    extract_tools(
        downloaded_tools_list,
        &tool_download_directory.to_str().unwrap(),
        &tool_install_directory.to_str().unwrap(),
    );
    idf_im_lib::add_path_to_path(tool_install_directory.to_str().unwrap());
    env::set_var("IDF_TOOLS_PATH", &tool_install_directory);
    // parametrize path too idf_tools.py
    let mut idf_tools_path = PathBuf::new();
    idf_tools_path.push(&instalation_path);
    idf_tools_path.push("esp-idf");
    idf_tools_path.push("tools");
    idf_tools_path.push("idf_tools.py");
    if !fs::metadata(&idf_tools_path).is_ok() {
        println!("idf_tools.py file not found. Please select valid idf_tools.py file");
        unimplemented!(); // TODO: select idf_tools.py file using file picker
    }
    let out = idf_im_lib::python_utils::run_python_script_from_file(
        idf_tools_path.to_str().unwrap(),
        Some("install"),
        None,
    );
    match out {
        Ok(output) => println!("{}", output),
        Err(err) => panic!("Failed to run idf tools: {:?}", err),
    }
    let output = idf_im_lib::python_utils::run_python_script_from_file(
        idf_tools_path.to_str().unwrap(),
        Some("install-python-env"),
        None,
    );
    match output {
        Ok(output) => println!("{}", output),
        Err(err) => panic!("Failed to run idf tools: {:?}", err),
    }
    let output2 = idf_im_lib::python_utils::run_python_script_from_file(
        idf_tools_path.to_str().unwrap(),
        Some("export"),
        None,
    );
    match output2 {
        Ok(output) => println!("{}", output),
        Err(err) => panic!("Failed to run idf tools with param export: {:?}", err),
    }
    Ok(())
}
