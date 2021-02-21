use std::path::PathBuf;

use crate::lib::config::*;

pub const DEFAULT_ORBITER_CONFIG_HOME: &str = ".orbiter";
pub const DEFAULT_ORBITER_PAYLOADS_HOME: &str = "payloads";
pub const DEFAULT_ORBITER_BIN_HOME: &str = "bin";
pub const DEFAULT_ORBITER_CONFIG_FILENAME: &str = ".orbiter.config.yml";
pub const DEFAULT_ORBITER_PAYLOAD_CONFIG_DIR: &str = ".__orbiter__";

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir();
    let config_file_path = home_dir.unwrap().join(DEFAULT_ORBITER_CONFIG_FILENAME);

    Ok(config_file_path)
}

pub fn get_config_home_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir();
    let config_home = home_dir.unwrap().join(DEFAULT_ORBITER_CONFIG_HOME);

    Ok(config_home)
}

pub fn get_payload_dir_path(payload: &Payload) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let payload_dir = get_config_home_path()?
        .join(DEFAULT_ORBITER_PAYLOADS_HOME)
        .join(payload.id.as_ref().unwrap());

    Ok(payload_dir)
}

pub fn get_payload_config_dir_path(
    payload: &Payload,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let payload_config_dir =
        get_payload_dir_path(payload)?.join(DEFAULT_ORBITER_PAYLOAD_CONFIG_DIR);

    Ok(payload_config_dir)
}

pub fn get_bin_dir_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bin_path = get_config_home_path()?.join(DEFAULT_ORBITER_BIN_HOME);

    Ok(bin_path)
}

pub fn get_bin_file_path(bin_fname: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bin_file_path = get_bin_dir_path()?.join(bin_fname);

    Ok(bin_file_path)
}
