use std::error::Error;

use crate::utils::{run, show_message};

pub fn on_update_keys_command() -> Result<(), Box<dyn Error>> {
    show_message("Go Grab a Coffe. This Might Take a While.");

    run(&["sudo", "pacman-key", "--refresh-keys"])?;

    show_message("Keys Successfully Updated");

    Ok(())
}
