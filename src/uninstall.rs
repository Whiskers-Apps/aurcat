use std::error::Error;

use crate::{list::get_aur_packages, utils::run};

pub fn on_uninstall_command(packages: Vec<String>, confirm: bool) -> Result<(), Box<dyn Error>> {
    uninstall_packages(&packages, confirm)?;
    Ok(())
}

pub fn uninstall_packages<S: AsRef<str>>(
    packages: &[S],
    confirm: bool,
) -> Result<(), Box<dyn Error>> {
    let mut command = vec!["sudo".to_string(), "pacman".to_string(), "-R".to_string()];
    let aur_packages = get_aur_packages(false)?;

    for package in packages {
        let package = package.as_ref().to_string();
        let debug_package = format!("{package}-debug");

        if aur_packages.iter().any(|ap| &ap.package == &debug_package) {
            command.push(debug_package);
        }

        command.push(package);
    }

    if !confirm {
        command.push("--noconfirm".to_string());
    }

    run(&command)?;

    Ok(())
}
