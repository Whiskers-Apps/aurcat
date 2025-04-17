use clap::Parser;
use clean::on_clean;
use cli::Cli;
use install::on_install;
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
            package,
            skip_confirm,
        } => on_install(&package.to_lowercase(), skip_confirm).await,

        cli::Commands::Search {
            query,
            skip_prompt,
            max_results,
        } => on_search(&query, skip_prompt, max_results).await,

        cli::Commands::Uninstall {
            package,
            skip_confirm,
        } => on_uninstall_package(&package.to_lowercase(), skip_confirm),

        cli::Commands::Clean {
            pacman_cache,
            pacman_lock_file,
        } => on_clean(pacman_cache, pacman_lock_file),

        cli::Commands::Update { skip_aur } => on_update(skip_aur),
    }
}
