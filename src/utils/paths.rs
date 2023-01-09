use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use super::config::*;

use glob::glob;

pub const DEFAULT_ORBITER_CONFIG_HOME: &str = ".orbiter";
pub const DEFAULT_ORBITER_PAYLOADS_HOME: &str = "payloads";
pub const DEFAULT_ORBITER_PAYLOADS_CURRENT_INSTALL_HOME: &str = "current";
pub const DEFAULT_ORBITER_DASHBOARD_HOME: &str = "dashboard";
pub const DEFAULT_ORBITER_DASHBOARD_BIN_HOME: &str = "bin";
pub const DEFAULT_ORBITER_CONFIG_FILENAME: &str = ".orbiter.config.yml";
pub const DEFAULT_ORBITER_PAYLOAD_CONFIG_DIR: &str = ".__orbiter__";

// .orbiter.config.yml
pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir();
    let config_file_path = home_dir.unwrap().join(DEFAULT_ORBITER_CONFIG_FILENAME);

    Ok(config_file_path)
}

// .orbiter
pub fn get_config_home_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir();
    let config_home = home_dir.unwrap().join(DEFAULT_ORBITER_CONFIG_HOME);

    Ok(config_home)
}

// .orbiter/payloads/<payload id>
pub fn get_payload_dir_path(payload: &Payload) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let payload_dir = get_config_home_path()?
        .join(DEFAULT_ORBITER_PAYLOADS_HOME)
        .join(payload.id.as_ref().unwrap());

    Ok(payload_dir)
}

// .orbiter/payloads/<payload id>/current
pub fn get_payload_current_install_dir_path(
    payload: &Payload,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = get_payload_dir_path(payload)?.join(DEFAULT_ORBITER_PAYLOADS_CURRENT_INSTALL_HOME);

    Ok(dir)
}

// .orbiter/payloads/<payload id>/.__orbiter__
pub fn get_payload_config_dir_path(
    payload: &Payload,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let payload_config_dir =
        get_payload_dir_path(payload)?.join(DEFAULT_ORBITER_PAYLOAD_CONFIG_DIR);

    Ok(payload_config_dir)
}

pub fn get_bin_dir_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bin_path = get_config_home_path()?
        .join(DEFAULT_ORBITER_DASHBOARD_HOME)
        .join(DEFAULT_ORBITER_DASHBOARD_BIN_HOME);

    Ok(bin_path)
}

pub fn get_bin_file_path(bin_fname: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bin_file_path = get_bin_dir_path()?.join(bin_fname);

    Ok(bin_file_path)
}

pub fn resolve_single_path(file_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if file_path.contains("*") {
        let entry = glob(&file_path)?
            .next()
            .ok_or(format!("unable to locate {}", file_path))??;

        return Ok(fs::canonicalize(&entry)?);
    }

    Ok(fs::canonicalize(file_path)?)
}

pub fn get_file_name(func: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file_name = Path::new(&func)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    Ok(String::from(file_name))
}

pub fn get_dir(bin_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut bin_path = bin_path.clone();
    bin_path.pop(); // get parent dir path

    Ok(bin_path)
}
