use crate::utils::{config::ShellSpecificEvaluatable, shells::SupportedShell};

pub fn process_evaluatable(
    current_shell: &SupportedShell,
    evaluatable: &ShellSpecificEvaluatable,
) -> Result<(), Box<dyn std::error::Error>> {
    match evaluatable {
        ShellSpecificEvaluatable::Generic(generic) => println!("{}", generic),
        ShellSpecificEvaluatable::ShellSpecific(shell_specific) => {
            let shell_specific_evaluatable = match current_shell {
                SupportedShell::Sh => &shell_specific.sh,
                SupportedShell::Bash => &shell_specific.bash,
                SupportedShell::Zsh => &shell_specific.zsh,
                SupportedShell::Fish => &shell_specific.fish,
                SupportedShell::PowerShell => &shell_specific.powershell,
                SupportedShell::WinCmd => &shell_specific.wincmd,
            };

            match shell_specific_evaluatable {
                Some(evaluatable) => println!("{}", evaluatable),
                None => (),
            }
        }
    };

    Ok(())
}
