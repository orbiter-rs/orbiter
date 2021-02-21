use std::fs;
use std::fs::File;
use std::io;

use crate::hooks::install::*;
use crate::hooks::resource::*;
use crate::lib::config::*;
use crate::lib::paths::*;
use crate::lib::shimmer::*;

fn persist_shim(cmd: &str, shim_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let shim_fname = get_func_name(&cmd)?;
    let shim_path = get_bin_file_path(&shim_fname)?;
    let mut dest = File::create(shim_path)?;
    io::copy(&mut shim_content.as_bytes(), &mut dest)?;

    Ok(())
}

pub fn process_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // check if already worked on
    let payload_orbiter_dir_path = get_payload_config_dir_path(payload)?;
    if !payload_orbiter_dir_path.exists() {
        fs::create_dir_all(&payload_orbiter_dir_path)?;
        // save file
        get_resource(payload)?;

        // TODO: extract resource

        // change current dir
        let payload_dir = get_payload_dir_path(payload)?;
        assert!(std::env::set_current_dir(&payload_dir).is_ok());

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
