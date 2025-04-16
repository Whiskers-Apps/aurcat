use std::{
    fs::{self, File, create_dir_all},
    io::Write,
    process::Command,
};

use colored::Colorize;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    paths::{get_cache_dir, get_package_cache_dir},
    utils::get_seperator,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AurSearchResponse {
    resultcount: usize,
    results: Vec<AurResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AurResult {
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "OutOfDate")]
    out_of_date: Option<usize>,
    #[serde(rename = "URLPath")]
    url_path: String,
    #[serde(rename = "Version")]
    version: String,
}

#[derive(Debug, Clone)]
pub struct AurPackage {
    pub installed: bool,
    pub package_name: String,
    pub description: String,
    pub version: String,
    pub url_path: String,
    pub outdated: bool,
}

#[derive(Debug, Clone)]
pub struct InstalledAurPackage {
    pub package_name: String,
    pub version: String,
}

pub async fn on_search_aur(package: &str, repos_package_count: usize) -> Vec<AurPackage> {
    let packages = search_aur_packages(&package).await;

    if packages.len() > 0 {
        let mut section = get_seperator();
        section += "\nAUR\n";
        section += &get_seperator();
        section = section.blue().to_string();

        println!("{section}");
    } else {
        let message = format!("🔍 Couldn't find any AUR package for '{package}'").yellow();
        println!("{message}");

        return vec![];
    }

    let limited_packages: Vec<AurPackage> = if packages.len() >= 100 {
        packages[packages.len() - 100..packages.len()].to_vec()
    } else {
        packages.to_vec()
    };

    limited_packages.display(repos_package_count);

    return packages;
}

pub fn get_installed_aur_packages() -> Vec<InstalledAurPackage> {
    let command = Command::new("pacman")
        .arg("-Qm")
        .output()
        .expect("Error running pacman command");

    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);

        let output_split = output
            .split("\n")
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let mut packages = Vec::<InstalledAurPackage>::new();

        for package_line in output_split {
            let split = package_line
                .split_whitespace()
                .map(|a| a.to_owned())
                .collect::<Vec<String>>();

            if split.len() == 2 {
                packages.push(InstalledAurPackage {
                    package_name: split.get(0).unwrap().to_owned(),
                    version: split.get(1).unwrap().to_owned(),
                });
            }
        }

        return packages;
    }

    return vec![];
}

async fn search_aur_packages(package: &str) -> Vec<AurPackage> {
    let url = format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={package}");
    let client = Client::new();

    let response = client.get(&url).send().await.expect("Error making request");
    let parsed_response: AurSearchResponse = response.json().await.expect("Error parsing json");

    let installed_packages = get_installed_aur_packages();

    let mut packages = parsed_response
        .results
        .iter()
        .map(|result| AurPackage {
            installed: installed_packages
                .iter()
                .any(|a| a.package_name == result.name),
            package_name: result.name.to_owned(),
            description: if let Some(description) = result.description.to_owned() {
                description.to_string()
            } else {
                "".to_string()
            },
            version: result.version.to_owned(),
            outdated: result.out_of_date.is_some(),
            url_path: result.url_path.to_owned(),
        })
        .collect::<Vec<AurPackage>>();

    packages.reverse();

    return packages;
}

trait DisplayAur {
    fn display(&self, base_index: usize);
}

impl DisplayAur for Vec<AurPackage> {
    fn display(&self, base_index: usize) {
        let seperator = get_seperator();

        for (index, package) in self.iter().enumerate() {
            println!("- Index: {}", base_index + index + 1);

            println!(
                "- Name: {}",
                if package.installed {
                    package.package_name.bold().green().to_string()
                } else if package.outdated {
                    package.package_name.bold().red().to_string()
                } else {
                    package.package_name.bold().to_string()
                }
            );

            println!("- Version: {}", package.version);

            println!("- Description: {}", package.description);

            if index < self.len() - 1 {
                println!("{seperator}");
            }
        }
    }
}

pub async fn install_aur_package(package: &AurPackage) {
    let url = format!("https://aur.archlinux.org{}", package.url_path);

    let cache_dir = get_cache_dir();

    if !cache_dir.exists() {
        create_dir_all(&cache_dir).expect("Error creating cache dir");
    }

    let package_dir = get_package_cache_dir(&package.package_name);

    if !package_dir.exists() {
        create_dir_all(&package_dir).expect("Error creating package cache dir");
    }

    let version_dir = package_dir.join(&package.version);

    if !version_dir.exists() {
        create_dir_all(&version_dir).expect("Error creating package version dir")
    }

    let package_path = version_dir.join(format!("{}.tar.gz", &package.version));

    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Error downloading package");

    let bytes = response.bytes().await.expect("Error getting bytes");
    fs::write(&package_path, &bytes).expect("Error writing package");

    let command = Command::new("tar")
        .args([
            "-xzf",
            &package_path.into_os_string().into_string().unwrap(),
            "--strip-components=1",
        ])
        .current_dir(&version_dir)
        .output()
        .unwrap();

    if !command.status.success() {
        println!("{:?}", String::from_utf8(command.stderr).unwrap());
        return;
    }

    let command = Command::new("makepkg")
        .args(["-si", "--noconfirm", "--needed", "--syncdeps"])
        .current_dir(&version_dir)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if command.success() {
        println!("Package installed successfully");
    }
}
