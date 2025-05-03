use std::{error::Error, process::Command};

use colored::Colorize;
use sniffer_rs::sniffer::Sniffer;

#[derive(Debug, Clone, PartialEq)]
pub struct InstalledPackage {
    pub package: String,
    pub version: String,
}

pub fn on_list(filter: Option<String>) {
    let installed_pkgs = if let Some(filter) = &filter {
        let sniffer = Sniffer::new()
            .set_do_jaro_winkler_match(false)
            .set_do_hamming_match(false)
            .set_do_levenshtein_match(false);

        get_installed_packages()
            .unwrap_or(vec![])
            .into_iter()
            .filter(|p| sniffer.matches(&p.package, filter))
            .collect::<Vec<InstalledPackage>>()
    } else {
        get_installed_packages().unwrap_or(vec![])
    };

    let aur_pkgs = if let Some(filter) = &filter {
        let sniffer = Sniffer::new()
            .set_do_jaro_winkler_match(false)
            .set_do_hamming_match(false)
            .set_do_levenshtein_match(false);

        get_installed_aur_packages(true)
            .unwrap_or(vec![])
            .into_iter()
            .filter(|p| sniffer.matches(&p.package, filter))
            .collect::<Vec<InstalledPackage>>()
    } else {
        get_installed_aur_packages(true).unwrap_or(vec![])
    };

    for pkg in installed_pkgs {
        println!("📦 {} {}", pkg.package.bold(), pkg.version);
    }

    for pkg in aur_pkgs {
        println!("🌍 {} {}", pkg.package.bold(), pkg.version);
    }
}

pub fn get_installed_packages() -> Result<Vec<InstalledPackage>, Box<dyn Error>> {
    let command = Command::new("pacman").arg("-Q").output()?;

    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);

        let output_split = output
            .split("\n")
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let mut packages = Vec::<InstalledPackage>::new();

        for package_line in output_split {
            let split = package_line
                .split_whitespace()
                .map(|a| a.to_owned())
                .collect::<Vec<String>>();

            if split.len() == 2 {
                packages.push(InstalledPackage {
                    package: split.get(0).unwrap().to_owned(),
                    version: split.get(1).unwrap().to_owned(),
                });
            }
        }

        let aur_pkgs = get_installed_aur_packages(false)?;

        let filtered_pkgs = packages
            .into_iter()
            .filter(|p| !aur_pkgs.contains(p))
            .collect::<Vec<InstalledPackage>>();

        return Ok(filtered_pkgs);
    }

    return Err("Error getting packages".into());
}

pub fn get_installed_aur_packages(
    filter_debug: bool,
) -> Result<Vec<InstalledPackage>, Box<dyn Error>> {
    let command = Command::new("pacman").arg("-Qm").output()?;

    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);

        let output_split = output
            .split("\n")
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let mut packages = Vec::<InstalledPackage>::new();

        for package_line in output_split {
            let split = package_line
                .split_whitespace()
                .map(|a| a.to_owned())
                .collect::<Vec<String>>();

            if split.len() == 2 {
                let package = split.get(0).unwrap().to_lowercase().to_owned();
                let version = split.get(1).unwrap().to_owned();

                if filter_debug && package.ends_with("-debug") {
                    continue;
                }

                packages.push(InstalledPackage { package, version });
            }
        }

        return Ok(packages);
    }

    return Ok(vec![]);
}
