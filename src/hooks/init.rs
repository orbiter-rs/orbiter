use crate::lib::{config::AdaptiveInit, script::*};

pub fn init(adaptive_init: &AdaptiveInit) -> Result<String, Box<dyn std::error::Error>> {
    let init_result = match adaptive_init {
        AdaptiveInit::Run(cmd) => run_cmd_with_output(&cmd)?,
        AdaptiveInit::OSSpecific {
            linux,
            macos,
            windows,
        } => {
            let os = std::env::consts::OS;
            match os {
                "linux" => {
                    if let Some(cmd) = &linux {
                        run_cmd_with_output(&cmd)?
                    } else {
                        "".to_owned()
                    }
                }
                "macos" => {
                    if let Some(cmd) = &macos {
                        run_cmd_with_output(&cmd)?
                    } else {
                        "".to_owned()
                    }
                }
                "windows" => {
                    if let Some(cmd) = &windows {
                        run_cmd_with_output(&cmd)?
                    } else {
                        "".to_owned()
                    }
                }
                _ => {
                    println!("init hook: unsupported os os={}", os);
                    "".to_owned()
                }
            }
        }
    };

    Ok(init_result)
}
