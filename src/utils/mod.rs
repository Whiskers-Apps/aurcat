use std::{
    ffi::OsStr,
    process::{Command, ExitStatus, Output, Stdio},
};

use colored::Colorize;
use inquire::{
    CustomType, Text,
    validator::{StringValidator, Validation},
};

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
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Error runnig command")
        .wait()
        .expect("Error waiting for command");

    show_status_message(status, success, failure);
}

pub fn get_seperator() -> String {
    let mut seperator = String::new();
    for _ in 0..(term_size::dimensions().unwrap_or((100, 100)).0) {
        seperator += "-";
    }
    return seperator;
}

pub fn get_number_in_range(message: &str, package_count: usize) -> usize {
    let number = CustomType::<usize>::new(&message)
        .with_validator(move |input: &usize| {
            if *input <= package_count {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Invalid Index".to_string().clone().into(),
                ))
            }
        })
        .prompt();

    return number.unwrap_or(0);
}

pub fn show_success_message(message: &str) {
    println!("{}", message.green());
}

pub fn show_error_message(message: &str) {
    println!("{}", message.red());
}
