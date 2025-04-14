use aur::aur_search;
use clap::Parser;
use cli::Cli;
use pacman::{pacman_install, pacman_search, pacman_uninstall, pacman_update, remove_lock_file};
use settings::get_settings;

pub mod aur;
pub mod cli;
pub mod pacman;
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
            pacman_search(&package);
            aur_search(&package);
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
