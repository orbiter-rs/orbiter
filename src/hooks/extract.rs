use crate::lib::script::*;

pub fn extract(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_cmd(&cmd)
}
