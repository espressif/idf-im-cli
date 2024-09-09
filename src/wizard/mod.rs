use crate::cli_args::Settings;
use dialoguer::FolderSelect;
use idf_im_lib::idf_tools::ToolsFile;
use idf_im_lib::ProgressMessage;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use log::{debug, error, info, trace, warn};
use rust_i18n::t;
use std::sync::mpsc;
use std::thread;
use std::{
    env,
    fmt::Write,
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

// maybe move the default values to the config too?
const DEFAULT_TOOLS_DOWNLOAD_FOLDER: &str = "dist";
const DEFAULT_TOOLS_INSTALL_FOLDER: &str = "tools";
const DEFAULT_TOOLS_JSON_LOCATION: &str = "tools/tools.json";
const DEFAULT_IDF_TOOLS_PY_LOCATION: &str = "./tools/idf_tools.py";

pub mod helpers;
use helpers::{
    create_progress_bar, create_theme, generic_confirm, generic_input, update_progress_bar,
    update_progress_bar_number,
};

mod prompts;
use prompts::*;

fn get_tools_export_paths(
    // TODO: move to library
    tools_file: ToolsFile,
    selected_chip: Vec<String>,
    tools_install_path: &str,
) -> Vec<String> {
    let bin_dirs = find_bin_directories(Path::new(tools_install_path));
    debug!("Bin directories: {:?}", bin_dirs);

    let list = idf_im_lib::idf_tools::filter_tools_by_target(tools_file.tools, &selected_chip);
    // debug!("Creating export paths for: {:?}", list);
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
    for bin_dir in bin_dirs {
        let str_p = bin_dir.to_str().unwrap().to_string();
        if paths.contains(&str_p) {
            trace!("Skipping duplicate export path: {}", str_p);
        } else {
            trace!("Adding export path: {}", str_p);
            paths.push(str_p);
        }
    }
    debug!("Export paths: {:?}", paths);
    paths
}

fn find_bin_directories(path: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().and_then(|n| n.to_str()) == Some("bin") {
                    result.push(path.clone());
                } else {
                    result.extend(find_bin_directories(&path));
                }
            }
        }
    }

    result
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

    let platform = match idf_im_lib::idf_tools::get_platform_identification(None) {
        Ok(platform) => platform,
        Err(err) => {
            if std::env::consts::OS == "windows" {
                // All this is for cases when on windows microsoft store creates "pseudolinks" for python
                let scp = idf_im_lib::system_dependencies::get_scoop_path();
                let usable_python = match scp {
                    Some(path) => {
                        let mut python_path = PathBuf::from(path);
                        python_path.push("python3.exe");
                        python_path.to_str().unwrap().to_string()
                    }
                    None => "python3.exe".to_string(),
                };
                match idf_im_lib::idf_tools::get_platform_identification(Some(&usable_python)) {
                    Ok(platform) => platform,
                    Err(err) => {
                        error!("Unable to identify platform: {}", err);
                        panic!("{}.  {:?}", t!("wizard.tools_platform_error"), err);
                    }
                }
            } else {
                panic!("{}.  {:?}", t!("wizard.tools_platform_error"), err);
            }
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

async fn select_targets_and_versions(mut config: Settings) -> Result<Settings, String> {
    if config.target.is_none() {
        config.target = Some(select_target().await?);
    }
    let target = config.target.clone().unwrap();
    debug!("Selected target: {:?}", target);

    if config.idf_versions.is_none() {
        config.idf_versions = Some(select_idf_version(&target[0]).await?);
        // TODO: handle multiple targets
    }
    let idf_versions = config.idf_versions.clone().unwrap();
    debug!("Selected idf version: {:?}", idf_versions);

    Ok(config)
}

pub struct DownloadConfig {
    pub idf_path: String,
    pub idf_version: String,
    pub idf_mirror: Option<String>,
    pub recurse_submodules: Option<bool>,
}

pub enum DownloadError {
    PathCreationFailed(String),
    DownloadFailed(String),
    UserCancelled,
}

fn handle_download_error(err: git2::Error) -> Result<(), DownloadError> {
    match err.code() {
        git2::ErrorCode::Exists => match generic_confirm("wizard.idf_path_exists.prompt") {
            Ok(true) => Ok(()),
            Ok(false) => Err(DownloadError::UserCancelled),
            Err(e) => Err(DownloadError::DownloadFailed(e.to_string())),
        },
        _ => Err(DownloadError::DownloadFailed(err.to_string())),
    }
}

pub fn download_idf(config: DownloadConfig) -> Result<(), DownloadError> {
    idf_im_lib::ensure_path(&config.idf_path)
        .map_err(|err| DownloadError::PathCreationFailed(err.to_string()))?;

    let (tx, rx) = mpsc::channel();

    // Spawn a thread to handle progress bar updates
    let handle = thread::spawn(move || {
        let mut progress_bar = create_progress_bar();

        loop {
            match rx.recv() {
                Ok(ProgressMessage::Finish) => {
                    update_progress_bar_number(&progress_bar, 100);
                    progress_bar.finish();
                    progress_bar = create_progress_bar();
                }
                Ok(ProgressMessage::Update(value)) => {
                    update_progress_bar_number(&progress_bar, value);
                }
                Err(_) => {
                    println!("Channel closed, exiting.");
                    break;
                }
            }
        }
    });

    // let progress_bar = create_progress_bar();

    let tag = if config.idf_version == "master" {
        None
    } else {
        Some(config.idf_version)
    };
    let group_name = config
        .idf_mirror
        .as_deref()
        .map(|mirror| {
            if mirror.contains("https://gitee.com/") {
                Some("EspressifSystems")
            } else {
                None
            }
        })
        .flatten();

    match idf_im_lib::get_esp_idf_by_tag_name(
        &config.idf_path,
        tag.as_deref(),
        tx,
        config.idf_mirror.as_deref(),
        group_name,
        config.recurse_submodules.unwrap_or(false),
    ) {
        Ok(_) => {
            debug!("{}", t!("wizard.idf.success"));
            handle.join().unwrap();
            Ok(())
        }
        Err(err) => handle_download_error(err),
    }
}

fn setup_directory(
    wizard_all_questions: Option<bool>,
    base_path: &PathBuf,
    config_field: &mut Option<String>,
    prompt_key: &str,
    default_name: &str,
) -> Result<PathBuf, String> {
    let mut directory = base_path.clone();

    if let Some(name) = config_field.clone() {
        directory.push(name);
    } else if wizard_all_questions.unwrap_or(false) {
        let name = generic_input(prompt_key, &format!("{}.failure", prompt_key), default_name)?;
        directory.push(&name);
        *config_field = Some(name);
    } else {
        directory.push(default_name);
        *config_field = Some(default_name.to_string());
    }

    idf_im_lib::ensure_path(&directory.display().to_string()).map_err(|err| err.to_string())?;
    Ok(directory)
}

fn get_tools_json_path(config: &mut Settings, idf_path: &Path) -> PathBuf {
    let mut tools_json_file = idf_path.to_path_buf();

    if let Some(file) = &config.tools_json_file {
        tools_json_file.push(file);
    } else if config.wizard_all_questions.unwrap_or(false) {
        let name = generic_input(
            "wizard.tools_json.prompt",
            "wizard.tools_json.prompt.failure",
            DEFAULT_TOOLS_JSON_LOCATION,
        )
        .unwrap();
        tools_json_file.push(&name);
        config.tools_json_file = Some(name);
    } else {
        tools_json_file.push(DEFAULT_TOOLS_JSON_LOCATION);
        config.tools_json_file = Some(DEFAULT_TOOLS_JSON_LOCATION.to_string());
    }

    tools_json_file
}

fn validate_tools_json_file(tools_json_file: &Path, config: &mut Settings) -> String {
    if fs::metadata(tools_json_file).is_err() {
        warn!("{}", t!("wizard.tools_json.not_found"));
        let selected_file = FolderSelect::with_theme(&create_theme())
            .with_prompt(t!("wizard.tools_json.select.prompt"))
            .folder(tools_json_file.to_str().unwrap())
            .file(true)
            .interact()
            .unwrap();
        if fs::metadata(&selected_file).is_ok() {
            config.tools_json_file = Some(selected_file.to_string());
            selected_file
        } else {
            // TODO: implement the retry logic -> in interactive mode the user should not be able to proceed until the files is found
            panic!("{}", t!("wizard.tools_json.unreachable"));
        }
    } else {
        tools_json_file.to_str().unwrap().to_string()
    }
}

async fn download_and_extract_tools(
    config: &Settings,
    tools: &ToolsFile,
    download_dir: &PathBuf,
    install_dir: &PathBuf,
) -> Result<(), String> {
    let downloaded_tools_list = download_tools(
        tools.clone(),
        config.target.clone().unwrap(),
        download_dir.to_str().unwrap(),
        config.mirror.as_deref(),
    )
    .await;

    extract_tools(
        downloaded_tools_list,
        download_dir.to_str().unwrap(),
        install_dir.to_str().unwrap(),
    );

    Ok(())
}

fn setup_environment_variables(
    tool_install_directory: &PathBuf,
    idf_path: &PathBuf,
) -> Result<Vec<(String, String)>, String> {
    let mut env_vars = vec![];

    // env::set_var("IDF_TOOLS_PATH", tool_install_directory);
    let instal_dir_string = tool_install_directory.to_str().unwrap().to_string();
    env_vars.push((
        "IDF_TOOLS_PATH".to_string(),
        if instal_dir_string.contains(" ") {
            format!("\"{}\"", instal_dir_string)
        } else {
            instal_dir_string
        },
    ));
    let idf_path_string = idf_path.to_str().unwrap().to_string();
    env_vars.push((
        "IDF_PATH".to_string(),
        if idf_path_string.contains(" ") {
            format!("\"{}\"", idf_path_string)
        } else {
            idf_path_string
        },
    ));

    let python_env_path_string = tool_install_directory
        .join("python")
        .to_str()
        .unwrap()
        .to_string();
    env_vars.push((
        "IDF_PYTHON_ENV_PATH".to_string(),
        if python_env_path_string.contains(" ") {
            format!("\"{}\"", python_env_path_string)
        } else {
            python_env_path_string
        },
    ));

    Ok(env_vars)
}

fn get_and_validate_idf_tools_path(
    config: &mut Settings,
    idf_path: &PathBuf,
) -> Result<PathBuf, String> {
    let mut idf_tools_path = idf_path.clone();

    if let Some(file) = config.idf_tools_path.clone() {
        idf_tools_path.push(&file);
    } else if config.wizard_all_questions.unwrap_or(false) {
        let name = generic_input(
            "wizard.idf_tools.prompt",
            "wizard.idf_tools.prompt.failure",
            DEFAULT_IDF_TOOLS_PY_LOCATION,
        )?;

        idf_tools_path.push(&name);
        config.idf_tools_path = Some(name);
    } else {
        idf_tools_path.push(DEFAULT_IDF_TOOLS_PY_LOCATION);
        config.idf_tools_path = Some(DEFAULT_IDF_TOOLS_PY_LOCATION.to_string());
    }

    if fs::metadata(&idf_tools_path).is_err() {
        warn!("{}", t!("wizard.idf_tools.not_found"));
        let idf_tools_py_select = FolderSelect::with_theme(&create_theme())
            .with_prompt(t!("wizard.idf_tools.select.prompt"))
            .folder(idf_path.to_str().unwrap())
            .file(true)
            .interact()
            .map_err(|e| format!("Failed to select: {}", e))?;

        if fs::metadata(&idf_tools_py_select).is_ok() {
            idf_tools_path = PathBuf::from(&idf_tools_py_select);
            config.idf_tools_path = Some(idf_tools_py_select);
        } else {
            return Err(t!("wizard.idf_tools.unreachable").to_string());
        }
    }

    Ok(idf_tools_path)
}

fn run_idf_tools_py(
    idf_tools_path: &str,
    environment_variables: &Vec<(String, String)>,
) -> Result<String, String> {
    run_install_script(idf_tools_path, environment_variables)?;
    run_install_python_env_script(idf_tools_path, environment_variables)
}

fn run_install_script(
    idf_tools_path: &str,
    environment_variables: &Vec<(String, String)>,
) -> Result<String, String> {
    let output = idf_im_lib::python_utils::run_python_script_from_file(
        idf_tools_path,
        Some("install"),
        None,
        Some(environment_variables),
    );

    trace!("idf_tools.py install output:\n{:?}", output);

    output
}

fn run_install_python_env_script(
    idf_tools_path: &str,
    environment_variables: &Vec<(String, String)>,
) -> Result<String, String> {
    let output = idf_im_lib::python_utils::run_python_script_from_file(
        idf_tools_path,
        Some("install-python-env"),
        None,
        Some(environment_variables),
    );

    trace!("idf_tools.py install-python-env output:\n{:?}", output);

    output
}

fn single_version_post_install(
    version_instalation_path: &str,
    idf_path: &str,
    idf_version: &str,
    tool_install_directory: &str,
    export_paths: Vec<String>,
    env_vars: Vec<(String, String)>, //probably dupliocate of idf_path and IDF_python_env_path
) {
    match std::env::consts::OS {
        "windows" => {
            println!("{}", t!("wizard.windows.succes_message"));
            // Creating desktop shortcut
            if let Err(err) = idf_im_lib::create_desktop_shortcut(
                version_instalation_path,
                idf_path,
                &idf_version,
                tool_install_directory,
                export_paths,
            ) {
                error!(
                    "{} {:?}",
                    t!("wizard.after_install.desktop_shortcut.failed"),
                    err.to_string()
                )
            } else {
                info!("{}", t!("wizard.after_install.desktop_shortcut.created"))
            }
        }
        _ => {
            let install_folder = PathBuf::from(version_instalation_path);
            let install_path = install_folder.parent().unwrap().to_str().unwrap();
            let _ = idf_im_lib::create_activation_shell_script(
                // todo: handle error
                install_path,
                idf_path,
                tool_install_directory,
                &idf_version,
                export_paths,
            );

            // let exports = env_vars
            //     .into_iter()
            //     .map(|(k, v)| format!("export {}=\"{}\"; ", k, v))
            //     .collect::<Vec<String>>();
            // let exp_strig = format!(
            //     "{}export PATH=\"$PATH:{:?}\"; ",
            //     exports.join(""),
            //     export_paths.join(":")
            // );
            // match generic_confirm("wizard.after_install.add_to_path.prompt") {
            //     Ok(true) => match add_to_shell_rc(&exp_strig) {
            //         Ok(_) => println!("{}", t!("wizard.posix.succes_message")),
            //         Err(err) => panic!("{:?}", err.to_string()),
            //     },
            //     Ok(false) => println!(
            //         "{}:\r\n\r\n{}\r\n\r\n",
            //         t!("wizard.posix.succes_message"),
            //         exp_strig
            //     ),
            //     Err(err) => panic!("{:?}", err.to_string()),
            // }
        }
    }
}

pub async fn run_wizzard_run(mut config: Settings) -> Result<(), String> {
    debug!("Config entering wizard: {:?}", config);

    if let Some(non_interactive) = config.non_interactive {
        if non_interactive {
            panic!("Non interactive instalation not yet supported.");
            // panic!("Running Wizard in non-interactive mode is not supported.");
        }
    }

    // Check prerequisites
    check_and_install_prerequisites()?;

    // Python sanity check
    check_and_install_python()?;

    // select target & idf version
    config = select_targets_and_versions(config).await?;

    // mirrors select
    config = select_mirrors(config)?;

    config = select_installation_path(config)?;

    // Multiple version starts here

    for idf_version in config.idf_versions.clone().unwrap() {
        let mut version_instalation_path = config.path.clone().unwrap();
        version_instalation_path.push(&idf_version);
        let mut idf_path = version_instalation_path.clone();
        idf_path.push("esp-idf");
        config.idf_path = Some(idf_path.clone());
        idf_im_lib::add_path_to_path(idf_path.to_str().unwrap());

        // download idf
        let download_config = DownloadConfig {
            idf_path: idf_path.to_str().unwrap().to_string(),
            idf_version: idf_version.to_string(),
            idf_mirror: config.idf_mirror.clone(),
            recurse_submodules: config.recurse_submodules,
        };

        match download_idf(download_config) {
            Ok(_) => {
                debug!("{}", t!("wizard.idf.sucess"));
            }
            Err(DownloadError::PathCreationFailed(err)) => {
                error!("{} {:?}", t!("wizard.idf.path_creation_failure"), err);
                return Err(err);
            }
            Err(DownloadError::DownloadFailed(err)) => {
                error!("{} {:?}", t!("wizard.idf.failure"), err);
                return Err(err);
            }
            Err(DownloadError::UserCancelled) => {
                error!("{}", t!("wizard.idf.user_cancelled"));
                return Err("User cancelled the operation".to_string());
            }
        }
        // setup tool directories

        let tool_download_directory = setup_directory(
            config.wizard_all_questions.clone(),
            &version_instalation_path,
            &mut config.tool_download_folder_name,
            "wizard.tools.download.prompt",
            DEFAULT_TOOLS_DOWNLOAD_FOLDER,
        )?;

        // Setup install directory
        let tool_install_directory = setup_directory(
            config.wizard_all_questions.clone(),
            &version_instalation_path,
            &mut config.tool_install_folder_name,
            "wizard.tools.install.prompt",
            DEFAULT_TOOLS_INSTALL_FOLDER,
        )?;

        idf_im_lib::add_path_to_path(tool_install_directory.to_str().unwrap());

        // tools_json_file

        let tools_json_file = get_tools_json_path(&mut config, &idf_path);
        let validated_file = validate_tools_json_file(&tools_json_file, &mut config);

        debug!("Tools json file: {}", tools_json_file.display());

        let tools = idf_im_lib::idf_tools::read_and_parse_tools_file(&validated_file)
            .map_err(|err| format!("{}: {}", t!("wizard.tools_json.unparsable"), err))?;

        download_and_extract_tools(
            &&config,
            &tools,
            &tool_download_directory,
            &tool_install_directory,
        )
        .await?;

        let env_vars = setup_environment_variables(&tool_install_directory, &idf_path)?;

        let idf_tools_path = get_and_validate_idf_tools_path(&mut config, &idf_path)?;

        run_idf_tools_py(idf_tools_path.to_str().unwrap(), &env_vars)?;

        let export_paths = get_tools_export_paths(
            tools,
            config.target.clone().unwrap().clone(),
            tool_install_directory.join("tools").to_str().unwrap(),
        );

        single_version_post_install(
            &version_instalation_path.to_str().unwrap(),
            &idf_path.to_str().unwrap(),
            &idf_version,
            &tool_install_directory.to_str().unwrap(),
            export_paths,
            env_vars,
        )
    }
    save_config_if_desired(&config)?;
    match std::env::consts::OS {
        "windows" => {
            println!("{}", t!("wizard.windows.finish_steps.line_1"));
            println!("{}", t!("wizard.windows.finish_steps.line_2"));
        }
        _ => {
            println!("{}", t!("wizard.posix.finish_steps.line_1"));
            println!("{}", t!("wizard.posix.finish_steps.line_2"));
            println!("{:?}", config.path.clone().unwrap());
            println!("{}", t!("wizard.posix.finish_steps.line_3"));
            println!("============================================");
            println!("{}:", t!("wizard.posix.finish_steps.line_4"));
            for idf_version in config.idf_versions.clone().unwrap() {
                println!(
                    "       {} \"{}/{}\"",
                    t!("wizard.posix.finish_steps.line_5"),
                    config.path.clone().unwrap().to_str().unwrap(),
                    format!("activate_idf_{}.sh", idf_version),
                );
            }
            println!("============================================");
        }
    }
    Ok(())
}
