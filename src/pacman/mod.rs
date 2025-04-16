use std::process::{Command, exit};

use colored::Colorize;
use inquire::{CustomType, validator::Validation};
use prettytable::Table;

use crate::{
    settings::get_settings,
    utils::{get_seperator, run_elevated_command, show_status_message},
};

#[derive(Debug, Clone)]
pub struct RepositoryPackage {
    pub repository: String,
    pub package: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
}

#[derive(Debug, Clone)]
pub struct InstalledRepoPackage {
    pub package_name: String,
    pub version: String,
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

pub fn on_search_repos(package: &str) -> Vec<RepositoryPackage> {
    // TODO: Fazer uma setting para a quantidade a ser mostrada
    let packages = search_repository_packages(package);

    if packages.len() == 0 {
        let message = format!("🔍 Couldn't find any repo package for '{package}'").yellow();
        println!("{message}");
        return vec![];
    }

    let mut section = get_seperator();
    section += "\nRepositories\n";
    section += &get_seperator();
    section = section.purple().to_string();

    println!("{section}");

    let limited_packages: Vec<RepositoryPackage> = if packages.len() >= 100 {
        packages[packages.len() - 100..packages.len()].to_vec()
    } else {
        packages.to_vec()
    };

    limited_packages.display(0);

    return limited_packages;
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

pub fn search_repository_packages(package: &str) -> Vec<RepositoryPackage> {
    let command = Command::new("pacman")
        .args(["-Ss", package])
        .output()
        .expect("Error running pacman search command");

    let output = String::from_utf8_lossy(&command.stdout);

    if output.trim().is_empty() {
        return vec![];
    }

    let output_split: Vec<String> = output
        .split("\n")
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();

    let packages_split: Vec<Vec<String>> = output_split
        .chunks(2)
        .map(|a| a.to_owned())
        .collect::<Vec<Vec<String>>>();

    let installed_packages = get_installed_repo_packages();
    let mut packages = Vec::<RepositoryPackage>::new();

    for package_split in packages_split {
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
        let installed = installed_packages.iter().any(|a| a.package_name == package);

        packages.push(RepositoryPackage {
            repository,
            package,
            version,
            description,
            installed,
        });
    }

    packages.reverse();

    return packages;
}

pub fn get_installed_repo_packages() -> Vec<InstalledRepoPackage> {
    let command = Command::new("pacman")
        .arg("-Q")
        .output()
        .expect("Error running search command");

    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);

        let output_split = output
            .split("\n")
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let mut packages = Vec::<InstalledRepoPackage>::new();

        for package_line in output_split {
            let split = package_line
                .split_whitespace()
                .map(|a| a.to_owned())
                .collect::<Vec<String>>();

            if split.len() == 2 {
                packages.push(InstalledRepoPackage {
                    package_name: split.get(0).unwrap().to_owned(),
                    version: split.get(1).unwrap().to_owned(),
                });
            }
        }

        return packages;
    }

    return vec![];
}

trait DisplayPackage {
    fn display(&self, base_index: usize);
}

impl DisplayPackage for Vec<RepositoryPackage> {
    fn display(&self, base_index: usize) {
        let seperator = get_seperator();

        for (index, package) in self.iter().enumerate() {
            println!("- Index: {}", base_index + index + 1);

            println!("- Repository: {}", package.repository);

            println!(
                "- Name: {}",
                if package.installed {
                    package.package.bold().green().to_string()
                } else {
                    package.package.bold().to_string()
                }
            );

            println!("- Version: {}", package.version);

            println!("- Description: {}", package.description);

            println!("{seperator}");
        }
    }
}
