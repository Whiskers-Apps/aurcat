use std::{
    error::Error,
    fs::{self},
    process::{Command, exit},
};

use reqwest::get;
use serde::{Deserialize, Serialize};

use crate::{
    config::get_config,
    search::{SearchQuery, on_search_command, search_aur_packages},
    utils::{run, run_hidden_in_path, run_in_path, show_message},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AurPackageInfoResponse {
    results: Vec<AurPackageInfoResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AurPackageInfoResult {
    #[serde(rename = "LastModified")]
    last_modified: usize,

    #[serde(rename = "URLPath")]
    url_path: String,

    #[serde(rename = "Version")]
    version: String,
}

pub async fn on_install_command(
    packages: Vec<String>,
    search_fallback: bool,
    review: bool,
    confirm_installation: bool,
) -> Result<(), Box<dyn Error>> {
    // println!("{} {}", search_fallback, review);

    let packages_len = packages.len();

    if packages_len == 1 {
        let package = packages.get(0).unwrap();
        let command = Command::new("pacman").args(["-Si", package]).output()?;
        let found = command.status.success();

        if found {
            install_packages(vec![package.to_string()], confirm_installation)?;
            exit(0);
        }

        let aur_packages = search_aur_packages(package).await?;

        let found = aur_packages
            .iter()
            .any(|aur_package| &aur_package.package == package);

        if found {
            install_aur_package(package, review).await?;
            exit(0);
        }

        if search_fallback {
            on_search_command(package.to_owned(), true, Some(review)).await?;
            exit(0);
        }

        show_message("Package not found");
        exit(1);
    }

    // This should be implemented in case a package exist on multiple repos like on cachyos
    // pacman -S <repo_name>/<package_name>

    Ok(())
}

pub fn install_packages(
    packages: Vec<String>,
    confirm_installation: bool,
) -> Result<(), Box<dyn Error>> {
    let mut command = vec!["sudo".to_string(), "pacman".to_string(), "-S".to_string()];
    let mut packages = packages;

    command.append(&mut packages);

    if !confirm_installation {
        command.push("--noconfirm".to_string());
    }

    run(&command)?;

    Ok(())
}

pub fn install_from_query(query: &SearchQuery) -> Result<(), Box<dyn Error>> {
    install_packages(
        vec![query.package.to_string()],
        get_config()?.confirm_installation,
    )?;

    Ok(())
}

pub async fn install_aur_package(package: &str, review: bool) -> Result<(), Box<dyn Error>> {
    let url = format!("https://aur.archlinux.org/rpc/?v=5&type=info&arg={package}");

    let response_json = reqwest::get(url).await?.text().await?;
    let response: AurPackageInfoResponse = serde_json::from_str(&response_json)?;

    let info = response
        .results
        .get(0)
        .ok_or_else(|| "Failed to get package info".to_string())?;

    let cache_dir = dirs::cache_dir()
        .expect("Failed to get cache dir")
        .join("aurcat")
        .join(&package)
        .join(format!(
            "{}-{}",
            &info.last_modified.to_string(),
            &info.version
        ));

    if !cache_dir.exists() {
        show_message("Downloading Package Build");

        fs::create_dir_all(&cache_dir).expect("Failed to create cache dir");

        let tgz_path = cache_dir.clone().join("content.tar.gz");

        let bytes = get(&format!("https://aur.archlinux.org{}", &info.url_path))
            .await?
            .bytes()
            .await?;

        fs::write(&tgz_path, &bytes)?;

        run_hidden_in_path(
            &[
                "tar",
                "-xzf",
                &tgz_path.display().to_string(),
                "--strip-components=1",
            ],
            &cache_dir,
        )
        .expect("Failed to run makepkg");
    }

    run_in_path(&["makepkg", "-si"], &cache_dir)?;

    Ok(())
}
