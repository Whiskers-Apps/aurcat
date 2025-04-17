use std::{
    error::Error,
    fs::{self, create_dir_all},
    process::Command,
};

use reqwest::Client;

use crate::{
    paths::{get_cache_dir, get_package_cache_dir},
    search::{AurPackage, get_aur_package_search, get_package_search},
    settings::get_settings,
    utils::{show_error_message, show_success_message},
};

pub async fn on_install(package: &str, confirm: Option<bool>) {
    let package_search = get_package_search(package).unwrap_or(vec![]);
    let aur_package_search = get_aur_package_search(package).await.unwrap_or(vec![]);

    let official_package = package_search.iter().find(|p| p.package == package);
    let aur_package = aur_package_search.iter().find(|p| p.package == package);

    if official_package.is_some() {
        show_success_message("🔍 Found official package");

        if install_package(package, confirm).is_ok() {
            show_success_message("📦 The package was successfully installed");
        } else {
            show_error_message("😮 Package fought back and couldn't be installed");
        }
    } else if aur_package.is_some() {
        let aur_package = aur_package.unwrap();

        show_success_message("🔍 Found AUR package");

        if install_aur_package(&aur_package, confirm).await.is_ok() {
            show_success_message("📦 The package was successfully installed");
        } else {
            show_error_message("😮 Package fought back and couldn't be installed");
        };
    } else {
        show_error_message(
            "🐇 The package went missing. Check if you typed the right package name.",
        );
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

    let command = Command::new("sudo").args(args).spawn()?.wait()?;

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
