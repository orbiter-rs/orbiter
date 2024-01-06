use crate::utils::{config::ShellSpecificCommand, script::*, shells::SupportedShell};

pub fn install(
    current_shell: &SupportedShell,
    install_cmd: &ShellSpecificCommand,
) -> Result<String, Box<dyn std::error::Error>> {
    match install_cmd {
        ShellSpecificCommand::Generic(generic) => {
            run_cmd_in_shell_with_output(current_shell, generic)
        }
        ShellSpecificCommand::ShellSpecific(shell_specific) => {
            run_shell_specific_cmd(current_shell, shell_specific)
        }
    }
}
