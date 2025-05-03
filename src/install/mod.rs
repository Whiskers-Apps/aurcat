use std::{
    error::Error,
    fs::{self, create_dir_all},
    process::{Command, Stdio},
};

use colored::Colorize;
use inquire::Confirm;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    list::{get_installed_aur_packages, get_installed_packages},
    paths::{get_cache_dir, get_package_cache_dir},
    search::{AurPackage, get_aur_package_search, get_package_search},
    settings::get_settings,
    uninstall::{uninstall_aur_package, uninstall_package},
    utils::{get_spinner, show_error_message, show_success_message},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InfoResponse {
    pub results: Vec<InfoResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InfoResult {
    #[serde(rename = "Conflicts", default = "get_empty_vec")]
    pub conflicts: Vec<String>,

    #[serde(rename = "Depends", default = "get_empty_vec")]
    pub depends: Vec<String>,

    #[serde(rename = "MakeDepends", default = "get_empty_vec")]
    pub make_depends: Vec<String>,
}

pub fn get_empty_vec() -> Vec<String> {
    vec![]
}

pub async fn on_install(packages: &Vec<String>, confirm: Option<bool>) {
    for package in packages {
        let spinner = get_spinner("Searching Package");

        let package_search = get_package_search(package).unwrap_or(vec![]);
        let aur_package_search = get_aur_package_search(package).await.unwrap_or(vec![]);

        let official_package = package_search.iter().find(|p| &p.package == package);
        let aur_package = aur_package_search.iter().find(|p| &p.package == package);

        spinner.finish_and_clear();

        if official_package.is_some() {
            if install_package(package, confirm).is_ok() {
                show_success_message("📦 The package was successfully installed");
            } else {
                show_error_message("The package couldn't be installed");
            }
        } else if aur_package.is_some() {
            let aur_package = aur_package.unwrap();

            if install_aur_package(&aur_package, confirm).await.is_ok() {
                show_success_message("📦 The package was successfully installed");
            } else {
                show_error_message("The package could't be installed");
            };
        } else {
            show_error_message("Couldn't find the package");
        }
    }
}

/// Installs a package from the official repositories
pub fn install_package(package: &str, confirm: Option<bool>) -> Result<(), Box<dyn Error>> {
    let skip_confirm = if let Some(confirm) = confirm {
        confirm
    } else {
        get_settings().skip_install_confirm
    };

    let args = if skip_confirm {
        vec!["pacman", "-S", package, "--noconfirm"]
    } else {
        vec!["pacman", "-S", package]
    };

    let command = Command::new("sudo")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    return if command.success() {
        Ok(())
    } else {
        Err("Error installing package".into())
    };
}

/// Installs a package from the AUR
pub async fn install_aur_package(
    package: &AurPackage,
    confirm: Option<bool>,
) -> Result<(), Box<dyn Error>> {
    let info_url = format!(
        "https://aur.archlinux.org/rpc/?v=5&type=info&arg={}",
        package.package
    );

    let info_client = Client::new();
    let info_response = info_client.get(&info_url).send().await?;
    let result = info_response
        .json::<InfoResponse>()
        .await?
        .results
        .get(0)
        .unwrap()
        .to_owned();

    let depends = result
        .depends
        .iter()
        .map(|p| format!("{} ", p))
        .collect::<String>()
        .bold();

    let make_depends = result
        .make_depends
        .iter()
        .map(|p| format!("{} ", p))
        .collect::<String>()
        .bold();

    let local_packages = get_installed_packages().unwrap_or(vec![]);
    let aur_packages = get_installed_aur_packages(false).unwrap_or(vec![]);

    if !result.conflicts.is_empty() {
        for conflict in result.conflicts {
            if local_packages.iter().any(|lp| lp.package == conflict) {
                println!(
                    "The package {} conflicts with {}",
                    package.package.bold(),
                    conflict.clone().red().bold()
                );

                let uninstall =
                    Confirm::new(&format!("Would you like to uninstall {}", conflict.bold()))
                        .with_default(false)
                        .prompt()?;

                if uninstall {
                    if uninstall_package(&conflict, confirm).is_err() {
                        return Err("Error uninstalling conflicting package".into());
                    };
                } else {
                    return Err("Conflict happened".into());
                }
            }

            if aur_packages.iter().any(|ap| ap.package == conflict) {
                println!(
                    "The package {} conflicts with {}",
                    package.package.blue().bold(),
                    conflict.red().bold()
                );

                let uninstall =
                    Confirm::new(&format!("Would you like to uninstall {}", conflict.bold()))
                        .with_default(false)
                        .prompt()?;

                if uninstall {
                    if uninstall_aur_package(&conflict, confirm).is_err() {
                        return Err("Error uninstalling conflicting package".into());
                    };
                } else {
                    return Err("Conflict happened".into());
                }
            }
        }
    }

    if !make_depends.is_empty() {
        let local_package_names = local_packages
            .clone()
            .into_iter()
            .map(|p| p.package)
            .collect::<Vec<String>>();

        let missing_deps = result
            .make_depends
            .clone()
            .into_iter()
            .filter(|p| local_package_names.contains(p))
            .collect::<Vec<String>>();

        for pkg in missing_deps {
            install_package(&pkg, confirm)?;
        }
    }

    if !depends.is_empty() {
        let local_package_names = local_packages
            .clone()
            .into_iter()
            .map(|p| p.package)
            .collect::<Vec<String>>();

        let missing_deps = result
            .make_depends
            .clone()
            .into_iter()
            .filter(|p| local_package_names.contains(p))
            .collect::<Vec<String>>();

        for pkg in missing_deps {
            install_package(&pkg, confirm)?;
        }
    }

    let url = format!("https://aur.archlinux.org{}", package.url_path);

    let cache_dir = get_cache_dir();

    if !cache_dir.exists() {
        create_dir_all(&cache_dir)?;
    }

    let package_dir = get_package_cache_dir(&package.package);

    if !package_dir.exists() {
        create_dir_all(&package_dir)?;
    }

    let version_dir = package_dir.join(&package.version);

    if !version_dir.exists() {
        create_dir_all(&version_dir)?;
    }

    let package_path = version_dir.join(format!("{}.tar.gz", &package.version));

    let client = Client::new();
    let response = client.get(&url).send().await?;

    let bytes = response.bytes().await?;
    fs::write(&package_path, &bytes)?;

    let command = Command::new("tar")
        .args([
            "-xzf",
            &package_path.into_os_string().into_string().unwrap(),
            "--strip-components=1",
        ])
        .current_dir(&version_dir)
        .output()?;

    if !command.status.success() {
        return Err("Couldn't untar the xz file".into());
    }

    let skip_confirm = if let Some(confirm) = confirm {
        confirm
    } else {
        get_settings().skip_install_confirm
    };

    let args = if skip_confirm {
        vec!["-si", "--noconfirm", "--needed", "--syncdeps"]
    } else {
        vec!["-si", "--needed", "--syncdeps"]
    };

    let command = Command::new("makepkg")
        .args(args)
        .current_dir(&version_dir)
        .spawn()?
        .wait()?;

    return if command.success() {
        Ok(())
    } else {
        Err("Couldn't uninstall package".into())
    };
}
