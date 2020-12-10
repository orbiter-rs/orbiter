use std::path::PathBuf;

use crate::config::*;

pub const DEFAULT_ORBITER_CONFIG_HOME: &str = ".orbiter";
pub const DEFAULT_ORBITER_PAYLOADS_HOME: &str = "payloads";
pub const DEFAULT_ORBITER_CONFIG_FILENAME: &str = ".orbiter.config.yml";
pub const DEFAULT_ORBITER_PAYLOAD_CONFIG_DIR: &str = ".__orbiter__";

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir();

    let logic_config_path: PathBuf = [
        home_dir.as_ref().unwrap().to_str().unwrap(),
        DEFAULT_ORBITER_CONFIG_FILENAME,
    ]
    .iter()
    .collect();

    Ok(PathBuf::from(&logic_config_path))
}

pub fn get_payload_dir_path(payload: &Payload) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir();

    // TODO: consider path to config home from config

    let logic_dir_path: PathBuf = [
        home_dir.as_ref().unwrap().to_str().unwrap(),
        DEFAULT_ORBITER_CONFIG_HOME,
        DEFAULT_ORBITER_PAYLOADS_HOME,
        payload.id.as_ref().unwrap(),
    ]
    .iter()
    .collect();

    Ok(PathBuf::from(&logic_dir_path))
}

pub fn get_payload_config_dir_path(
    payload: &Payload,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let payload_dir = get_payload_dir_path(payload)?;
    Ok(payload_dir.join(DEFAULT_ORBITER_PAYLOAD_CONFIG_DIR))
}
