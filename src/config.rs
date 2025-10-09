use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    /// Search for package if not found
    pub search_fallback: bool,

    /// Prompt a confirm message when installing a package
    pub confirm_installation: bool,

    /// Prompt a confirm message when updating a package
    pub confirm_update: bool,

    /// Prompt a confirm message when uninstalling a package
    pub uninstall_confirm: bool,

    /// Review Package Build
    pub aur_review: bool,

    /// The amount of cached package versions to keep
    pub cache_version_count: usize,

    /// Update AUR packages
    pub update_aur: bool,

    /// Max Results Per Search Query. For example 30 will show 30 repo packages and 30 aur packages
    pub max_results: usize,

    /// Fallback to pacman commands. If disabled it will use install command instead.
    pub pacman_fallback: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aur_review: false,
            cache_version_count: 5,
            search_fallback: true,
            confirm_installation: true,
            uninstall_confirm: true,
            update_aur: true,
            max_results: 30,
            pacman_fallback: false,
            confirm_update: true,
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
