// run shell script in subshell

use std::error::Error;
use std::process::{Command, Output};
use std::str;

use super::config::{OSSpecificCommand, SupportedShellSpecificCommand};
use super::shells::SupportedShell;

pub fn run_cmd(
    current_shell: &SupportedShell,
    full_cmd: &str,
) -> Result<Output, Box<dyn std::error::Error>> {
    Ok(Command::new(current_shell.as_program_str())
        .arg(current_shell.as_dflt_arg_str())
        .arg(full_cmd)
        .output()?)
}

#[allow(dead_code)]
pub fn run_cmd_with_output(
    current_shell: &SupportedShell,
    full_cmd: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let proc = run_cmd(current_shell, full_cmd)?;

    let stdout_content = str::from_utf8(&proc.stdout).unwrap_or("");
    let stderr_content = str::from_utf8(&proc.stderr).unwrap_or("");

    let output = format!("{}{}", stdout_content, stderr_content);

    Ok(output)
}

pub fn run_os_specific_cmd(
    current_shell: &SupportedShell,
    op_os_specific_cmd: &Option<OSSpecificCommand>,
) -> Result<String, Box<dyn Error>> {
    Ok(if let Some(all_os_specific_cmd) = op_os_specific_cmd {
        match all_os_specific_cmd {
            OSSpecificCommand::Generic(cmd) => run_cmd_with_output(current_shell, &cmd)?,
            OSSpecificCommand::OSSpecific(os_specific_cmd) => {
                let os = std::env::consts::OS;
                match os {
                    "linux" => {
                        if let Some(cmd) = &os_specific_cmd.linux {
                            run_cmd_with_output(current_shell, &cmd)?
                        } else {
                            "".to_owned()
                        }
                    }
                    "macos" => {
                        if let Some(cmd) = &os_specific_cmd.macos {
                            run_cmd_with_output(current_shell, &cmd)?
                        } else {
                            "".to_owned()
                        }
                    }
                    "windows" => {
                        if let Some(cmd) = &os_specific_cmd.windows {
                            run_cmd_with_output(current_shell, &cmd)?
                        } else {
                            "".to_owned()
                        }
                    }
                    _ => "".to_owned(),
                }
            }
        }
    } else {
        "".to_owned()
    })
}

pub fn run_shell_cmd(
    current_shell: &SupportedShell,
    shell_specific_cmd: &SupportedShellSpecificCommand,
) -> Result<String, Box<dyn std::error::Error>> {
    let os_specific_cmd = match current_shell {
        SupportedShell::Sh => &shell_specific_cmd.sh,
        SupportedShell::Bash => &shell_specific_cmd.bash,
        SupportedShell::Zsh => &shell_specific_cmd.zsh,
        SupportedShell::Fish => &shell_specific_cmd.fish,
        SupportedShell::PowerShell => &shell_specific_cmd.powershell,
        SupportedShell::WinCmd => &shell_specific_cmd.wincmd,
    };

    Ok(run_os_specific_cmd(current_shell, os_specific_cmd)?)
}
