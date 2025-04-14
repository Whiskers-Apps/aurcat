use std::process::{Command, exit};

use colored::Colorize;
use inquire::{CustomType, InputAction, Text, validator::Validation};
use prettytable::{Table, format::FormatBuilder, row};

use crate::{
    settings::get_settings,
    utils::{run_elevated_command, show_status_message},
};

#[derive(Debug, Clone)]
pub struct PacmanPackage {
    index: usize,
    repository: String,
    package: String,
    version: String,
    description: String,
    installed: bool,
}

pub fn pacman_install(package: &str) {
    let command = if get_settings().confirm_install {
        vec!["pacman", "-S", package]
    } else {
        vec!["pacman", "-S", package, "--noconfirm"]
    };

    run_elevated_command(
        command,
        &format!("✅ Package '{package}' insalled successfully "),
        &format!("⚠️ Package '{package}' could not be installed"),
    );
}

pub fn pacman_search(package: &str) {
    let command = Command::new("pacman")
        .args(["-Ss", package])
        .output()
        .expect("Error running pacman search command");

    let output = String::from_utf8_lossy(&command.stdout);

    if output.trim().is_empty() {
        let message = format!("Could not find any package matching '{}'", &package).yellow();
        println!("{message}");
        exit(0)
    }

    let output_split: Vec<String> = output
        .split("\n")
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();

    let packages_split: Vec<Vec<String>> = output_split
        .chunks(2)
        .map(|a| a.to_owned())
        .collect::<Vec<Vec<String>>>();

    let mut packages = Vec::<PacmanPackage>::new();

    for (index, package_split) in packages_split.iter().enumerate() {
        let info = package_split.get(0).unwrap_or(&"".to_string()).to_owned();
        let info_split = info
            .split_whitespace()
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let description = package_split
            .get(1)
            .unwrap_or(&"".to_string())
            .to_owned()
            .trim_start()
            .to_owned();

        if info_split.len() < 2 {
            continue;
        }

        let repo_package = info_split
            .get(0)
            .unwrap()
            .split("/")
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        if repo_package.len() != 2 {
            continue;
        }

        let repository = repo_package.get(0).unwrap().to_owned();
        let package = repo_package.get(1).unwrap().to_owned();
        let version = info_split.get(1).unwrap().to_owned();

        let installed = if let Some(installed_str) = info_split.get(2) {
            installed_str == "[installed]"
        } else {
            false
        };

        packages.push(PacmanPackage {
            index: index + 1,
            repository,
            package,
            version,
            description,
            installed,
        });
    }

    let mut table = Table::new();
    table.set_titles(row!["Index", "Repository", "Package", "Description"]);
    let format = FormatBuilder::new().borders(' ').padding(1, 1).build();

    table.set_format(format);

    packages.reverse();

    // TODO: Fazer uma setting para a quantidade a ser mostrada

    let limited_packages: Vec<PacmanPackage> = if packages.len() >= 100 {
        packages[packages.len() - 100..packages.len()].to_vec()
    } else {
        packages.to_vec()
    };

    for package in &limited_packages {
        table.add_row(row![
            package.index.to_string().blue().to_string(),
            package.repository,
            if package.installed {
                package.package.green().to_string()
            } else {
                package.package.to_string()
            },
            package.description
        ]);
    }

    if !table.print_tty(true).is_ok() {
        table.printstd();
    };

    let packages_found = format!("Found {} packages", &packages.len().to_string().blue());

    println!("\n\n{packages_found}");

    let package_count = limited_packages.len();

    let prompt_message = format!(
        "Select a package you would like to install [0..{}]. Use 0 to do leave.\n>",
        &package_count
    );

    let error_message = format!("The index must be between 0..{}", package_count);

    let package_index = CustomType::<usize>::new(&prompt_message)
        .with_validator(move |input: &usize| {
            if *input <= package_count {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(error_message.clone().into()))
            }
        })
        .prompt();

    if let Ok(package_index) = package_index {
        if package_index == 0 {
            exit(0);
        }

        let package = limited_packages
            .iter()
            .find(|a| a.index == package_index)
            .unwrap();

        pacman_install(&package.package);
    }
}

pub fn pacman_uninstall(package: &str) {
    let command = if get_settings().confirm_uninstall {
        vec!["pacman", "-R", package]
    } else {
        vec!["pacman", "-R", package, "--noconfirm"]
    };

    run_elevated_command(
        command,
        &format!("🧹 Package '{package}' uninstalled successfully"),
        &format!("⚠️ Package '{package}' could not be uninstalled"),
    );
}

pub fn clear_packman_cache() {
    let status = Command::new("sudo")
        .arg("pacman")
        .arg("")
        .spawn()
        .expect("Error using command")
        .wait()
        .expect("Error waiting for command");

    show_status_message(
        status,
        "Lock File removed successfully",
        "Error removing lock file",
    );
}

pub fn remove_lock_file() {
    let status = Command::new("sudo")
        .arg("rm")
        .arg("-f")
        .arg("/var/lib/pacman/db.lck")
        .spawn()
        .expect("Error using command")
        .wait()
        .expect("Error waiting for command");

    show_status_message(
        status,
        "🧹  Lock File removed successfully",
        "⚠️  Error removing lock file",
    );
}

pub fn pacman_update() {
    let command = if get_settings().confirm_install {
        vec!["pacman", "-Syyu"]
    } else {
        vec!["pacman", "-Syyu", "--noconfirm"]
    };

    run_elevated_command(
        command,
        "✅ System updated successfully",
        "⚠️ Error while updating system",
    );
}
