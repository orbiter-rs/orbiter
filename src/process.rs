use crate::config::*;
use crate::download::*;
use crate::paths::*;
use crate::shimmer::*;
use std::fs;

pub fn process_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // check if already worked on
    let payload_orbiter_dir_path = get_payload_config_dir_path(payload)?;
    if !payload_orbiter_dir_path.exists() {
        fs::create_dir_all(&payload_orbiter_dir_path)?;
        // save file
        download_payload(payload)?;
        // change current dir
        let payload_dir = get_payload_dir_path(payload)?;
        assert!(std::env::set_current_dir(&payload_dir).is_ok());

        // create shim
        let shim = match &payload.exec {
            Executable::Run(cmd) => get_shim(&cmd, &cmd, None),
            Executable::Command { run, alias } => {
                Ok(format!("{} {}", run, alias.as_ref().unwrap()))
            }
        };

        // TODO: persist shim
    }

    Ok(())
}
