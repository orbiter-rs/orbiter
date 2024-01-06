use crate::providers::github::get_repo_release_asset_url;
use crate::providers::Providers;
use crate::utils::config::*;
use crate::utils::httpclient::get_resource_name;
use crate::utils::httpclient::get_resource_name_from_url;
use crate::utils::paths::*;
use crate::utils::script::*;

use log::error;
use reqwest::Url;
use reqwest::{self};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::{rename, File};
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn move_resource_to_current_dir(
    resource_path: &Path,
    payload_current_install_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let resource_name = Path::new(&resource_path)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    fs::create_dir_all(&payload_current_install_dir)?;

    let dest = payload_current_install_dir.join(&resource_name);

    // move file/dir to dest
    rename(&resource_path, &dest)?;

    Ok(dest)
}

fn set_resource_as_current(
    resource_path: &Path,
    payload_current_install_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let dest = payload_current_install_dir;

    // move file/dir to dest
    rename(&resource_path, &dest)?;

    Ok(())
}

fn clone_repo(
    payload_config_dir: &Path,
    repo: &Repo,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = match Providers::from(&repo.provider) {
        Providers::GitHub => Url::parse(&format!("https://github.com/{}", &repo.repo))?,
        Providers::GitLab => Url::parse(&format!("https://gitlab.com/{}", &repo.repo))?,
        Providers::Gitee => Url::parse(&format!("https://gitee.com/{}", &repo.repo))?,
    };

    assert!(std::env::set_current_dir(&payload_config_dir).is_ok());

    git_cmd(&["clone", url.as_ref()])?;

    get_resource_name_from_url(&url)
}

fn get_asset(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    url: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().timeout(None).build()?;
    let res = client.get(url).send()?;
    let mut dest = {
        let file_name = get_resource_name(&res)?;
        let file_path = payload_config_dir.join(&file_name);
        (File::create(&file_path)?, file_path)
    };

    let file_content = res.bytes()?;
    io::copy(&mut file_content.as_ref(), &mut dest.0)?;

    let dest_path = move_resource_to_current_dir(&dest.1, &current_install_dir)?;

    Ok(dest_path)
}

fn clone_and_checkout_repo(
    repo: &Repo,
    payload_config_dir: &Path,
    current_install_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo_name = clone_repo(payload_config_dir, repo)?;
    let resource_path = payload_config_dir.join(&repo_name);
    set_resource_as_current(&resource_path, &current_install_dir)?;

    // checkout branch/tag
    if let Some(ver) = &repo.ver {
        assert!(std::env::set_current_dir(&current_install_dir).is_ok());
        git_cmd(&["checkout", "-q", &ver])?;
    };

    Ok(())
}

fn get_resource_repo(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    repo: &Repo,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let asset_path = if let Some(from_release) = repo.from_release {
        if from_release {
            // repo release
            let url = get_repo_release_asset_url(&repo)?;
            Some(get_asset(&payload_config_dir, &current_install_dir, &url)?)
        } else {
            clone_and_checkout_repo(repo, payload_config_dir, current_install_dir)?;
            None
        }
    } else {
        clone_and_checkout_repo(repo, payload_config_dir, current_install_dir)?;
        None
    };

    Ok(asset_path)
}

fn get_resource_location(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    url: &str,
    init_result: Option<&str>,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let asset_path = if let Some(init) = init_result {
        let location = url.replace("{init}", init);
        Some(get_asset(
            &payload_config_dir,
            &current_install_dir,
            &location,
        )?)
    } else {
        Some(get_asset(&payload_config_dir, &current_install_dir, &url)?)
    };

    Ok(asset_path)
}

pub fn get_resource(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    resource: &Resource,
    init_result: Option<&str>,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    match &resource {
        Resource::Repo(repo) => get_resource_repo(payload_config_dir, current_install_dir, repo),
        Resource::Location(url) => {
            get_resource_location(payload_config_dir, current_install_dir, url, init_result)
        }
    }
}

fn get_os_specific_resource(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    init_result: Option<&str>,
    resource: &SupportedOSSpecificResource,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;
    let supported_os_specific_resource = match os {
        "linux" => &resource.linux,
        "macos" => &resource.macos,
        "windows" => &resource.windows,
        _ => {
            error!("unsupported os: {}", os);

            &None
        }
    };

    let resource_path = if let Some(os_specific_resource) = supported_os_specific_resource {
        match os_specific_resource {
            OSSpecificResource::Standard(res) => {
                get_resource(payload_config_dir, current_install_dir, res, init_result)?
            }
            OSSpecificResource::ArchSpecific(res) => get_arch_specific_resource(
                &payload_config_dir,
                &current_install_dir,
                init_result,
                res,
            )?,
        }
    } else {
        None
    };

    Ok(resource_path)
}

fn get_arch_specific_resource(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    init_result: Option<&str>,
    resource: &SupportedArchSpecificResource,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let machine_arch = env::consts::ARCH;
    let supported_arch_specific_resource = match machine_arch.as_ref() {
        "x86_64" | "amd64" => &resource.x86_64,
        "aarch64" | "arm64" => &resource.aarch64,
        _ => {
            error!("Unsupported architecture: {}", machine_arch);
            &None
        }
    };

    let resource_path = if let Some(res) = supported_arch_specific_resource {
        get_resource(payload_config_dir, current_install_dir, res, init_result)?
    } else {
        None
    };

    Ok(resource_path)
}

pub fn get_adaptive_resource(
    payload: &Payload,
    init_result: Option<&str>,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let payload_config_dir = get_payload_config_dir_path(&payload)?;
    let current_install_dir = get_payload_current_install_dir_path(&payload)?;

    let asset_path = match &payload.resource {
        AdaptiveResource::Standard(resource) => match resource {
            Resource::Repo(repo) => {
                get_resource_repo(&payload_config_dir, &current_install_dir, repo)?
            }
            Resource::Location(url) => {
                get_resource_location(&payload_config_dir, &current_install_dir, url, init_result)?
            }
        },
        AdaptiveResource::OSSpecific(os_specific_resource) => get_os_specific_resource(
            &payload_config_dir,
            &current_install_dir,
            init_result,
            os_specific_resource,
        )?,
    };

    Ok(asset_path)
}
