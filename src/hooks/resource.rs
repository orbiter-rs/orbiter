use crate::lib::config::*;
use crate::lib::paths::*;
use crate::lib::script::*;
use regex::Regex;
use reqwest::header::CONTENT_DISPOSITION;
use reqwest::Url;
use reqwest::{self, Response};
use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs;
use std::fs::{rename, File};
use std::io;
use std::path::Path;

const DEFAULT_PROVIDER: &str = "github";

fn move_resource_to_current_dir(
    resource_path: &Path,
    payload_current_install_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let resource_name = Path::new(&resource_path)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    fs::create_dir_all(&payload_current_install_dir)?;

    let dest = payload_current_install_dir.join(&resource_name);

    // move file/dir to dest
    rename(&resource_path, &dest)?;

    Ok(())
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

fn get_resource_name(
    response: &reqwest::blocking::Response,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("response: [{:?}]", &response);

    let resource_name =
        if let Some(content_disposition) = response.headers().get(CONTENT_DISPOSITION) {
            // try response content-disposition header
            let re = Regex::new(r"filename=(.*$)")?;
            let caps = re.captures(content_disposition.to_str()?).unwrap();
            caps.get(1).map_or("", |m| m.as_str()).to_string()
        } else {
            // if CDN get resource name from url
            get_resource_name_from_url(response.url())?
        };

    Ok(resource_name)
}

fn get_resource_name_from_url(url: &Url) -> Result<String, Box<dyn std::error::Error>> {
    let resource_name = url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    Ok(resource_name.to_string())
}

fn get_provider(repo_provider_type: &Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let provider = if let Some(provider_type) = &repo_provider_type {
        String::from(provider_type)
    } else {
        String::from(DEFAULT_PROVIDER)
    };

    Ok(provider)
}

fn clone_repo(
    payload_config_dir: &Path,
    repo: &Repo,
) -> Result<String, Box<dyn std::error::Error>> {
    let provider = get_provider(&repo.provider)?;

    let url = match provider.as_ref() {
        "github" => Url::parse(&format!("https://github.com/{}", &repo.repo))?,
        _ => Url::parse(&format!("https://github.com/{}", &repo.repo))?,
    };

    assert!(std::env::set_current_dir(&payload_config_dir).is_ok());
    run_cmd(&format!("git clone {}", &url))?;

    get_resource_name_from_url(&url)
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubReleaseAsset {
    name: String,
    content_type: String,
    browser_download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubReleaseAsset>,
}

fn get_binary_pattern_by_arch() -> Result<regex::Regex, Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;
    let os_regex = match os {
        "linux" => Regex::new(r"^\d{4}-\d{2}-\d{2}$")?,
        "macos" => Regex::new(r"^\d{4}-\d{2}-\d{2}$")?,
        "windows" => Regex::new(r"^\d{4}-\d{2}-\d{2}$")?,
        _ => todo!(),
    };

    Ok(os_regex)
}

fn match_file_ext_by_arch(file_name: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let re = get_binary_pattern_by_arch()?;

    Ok(re.is_match(&file_name))
}

fn get_repo_release_asset_url(repo: &Repo) -> Result<String, Box<dyn std::error::Error>> {
    let provider = get_provider(&repo.provider)?;

    let first_release_url = match provider.as_ref() {
        "github" => Url::parse(&format!(
            "https://api.github.com/repos/{}/releases",
            &repo.repo
        ))?,
        _ => Url::parse(&format!(
            "https://api.github.com/repos/{}/releases",
            &repo.repo
        ))?,
    }
    .to_string();

    let client = reqwest::blocking::Client::builder().timeout(None).build()?;
    let res = client
        .get(&first_release_url)
        .header("Accept", "*/*")
        .header("User-Agent", "orbiter/0.1.0")
        .send()?;
    let release: Vec<GitHubRelease> = res.json()?;
    let assets = &release.first().unwrap().assets;

    let os_matched_assets: Vec<&GitHubReleaseAsset> = assets
        .into_iter()
        .filter(|asset| match_file_ext_by_arch(&asset.name).is_ok())
        .collect();

    Ok(os_matched_assets
        .first()
        .unwrap()
        .browser_download_url
        .clone())
}

fn get_asset(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().timeout(None).build()?;
    let res = client.get(url).send()?;
    let mut dest = {
        let file_name = get_resource_name(&res)?;

        println!("file to download: '{}'", file_name);
        let file_path = payload_config_dir.join(&file_name);
        println!("will be located under: '{:?}'", &file_path);
        (File::create(&file_path)?, file_path)
    };

    let file_content = res.bytes()?;
    io::copy(&mut file_content.as_ref(), &mut dest.0)?;

    move_resource_to_current_dir(&dest.1, &current_install_dir)?;

    Ok(())
}

pub fn get_resource(payload: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    let payload_config_dir = get_payload_config_dir_path(&payload)?;
    let current_install_dir = get_payload_current_install_dir_path(&payload)?;

    match &payload.resource {
        Resource::Repo(repo) => {
            if let Some(is_release) = repo.is_release {
                if is_release {
                    // repo release
                    let url = get_repo_release_asset_url(&repo)?;
                    println!("url: [{:?}]", &url);
                    get_asset(&payload_config_dir, &current_install_dir, &url)?;
                } else {
                    let repo_name = clone_repo(&&payload_config_dir, &repo)?;
                    let resource_path = payload_config_dir.join(&repo_name);
                    set_resource_as_current(&resource_path, &current_install_dir)?;

                    // checkout branch/tag
                    if let Some(ver) = &repo.ver {
                        assert!(std::env::set_current_dir(&current_install_dir).is_ok());
                        run_cmd(&format!("git checkout {}", &ver))?;
                    };
                };
            }
        }
        Resource::Location(url) => {
            get_asset(&payload_config_dir, &current_install_dir, &url)?;
        }
    };

    Ok(())
}
