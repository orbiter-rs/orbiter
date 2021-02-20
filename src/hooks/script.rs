// run shell script in subshell

use std::process::Command;

pub fn run_cmd(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("sh")
        .arg("-c")
        .arg("export XXX=testfromrust")
        .output()
        .expect("failed to execute process");


    Ok(())
}
