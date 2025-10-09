use std::{
    error::Error,
    path::Path,
    process::{Command, Stdio},
};

use inquire::ui::{RenderConfig, Styled};
use strip_ansi_escapes::strip;

pub fn run_hidden(command: &[&str]) -> Result<String, Box<dyn Error>> {
    let main = command.get(0).ok_or_else(|| "Empty Vector".to_string())?;

    let args: Vec<String> = command
        .iter()
        .enumerate()
        .filter_map(|(index, arg)| {
            if index > 0 {
                Some(arg.to_string())
            } else {
                None
            }
        })
        .collect();

    let result = Command::new(main)
        .args(args)
        .stdin(Stdio::inherit())
        .output()?;

    let bytes = result.stdout;
    let clean_bytes = strip(&bytes);

    let output = String::from_utf8(clean_bytes)?;

    return Ok(output);
}

pub fn run_hidden_in_path<P: AsRef<Path>>(
    command: &[&str],
    path: P,
) -> Result<String, Box<dyn Error>> {
    let main = command.get(0).ok_or_else(|| "Empty Vector".to_string())?;

    let args: Vec<String> = command
        .iter()
        .enumerate()
        .filter_map(|(index, arg)| {
            if index > 0 {
                Some(arg.to_string())
            } else {
                None
            }
        })
        .collect();

    let result = Command::new(main)
        .args(args)
        .stdin(Stdio::inherit())
        .current_dir(path.as_ref())
        .output()?;

    let bytes = result.stdout;
    let clean_bytes = strip(&bytes);

    let output = String::from_utf8(clean_bytes)?;

    return Ok(output);
}

pub fn run<S: AsRef<str>>(command: &[S]) -> Result<String, Box<dyn Error>> {
    let main = command
        .get(0)
        .ok_or_else(|| "Empty Vector".to_string())?
        .as_ref();

    let args: Vec<String> = command
        .iter()
        .enumerate()
        .filter_map(|(index, arg)| {
            if index > 0 {
                Some(arg.as_ref().to_string())
            } else {
                None
            }
        })
        .collect();

    let result = Command::new(main)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    let bytes = result.stdout;
    let clean_bytes = strip(&bytes);

    let output = String::from_utf8(clean_bytes)?;

    return Ok(output);
}

pub fn run_in_path<S: AsRef<str>, P: AsRef<Path>>(
    command: &[S],
    path: P,
) -> Result<String, Box<dyn Error>> {
    let main = command
        .get(0)
        .ok_or_else(|| "Empty Vector".to_string())?
        .as_ref();

    let args: Vec<String> = command
        .iter()
        .enumerate()
        .filter_map(|(index, arg)| {
            if index > 0 {
                Some(arg.as_ref().to_string())
            } else {
                None
            }
        })
        .collect();

    let result = Command::new(main)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(path.as_ref())
        .output()?;

    let bytes = result.stdout;
    let clean_bytes = strip(&bytes);

    let output = String::from_utf8(clean_bytes)?;

    return Ok(output);
}

pub fn show_message<S: AsRef<str>>(message: S) {
    println!("😺 {}", message.as_ref());
}

pub fn get_empty_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("");
    render_config
}
