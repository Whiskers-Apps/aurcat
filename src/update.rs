use std::error::Error;

use crate::{
    install::{AurPackageInfoResponse, install_aur_package},
    list::get_aur_packages,
    utils::run,
};

pub async fn on_update_command(
    aur: bool,
    review: bool,
    confirm: bool,
) -> Result<(), Box<dyn Error>> {
    update_repo_packages(confirm)?;

    if !aur {
        return Ok(());
    }

    update_aur_packages(review).await?;

    Ok(())
}

pub fn update_repo_packages(confirm: bool) -> Result<(), Box<dyn Error>> {
    let mut command = vec![
        "sudo".to_string(),
        "pacman".to_string(),
        "-Syyu".to_string(),
    ];

    if !confirm {
        command.push("--noconfirm".to_string());
    }

    run(&command)?;

    Ok(())
}

pub async fn update_aur_packages(review: bool) -> Result<(), Box<dyn Error>> {
    let mut url = "https://aur.archlinux.org/rpc/?v=5&type=info".to_string();
    let packages = get_aur_packages(true)?;

    for package in &packages {
        url = format!("{url}&arg[]={}", package.package);
    }

    let response_json = reqwest::get(&url).await?.text().await?;
    let response: AurPackageInfoResponse = serde_json::from_str(&response_json)?;

    for result in &response.results {
        for package in &packages {
            if result.name == package.package && result.version != package.version {
                install_aur_package(&package.package, review).await?;
            }
        }
    }

    Ok(())
}
