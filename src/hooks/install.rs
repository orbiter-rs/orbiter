use crate::utils::{config::AdaptiveInstall, script::*};

pub fn install(adaptive_install: &AdaptiveInstall) -> Result<(), Box<dyn std::error::Error>> {
    match adaptive_install {
        AdaptiveInstall::Run(cmd) => {
            run_cmd_with_output(&cmd)?;
        }
        AdaptiveInstall::OSSpecific {
            linux,
            macos,
            windows,
        } => {
            let os = std::env::consts::OS;
            match os {
                "linux" => {
                    if let Some(cmd) = &linux {
                        run_cmd(&cmd)?;
                    }
                }
                "macos" => {
                    if let Some(cmd) = &macos {
                        run_cmd(&cmd)?;
                    }
                }
                "windows" => {
                    if let Some(cmd) = &windows {
                        run_cmd(&cmd)?;
                    }
                }
                _ => {
                    println!("install hook: unsupported os os={}", os);
                }
            }
        }
    };

    Ok(())
}
