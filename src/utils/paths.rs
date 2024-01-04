use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use super::config::*;
use super::shells::SupportedShell;

use glob::glob;

pub const DEFAULT_ORBITER_HOME: &str = ".orbiter";
pub const DEFAULT_ORBITER_PAYLOADS_HOME: &str = "payloads";
pub const DEFAULT_ORBITER_PAYLOADS_CURRENT_INSTALL_HOME: &str = "current";
pub const DEFAULT_ORBITER_DASHBOARD_HOME: &str = "dashboard";
pub const DEFAULT_ORBITER_DASHBOARD_BIN_HOME: &str = "bin";
pub const DEFAULT_ORBITER_CONFIG_FILENAME: &str = ".orbiter.config.yml";
pub const DEFAULT_ORBITER_PAYLOAD_CONFIG_DIR: &str = ".__orbiter__";

pub const ORBITER_CONFIG_ENV_KEY: &str = "ORBITER_CONFIG";
pub const ORBITER_HOME_ENV_KEY: &str = "ORBITER_HOME";

// .orbiter.config.yml
fn get_dflt_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap()
        .join(DEFAULT_ORBITER_CONFIG_FILENAME)
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(match env::var(ORBITER_CONFIG_ENV_KEY) {
        Ok(orbiter_config_envvar) => {
            let path = Path::new(&orbiter_config_envvar);
            if path.exists() {
                path.to_path_buf()
            } else {
                get_dflt_config_path()
            }
        }
        Err(_) => get_dflt_config_path(),
    })
}

// .orbiter/

fn get_dflt_home_dir_path() -> PathBuf {
    dirs::home_dir().unwrap().join(DEFAULT_ORBITER_HOME)
}

pub fn get_home_dir_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(match env::var(ORBITER_HOME_ENV_KEY) {
        Ok(orbiter_config_envvar) => {
            let path = Path::new(&orbiter_config_envvar);
            if path.exists() {
                path.to_path_buf()
            } else {
                get_dflt_home_dir_path()
            }
        }
        Err(_) => get_dflt_home_dir_path(),
    })
}

// .orbiter/payloads/<payload id>
pub fn get_payload_dir_path(payload: &Payload) -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_home_dir_path()?
        .join(DEFAULT_ORBITER_PAYLOADS_HOME)
        .join(&payload.id))
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
    let bin_path = get_home_dir_path()?
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

pub fn update_path(current_shell: &SupportedShell) {
    match current_shell {
        SupportedShell::Sh => export_path_sh(),
        SupportedShell::Bash => export_path_sh(),
        SupportedShell::Zsh => export_path_sh(),
        SupportedShell::Fish => export_path_fish(),
        SupportedShell::PowerShell => export_path_powershell(),
        SupportedShell::WinCmd => export_path_wincmd(),
    }
}

fn export_path_sh() {
    // update PATH with orbiter dashboard bin dir
    println!(
        "export PATH=\"{}:$PATH\"",
        get_bin_dir_path().unwrap().display()
    );
}

fn export_path_fish() {
    // update PATH with orbiter dashboard bin dir
    println!(
        "set -x PATH \"{}\" $PATH",
        get_bin_dir_path().unwrap().display()
    );
}

fn export_path_powershell() {
    // update PATH with orbiter dashboard bin dir
    println!(
        "$env:PATH = \"{};$env:PATH\"",
        get_bin_dir_path().unwrap().display()
    );
}

fn export_path_wincmd() {
    // update PATH with orbiter dashboard bin dir
    println!(
        "setx PATH \"{};%PATH%\"",
        get_bin_dir_path().unwrap().display()
    );
}
