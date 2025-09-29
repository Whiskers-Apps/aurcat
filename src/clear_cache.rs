use std::error::Error;

use crate::utils::run;

pub fn on_clear_cache_command(versions: usize) -> Result<(), Box<dyn Error>> {
    let versions_str = format!("-rk{versions}");
    let command = vec!["sudo", "paccache", &versions_str];

    run(&command)?;

    Ok(())
}
