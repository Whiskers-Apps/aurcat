use std::error::Error;

use crate::utils::run;

pub fn on_install_command(
    packages: Vec<String>,
    search_fallback: bool,
    review: bool,
    confirm_installation: bool,
) -> Result<(), Box<dyn Error>> {
    let mut command = vec!["sudo".to_string(), "pacman".to_string(), "-S".to_string()];
    let mut packages = packages;

    command.append(&mut packages);

    if !confirm_installation {
        command.push("--noconfirm".to_string());
    }

    run(&command)?;

    println!("{} {}", search_fallback, review);

    // TODO: Implement search fallback and AUR install

    // This should be implemented in case a package exist on multiple repos like on cachyos
    // pacman -S <repo_name>/<package_name>

    Ok(())
}
