use std::error::Error;

use clap::Parser;

use crate::{
    cli::{Cli, MainCommand},
    config::get_config,
    list::on_list_command,
};

pub mod cli;
pub mod config;
pub mod list;
pub mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config = get_config()?;

    match cli.command {
        Some(command) => match command {
            MainCommand::Install {
                skip_search,
                search,
                skip_confirm,
                confirm,
                skip_review,
                review,
                packages,
            } => {}
            MainCommand::Uninstall {
                skip_confirm,
                confirm,
                packages,
            } => todo!(),
            MainCommand::Update {
                skip_aur,
                aur,
                skip_review,
                review,
            } => todo!(),
            MainCommand::Search { package } => todo!(),
            MainCommand::List { aur, filter } => on_list_command(aur, filter)?,
            MainCommand::UpdateKeys {} => todo!(),
            MainCommand::RemoveLock {} => todo!(),
            MainCommand::ClearCache { versions } => todo!(),
        },
        None => {
            // Install given packages
            if cli.packages.is_empty() {
                // Updating
                return Ok(());
            }

            // Install packages
        }
    }

    return Ok(());
}
