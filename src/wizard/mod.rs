use crate::cli_args::Settings;
use console::Style;
use dialoguer::theme::ColorfulTheme;
use idf_im_lib::idf_tools::ToolsFile;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::{debug, error, info, trace, warn};
use rust_i18n::t;
use std::fmt::Write;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{env, fs};
// folder select
use dialoguer::{Confirm, FolderSelect, Input, MultiSelect, Select};

fn run_with_spinner<F, T>(func: F) -> T
where
    F: FnOnce() -> T,
{
    // Create a new spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template(&format!("{{spinner}} {}", t!("wizard.spinner.message")))
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
    debug!("Function completed in: {:?}", duration);

    // Return the result
    result
}

fn check_prerequisites() -> Result<Vec<String>, String> {
    let unsatisfied_prerequisities = idf_im_lib::system_dependencies::check_prerequisites();
    match unsatisfied_prerequisities {
        Ok(prerequisities) => {
            if prerequisities.is_empty() {
                debug!("All prerequisities are satisfied!");
                Ok(vec![])
            } else {
                debug!("The following prerequisities are not satisfied:");
                debug!("{:?}", prerequisities);
                Ok(prerequisities.into_iter().map(|p| p.to_string()).collect())
            }
        }
        Err(err) => Err(err),
    }
}

async fn select_target(theme: &ColorfulTheme) -> Result<Vec<String>, String> {
    let avalible_targets_result = idf_im_lib::idf_versions::get_avalible_targets().await;
    match avalible_targets_result {
        Ok(mut avalible_targets) => {
            avalible_targets.insert(0, "all".to_string());
            let mut defaults = vec![true];
            defaults.extend(vec![false; avalible_targets.len() - 1]);
            let selection = MultiSelect::with_theme(theme)
                .with_prompt(t!("wizard.select_target.prompt"))
                .items(avalible_targets.clone())
                .defaults(&defaults)
                .interact()
                .unwrap();

            if selection.is_empty() {
                return Err("You must select target".to_string());
            }
            let result = selection
                .into_iter()
                .map(|i| avalible_targets[i].clone())
                .collect();
            Ok(result)
        }
        Err(err) => Err(format!(
            "{} {:?}",
            t!("wizard.select_target.prompt.failure"),
            err
        )),
    }
}

async fn select_idf_version(target: &str, theme: &ColorfulTheme) -> Result<Vec<String>, String> {
    let mut avalible_versions = if target == "all" {
        //todo process vector of targets
        idf_im_lib::idf_versions::get_idf_names().await
    } else {
        idf_im_lib::idf_versions::get_idf_name_by_target(&target.to_string().to_lowercase()).await
    };
    avalible_versions.push("master".to_string());
    let mut defaults = vec![true];
    defaults.extend(vec![false; avalible_versions.len() - 1]);
    let selected_version = MultiSelect::with_theme(theme)
        .with_prompt(t!("wizard.select_idf_version.prompt"))
        .items(&avalible_versions)
        .defaults(&defaults)
        .interact()
        .unwrap();
    if selected_version.is_empty() {
        return Err("You must select IDF version".to_string());
    }

    return Ok(selected_version
        .into_iter()
        .map(|i| avalible_versions[i].clone())
        .collect());
}

