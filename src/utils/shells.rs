use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SupportedShell {
    Sh,
    Zsh,
    Bash,
    Fish,
    PowerShell,
    WinCmd,
}

impl SupportedShell {
    pub fn as_program_str(&self) -> &'static str {
        match self {
            SupportedShell::Sh => "sh",
            SupportedShell::Zsh => "zsh",
            SupportedShell::Bash => "bash",
            SupportedShell::Fish => "fish",
            SupportedShell::PowerShell => "powershell",
            SupportedShell::WinCmd => "cmd.exe",
        }
    }

    pub fn as_dflt_arg_str(&self) -> &'static str {
        match self {
            SupportedShell::Sh => "-c",
            SupportedShell::Zsh => "-c",
            SupportedShell::Bash => "-c",
            SupportedShell::Fish => "-c",
            SupportedShell::PowerShell => "-command",
            SupportedShell::WinCmd => "/C",
        }
    }

    pub fn from_str(shell: &str) -> SupportedShell {
        match shell {
            "sh" => SupportedShell::Sh,
            "zsh" => SupportedShell::Zsh,
            "bash" => SupportedShell::Bash,
            "fish" => SupportedShell::Fish,
            "powershell" => SupportedShell::PowerShell,
            "cmd" => SupportedShell::WinCmd,
            _ => SupportedShell::Sh,
        }
    }
}
