use std::error::Error;

use crate::utils::run;

pub fn on_update_command(aur: bool, review: bool, confirm: bool) -> Result<(), Box<dyn Error>> {
    update_repo_packages(confirm)?;

    if !aur {
        return Ok(());
    }

    Ok(())
}

pub fn update_repo_packages(confirm: bool) -> Result<(), Box<dyn Error>> {
    let mut command = vec![
        "sudo".to_string(),
        "pacman".to_string(),
        "-Syyu".to_string(),
    ];

    if !confirm {
        command.push("--noconfirm".to_string());
    }

    run(&command)?;

    Ok(())
}
