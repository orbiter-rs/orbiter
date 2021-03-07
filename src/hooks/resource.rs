use crate::lib::config::*;
use crate::lib::paths::*;
use crate::lib::script::*;
use reqwest;
use reqwest::Url;
use std::ffi::OsStr;
use std::fs::{rename, File};
use std::io;
use std::path::{Path, PathBuf};

const DEFAULT_PROVIDER: &str = "github";

fn move_resource_to_current_dir(
    payload_current_install_dir: &Path,
    resource_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let resource_name = Path::new(&resource_path)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    let dest = payload_current_install_dir.join(&resource_name);

    rename(&resource_path, &dest)?;

    Ok(())
}

fn get_url_resource_name(url: &Url) -> Result<String, Box<dyn std::error::Error>> {
    let resource_name = url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    Ok(resource_name.to_string())
}

fn clone_from_github(repo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("https://github.com/{}", repo))?;

    run_cmd(&format!("git clone {}", &url))?;
    get_url_resource_name(&url)
}

pub fn get_resource(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    let payload_config_dir = get_payload_config_dir_path(&payload)?;

    let resource_path = match &payload.resource {
        Resource::RepoRelease(rel) => todo!(),
        Resource::Repo(repo) => {
            let provider = if let Some(provider) = &repo.provider {
                &provider
            } else {
                DEFAULT_PROVIDER
            };

            assert!(std::env::set_current_dir(&payload_config_dir).is_ok());

            let repo_name = match provider {
                "github" => clone_from_github(&repo.repo)?,
                _ => clone_from_github(&repo.repo)?,
            };

            payload_config_dir.join(&repo_name)
        }
        Resource::Location(url) => {
            let res = reqwest::blocking::get(url)?;
            let mut dest = {
                let file_name = get_url_resource_name(res.url())?;

                println!("file to download: '{}'", file_name);
                let file_path = payload_config_dir.join(&file_name);
                println!("will be located under: '{:?}'", &file_path);
                (File::create(&file_path)?, file_path)
            };

            let file_content = res.bytes()?;
            io::copy(&mut file_content.as_ref(), &mut dest.0)?;

            dest.1
        }
    };

    let current_install_dir = get_payload_current_install_dir_path(&payload)?;
    move_resource_to_current_dir(&resource_path, &current_install_dir)?;

    Ok(())
}
