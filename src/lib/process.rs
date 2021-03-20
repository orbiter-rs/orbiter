use std::fs;

use crate::hooks::extract::*;
use crate::hooks::install::*;
use crate::hooks::resource::*;
use crate::lib::config::*;
use crate::lib::paths::*;
use crate::lib::shimmer::*;

pub fn process_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // check if already worked on
    let payload_orbiter_dir_path = get_payload_config_dir_path(payload)?;
    if !payload_orbiter_dir_path.exists() {
        fs::create_dir_all(&payload_orbiter_dir_path)?;
        // save resource
        get_resource(payload)?;

        // extract resource
        if let Some(extract_cmd) = &payload.extract {
            // set wd to payload config dir
            let current_install_dir = get_payload_current_install_dir_path(payload)?;
            assert!(std::env::set_current_dir(&current_install_dir).is_ok());

            extract(&extract_cmd)?;
        }

        // set wd to payload dir
        let payload_dir = get_payload_dir_path(payload)?;
        assert!(std::env::set_current_dir(&payload_dir).is_ok());

        // install resource
        if let Some(install_cmd) = &payload.install {
            // set wd to payload config dir
            let current_install_dir = get_payload_current_install_dir_path(payload)?;
        println!("current_install_dir for install: {}", &current_install_dir.display().to_string());

            assert!(std::env::set_current_dir(&current_install_dir).is_ok());

            install(&install_cmd)?;
        }

        println!("payload_dir: {}", &payload_dir.display().to_string());


        // create shim
        match &payload.exec {
            Executable::Run(cmd) => {
                let shim_content = get_shim(&cmd, &cmd, None)?;
                persist_shim(&cmd, &shim_content)?;
            }

            Executable::Command { run, alias } => {
                if let Some(alias) = alias.as_ref() {
                    let shim_content = get_shim(run, alias, None)?;
                    persist_shim(alias, &shim_content)?;
                } else {
                    let shim_content = get_shim(run, run, None)?;
                    persist_shim(run, &shim_content)?;
                };
            }
        };
    }

    Ok(())
}
