use std::{
    error::Error,
    process::{Command, exit},
};

use crate::{
    list::{get_installed_aur_packages, get_installed_packages},
    settings::get_settings,
    utils::{show_error_message, show_success_message},
};

/// Handles the uninstall command from CLI
pub fn on_uninstall_package(package: &str, confirm: Option<bool>) {
    let installed_packages = get_installed_packages().unwrap_or(vec![]);
    let installed_aur_packages = get_installed_aur_packages(false).unwrap_or(vec![]);

    if installed_aur_packages.iter().any(|p| p.package == package) {
        if uninstall_aur_package(package, confirm).is_ok() {
            show_success_message("🌌 Package sent to the void");
        } else {
            show_error_message("😮 Package fought back and couldn't be uninstalled");
        };
    } else if installed_packages.iter().any(|p| p.package == package) {
        if uninstall_package(package, confirm).is_ok() {
            show_success_message("🌌 Package sent to the void");
        } else {
            show_error_message("😮 Package fought back and couldn't be uninstalled");
        };
    } else {
        show_error_message(
            "🐇 The package went missing. Check if you typed the right package name.",
        );
        exit(1);
    }
}

/// Uninstalls an official package using pacman
pub fn uninstall_package(package: &str, confirm: Option<bool>) -> Result<(), Box<dyn Error>> {
    let skip_confirm = if let Some(confirm) = confirm {
        confirm
    } else {
        get_settings().skip_uninstall_confirm
    };

    let args = if skip_confirm {
        vec!["pacman", "-R", package, "--noconfirm"]
    } else {
        vec!["pacman", "-R", package]
    };

    let command = Command::new("sudo").args(args).spawn()?.wait()?;

    return if command.success() {
        Ok(())
    } else {
        Err(format!("Error uninstalling {}", &package).into())
    };
}

/// Uninstall a package that was previously installed using the AUR
pub fn uninstall_aur_package(package: &str, confirm: Option<bool>) -> Result<(), Box<dyn Error>> {
    let debug_package = format!("{package}-debug");
    let debug_package_exists = get_installed_aur_packages(false)
        .unwrap_or(vec![])
        .iter()
        .any(|p| p.package == debug_package);

    let mut args = if debug_package_exists {
        vec!["pacman", "-Rns", package, &debug_package]
    } else {
        vec!["pacman", "-Rns", package]
    };

    let skip_confirm = if let Some(confirm) = confirm {
        confirm
    } else {
        get_settings().skip_uninstall_confirm
    };

    if skip_confirm {
        args.push("--noconfirm");
    }

    let command = Command::new("sudo").args(args).spawn()?.wait()?;

    return if command.success() {
        Ok(())
    } else {
        Err(format!("Error uninstalling {}", &package).into())
    };
}
