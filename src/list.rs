use std::{error::Error, process::Command};

use sniffer_rs::sniffer::Sniffer;

use crate::utils::run_hidden;

#[derive(Debug)]
pub struct PackageQuery {
    pub package: String,
    pub version: String,
    pub aur: bool,
}

pub fn on_list_command(aur: bool, filter: Option<String>) -> Result<(), Box<dyn Error>> {
    let sniffer = Sniffer::new()
        .set_do_levenshtein_match(false)
        .set_do_hamming_match(false)
        .set_do_jaro_winkler_match(false);

    let mut packages: Vec<PackageQuery> = if aur {
        get_aur_packages()?
    } else {
        get_all_packages()?
    };

    if let Some(filter) = filter {
        packages = packages
            .into_iter()
            .filter(|query| sniffer.matches(&query.package, &filter))
            .collect();
    }

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
    let packages_output = run_hidden(&["pacman", "-Qn"])?;

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

pub fn get_all_packages() -> Result<Vec<PackageQuery>, Box<dyn Error>> {
    let mut pkgs = get_repo_packages()?;
    let mut aur_pkgs = get_aur_packages()?;
    pkgs.append(&mut aur_pkgs);

    return Ok(pkgs);
}
