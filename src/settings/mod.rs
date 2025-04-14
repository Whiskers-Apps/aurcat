use std::{fs, path::PathBuf};

use prettytable::{Table, row};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "get_true")]
    pub confirm_install: bool,

    #[serde(default = "get_true")]
    pub confirm_uninstall: bool,
}

pub fn get_settings_path() -> PathBuf {
    dirs::config_dir()
        .expect("Error getting config dir")
        .join("aurcat.toml")
}

// ===============================================
// ==== Settings Values
// ===============================================

fn get_true() -> bool {
    true
}

// ===============================================
// ==== Settings
// ===============================================

fn get_default_settings() -> Settings {
    Settings {
        confirm_install: get_true(),
        confirm_uninstall: get_true(),
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

impl Settings {
    pub fn show_settings(&self) {
        let mut table = Table::new();

        table.add_row(row![
            "Setting",
            "TOML",
            "Description",
            "Current",
            "Possible Values"
        ]);

        table.add_row(row![
            "Confirm Install",
            "confirm_install",
            "Require confirmation before installing a package",
            format!("{}", self.confirm_install),
            "true|false"
        ]);

        table.add_row(row![
            "Confirm Uninstall",
            "confirm_uninstall",
            "Require confirmation before uninstalling a package",
            format!("{}", self.confirm_uninstall),
            "true|false"
        ]);

        table.printstd();
    }
}
