use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    /// Search for package if not found
    pub search_fallback: bool,

    /// Skips the install confirmation
    pub confirm_installation: bool,

    /// Skips the uninstall confirmation
    pub skip_uninstall_confirm: bool,

    /// Skips the review prompt
    pub review: bool,

    /// Show PKGBUILD
    pub aur_review: bool,

    /// The amount of cached package versions to keep
    pub cache_version_count: usize,

    /// Don't update AUR packages
    pub skip_aur_update: bool,

    /// Max Results Per Search Query
    pub max_results: usize,

    /// Fallback to install packages. If disabled it will use pacman with the parameters provided.
    pub install_fallback: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aur_review: false,
            cache_version_count: 5,
            search_fallback: true,
            confirm_installation: false,
            skip_uninstall_confirm: false,
            review: true,
            skip_aur_update: false,
            max_results: 30,
            install_fallback: true,
        }
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let config = dirs::config_dir().ok_or_else(|| "Failed to get config dir".to_string())?;

    return Ok(config.join("aurcat").join("config.toml"));
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let config_path = get_config_path()?;

    let config_dir = config_path
        .clone()
        .parent()
        .ok_or_else(|| "Failed to get config dir".to_string())?
        .to_path_buf();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    if !config_path.exists() {
        let content = toml::to_string_pretty(&Config::default())?;

        fs::write(&config_path, &content)?;
    }

    let bytes = fs::read(&config_path)?;
    let config: Config = toml::from_slice(&bytes)?;

    return Ok(config);
}
