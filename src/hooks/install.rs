use crate::lib::script::*;

pub fn install(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_cmd(&cmd)?;

    Ok(())
}
