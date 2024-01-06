use std::fs;

use super::config::*;
use super::paths::*;
use super::shells::SupportedShell;
use super::shim::*;
use super::symlink::*;
use crate::hooks::extract::*;
use crate::hooks::init::*;
use crate::hooks::install::*;
use crate::hooks::load::*;
use crate::hooks::resource::*;
use crate::hooks::src::*;

use log::info;

pub fn process_payload(
    current_shell: &SupportedShell,
    payload: &Payload,
) -> Result<(), Box<dyn std::error::Error>> {
    // check if already worked on
    let payload_orbiter_dir_path = get_payload_config_dir_path(payload)?;
    if !payload_orbiter_dir_path.exists()
        || !get_payload_current_install_dir_path(payload)?.exists()
    {
        info!(
            "Creating payload config directory {}",
            payload_orbiter_dir_path.to_str().unwrap()
        );
        fs::create_dir_all(&payload_orbiter_dir_path)?;

        let init_result = if let Some(init_cmd) = &payload.init {
            Some(init(current_shell, init_cmd)?)
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
            extract(current_shell, &extract_cmd)?;
        } else if let Some(asset_path) = &resource_path {
            extract_asset(current_shell, &asset_path)?;
        }

        // install resource
        if let Some(install_cmd) = &payload.install {
            install(current_shell, install_cmd)?;
        }

        // create shim
        if let Some(exec) = &payload.exec {
            match exec {
                Executable::Run(cmd) => {
                    create_shim(
                        current_shell,
                        &cmd,
                        &get_shim_content(current_shell, &cmd, &cmd, None)?,
                    )?;
                }

                Executable::Command {
                    run,
                    alias,
                    use_symlink,
                } => {
                    if let Some(is_use_symlink) = use_symlink {
                        if is_use_symlink.to_owned() {
                            create_symlink(current_shell, run, alias)?;
                        }
                    } else {
                        if let Some(alias) = alias.as_ref() {
                            create_shim(
                                current_shell,
                                alias,
                                &get_shim_content(current_shell, run, alias, None)?,
                            )?;
                        } else {
                            create_shim(
                                current_shell,
                                run,
                                &get_shim_content(current_shell, run, run, None)?,
                            )?;
                        };
                    };
                }
            };
        };
    }

    // source scripts
    if let Some(src_target) = &payload.src {
        // set wd to payload config dir
        let current_install_dir = get_payload_current_install_dir_path(payload)?;
        assert!(std::env::set_current_dir(&current_install_dir).is_ok());

        src(current_shell, src_target)?;
    }

    // post load
    if let Some(load_cmd) = &payload.load {
        // set wd to payload config dir
        let current_install_dir = get_payload_current_install_dir_path(payload)?;
        assert!(std::env::set_current_dir(&current_install_dir).is_ok());

        load(current_shell, load_cmd)?;
    }

    Ok(())
}
