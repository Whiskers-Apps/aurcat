use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "get_false")]
    pub skip_install_confirm: bool,

    #[serde(default = "get_false")]
    pub skip_uninstall_confirm: bool,

    #[serde(default = "get_false")]
    pub skip_search_prompt: bool,

    #[serde(default = "default_max_search_results")]
    pub max_search_results: usize,

    #[serde(default = "get_false")]
    pub skip_aur_update: bool,
}

pub fn get_settings_path() -> PathBuf {
    dirs::config_dir()
        .expect("Error getting config dir")
        .join("aurcat.toml")
}

// ===============================================
// ==== Settings Values
// ===============================================

fn get_false() -> bool {
    false
}

fn default_max_search_results() -> usize {
    30
}

// ===============================================
// ==== Settings
// ===============================================

fn get_default_settings() -> Settings {
    Settings {
        skip_install_confirm: false,
        skip_uninstall_confirm: false,
        skip_search_prompt: false,
        max_search_results: default_max_search_results(),
        skip_aur_update: false,
    }
}

pub fn get_settings() -> Settings {
    let path = get_settings_path();

    if path.exists() {
        let content = fs::read_to_string(&path).expect("Error reading settings file");
        let settings = toml::from_str::<Settings>(&content).expect("Error parsing settings toml");

        return settings;
    }

    get_default_settings()
}
