use std::{env, error::Error};

use clap::{CommandFactory, Parser, error::ErrorKind};

use crate::{
    clear_cache::on_clear_cache_command,
    cli::{Cli, MainCommand},
    config::get_config,
    list::on_list_command,
    remove_lock::on_remove_lock_command,
    search::on_search_command,
    update_keys::on_update_keys_command,
    utils::run,
};

pub mod clear_cache;
pub mod cli;
pub mod config;
pub mod list;
pub mod remove_lock;
pub mod search;
pub mod update_keys;
pub mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::try_parse();
    let config = get_config()?;

    match cli {
        Ok(cli) => {
            if let Some(command) = cli.command {
                match command {
                    MainCommand::Install {
                        skip_search,
                        search,
                        skip_confirm,
                        confirm,
                        skip_review,
                        review,
                        packages,
                    } => {
                        // pacman -S <repo_name>/<package_name>
                    }
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
                    MainCommand::Search { package } => on_search_command(package)?,
                    MainCommand::List { aur, filter } => on_list_command(aur, filter)?,
                    MainCommand::UpdateKeys {} => on_update_keys_command()?,
                    MainCommand::RemoveLock {} => on_remove_lock_command()?,
                    MainCommand::ClearCache { versions } => {
                        let versions = match versions {
                            Some(versions) => versions,
                            None => config.cache_version_count,
                        };

                        on_clear_cache_command(versions)?;
                    }
                }
            }
        }
        Err(e) => {
            let mut args: Vec<String> = env::args().collect();
            args = args
                .into_iter()
                .enumerate()
                .filter_map(|(index, arg)| if index > 0 { Some(arg) } else { None })
                .collect();

            if args.len() >= 1 {
                let commands = vec![
                    "install",
                    "uninstall",
                    "update",
                    "search",
                    "list",
                    "update-keys",
                    "remove-lock",
                    "clear-cache",
                ];

                let main_command = args.get(0).unwrap();

                if e.kind() == ErrorKind::UnknownArgument
                    && !commands.iter().any(|c| c == &main_command)
                {
                    let mut command = vec!["sudo".to_string(), "pacman".to_string()];
                    command.append(&mut args);

                    run(&command)?;

                    return Ok(());
                }
            }

            e.print()?;
        }
    }

    return Ok(());
}
