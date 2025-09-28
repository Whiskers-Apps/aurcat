use std::{env, error::Error};

use clap::Parser;

use crate::{
    cli::{Cli, MainCommand},
    config::get_config,
    list::on_list_command,
    utils::run,
};

pub mod cli;
pub mod config;
pub mod list;
pub mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::try_parse();
    let config = get_config()?;

    if let Ok(cli) = cli {
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
                // Update
            }
        }
    } else {
        let mut args: Vec<String> = env::args().collect();

        args = args
            .into_iter()
            .enumerate()
            .filter_map(|(index, arg)| if index > 0 { Some(arg) } else { None })
            .collect();

        let mut command = vec!["sudo".to_string(), "pacman".to_string()];
        command.append(&mut args);

        run(&command)?;
    }

    return Ok(());
}
