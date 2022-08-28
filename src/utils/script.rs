// run shell script in subshell

use std::env;
use std::process::{Command, Output};
use std::str;

pub fn run_cmd(full_cmd: &str) -> Result<Output, Box<dyn std::error::Error>> {
    let dflt_shell = env::var("SHELL")?;
    let proc = Command::new(dflt_shell).arg("-c").arg(full_cmd).output()?;

    Ok(proc)
}

#[allow(dead_code)]
pub fn run_cmd_with_output(full_cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
    let proc = run_cmd(full_cmd)?;

    let stdout_content = str::from_utf8(&proc.stdout).unwrap_or("");
    let stderr_content = str::from_utf8(&proc.stderr).unwrap_or("");

    let output = format!("{}{}", stdout_content, stderr_content);

    Ok(output)
}
