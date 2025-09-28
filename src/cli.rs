use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<MainCommand>,
}

#[derive(Subcommand, Debug)]
pub enum MainCommand {
    #[command(about = "Install a package")]
    Install {
        #[arg(long, group = "search_fallback", action = ArgAction::SetTrue, help = "Skip search if the package is not found")]
        skip_search: bool,

        #[arg(long, group = "search_fallback", action = ArgAction::SetTrue, help = "Search if the package is not found")]
        search: bool,

        #[arg(long, group = "confirm_install", action = ArgAction::SetTrue, help = "Skip the prompt to confirm package installation")]
        skip_confirm: bool,

        #[arg(long, group = "confirm_install", action = ArgAction::SetTrue, help = "Prompts a message to confirm installation")]
        confirm: bool,

        #[arg(long, group = "review_pkgbuild", action = ArgAction::SetTrue, help = "Skip PKGBUILD review")]
        skip_review: bool,

        #[arg(long, group = "review_pkgbuild", action = ArgAction::SetTrue, help = "Review PKGBUILD")]
        review: bool,

        #[arg(help = "The packages to install", required = false)]
        packages: Vec<String>,
    },

    #[command(about = "Uninstall a package")]
    Uninstall {
        #[arg(long, group = "confirm_uninstall", action = ArgAction::SetTrue, help = "Skips the prompt to confirm package uninstall")]
        skip_confirm: bool,

        #[arg(long, group = "confirm_uninstall", action = ArgAction::SetTrue, help = "Prompts a message to confirm uninstall")]
        confirm: bool,

        #[arg(help = "The packages to uninstall", required = false)]
        packages: Vec<String>,
    },

    #[command(about = "Update packages")]
    Update {
        #[arg(long, group = "update_aur", action = ArgAction::SetTrue, help = "Skip AUR package updates")]
        skip_aur: bool,

        #[arg(long, group = "update_aur", action = ArgAction::SetTrue, help = "Updates AUR packages")]
        aur: bool,

        #[arg(long, group = "review_pkgbuild", action = ArgAction::SetTrue, help = "Skip PKGBUILD review")]
        skip_review: bool,

        #[arg(long, group = "review_pkgbuild", action = ArgAction::SetTrue, help = "Review PKGBUILD")]
        review: bool,
    },

    #[command(about = "Search for a package")]
    Search { package: String },

    #[command(about = "List installed packages")]
    List {
        #[arg(long, action = ArgAction::SetTrue, help = "Only list AUR packages")]
        aur: bool,

        #[arg(help = "Filter by name")]
        filter: Option<String>,
    },

    #[command(about = "Update pacman signing keys")]
    UpdateKeys {},

    #[command(about = "Remove pacman lock file")]
    RemoveLock {},

    #[command(about = "Clear Cache")]
    ClearCache {
        #[arg(long, help = "Skip AUR package updates")]
        versions: usize,
    },
}
