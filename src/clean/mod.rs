use std::{
    error::Error,
    process::{Command, Stdio},
};

use crate::utils::{show_error_message, show_success_message};

/// Handles the clean command from CLI
pub fn on_clean(clean_cache: Option<usize>, remove_lock: bool) {
    // TODO: Use setting in case of nothing is provided in parameters

    if let Some(cache_versions) = clean_cache {
        if clean_pacman_cache(cache_versions).is_ok() {
            show_success_message(
                "🧹 I bet you had a lot of cached packages. Your disk will thank you 🙏",
            );
        } else {
            show_error_message(
                "💣 Something must went really badly for this to fail. Verify if you typed your sudo password",
            );
        }
    }

    if remove_lock {
        if remove_lock_file().is_ok() {
            show_success_message(
                "🧹 Your house just got a bit cleaner. Lock file has been removed",
            );
        } else {
            show_error_message(
                "🤨 An error? Really? Verify if you actually typed your sudo password",
            );
        };
    }
}

/// Uses paccache command to clean the pacman cache with X amount of versions
fn clean_pacman_cache(cache_versions: usize) -> Result<(), Box<dyn Error>> {
    let command = Command::new("sudo")
        .args(["paccache", &format!("-rk{cache_versions}")])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    if command.success() {
        Ok(())
    } else {
        Err("Error cleaning cache".into())
    }
}

/// Removes the pacman lock file at `/var/lib/pacman/`
fn remove_lock_file() -> Result<(), Box<dyn Error>> {
    let command = Command::new("sudo")
        .args(["rm", "-f", "/var/lib/pacman/db.lock"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    return if command.success() {
        Ok(())
    } else {
        Err("Error ".into())
    };
}
