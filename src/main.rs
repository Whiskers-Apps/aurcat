use std::process::exit;

use aur::{install_aur_package, on_search_aur};
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use pacman::{on_search_repos, pacman_install, pacman_uninstall, pacman_update, remove_lock_file};
use settings::get_settings;
use utils::get_index;

pub mod aur;
pub mod cli;
pub mod pacman;
pub mod paths;
pub mod settings;
pub mod utils;

#[tokio::main]
async fn main() {
    match Cli::parse().command {
        cli::Commands::Install { package, aur } => match aur {
            true => {}
            false => {
                pacman_install(&package);
            }
        },
        cli::Commands::Search { package } => {
            let repo_packages = on_search_repos(&package);
            let aur_packages = on_search_aur(&package, repo_packages.len()).await;

            let message = format!(
                "Found {} packages",
                repo_packages.len() + aur_packages.len()
            )
            .bold()
            .blue();

            println!("{message}");

            let index = get_index(
                "Select the package index you would like to install.\n> ",
                repo_packages.len() + aur_packages.len(),
            );

            if index == 0 {
                exit(0);
            } else if index <= repo_packages.len() {
                pacman_install(&package);
            } else {
                install_aur_package(&aur_packages.get(repo_packages.len() + index - 1).unwrap())
                    .await;
            }
        }
        cli::Commands::Uninstall { package } => pacman_uninstall(&package),
        cli::Commands::Clean {
            pacman_cache,
            pacman_lock_file,
        } => {
            //if pacman_cache {}

            if pacman_lock_file {
                remove_lock_file();
            }
        }
        cli::Commands::Settings {} => {
            get_settings().show_settings();
        }
        cli::Commands::Update { aur } => {
            pacman_update();

            if aur {}
        }
    }
}
