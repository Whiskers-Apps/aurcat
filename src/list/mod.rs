use std::{error::Error, process::Command};

#[derive(Debug, Clone)]
pub struct InstalledPackage {
    pub package: String,
    pub version: String,
}

pub fn get_installed_packages() -> Result<Vec<InstalledPackage>, Box<dyn Error>> {
    let command = Command::new("pacman").arg("-Q").output()?;

    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);

        let output_split = output
            .split("\n")
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let mut packages = Vec::<InstalledPackage>::new();

        for package_line in output_split {
            let split = package_line
                .split_whitespace()
                .map(|a| a.to_owned())
                .collect::<Vec<String>>();

            if split.len() == 2 {
                packages.push(InstalledPackage {
                    package: split.get(0).unwrap().to_owned(),
                    version: split.get(1).unwrap().to_owned(),
                });
            }
        }

        return Ok(packages);
    }

    return Err("Error getting packages".into());
}

pub fn get_installed_aur_packages() -> Result<Vec<InstalledPackage>, Box<dyn Error>> {
    let command = Command::new("pacman").arg("-Qm").output()?;

    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);

        let output_split = output
            .split("\n")
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();

        let mut packages = Vec::<InstalledPackage>::new();

        for package_line in output_split {
            let split = package_line
                .split_whitespace()
                .map(|a| a.to_owned())
                .collect::<Vec<String>>();

            if split.len() == 2 {
                packages.push(InstalledPackage {
                    package: split.get(0).unwrap().to_lowercase().to_owned(),
                    version: split.get(1).unwrap().to_owned(),
                });
            }
        }

        return Ok(packages);
    }

    return Ok(vec![]);
}
