use glob::glob;

use std::fs;

use crate::utils::{
    config::{ShellSpecificSourceTarget, SourceTarget},
    shells::SupportedShell,
};

pub fn src(
    current_shell: &SupportedShell,
    specified_src_target: &ShellSpecificSourceTarget,
) -> Result<(), Box<dyn std::error::Error>> {
    match specified_src_target {
        ShellSpecificSourceTarget::Generic(generic) => process_src_target(&generic),
        ShellSpecificSourceTarget::ShellSpecific(shell_specific) => {
            let op_shell_specific_target = match current_shell {
                SupportedShell::Sh => &shell_specific.sh,
                SupportedShell::Bash => &shell_specific.bash,
                SupportedShell::Zsh => &shell_specific.zsh,
                SupportedShell::Fish => &shell_specific.fish,
                SupportedShell::PowerShell => &shell_specific.powershell,
                SupportedShell::WinCmd => &shell_specific.wincmd,
            };

            if let Some(shell_specific_target) = op_shell_specific_target {
                process_src_target(&shell_specific_target)
            } else {
                Ok(())
            }
        }
    }
}

pub fn src_files(files: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for f in files {
        if f.contains("*") {
            // handle globs
            for entry in glob(&f).expect(&format!("unable to locate {}", f)) {
                if let Ok(entry_path) = entry {
                    print_src_path_canonical(&entry_path.display().to_string());
                }
            }
        } else {
            print_src_path_canonical(&f);
        }
    }

    Ok(())
}

fn print_src_path_canonical(path: &str) {
    if let Ok(canonical_path) = fs::canonicalize(path) {
        println!(". {}", &canonical_path.display().to_string());
    };
}

fn process_src_target(target: &SourceTarget) -> Result<(), Box<dyn std::error::Error>> {
    match target {
        SourceTarget::Single(target) => {
            let src_target = vec![target.to_owned()];
            src_files(&src_target)
        }
        SourceTarget::Multiple(targets) => src_files(targets),
    }
}
