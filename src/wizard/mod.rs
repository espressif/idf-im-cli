use crate::cli_args::Settings;
use console::Style;
use dialoguer::theme::ColorfulTheme;
use idf_im_lib::idf_tools::ToolsFile;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use ratatui::prelude::*;
use ratatui_splash_screen::{SplashConfig, SplashScreen};
use std::error::Error;
use std::fmt::Write;
use std::io::Stdout;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{env, fs, thread};
// folder select
use dialoguer::{Confirm, Input, Select};
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

fn run_with_spinner<F, T>(func: F) -> T
where
    F: FnOnce() -> T,
{
    // Create a new spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner} Running...")
            .unwrap(),
    );

    // Start the spinner
    spinner.enable_steady_tick(Duration::from_millis(50));

    // Start measuring time
    let start_time = Instant::now();

    // Run the function
    let result = func();

    // Stop the spinner
    spinner.finish_and_clear();

    // Calculate duration
    let duration = start_time.elapsed();

    // Print the duration
    // println!("Function completed in: {:?}", duration); //TODO: move to debug

    // Return the result
    result
}

fn check_prerequisites() -> Result<Vec<String>, String> {
    let unsatisfied_prerequisities = idf_im_lib::system_dependencies::check_prerequisites();
    match unsatisfied_prerequisities {
        Ok(prerequisities) => {
            if prerequisities.is_empty() {
                // println!("All prerequisities are satisfied!"); //TODO: move to interactive wizard
                Ok(vec![])
            } else {
                // println!("The following prerequisities are not satisfied:");
                // println!("{:?}", prerequisities);
                Ok(prerequisities.into_iter().map(|p| p.to_string()).collect())
            }
        }
        Err(err) => Err(err),
    }
}

async fn select_target(theme: &ColorfulTheme) -> Result<String, String> {
    let avalible_targets_result = idf_im_lib::idf_versions::get_avalible_targets().await;
    match avalible_targets_result {
        Ok(avalible_targets) => {
            let selection = Select::with_theme(theme)
                .with_prompt("What target do you choose?")
                .items(&avalible_targets)
                .interact()
                .unwrap();

            return Ok(avalible_targets[selection].clone());
        }
        Err(err) => Err(format!("We were unable to get avalible targets {:?}", err)),
    }
}

async fn select_idf_version(target: &str, theme: &ColorfulTheme) -> Result<String, String> {
    let avalible_versions =
        idf_im_lib::idf_versions::get_idf_name_by_target(&target.to_string().to_lowercase()).await;

    // TODO: map latest to the latest version
    let selected_version = Select::with_theme(theme)
        .with_prompt("What varsion do you choose?")
        .items(&avalible_versions)
        .interact()
        .unwrap();
    return Ok(avalible_versions[selected_version].clone());
    // custom_path = format!("{}/{}", base_install_path, selected_target).clone();
    // println!("Selected IDF version {:?}", selected_target.to_string());
}

fn download_idf(path: &str, tag: &str) -> Result<String, String> {
    let _: Result<String, String> = match idf_im_lib::ensure_path(&path.to_string()) {
        Ok(_) => {
            // println!("Path is ok");
            Ok("ok".to_string())
        }
        Err(err) => return Err(err.to_string()), // probably panic
    };
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    let output =
        idf_im_lib::get_esp_idf_by_tag_name(&path.to_string(), &tag.to_string(), |stats| {
            // println!("{}/{}", stats.received_objects(), stats.total_objects());
            let current_progress =
                ((stats.received_objects() as f64) / (stats.total_objects() as f64)) * 100.0;
            progress_bar.set_position(current_progress as u64);

            true
        });
    match output {
        Ok(_) => Ok("ok".to_string()),
        Err(err) => match err.code() {
            git2::ErrorCode::Exists => {
              match Confirm::new().with_prompt("The path already exists. Do you want to procees with instalation withoud redownloading IDF?").interact() {
                Ok(true) => Ok("ok".to_string()),
                Ok(false) => Err(err.to_string()),
                Err(err) => Err(err.to_string()),
              }
            },
            _ => Err(err.to_string()),
        },
    }
}

