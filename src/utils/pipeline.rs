use std::fs;

use super::config::*;
use super::paths::*;
use super::shim::*;
use super::symlink::*;
use crate::hooks::extract::*;
use crate::hooks::init::*;
use crate::hooks::install::*;
use crate::hooks::resource::*;
use crate::hooks::src::*;

use log::info;

pub fn process_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // check if already worked on
    let payload_orbiter_dir_path = get_payload_config_dir_path(payload)?;
    if !payload_orbiter_dir_path.exists() {
        info!(
            "Creating payload config directory {}",
            payload_orbiter_dir_path.to_str().unwrap()
        );
        fs::create_dir_all(&payload_orbiter_dir_path)?;

        let init_result = if let Some(init_cmd) = &payload.init {
            Some(init(init_cmd)?)
        } else {
            None
        };

        // save resource
        let resource_path = get_adaptive_resource(payload, init_result.as_deref())?;

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
            install(install_cmd)?;
        }

        // create shim
        if let Some(exec) = &payload.exec {
            match exec {
                Executable::Run(cmd) => {
                    create_shim(&cmd, &get_shim_content(&cmd, &cmd, None)?)?;
                }

                Executable::Command {
                    run,
                    alias,
                    use_symlink,
                } => {
                    if let Some(is_use_symlink) = use_symlink {
                        if is_use_symlink.to_owned() {
                            create_symlink(run, alias)?;
                        }
                    } else {
                        if let Some(alias) = alias.as_ref() {
                            create_shim(alias, &get_shim_content(run, alias, None)?)?;
                        } else {
                            create_shim(run, &get_shim_content(run, run, None)?)?;
                        };
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

pub fn update_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // 1. remove shim
    if let Some(exec) = &payload.exec {
        match exec {
            Executable::Run(cmd) => {
                remove_shim(&cmd)?;
            }
            Executable::Command { run, alias, .. } => {
                if let Some(alias) = alias.as_ref() {
                    remove_shim(alias)?;
                } else {
                    remove_shim(run)?;
                }
            }
        };
    };

    // 2. rename current folder
    let current_folder_path = get_payload_current_install_dir_path(&payload);

    if (dirs.exists(current_folder_path)) {}
    // 3. process payload (create new current folder)
    Ok(())
}
