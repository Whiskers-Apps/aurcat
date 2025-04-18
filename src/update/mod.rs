use std::{error::Error, process::Command};

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    install::install_aur_package,
    list::get_installed_aur_packages,
    search::AurPackage,
    settings::get_settings,
    utils::{show_error_message, show_success_message},
};

#[derive(Serialize, Deserialize)]
struct AurUpdateResponse {
    pub results: Vec<UpdateResult>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdateResult {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "URLPath")]
    pub url_path: String,
}

pub async fn on_update(skip: Option<bool>) {
    let skip_aur = if let Some(skip) = skip {
        skip
    } else {
        get_settings().skip_aur_update
    };

    let packages_update_success = update_packages().is_ok();

    let aur_update_success = if !skip_aur {
        update_aur_packages().await.is_ok()
    } else {
        true
    };

    if packages_update_success && aur_update_success {
        show_success_message("System was updated successfully. Hopefully nothing breaks 🤞");
    } else {
        show_error_message("😬 Let's just hope nothing broke");
    }
}

fn update_packages() -> Result<(), Box<dyn Error>> {
    let args = if get_settings().skip_install_confirm {
        vec!["pacman", "-Syyu", "--noconfirm"]
    } else {
        vec!["pacman", "-Syyu"]
    };

    let command = Command::new("sudo").args(args).spawn()?.wait()?;

    return if command.success() {
        Ok(())
    } else {
        Err("Error updating".into())
    };
}

async fn update_aur_packages() -> Result<(), Box<dyn Error>> {
    let aur_packages = get_installed_aur_packages(true).unwrap_or(vec![]);

    if aur_packages.is_empty() {
        return Ok(());
    }

    let mut url = String::from("https://aur.archlinux.org/rpc/?v=5&type=info");

    for package in &aur_packages {
        url += &format!("&arg[]={}", package.package);
    }

    let client = Client::new();
    let response = client.get(&url).send().await?;
    let parsed_response: AurUpdateResponse = response.json().await?;

    let mut packages_to_update = Vec::<AurPackage>::new();

    for result in parsed_response.results {
        let package = &aur_packages
            .iter()
            .find(|p| p.package == result.name)
            .unwrap();

        if &package.version == &result.version {
            packages_to_update.push(AurPackage {
                installed: true,
                package: package.package.to_owned(),
                description: "".to_string(),
                version: package.version.to_string(),
                url_path: result.url_path.to_string(),
                outdated: false,
            });
        }
    }

    for package in packages_to_update {
        let _ = install_aur_package(&package, None).await;
    }

    Ok(())
}