async fn download_tools(
    tools_file: ToolsFile,
    selected_chip: String,
    destination_path: &str,
) -> Vec<String> {
    let pepa: Vec<String> = tools_file
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect();
    print!("Downloading tools: {:?}", pepa);
    let list = idf_im_lib::idf_tools::filter_tools_by_target(
        tools_file.tools,
        &selected_chip.to_lowercase(),
    );

    let platform = match idf_im_lib::idf_tools::get_platform_identification() {
        Ok(platform) => platform,
        Err(err) => {
            panic!("Can not identify platfor for tools instalation.  {:?}", err);
        }
    };
    // println!("Platform: {}", platform);
    let download_links = idf_im_lib::idf_tools::change_links_donwanload_mirror(
        idf_im_lib::idf_tools::get_download_link_by_platform(list, &platform),
        // Some("https://dl.espressif.com/github_assets"), // this switches mirror, should be parametrized
        None,
    );
    let mut downloaded_tools: Vec<String> = vec![];
    for (tool_name, download_link) in download_links.iter() {
        println!("Downloading tool: {}", tool_name);
        let progress_bar = ProgressBar::new(100);
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

        let update_progress = |amount_downloaded: u64, total_size: u64| {
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
                progress_bar.finish();
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

fn python_sanity_check() -> Result<(), String> {
    let outpusts = idf_im_lib::python_utils::python_sanity_check(None);
    let mut all_ok = true;
    for output in outpusts {
        match output {
            Ok(_) => {}
            Err(err) => {
                all_ok = false;
                println!("{:?}", err)
            }
        }
    }
    if all_ok {
        // println!("All good!")
        Ok(())
    } else {
        Err(" Your python does not meets the requirements!".to_string())
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

pub async fn run_wizzard_run(mut config: Settings) -> Result<(), String> {
    // println!("Config: {:?}", config); // TODO remove

    if let Some(non_interactive) = config.non_interactive {
        if non_interactive {
            panic!("Non interactive instalation not yet supported.");
            // panic!("Running Wizard in non-interactive mode is not supported.");
        }
    }

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    // check prerequisites
    match run_with_spinner::<_, Result<Vec<String>, String>>(|| check_prerequisites()) {
        Ok(list) => {
            if list.is_empty() {
                println!("All prerequisities are satisfied!");
            } else {
                unimplemented!("Please install the following prerequisities: {:?}", list);
                //TODO: offer to install prerequisities
            }
        }
        Err(err) => {
            println!("{:?}", err);
            return Err(err);
        }
    }

    match run_with_spinner::<_, Result<(), String>>(|| python_sanity_check()) {
        Ok(_) => {
            println!("Your python meets the requirements")
        }
        Err(err) => {
            println!("{:?}", err); // python does not meets requirements: TODO: on windows proceeed with instalation of our python
            return Err(err);
        }
    }
    // select target
    if config.target.is_none() {
        let chip_id = match select_target(&theme).await {
            Ok(target) => target,
            Err(err) => {
                println!("{:?}", err);
                return Err(err);
            }
        };
        config.target = Some(chip_id);
    }
    let target = config.target.clone().unwrap().to_string();
    // select version
    // TODO: verify the version support target
    if config.idf_version.is_none() {
        config.idf_version = Some(select_idf_version(&target, &theme).await.unwrap());
    }
    let idf_versions = config.idf_version.unwrap();
    // let version: String = select_idf_version(&config.target).await.unwrap();
    // select folder
    // instalation path consist from base path and idf version
    let mut instalation_path: PathBuf = PathBuf::new();
    if let Some(path) = config.path {
        instalation_path.push(&path);
    } else {
        let path = match Input::with_theme(&theme)
            .with_prompt("base instalation path")
            .default("/tmp/esp-new/".to_string()) // default testing folder TODO: remove and move to config
            .interact()
        {
            Ok(path) => path,
            Err(err) => {
                panic!("Need to select some instalation path:{:?}", err);
            }
        };
        instalation_path.push(path); // default testing folder TODO: remove and move to config
        config.path = Some(instalation_path.clone());
    }
    instalation_path.push(&idf_versions);
    let mut idf_path = instalation_path.clone();
    idf_path.push("esp-idf");
    config.idf_path = Some(idf_path.clone());
    idf_im_lib::add_path_to_path(idf_path.to_str().unwrap());

    // download idf
    match download_idf(&idf_path.to_str().unwrap(), &idf_versions) {
        Ok(_) => {}
        Err(err) => {
            // TODO: offer purging directory and retry
            println!("Please choose valid instalation directory {:?}", err);
            return Err(err);
        }
    }

    let mut tool_download_directory = PathBuf::new();
    tool_download_directory.push(&instalation_path);
    if let Some(name) = config.tool_download_folder_name {
        tool_download_directory.push(&name);
    } else {
        let name = match Input::with_theme(&theme)
            .with_prompt("name folder where the tools will be downloaded")
            .default("dist".to_string())
            .interact()
        {
            Ok(path) => path,
            Err(err) => {
                panic!("Need to select some folder name:{:?}", err);
            }
        };
        tool_download_directory.push(&name); // default testing folder TODO: remove and move to config
        config.tool_download_folder_name = Some(name);
    }
    match idf_im_lib::ensure_path(&tool_download_directory.display().to_string()) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
            return Err(err.to_string());
        }
    }
    let mut tool_install_directory = PathBuf::new();
    tool_install_directory.push(&instalation_path);
    if let Some(name) = config.tool_install_folder_name {
        tool_install_directory.push(&name);
    } else {
        let name = match Input::with_theme(&theme)
            .with_prompt("name folder where the tools will be installed")
            .default("tools".to_string())
            .interact()
        {
            Ok(path) => path,
            Err(err) => {
                panic!("Need to select some folder name:{:?}", err);
            }
        };
        tool_install_directory.push(&name); // default testing folder TODO: remove and move to config
        config.tool_install_folder_name = Some(name);
    }
    idf_im_lib::add_path_to_path(tool_install_directory.to_str().unwrap());

    match idf_im_lib::ensure_path(&tool_install_directory.display().to_string()) {
        Ok(_) => {}
        Err(err) => {
            println!("{:?}", err);
            return Err(err.to_string());
        }
    }

    // tools_json_file

    let mut tools_json_file = PathBuf::new();
    tools_json_file.push(&idf_path);
    if let Some(file) = config.tools_json_file {
        tools_json_file.push(&file);
    } else {
        let name = match Input::with_theme(&theme)
            .with_prompt("specify the relative (from instalation path) path to tools.json file")
            .default("tools/tools.json".to_string()) // TODO: test on windows
            .interact()
        {
            Ok(path) => path,
            Err(err) => {
                panic!("Need to select some folder name:{:?}", err);
            }
        };
        tools_json_file.push(&name); // this may need some multiplatform handling
        config.tools_json_file = Some(name);
    }
    // this approach works regardless platform:
    // tools_json_file.push("esp-idf");
    // tools_json_file.push("tools");
    // tools_json_file.push("tools.json");

    // println!("Tools json file: {}", tools_json_file.display());

    if !fs::metadata(&tools_json_file).is_ok() {
        println!("Tools.json file does not exist. Please select valied tools.json file");
        unimplemented!(); // TODO: select tools.json file using file picker
                          // TODO: implement the retry logic -> in interactive mode the user should not be able to proceed until the files is found
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
    let mut env_vars = vec![];
    env::set_var("IDF_TOOLS_PATH", &tool_install_directory);
    env_vars.push((
        "IDF_TOOLS_PATH".to_string(),
        tool_install_directory.to_str().unwrap().to_string(),
    ));

    let mut python_env_path = PathBuf::new();
    python_env_path.push(&tool_install_directory);
    python_env_path.push("python");

    env::set_var("IDF_PYTHON_ENV_PATH", &python_env_path);
    env_vars.push((
        "IDF_PYTHON_ENV_PATH".to_string(),
        python_env_path.to_str().unwrap().to_string(),
    ));

    let mut idf_tools_path = PathBuf::new();
    idf_tools_path.push(&idf_path);
    if let Some(file) = config.idf_tools_path {
        idf_tools_path.push(&file);
    } else {
        let name = match Input::with_theme(&theme)
            .with_prompt("specify the relative (from instalation path) path to idf_tools.py file")
            .default("tools/idf_tools.py".to_string()) // TODO: test on windows
            .interact()
        {
            Ok(path) => path,
            Err(err) => {
                panic!("Need to select the idf_tools.py file location:{:?}", err);
            }
        };
        idf_tools_path.push(&name); // this may need some multiplatform handling
        config.idf_tools_path = Some(name);
    }
    if !fs::metadata(&idf_tools_path).is_ok() {
        println!("idf_tools.py file not found. Please select valid idf_tools.py file");
        unimplemented!(); // TODO: select idf_tools.py file using file picker
    }
    let out = run_with_spinner::<_, Result<String, String>>(|| {
        idf_im_lib::python_utils::run_python_script_from_file(
            idf_tools_path.to_str().unwrap(),
            Some("install"),
            None,
            Some(&env_vars),
        )
    });
    match out {
        Ok(_output) => {
            // println!("{}", output) // if it's success we should onlyp rint the output to the debug
        }
        Err(err) => panic!("Failed to run idf tools: {:?}", err),
    }
    let output = idf_im_lib::python_utils::run_python_script_from_file(
        idf_tools_path.to_str().unwrap(),
        Some("install-python-env"),
        None,
        Some(&env_vars),
    );
    match output {
        Ok(_output) => {
            // println!("{}", output) // if it's success we should onlyp rint the output to the debug
        }
        Err(err) => panic!("Failed to run idf tools: {:?}", err),
    }
    env_vars.push(("PATH".to_string(), env::var("PATH").unwrap_or_default()));
    // TODO: this should be done in rust instead of python
    let output2 = idf_im_lib::python_utils::run_python_script_from_file(
        idf_tools_path.to_str().unwrap(),
        Some("export"),
        None,
        Some(&env_vars),
    );
    match output2 {
        Ok(output) => {
            // println!("TADY: {}", output);
            let exports = env_vars
                .into_iter()
                .map(|(k, v)| format!("export {}=\"{}\"; ", k, v))
                .collect::<Vec<String>>();
            // println!("exportujeme: {}", exports.join(""));
            println!(
                "please copy and paste the following lines to your terminal:\r\n\r\n {}",
                format!("{} {}; ", exports.join(""), output)
            );
        }
        Err(err) => panic!("Failed to run idf tools with param export: {:?}", err),
    }
    // TODO: offer to save settings
    Ok(())
}