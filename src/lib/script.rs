// run shell script in subshell

use std::io::{self, Write};
use std::process::Command;

pub fn run_cmd(full_cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(full_cmd)
        .output()
        .expect("Failed to execute command");

    // println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    Ok(())
}
