use clap::Parser;
use clean::on_clean;
use cli::Cli;
use install::on_install;
use list::on_list;
use search::on_search;
use uninstall::on_uninstall_package;
use update::on_update;

pub mod clean;
pub mod cli;
pub mod install;
pub mod list;
pub mod pacman;
pub mod paths;
pub mod search;
pub mod settings;
pub mod uninstall;
pub mod update;
pub mod utils;

#[tokio::main]
async fn main() {
    match Cli::parse().command {
        cli::Commands::Install {
            packages,
            skip_confirm,
        } => on_install(&packages, skip_confirm).await,
        cli::Commands::Search {
            query,
            skip_prompt,
            max_results,
        } => on_search(&query, skip_prompt, max_results).await,
        cli::Commands::Uninstall {
            packages,
            skip_confirm,
        } => on_uninstall_package(&packages, skip_confirm),
        cli::Commands::Clean {
            pacman_cache,
            pacman_lock_file,
        } => on_clean(pacman_cache, pacman_lock_file),
        cli::Commands::Update { skip_aur } => on_update(skip_aur).await,
        cli::Commands::List { filter } => on_list(filter),
    }
}
