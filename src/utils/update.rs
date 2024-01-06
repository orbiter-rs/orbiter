use std::{env, fs::File, path::Path};

use time::{format_description, OffsetDateTime};

use crate::{providers::github::get_repo_release_asset_url, utils::config::Repo};

use super::{
    config::{Executable, Payload},
    httpclient::get_resource_name,
    paths::{get_payload_current_install_dir_path, get_payload_dir_path},
    shim::remove_shim,
};

const ARCHIVE_DIR_DATETIME_FORMAT: &str = "[year]-[month]-[day]_[hour]:[minute]:[second]";

pub fn update_payload(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    // 1. remove shim
    if let Some(exec) = &payload.exec {
        match exec {
            Executable::Run(cmd) => {
                remove_shim(cmd)?;
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
    Ok(std::fs::rename(
        get_payload_current_install_dir_path(payload)?,
        get_payload_dir_path(payload)?.join(format!(
            "archive_{}",
            OffsetDateTime::now_local()?
                .format(&format_description::parse(ARCHIVE_DIR_DATETIME_FORMAT)?)?
        )),
    )?)
}

fn update_orbiter_executable(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().timeout(None).build()?;
    let orbiter_repo = Repo {
        repo: "orbiter-rs/orbiter".to_string(),
        provider: None,
        ver: None,
        from_release: Some(true),
        binary_pattern: None,
    };
    let res = client
        .get(get_repo_release_asset_url(&orbiter_repo)?)
        .send()?;
    let mut dest = {
        let file_name = get_resource_name(&res)?;
        let file_path = dir.join(file_name);
        (File::create(&file_path)?, file_path)
    };

    let file_content = res.bytes()?;
    std::io::copy(&mut file_content.as_ref(), &mut dest.0)?;

    Ok(())
}

pub fn self_update() -> Result<(), Box<dyn std::error::Error>> {
    // 1. find where the orbiter executable is
    if let Ok(current_exe) = env::current_exe() {
        if let Some(dir) = current_exe.parent() {
            // 2. update the executable
            update_orbiter_executable(dir)?
        }
    };

    Ok(())
}