fn download_idf(
    path: &str,
    tag: Option<&str>,
    mirror: Option<&str>,
    group_name: Option<&str>,
) -> Result<String, String> {
    let _: Result<String, String> = match idf_im_lib::ensure_path(&path.to_string()) {
        Ok(_) => Ok("ok".to_string()),
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

    let output = idf_im_lib::get_esp_idf_by_tag_name(
        &path.to_string(),
        tag,
        |stats| {
            let current_progress =
                ((stats.received_objects() as f64) / (stats.total_objects() as f64)) * 100.0;
            progress_bar.set_position(current_progress as u64);

            true
        },
        mirror,
        group_name,
    );
    match output {
        Ok(_) => Ok("ok".to_string()),
        Err(err) => match err.code() {
            //TODO: give option to delete and retry
            git2::ErrorCode::Exists => {
                match Confirm::new()
                    .with_prompt(t!("wizard.idf_path_exists.prompt"))
                    .interact()
                {
                    Ok(true) => Ok("ok".to_string()),
                    Ok(false) => Err(err.to_string()),
                    Err(err) => Err(err.to_string()),
                }
            }
            _ => Err(err.to_string()),
        },
    }
}

fn get_tools_export_paths(
    tools_file: ToolsFile,
    selected_chip: Vec<String>,
    tools_install_path: &str,
) -> Vec<String> {
    let list = idf_im_lib::idf_tools::filter_tools_by_target(tools_file.tools, &selected_chip);
    debug!("Creating export paths for: {:?}", list);
    let mut paths = vec![];
    for tool in &list {
        tool.export_paths.iter().for_each(|path| {
            let mut p = PathBuf::new();
            p.push(tools_install_path);
            for level in path {
                p.push(level);
            }
            paths.push(p.to_str().unwrap().to_string());
        });
    }
    debug!("Export paths: {:?}", paths);
    paths
}
async fn download_tools(
    tools_file: ToolsFile,
    selected_chip: Vec<String>,
    destination_path: &str,
    mirror: Option<&str>,
) -> Vec<String> {
    let tool_name_list: Vec<String> = tools_file
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect();
    info!(
        "{}: {:?}",
        t!("wizard.tools_download.progress"),
        tool_name_list
    );
    let list = idf_im_lib::idf_tools::filter_tools_by_target(tools_file.tools, &selected_chip);

    let platform = match idf_im_lib::idf_tools::get_platform_identification() {
        Ok(platform) => platform,
        Err(err) => {
            panic!("{}.  {:?}", t!("wizard.tools_platform_error"), err);
        }
    };
    debug!("Python platform: {}", platform);
    let download_links = idf_im_lib::idf_tools::change_links_donwanload_mirror(
        idf_im_lib::idf_tools::get_download_link_by_platform(list, &platform),
        // Some("https://dl.espressif.com/github_assets"), // this switches mirror, should be parametrized
        mirror,
    );
    let mut downloaded_tools: Vec<String> = vec![];
    for (tool_name, download_link) in download_links.iter() {
        info!("{}: {}", t!("wizard.tool_download.progress"), tool_name);
        let progress_bar = ProgressBar::new(download_link.size);
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

        let update_progress = |amount_downloaded: u64, _total_size: u64| {
            progress_bar.set_position(amount_downloaded);
        };
        debug!("Download link: {}", download_link.url);
        debug!("destination: {}", destination_path);

        let file_path = Path::new(&download_link.url);
        let filename: &str = file_path.file_name().unwrap().to_str().unwrap();

        let full_file_path = Path::new(&destination_path).join(Path::new(filename));
        match idf_im_lib::verify_file_checksum(
            &download_link.sha256,
            full_file_path.to_str().unwrap(),
        ) {
            Ok(true) => {
                downloaded_tools.push(filename.to_string()); // add it to the list for extraction even if it's already downloaded
                info!("{}", t!("wizard.tool_file.present"));
                progress_bar.finish();
                continue;
            }
            _ => {
                debug!("{}", t!("wizard.tool_file.missing"));
            }
        }

        match idf_im_lib::download_file(&download_link.url, destination_path, &update_progress)
            .await
        {
            Ok(_) => {
                downloaded_tools.push(filename.to_string());
                progress_bar.finish();
                info!("{} {}", t!("wizard.tool.downloaded"), tool_name);
            }
            Err(err) => {
                error!("{}: {}", t!("wizard.tool.download_failed"), tool_name);
                error!("Error: {:?}", err);
                panic!();
            }
        }
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
                info!("{}: {}", t!("wizard.tool.extracted"), tool);
            }
            Err(err) => warn!("{:?}", err),
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
        debug!("Python sanity check passed.");
        Ok(())
    } else {
        Err(t!("python.sanitycheck.fail").to_string())
    }
}

