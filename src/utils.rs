use std::{error::Error, process::Command};

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

    let result = Command::new(main).args(args).output()?;

    let bytes = result.stdout;
    let clean_bytes = strip(&bytes);

    let output = String::from_utf8(clean_bytes)?;

    return Ok(output);
}
