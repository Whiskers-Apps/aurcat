use std::{env, error::Error};

use clap::{Parser, error::ErrorKind};

use crate::{
    clear_cache::on_clear_cache_command,
    cli::{Cli, MainCommand},
    config::get_config,
    install::on_install_command,
    list::on_list_command,
    remove_lock::on_remove_lock_command,
    search::on_search_command,
    uninstall::on_uninstall_command,
    update::on_update_command,
    update_keys::on_update_keys_command,
    utils::{run, show_message},
};

pub mod clear_cache;
pub mod cli;
pub mod config;
pub mod install;
pub mod list;
pub mod remove_lock;
pub mod search;
pub mod uninstall;
pub mod update;
pub mod update_keys;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::try_parse();
    let config = get_config()?;

    match cli {
        Ok(cli) => {
            if let Some(command) = cli.command {
                match command {
                    MainCommand::Install {
                        nosearch: skip_search,
                        search,
                        noconfirm: skip_confirm,
                        confirm,
                        noreview: skip_review,
                        review,
                        packages,
                    } => {
                        let search_fallback = match (skip_search, search) {
                            (true, false) => false,
                            (false, true) => true,
                            (false, false) => config.search_fallback,
                            _ => panic!("UUH?"),
                        };

                        let confirm_installation = match (skip_confirm, confirm) {
                            (true, false) => false,
                            (false, true) => true,
                            (false, false) => config.confirm_installation,
                            _ => panic!("UUH?"),
                        };

                        let review = match (skip_review, review) {
                            (true, false) => false,
                            (false, true) => true,
                            (false, false) => config.aur_review,
                            _ => panic!("UUH?"),
                        };

                        on_install_command(packages, search_fallback, review, confirm_installation)
                            .await?;
                    }
                    MainCommand::Uninstall {
                        noconfirm,
                        confirm,
                        packages,
                    } => {
                        let confirm = match (noconfirm, confirm) {
                            (true, false) => false,
                            (false, true) => true,
                            (false, false) => config.uninstall_confirm,
                            _ => panic!("UUH?"),
                        };

                        on_uninstall_command(packages, confirm)?;
                    }
                    MainCommand::Update {
                        noaur,
                        aur,
                        noreview,
                        review,
                        noconfirm,
                        confirm,
                    } => {
                        let aur = match (noaur, aur) {
                            (true, false) => false,
                            (false, true) => true,
                            (false, false) => config.update_aur,
                            _ => panic!("UUH?"),
                        };

                        let review = match (noreview, review) {
                            (true, false) => false,
                            (false, true) => true,
                            (false, false) => config.aur_review,
                            _ => panic!("UUH?"),
                        };

                        let confirm = match (noconfirm, confirm) {
                            (true, false) => false,
                            (false, true) => true,
                            (false, false) => config.confirm_update,
                            _ => panic!("UUH?"),
                        };

                        on_update_command(aur, review, confirm)?;
                    }
                    MainCommand::Search { package } => {
                        on_search_command(package, true, None).await?
                    }
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
                    || e.kind() == ErrorKind::InvalidSubcommand
                        && !commands.iter().any(|c| c == &main_command)
                {
                    match config.pacman_fallback {
                        true => {
                            show_message("Using Pacman");

                            let mut command = vec!["sudo".to_string(), "pacman".to_string()];
                            command.append(&mut args);

                            run(&command)?;
                        }
                        false => {
                            on_install_command(
                                args,
                                config.search_fallback,
                                config.aur_review,
                                config.confirm_installation,
                            )
                            .await?;
                        }
                    }

                    return Ok(());
                }
            }

            e.print()?;
        }
    }

    return Ok(());
}
