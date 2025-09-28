use std::{default, error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub skip_search: bool,
    pub skip_install_confirm: bool,
    pub skip_uninstall_confirm: bool,
    pub skip_review: bool,
    pub aur_review: bool,
    pub cache_version_count: usize,
    pub skip_aur_update: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            aur_review: false,
            cache_version_count: 5,
            skip_search: false,
            skip_install_confirm: false,
            skip_uninstall_confirm: false,
            skip_review: true,
            skip_aur_update: false,
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
