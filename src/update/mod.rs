use std::{error::Error, process::Command};

use crate::{
    settings::get_settings,
    utils::{show_error_message, show_success_message},
};

pub fn on_update(skip: Option<bool>) {
    let skip_aur = if let Some(skip) = skip {
        skip
    } else {
        get_settings().skip_aur_update
    };

    let packages_update_success = update_packages().is_ok();

    let aur_update_success = if !skip_aur {
        update_aur_packages().is_ok()
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

fn update_aur_packages() -> Result<(), Box<dyn Error>> {
    Ok(())
}