fn add_to_shell_rc(content: &str) -> Result<(), String> {
    let shell = env::var("SHELL").unwrap_or_else(|_| String::from(""));
    let home = dirs::home_dir().unwrap();

    let rc_file = match shell.as_str() {
        "/bin/bash" => home.join(".bashrc"),
        "/bin/zsh" => home.join(".zshrc"),
        "/bin/fish" => home.join(".config/fish/config.fish"),
        _ => return Err("Unsupported shell".to_string()),
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(rc_file)
        .unwrap();

    match std::io::Write::write_all(&mut file, content.as_bytes()) {
        Ok(_) => info!("{}", t!("wizard.shellrc.update.success")),
        Err(err) => {
            error!("{}", t!("wizard.shellrc.update.error"));
            error!("Error: {:?}", err);
        }
    };

    Ok(())
}

fn install_python_environment() {}

pub async fn run_wizzard_run(mut config: Settings) -> Result<(), String> {
    debug!("Config entering wizard: {:?}", config);

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
                info!("{}", t!("prerequisites.ok"));
            } else {
                info!("{}", t!("prerequisites.not_ok", l = list.join(", ")));
                // info!("The following prerequisities are not satisfied: {:?}", list);
                if Confirm::with_theme(&theme)
                    .with_prompt(t!("prerequisites.install.prompt"))
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    //TODO: add progress or spinner
                    match idf_im_lib::system_dependencies::install_prerequisites(list) {
                        Ok(_) => match check_prerequisites() {
                            Ok(list) => {
                                if list.is_empty() {
                                    info!("{}", t!("prerequisites.install.success"));
                                } else {
                                    error!(
                                        "{}",
                                        t!(
                                            "prerequisites.install.catastrophic",
                                            l = list.join(", ")
                                        )
                                    );
                                    panic!("{}", t!("prerequisites.install.failure"));
                                }
                            }
                            Err(err) => {
                                error!("{:?}", err);
                                return Err(err);
                            }
                        },
                        Err(err) => {
                            error!("{:?}", err);
                            return Err(err);
                        }
                    }
                } else {
                    error!("{}", t!("prerequisites.install.ask"));
                    return Err(t!("prerequisites.install.ask").to_string());
                }
            }
        }
        Err(err) => {
            error!("{:?}", err);
            return Err(err);
        }
    }
    info!("{}", t!("python.sanitycheck.info"));
    match run_with_spinner::<_, Result<(), String>>(|| python_sanity_check()) {
        Ok(_) => {
            info!("{}", t!("python.sanitycheck.ok"))
        }
        Err(err) => match std::env::consts::OS {
            "windows" => {
                info!("{}", t!("python.sanitycheck.fail"));
                if Confirm::with_theme(&theme)
                    .with_prompt(t!("pythhon.install.prompt"))
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    match idf_im_lib::system_dependencies::install_prerequisites(Vec::from([
                        String::from("python"),
                    ])) {
                        Ok(_) => {
                            info!("{}", t!("python.install.success"));
                        }
                        Err(err) => {
                            error!("{} {:?}", t!("python.install.failure"), err);
                            return Err(err);
                        }
                    }
                } else {
                    return Err(t!("python.install.refuse").to_string());
                }
            }
            _ => {
                error!("{} {:?}", t!("python.sanitycheck.fail"), err);
                return Err(err);
            }
        },
    }
    // select target
    if config.target.is_none() {
        let chip_id = match select_target(&theme).await {
            Ok(target) => target,
            Err(err) => {
                error!("{:?}", err);
                return Err(err);
            }
        };
        config.target = Some(chip_id);
    }
    let target = config.target.clone().unwrap();
    debug!("Selected target: {:?}", target);
    // select version
    if config.idf_versions.is_none() {
        let selected_idf_version = select_idf_version(&target.clone()[0], &theme).await; // TODO: handle multiple targets
        match selected_idf_version {
            Ok(selected_idf_version) => config.idf_versions = Some(selected_idf_version),
            Err(err) => {
                error!("{:?}", err);
                return Err(err);
            }
        }
    }
    let idf_versions = config.idf_versions.clone().unwrap();
    debug!("Selected idf version: {:?}", idf_versions);

    // mirrors select

    let idf_mirror = match config.idf_mirror.clone() {
        Some(mirror) => mirror,
        None => {
            let mirrors = vec![
                "https://github.com",
                "https://jihulab.com/esp-mirror",
                "https://gitee.com/",
            ];
            mirrors[Select::with_theme(&theme)
                .with_prompt(t!("wizard.idf.mirror"))
                .items(&mirrors)
                .default(0)
                .interact()
                .unwrap()]
            .to_string()
        }
    };
    config.idf_mirror = Some(idf_mirror.clone());
    let group_name = if idf_mirror.contains("https://gitee.com/") {
        Some("EspressifSystems")
    } else {
        None
    };

    let dl_mirror = match config.mirror.clone() {
        Some(mirror) => mirror,
        None => {
            let mirrors = vec![
                "https://github.com",
                "https://dl.espressif.com/github_assets",
                "https://dl.espressif.cn/github_assets",
            ];
            mirrors[Select::with_theme(&theme)
                .with_prompt(t!("wizard.tools.mirror"))
                .items(&mirrors)
                .default(0)
                .interact()
                .unwrap()]
            .to_string()
        }
    };
    config.mirror = Some(dl_mirror.clone());

    // select folder
    // instalation path consist from base path and idf version
    let mut instalation_path: PathBuf = PathBuf::new();
    if let Some(path) = config.path.clone() {
        instalation_path.push(&path);
    } else {
        let mut default_path = "/tmp/esp-new/".to_string();
        if std::env::consts::OS == "windows" {
            default_path = "C:\\esp\\".to_string();
        }

        let path = match Input::with_theme(&theme)
            .with_prompt(t!("wizard.instalation_path.prompt"))
            .default(default_path) // default testing folder TODO: remove and move to config
            .interact()
        {
            Ok(path) => path,
            Err(err) => {
                panic!("{} :{:?}", t!("wizard.instalation_path.unselected"), err);
            }
        };
        instalation_path.push(path); // default testing folder TODO: remove and move to config
        config.path = Some(instalation_path.clone());
    }

    // Multiple version starts here

    for idf_version in idf_versions {
        let mut version_instalation_path = instalation_path.clone();
        version_instalation_path.push(&idf_version);
        let mut idf_path = version_instalation_path.clone();
        idf_path.push("esp-idf");
        config.idf_path = Some(idf_path.clone());
        idf_im_lib::add_path_to_path(idf_path.to_str().unwrap());

        // download idf
        let tag = if idf_version == "master" {
            None
        } else {
            Some(idf_version.clone())
        };
        match download_idf(
            &idf_path.to_str().unwrap(),
            tag.as_deref(),
            Some(&idf_mirror),
            group_name,
        ) {
            Ok(_) => {
                debug!("{}", t!("wizard.idf.sucess"));
            }
            Err(err) => {
                // TODO: offer purging directory and retry
                error!("{} {:?}", t!("wizard.idf.failure"), err);
                return Err(err);
            }
        }

        let mut tool_download_directory = PathBuf::new();
        tool_download_directory.push(&version_instalation_path);
        let default_tools_download_folder_name = "dist"; // TODO: move to config too?
        if let Some(name) = config.tool_download_folder_name.clone() {
            tool_download_directory.push(&name);
        } else if config.wizard_all_questions.is_some() && config.wizard_all_questions.unwrap() {
            let name = match Input::with_theme(&theme)
                .with_prompt(t!("wizard.tools.donwload.prompt"))
                .default(default_tools_download_folder_name.to_string())
                .interact()
            {
                Ok(path) => path,
                Err(err) => {
                    panic!("{} :{:?}", t!("wizard.tools.donwload.prompt.failure"), err);
                }
            };
            tool_download_directory.push(&name);
            config.tool_download_folder_name = Some(name);
        } else {
            tool_download_directory.push(default_tools_download_folder_name);
            config.tool_download_folder_name = Some(default_tools_download_folder_name.to_string());
        }
        match idf_im_lib::ensure_path(&tool_download_directory.display().to_string()) {
            Ok(_) => {}
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        }
        let mut tool_install_directory = PathBuf::new();
        tool_install_directory.push(&version_instalation_path);
        let default_tools_install_folder_name = "tools"; // TODO: move to config too?
        if let Some(name) = config.tool_install_folder_name.clone() {
            tool_install_directory.push(&name);
        } else if config.wizard_all_questions.is_some() && config.wizard_all_questions.unwrap() {
            let name = match Input::with_theme(&theme)
                .with_prompt(t!("wizard.tools.install.prompt"))
                .default(default_tools_install_folder_name.to_string())
                .interact()
            {
                Ok(path) => path,
                Err(err) => {
                    panic!("{} :{:?}", t!("wizard.tools.install.prompt.failure"), err);
                }
            };
            tool_install_directory.push(&name);
            config.tool_install_folder_name = Some(name);
        } else {
            tool_install_directory.push(default_tools_install_folder_name);
            config.tool_install_folder_name = Some(default_tools_install_folder_name.to_string());
        }
        idf_im_lib::add_path_to_path(tool_install_directory.to_str().unwrap());

        match idf_im_lib::ensure_path(&tool_install_directory.display().to_string()) {
            Ok(_) => {}
            Err(err) => {
                error!("{:?}", err);
                return Err(err.to_string());
            }
        }

        // tools_json_file

        let mut tools_json_file = PathBuf::new();
        tools_json_file.push(&idf_path);
        let default_tools_json_file_location = "tools/tools.json"; // TODO: move to config too?
        if let Some(file) = config.tools_json_file.clone() {
            tools_json_file.push(&file);
        } else if config.wizard_all_questions.is_some() && config.wizard_all_questions.unwrap() {
            let name = match Input::with_theme(&theme)
                .with_prompt(t!("wizard.tooljs_json.prompt"))
                .default(default_tools_json_file_location.to_string())
                .interact()
            {
                Ok(path) => path,
                Err(err) => {
                    panic!("{} :{:?}", t!("wizard.tools_json.prompt.failure"), err);
                }
            };
            tools_json_file.push(&name);
            config.tools_json_file = Some(name);
        } else {
            tools_json_file.push(default_tools_json_file_location);
            config.tools_json_file = Some(default_tools_json_file_location.to_string());
        }

        if !fs::metadata(&tools_json_file).is_ok() {
            warn!("{}", t!("wizard.tools_json.not_found"));
            let tools_json_file_select = FolderSelect::with_theme(&theme)
                .with_prompt(t!("wizard.tools_json.select.prompt"))
                .folder(idf_path.to_str().unwrap())
                .file(true)
                .interact()
                .unwrap();
            if fs::metadata(&tools_json_file_select).is_ok() {
                tools_json_file = PathBuf::from(tools_json_file_select);
                config.tools_json_file = Some(tools_json_file.to_str().unwrap().to_string());
            } else {
                // TODO: implement the retry logic -> in interactive mode the user should not be able to proceed until the files is found
                panic!("{}", t!("wizard.tools_json.unreachable"));
            }
        }

        debug!("Tools json file: {}", tools_json_file.display());

        let tools = match idf_im_lib::idf_tools::read_and_parse_tools_file(
            tools_json_file.to_str().unwrap(),
        ) {
            Ok(tools) => tools,
            Err(err) => {
                panic!(
                    "{}",
                    t!("wizard.tools_json.unparsable", e = err.to_string())
                );
            }
        };

        let downloaded_tools_list = download_tools(
            tools.clone(),
            target.clone(),
            tool_download_directory.to_str().unwrap(),
            Some(&dl_mirror),
        )
        .await;
        extract_tools(
            downloaded_tools_list,
            tool_download_directory.to_str().unwrap(),
            tool_install_directory.to_str().unwrap(),
        );
        idf_im_lib::add_path_to_path(tool_install_directory.to_str().unwrap());
        let mut env_vars = vec![];
        env::set_var("IDF_TOOLS_PATH", &tool_install_directory);
        env_vars.push((
            "IDF_TOOLS_PATH".to_string(),
            tool_install_directory.to_str().unwrap().to_string(),
        ));
        env_vars.push((
            "IDF_PATH".to_string(),
            idf_path.to_str().unwrap().to_string(),
        ));

        let mut python_env_path = PathBuf::new();
        python_env_path.push(&tool_install_directory);
        python_env_path.push("python");

        env::set_var("IDF_PYTHON_ENV_PATH", &python_env_path);
        debug!("Python env path: {}", python_env_path.display());
        env_vars.push((
            "IDF_PYTHON_ENV_PATH".to_string(),
            python_env_path.to_str().unwrap().to_string(),
        ));

        let mut idf_tools_path = PathBuf::new();
        idf_tools_path.push(&idf_path);
        let default_idf_tools_py_file_location: &str = "./tools/idf_tools.py"; // TODO: move to config too?
        if let Some(file) = config.idf_tools_path.clone() {
            idf_tools_path.push(&file);
        } else if config.wizard_all_questions.is_some() && config.wizard_all_questions.unwrap() {
            let name = match Input::with_theme(&theme)
                .with_prompt(t!("wizard.idf_tools.prompt"))
                .default(default_idf_tools_py_file_location.to_string())
                .interact()
            {
                Ok(path) => path,
                Err(err) => {
                    panic!("{} :{:?}", t!("wizard.idf_tools.select.prompt"), err);
                }
            };
            idf_tools_path.push(&name); // this may need some multiplatform handling
            config.idf_tools_path = Some(name);
        } else {
            idf_tools_path.push(default_idf_tools_py_file_location);
            config.idf_tools_path = Some(default_idf_tools_py_file_location.to_string());
        }
        if !fs::metadata(&idf_tools_path).is_ok() {
            warn!("{}", t!("wizard.idf_tools.not_found"));
            let idf_tools_py_select = FolderSelect::with_theme(&theme)
                .with_prompt(t!("wizard.idf_tools.select.prompt"))
                .folder(idf_path.to_str().unwrap())
                .file(true)
                .interact()
                .unwrap();
            if fs::metadata(&idf_tools_py_select).is_ok() {
                idf_tools_path = PathBuf::from(&idf_tools_py_select);
                config.idf_tools_path = Some(idf_tools_py_select);
            } else {
                // TODO: implement the retry logic -> in interactive mode the user should not be able to proceed until the files is found
                panic!("{}", t!("wizard.idf_tools.unreachable"));
            }
        }
        println!("ENV before install {:?}", env_vars);
        let out = run_with_spinner::<_, Result<String, String>>(|| {
            idf_im_lib::python_utils::run_python_script_from_file(
                idf_tools_path.to_str().unwrap(),
                Some("install"),
                None,
                Some(&env_vars),
            )
        });
        match out {
            Ok(output) => {
                trace!("idf_tools.py install output:\r\n{}", output) // if it's success we should onlyp rint the output to the debug
            }
            Err(err) => panic!(
                "{}",
                t!("wizard.idf_tools.failed_to_run", e = err.to_string())
            ),
        }
        println!("ENV before install-python-env {:?}", env_vars);
        let output = idf_im_lib::python_utils::run_python_script_from_file(
            idf_tools_path.to_str().unwrap(),
            Some("install-python-env"),
            None,
            Some(&env_vars),
        );
        match output {
            Ok(output) => {
                trace!("idf_tools.py install-python-env output:\r\n{}", output)
            }
            Err(err) => panic!(
                "{}",
                t!("wizard.idf_tools.failed_to_run", e = err.to_string())
            ),
        }
        let export_paths = get_tools_export_paths(
            tools,
            target.clone(),
            tool_install_directory.to_str().unwrap(),
        );

        if std::env::consts::OS == "windows" {
            // for p in export_paths {
            //     let _ = idf_im_lib::win_tools::add_to_win_path(&p);
            // }
            println!("{}", t!("wizard.windows.succes_message"));
            // Creating desktop shortcut
            match idf_im_lib::create_desktop_shortcut(
                version_instalation_path.to_str().unwrap(),
                idf_path.to_str().unwrap(),
                &idf_version,
                tool_install_directory.to_str().unwrap(),
            ) {
                Ok(_) => info!("{}", t!("wizard.after_install.desktop_shortcut.created")),
                Err(err) => {
                    error!(
                        "{} {:?}",
                        t!("wizard.after_install.desktop_shortcut.failed"),
                        err.to_string()
                    )
                }
            }
        }

        if std::env::consts::OS != "windows" {
            let exports = env_vars
                .into_iter()
                .map(|(k, v)| format!("export {}=\"{}\"; ", k, v))
                .collect::<Vec<String>>();
            let exp_strig = format!(
                "{}{}; ",
                exports.join(""),
                format!("export PATH=\"$PATH:{:?}\"", export_paths.join(":"))
            );

            match Confirm::new()
                .with_prompt(t!("wizard.after_install.add_to_path.prompt"))
                .interact()
            {
                Ok(true) => match add_to_shell_rc(&exp_strig) {
                    Ok(_) => println!("{}", t!("wizard.posix.succes_message")),
                    Err(err) => panic!("{:?}", err.to_string()),
                },
                Ok(false) => println!(
                    "{}:\r\n\r\n{}\r\n\r\n",
                    t!("wizard.posix.succes_message"),
                    exp_strig
                ),
                Err(err) => panic!("{:?}", err.to_string()),
            }
        }
    }

    match Confirm::new()
        .with_prompt(t!("wizard.after_install.save_config.prompt"))
        .interact()
    {
        Ok(true) => match config.save("eim_config.toml") {
            // TODO: make the name configurable
            // TODO: make path configurable
            Ok(_) => println!("{}", t!("wizard.after_install.config.saved")),
            Err(err) => panic!(
                "{} {:?}",
                t!("wizard.after_install.config.save_failed"),
                err.to_string()
            ),
        },
        _ => (),
    }

    Ok(())
}
