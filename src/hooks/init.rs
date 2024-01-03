use crate::utils::{config::ShellSpecificCommand, script::*, shells::SupportedShell};

pub fn init(
    current_shell: &SupportedShell,
    init_cmd: &ShellSpecificCommand,
) -> Result<String, Box<dyn std::error::Error>> {
    match init_cmd {
        ShellSpecificCommand::Generic(generic) => run_cmd_with_output(current_shell, generic),
        ShellSpecificCommand::ShellSpecific(shell_specific) => {
            run_shell_cmd(current_shell, shell_specific)
        }
    }
}
