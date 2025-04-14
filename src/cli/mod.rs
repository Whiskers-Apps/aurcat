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
        package: String,
        #[arg(
            short = 'a',
            long = "aur",
            help = "Install the package from AUR instead of the repositories",
            default_value = "false"
        )]
        aur: bool,
    },

    #[command(visible_alias = "s", about = "Search for a package")]
    Search { package: String },

    #[command(visible_alias = "un", about = "Uninstall a pacman/aur package")]
    Uninstall { package: String },

    #[command(visible_alias = "c", about = "Clean lock file or cache")]
    Clean {
        #[arg(
            short = 'c',
            long = "cache",
            help = "Clear pacman cache",
            default_value = "false"
        )]
        pacman_cache: bool,
        #[arg(
            short = 'l',
            long = "lock-file",
            help = "Remove pacman lock file",
            default_value = "false"
        )]
        pacman_lock_file: bool,
    },

    #[command(about = "Show possible settings and their current value")]
    Settings {},

    #[command(visible_alias = "up", about = "Update your packages and mirrors")]
    Update {
        #[arg(
            short = 'a',
            long = "aur",
            help = "Update AUR packages",
            default_value = "false"
        )]
        aur: bool,
    },
}
