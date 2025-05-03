use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(visible_alias = "i", about = "Install a pacman/aur package")]
    Install {
        packages: Vec<String>,

        #[arg(
            short = 's',
            long = "skip",
            help = "Skip confirmation while installing",
            default_missing_value = "true",
            num_args = 0..=1
        )]
        skip_confirm: Option<bool>,
    },

    #[command(
        visible_alias = "s",
        about = "Search for a package and type the number to uninstall it"
    )]
    Search {
        query: String,

        #[arg(
            short = 'p',
            long = "prompt",
            help = "Skip the installation prompt",
            default_missing_value = "true",
            num_args = 0..=1
        )]
        skip_prompt: Option<bool>,

        #[arg(
            short = 'm',
            long = "max",
            help = "The max amount of results to display",
            num_args = 0..=1
        )]
        max_results: Option<usize>,
    },

    #[command(visible_alias = "un", about = "Uninstall a pacman/aur package")]
    Uninstall {
        #[arg(value_name = "PACKAGE")]
        packages: Vec<String>,

        #[arg(
            short = 's',
            long = "skip",
            help = "Skip confirmation while uninstalling",
            default_missing_value = "true",
            num_args = 0..=1
        )]
        skip_confirm: Option<bool>,
    },

    #[command(visible_alias = "c", about = "Clean lock file or cache")]
    Clean {
        #[arg(
            short = 'c',
            long = "cache",
            help = "Clear pacman cache for X versions",
            value_name = "VERSIONS COUNT"
        )]
        pacman_cache: Option<usize>,
        #[arg(
            short = 'l',
            long = "lock-file",
            help = "Remove pacman lock file",
            default_value = "false"
        )]
        pacman_lock_file: bool,
    },

    #[command(visible_alias = "up", about = "Update your packages and mirrors")]
    Update {
        #[arg(
            short = 'a',
            long = "aur",
            help = "Skip AUR updates",
            default_missing_value = "true",
            num_args = 0..=1,
        )]
        skip_aur: Option<bool>,
    },

    #[command(visible_alias = "l", about = "List installed packages on your system")]
    List {
        #[arg(
            short = 'f',
            long = "filter",
            help = "Filter the list by a package name"
        )]
        filter: Option<String>,
    },
}
