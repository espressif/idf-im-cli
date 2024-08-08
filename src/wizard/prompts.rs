use std::path::PathBuf;

use crate::cli_args::Settings;
pub use crate::wizard::helpers;
use helpers::{
    first_defaulted_multiselect, generic_confirm, generic_input, generic_select, run_with_spinner,
};
use idf_im_lib::system_dependencies;
use log::{debug, info};
use rust_i18n::t;

pub async fn select_target() -> Result<Vec<String>, String> {
    let mut available_targets = idf_im_lib::idf_versions::get_avalible_targets().await?;
    available_targets.insert(0, "all".to_string());
    first_defaulted_multiselect("wizard.select_target.prompt", &available_targets)
}

pub async fn select_idf_version(target: &str) -> Result<Vec<String>, String> {
    let mut avalible_versions = if target == "all" {
        //todo process vector of targets
        idf_im_lib::idf_versions::get_idf_names().await
    } else {
        idf_im_lib::idf_versions::get_idf_name_by_target(&target.to_string().to_lowercase()).await
    };
    avalible_versions.push("master".to_string());
    first_defaulted_multiselect("wizard.select_idf_version.prompt", &avalible_versions)
}

fn check_prerequisites() -> Result<Vec<String>, String> {
    match system_dependencies::check_prerequisites() {
        Ok(prerequisites) => {
            if prerequisites.is_empty() {
                debug!("All prerequisites are satisfied!");
                Ok(vec![])
            } else {
                info!(
                    "The following prerequisites are not satisfied: {:?}",
                    prerequisites
                );
                Ok(prerequisites.into_iter().map(|p| p.to_string()).collect())
            }
        }
        Err(err) => Err(err),
    }
}
pub fn check_and_install_prerequisites() -> Result<(), String> {
    let unsatisfied_prerequisites = run_with_spinner(check_prerequisites)?;
    if !unsatisfied_prerequisites.is_empty() {
        info!(
            "{}",
            t!(
                "prerequisites.not_ok",
                l = unsatisfied_prerequisites.join(", ")
            )
        );
        if generic_confirm("prerequisites.install.prompt").map_err(|e| e.to_string())? {
            system_dependencies::install_prerequisites(unsatisfied_prerequisites)
                .map_err(|e| e.to_string())?;

            let remaining_prerequisites = check_prerequisites()?;
            if !remaining_prerequisites.is_empty() {
                return Err(format!(
                    "{}",
                    t!(
                        "prerequisites.install.catastrophic",
                        l = remaining_prerequisites.join(", ")
                    ),
                ));
            }
        } else {
            return Err(t!("prerequisites.install.ask").to_string());
        }
    } else {
        info!("{}", t!("prerequisites.ok"))
    }

    Ok(())
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
pub fn check_and_install_python() -> Result<(), String> {
    info!("{}", t!("python.sanitycheck.info"));
    if let Err(err) = run_with_spinner(python_sanity_check) {
        if std::env::consts::OS == "windows" {
            info!("{}", t!("python.sanitycheck.fail"));
            if generic_confirm("pythhon.install.prompt").map_err(|e| e.to_string())? {
                system_dependencies::install_prerequisites(vec!["python".to_string()])
                    .map_err(|e| e.to_string())?;
                info!("{}", t!("python.install.success"));
            } else {
                return Err(t!("python.install.refuse").to_string());
            }
        } else {
            return Err(format!("{} {:?}", t!("python.sanitycheck.fail"), err));
        }
    } else {
        info!("{}", t!("python.sanitycheck.ok"))
    }
    Ok(())
}

pub fn select_mirrors(mut config: Settings) -> Result<Settings, String> {
    config.idf_mirror = match config.idf_mirror {
        Some(mirror) => Some(mirror),
        None => Some(generic_select(
            "wizard.idf.mirror",
            &[
                "https://github.com",
                "https://jihulab.com/esp-mirror",
                "https://gitee.com/",
            ],
        )?),
    };

    config.mirror = match config.mirror {
        Some(mirror) => Some(mirror),
        None => Some(generic_select(
            "wizard.tools.mirror",
            &[
                "https://github.com",
                "https://dl.espressif.com/github_assets",
                "https://dl.espressif.cn/github_assets",
            ],
        )?),
    };

    Ok(config)
}

pub fn select_installation_path(mut config: Settings) -> Result<Settings, String> {
    if config.path.is_none() {
        let default_path = if std::env::consts::OS == "windows" {
            "C:\\esp\\".to_string()
        } else {
            format!("{}/.espressif", dirs::home_dir().unwrap().display())
        };
        let mut installation_path = PathBuf::new();
        let path = generic_input(
            "wizard.installation_path.prompt",
            "wizard.installation_path.unselected",
            &default_path,
        )
        .unwrap();

        installation_path.push(path);
        config.path = Some(installation_path.clone());
    }

    Ok(config)
}

pub fn save_config_if_desired(config: &Settings) -> Result<(), String> {
    if let Ok(true) = generic_confirm("wizard.after_install.save_config.prompt") {
        config
            .save("eim_config.toml")
            .map_err(|e| format!("{} {:?}", t!("wizard.after_install.config.save_failed"), e))?;
        println!("{}", t!("wizard.after_install.config.saved"));
    }
    Ok(())
}
