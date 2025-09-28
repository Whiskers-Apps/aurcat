use std::{error::Error, process::Command};

use crate::utils::run_hidden;

#[derive(Debug)]
pub struct PackageQuery {
    pub package: String,
    pub version: String,
    pub aur: bool,
}

pub fn on_list_command(aur: bool) -> Result<(), Box<dyn Error>> {
    let packages = if aur {
        get_aur_packages()?
    } else {
        let mut pkgs = get_repo_packages()?;
        let mut aur_pkgs = get_aur_packages()?;
        pkgs.append(&mut aur_pkgs);
        pkgs
    };

    for query in packages {
        println!(
            "{} {} {}",
            if query.aur { "🌍" } else { "📦" },
            &query.package,
            &query.version
        )
    }

    return Ok(());
}

pub fn get_repo_packages() -> Result<Vec<PackageQuery>, Box<dyn Error>> {
    let packages_output = run_hidden(&["pacman", "-Q"])?;

    let packages: Vec<PackageQuery> = packages_output
        .lines()
        .into_iter()
        .filter_map(|query| {
            let mut splitted = query.split_whitespace();

            let package = splitted.next()?.to_string();
            let version = splitted.next()?.to_string();

            Some(PackageQuery {
                package,
                version,
                aur: false,
            })
        })
        .collect();

    return Ok(packages);
}

pub fn get_aur_packages() -> Result<Vec<PackageQuery>, Box<dyn Error>> {
    let packages_output = run_hidden(&["pacman", "-Qm"])?;

    let packages: Vec<PackageQuery> = packages_output
        .lines()
        .into_iter()
        .filter_map(|query| {
            let mut splitted = query.split_whitespace();

            let package = splitted.next()?.to_string();
            let version = splitted.next()?.to_string();

            Some(PackageQuery {
                package,
                version,
                aur: true,
            })
        })
        .collect();

    return Ok(packages);
}
