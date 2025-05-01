use colored::Colorize;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    process::{Command, exit},
};

use crate::{
    install::{install_aur_package, install_package},
    list::{get_installed_aur_packages, get_installed_packages},
    settings::get_settings,
    utils::{get_number_in_range, show_error_message, show_success_message},
};

#[derive(Debug, Clone)]
pub struct Package {
    pub repository: String,
    pub package: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
}

#[derive(Debug, Clone)]
pub struct AurPackage {
    pub installed: bool,
    pub package: String,
    pub description: String,
    pub version: String,
    pub url_path: String,
    pub outdated: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AurSearchResponse {
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

impl Package {
    pub fn display(&self, index: Option<usize>) {
        if let Some(index) = index {
            println!("Number: {}", index);
        }

        println!(
            "Package: {}",
            if self.installed {
                self.package.bold().green().to_string()
            } else {
                self.package.bold().to_string()
            }
        );

        println!("Version {}", self.version);

        println!("Description {}\n\n", self.description);
    }
}

impl AurPackage {
    pub fn display(&self, index: Option<usize>) {
        if let Some(index) = index {
            println!("Number: {}", index);
        }

        println!(
            "Package: {}",
            if self.installed {
                self.package.bold().green().to_string()
            } else if self.outdated {
                self.package.bold().red().to_string()
            } else {
                self.package.bold().to_string()
            }
        );

        println!("Version {}", self.version);

        println!("Description {}\n\n", self.description);
    }
}

/// Handle CLI search command
pub async fn on_search(query: &str, skip: Option<bool>, max: Option<usize>) {
    let mut packages = get_package_search(query).unwrap_or(vec![]);
    let mut aur_packages = get_aur_package_search(query).await.unwrap_or(vec![]);
    let skip_prompt = if let Some(skip) = skip {
        skip
    } else {
        get_settings().skip_search_prompt
    };

    let package_limit = if let Some(max) = max {
        max
    } else {
        get_settings().max_search_results
    };

    if packages.len() > package_limit {
        packages = packages[..package_limit].into();
    }

    if aur_packages.len() > package_limit {
        aur_packages = aur_packages[..package_limit].into();
    }

    if !packages.is_empty() {
        println!(
            "{}",
            r#"
        ____  _____ ____   ___  ____ ___ _____ ___  ____  ___ _____ ____
       |  _ \| ____|  _ \ / _ \/ ___|_ _|_   _/ _ \|  _ \|_ _| ____/ ___|
       | |_) |  _| | |_) | | | \___ \| |  | || | | | |_) || ||  _| \___ \
       |  _ <| |___|  __/| |_| |___) | |  | || |_| |  _ < | || |___ ___) |
       |_| \_\_____|_|    \___/|____/___| |_| \___/|_| \_\___|_____|____/
                                                                          "#
            .green()
            .bold()
        );
    }

    for (index, package) in packages.iter().enumerate() {
        package.display(if skip_prompt { None } else { Some(index + 1) });
    }

    println!(
        "{}",
        r#"
             _   _   _ ____
            / \ | | | |  _ \
           / _ \| | | | |_) |
          / ___ \ |_| |  _ <
         /_/   \_\___/|_| \_\

"#
        .blue()
        .bold()
    );

    for (index, package) in aur_packages.iter().enumerate() {
        package.display(if skip_prompt {
            None
        } else {
            Some(index + packages.len() + 1)
        });
    }

    if !skip_prompt {
        let number = get_number_in_range(
            "Type the number of the package to install: ",
            packages.len() + aur_packages.len(),
        );

        if number == 0 {
            exit(0);
        } else if number <= packages.len() {
            if install_package(&packages.get(number - 1).unwrap().package, None).is_ok() {
                show_success_message("📦 The package was successfully installed");
            } else {
                show_error_message("😮 Package fought back and couldn't be installed");
            };
        } else {
            if install_aur_package(&aur_packages.get(number - 1).unwrap(), None)
                .await
                .is_ok()
            {
                show_success_message("📦 The package was successfully installed");
            } else {
                show_error_message("😮 Package fought back and couldn't be installed");
            };
        }
    }
}

/// Searches for packages in the official repoistories matching the search query
pub fn get_package_search(query: &str) -> Result<Vec<Package>, Box<dyn Error>> {
    let command = Command::new("pacman").args(["-Ss", query]).output()?;
    let output = String::from_utf8_lossy(&command.stdout);

    if output.trim().is_empty() {
        return Ok(vec![]);
    }

    let output_split: Vec<String> = output
        .split("\n")
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();

    let packages_split: Vec<Vec<String>> = output_split
        .chunks(2)
        .map(|a| a.to_owned())
        .collect::<Vec<Vec<String>>>();

    let installed_packages = get_installed_packages().unwrap_or(vec![]);
    let mut packages = Vec::<Package>::new();

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
        let installed = installed_packages.iter().any(|a| a.package == package);

        packages.push(Package {
            repository,
            package,
            version,
            description,
            installed,
        });
    }

    Ok(packages)
}

/// Searches for AUR packages matching the search query
pub async fn get_aur_package_search(query: &str) -> Result<Vec<AurPackage>, Box<dyn Error>> {
    let url = format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={query}");
    let client = Client::new();

    let response = client.get(&url).send().await?;
    let parsed_response: AurSearchResponse = response.json().await?;

    let installed_packages = get_installed_aur_packages(true)?;

    let packages = parsed_response
        .results
        .iter()
        .map(|result| AurPackage {
            installed: installed_packages.iter().any(|a| a.package == result.name),
            package: result.name.to_owned(),
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

    Ok(packages)
}
