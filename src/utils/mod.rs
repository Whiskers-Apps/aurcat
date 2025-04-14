use std::{
    env::args,
    ffi::OsStr,
    process::{Command, ExitStatus, Output},
};

use clap::Arg;
use colored::Colorize;

pub fn show_status_message(status: ExitStatus, success: &str, failure: &str) {
    let message = match status.success() {
        true => format!("{success}").green(),
        false => format!("{failure}").red(),
    };

    println!("{message}");
}

pub fn show_output_message(output: Output, success: &str, failure: &str) {
    let message = match output.status.success() {
        true => format!("{success}").green(),
        false => format!(
            "{failure}.\nDetails: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .red(),
    };

    println!("{message}");
}

pub fn run_elevated_command<I, S>(command: I, success: &str, failure: &str)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new("sudo")
        .args(command)
        .spawn()
        .expect("Error runnig command")
        .wait()
        .expect("Error waiting for command");

    show_status_message(status, success, failure);
}
