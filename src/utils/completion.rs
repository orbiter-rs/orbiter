use super::shells::SupportedShell;

pub fn load_completion(current_shell: &SupportedShell) {
    let shell_specific_evaluatable = match current_shell {
        SupportedShell::Sh => "",
        SupportedShell::Bash => "",
        SupportedShell::Zsh => "autoload -Uz compinit; compinit",
        SupportedShell::Fish => "",
        SupportedShell::PowerShell => "",
        SupportedShell::WinCmd => "",
    };

    println!("{}", shell_specific_evaluatable);
}
