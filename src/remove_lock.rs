use std::error::Error;

use crate::utils::{run, show_message};

pub fn on_remove_lock_command() -> Result<(), Box<dyn Error>> {
    run(&["sudo", "rm", "-f", "/var/lib/pacman/db.lck"])?;

    show_message("Removed Lock File");

    Ok(())
}
