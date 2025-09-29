use std::error::Error;

use colored::Colorize;

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

pub fn on_search_command(package: String) -> Result<(), Box<dyn Error>> {
    let max_results = get_config()?.max_results;

    let repo_packages = search_repo_packages(&package)?;
    let repo_packages_len = repo_packages.len();

    let cut_repo_packages = repo_packages[0..(if repo_packages_len >= max_results {
        max_results
    } else {
        repo_packages.len()
    })]
        .to_vec();

    if repo_packages_len > 0 {
        println!("📦 Repositories Packages\n");
    }

    for query in cut_repo_packages {
        println!(
            "{} {} {}\n{}\n",
            &query.repo.green().bold(),
            &query.package.bold(),
            &query.version.bold(),
            &query.package
        );
    }

    if repo_packages_len == 0 {
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
            description: description.to_owned(),
        });
    }

    Ok(search_queries)
}
