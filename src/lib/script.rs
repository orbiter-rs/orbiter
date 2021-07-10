// run shell script in subshell

use std::env;
use std::io::{self, Write};
use std::process::{Command, Output};

pub fn run_cmd(full_cmd: &str) -> Result<Output, Box<dyn std::error::Error>> {
    let dflt_shell = env::var("SHELL")?;
    let proc = Command::new(dflt_shell).arg("-c").arg(full_cmd).output()?;

    Ok(proc)
}

#[allow(dead_code)]
pub fn run_cmd_with_output(full_cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let proc = run_cmd(full_cmd)?;

    io::stdout().write_all(&proc.stdout).unwrap();
    io::stderr().write_all(&proc.stderr).unwrap();

    Ok(())
}
