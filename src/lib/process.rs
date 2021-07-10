use std::fs;

use crate::hooks::extract::*;
use crate::hooks::install::*;
use crate::hooks::resource::*;
use crate::hooks::src::*;
use crate::lib::config::*;
use crate::lib::paths::*;
use crate::lib::shimmer::*;

pub fn process_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // check if already worked on
    let payload_orbiter_dir_path = get_payload_config_dir_path(payload)?;
    if !payload_orbiter_dir_path.exists() {
        fs::create_dir_all(&payload_orbiter_dir_path)?;
        // save resource
        let resource_path = get_resource(payload)?;

        // set wd to payload current dir
        let current_install_dir = get_payload_current_install_dir_path(payload)?;
        assert!(std::env::set_current_dir(&current_install_dir).is_ok());

        // extract resource
        if let Some(extract_cmd) = &payload.extract {
            extract(&extract_cmd)?;
        } else if let Some(asset_path) = &resource_path {
            extract_asset(&asset_path)?;
        }

        // install resource
        if let Some(install_cmd) = &payload.install {
            install(&install_cmd)?;
        }

        // create shim
        if let Some(exec) = &payload.exec {
            match exec {
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
        };
    }

    // source scripts
    if let Some(src_files) = &payload.src {
        // set wd to payload config dir
        let current_install_dir = get_payload_current_install_dir_path(payload)?;
        assert!(std::env::set_current_dir(&current_install_dir).is_ok());

        match src_files {
            SourceTarget::Single(target) => {
                let src_target = vec![target.to_owned()];
                src(&src_target)?;
            }
            SourceTarget::Multiple(targets) => src(targets)?,
        };
    }

    // post load
    if let Some(load_script) = &payload.load {
        // set wd to payload config dir
        let current_install_dir = get_payload_current_install_dir_path(payload)?;
        assert!(std::env::set_current_dir(&current_install_dir).is_ok());

        println!("{}", &load_script);
    }

    Ok(())
}
