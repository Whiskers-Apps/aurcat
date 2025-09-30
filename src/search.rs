use std::error::Error;

use colored::Colorize;
use convert_case::{Case, Casing};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    config::get_config,
    utils::{run_hidden, show_message},
};

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub repo: String,
    pub package: String,
    pub version: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AurSearchReponse {
    pub results: Vec<AurSearchResult>,
}

#[derive(Debug, Clone)]
pub struct AurSearchQuery {
    pub package: String,
    pub version: String,
    pub last_modified: usize,
    pub description: String,
    pub url_path: String,
    pub out_of_date: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AurSearchResult {
    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "LastModified")]
    pub last_modified: usize,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<usize>,

    #[serde(rename = "URLPath")]
    pub url_path: String,

    #[serde(rename = "Version")]
    pub version: String,
}

pub async fn on_search_command(package: String) -> Result<(), Box<dyn Error>> {
    let max_results = get_config()?.max_results;

    let repo_packages = search_repo_packages(&package)?;
    let repo_packages_len = repo_packages.len();

    let aur_packages = search_aur_packages(&package).await?;
    let aur_packages_len = aur_packages.len();

    let cut_repo_packages = repo_packages[0..(if repo_packages_len >= max_results {
        max_results
    } else {
        repo_packages_len
    })]
        .to_vec();

    let cut_aur_packages = aur_packages[0..(if aur_packages_len >= max_results {
        max_results
    } else {
        aur_packages_len
    })]
        .to_vec();

    if repo_packages_len > 0 {
        println!("📦 Repositories Packages\n");
    }

    for query in cut_repo_packages {
        println!(
            "{} {} {}\n{}\n",
            &query.repo.to_case(Case::Upper).green().bold(),
            &query.package.bold(),
            &query.version.bold(),
            &query.description
        );
    }

    if aur_packages_len > 0 {
        println!("🌍 AUR Packages\n");
    }

    for query in cut_aur_packages {
        println!(
            "{} {} {}\n{}\n",
            "AUR".blue().bold(),
            if query.out_of_date {
                &query.package.red().bold()
            } else {
                &query.package.bold()
            },
            query.version.bold(),
            &query.description
        );
    }

    if repo_packages_len == 0 && aur_packages_len == 0 {
        show_message("Package not Found");
    }

    Ok(())
}

pub fn search_repo_packages(package: &str) -> Result<Vec<SearchQuery>, Box<dyn Error>> {
    let output = run_hidden(&["pacman", "-Ss", package])?;
    let mut search_queries: Vec<SearchQuery> = vec![];

    let output_split: Vec<String> = output.split("\n").map(|s| s.to_string()).collect();

    for chunk in output_split.chunks_exact(2) {
        let info = chunk
            .get(0)
            .ok_or_else(|| "Failed to get info".to_string())?;

        let description = chunk.get(1).ok_or_else(|| "Failed to get description")?;

        let info_parts: Vec<String> = info
            .split_whitespace()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let repo_package_string = info_parts
            .get(0)
            .ok_or_else(|| "Failed to get repo package split".to_string())?;

        let repo_package_parts: Vec<String> = repo_package_string
            .split('/')
            .map(|s| s.to_string())
            .collect();

        let repo = repo_package_parts
            .get(0)
            .ok_or_else(|| "Failed to get repo".to_string())?;

        let package_name = repo_package_parts
            .get(1)
            .ok_or_else(|| "Failed to get package".to_string())?;

        let version = info_parts
            .get(1)
            .ok_or_else(|| "Failed to get version".to_string())?;

        search_queries.push(SearchQuery {
            repo: repo.to_owned(),
            package: package_name.to_owned(),
            version: version.to_owned(),
            description: description.trim_start().to_owned(),
        });
    }

    Ok(search_queries)
}

pub async fn search_aur_packages(package: &str) -> Result<Vec<AurSearchQuery>, Box<dyn Error>> {
    let url = format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={package}");

    let response_json = reqwest::get(url).await?.text().await?;
    let response: AurSearchReponse = serde_json::from_str(&response_json)?;

    let packages: Vec<AurSearchQuery> = response
        .results
        .into_iter()
        .map(|result| AurSearchQuery {
            package: result.name,
            version: result.version,
            last_modified: result.last_modified,
            description: if let Some(description) = result.description {
                description
            } else {
                "Missing Description".to_string()
            },
            url_path: result.url_path,
            out_of_date: result.out_of_date.is_some(),
        })
        .collect();

    Ok(packages)
}
