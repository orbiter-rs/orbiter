use crate::lib::config::*;
use crate::lib::paths::*;
use crate::lib::script::*;
use regex::Regex;
use reqwest::header::CONTENT_DISPOSITION;
use reqwest::Url;
use reqwest::{self};
use serde::Deserialize;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs;
use std::fs::{rename, File};
use std::io;
use std::path::Path;
use std::path::PathBuf;
use uname::uname;

const DEFAULT_PROVIDER: &str = "github";

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

fn get_resource_name(
    response: &reqwest::blocking::Response,
) -> Result<String, Box<dyn std::error::Error>> {
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

fn get_binary_pattern_by_os() -> Result<regex::Regex, Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;
    let os_regex = match os {
        "linux" => Regex::new(r"(linux|linux-gnu)")?,
        "macos" => Regex::new(r"(darwin|mac|osx|os-x)")?,
        "windows" => Regex::new(r"(windows|cygwin|[-_]win|win64|win32)")?,
        _ => todo!(),
    };

    Ok(os_regex)
}

fn get_binary_pattern_by_arch() -> Result<regex::Regex, Box<dyn std::error::Error>> {
    let machine_arch = uname().unwrap().machine;
    let arch_regex = match machine_arch.as_ref() {
        "x86_64" => Regex::new(r"(x86_64|amd64|intel|linux64)")?,
        "amd64" => Regex::new(r"(x86_64|amd64|intel|linux64)")?,
        _ => todo!(),
    };

    Ok(arch_regex)
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

    let matched_assets = if let Some(binary_pattern) = &repo.binary_pattern {
        let re = Regex::new(&binary_pattern)?;

        let binary_pattern_matched_assets: Vec<&GitHubReleaseAsset> = assets
            .into_iter()
            .filter(|asset| re.is_match(&asset.name))
            .collect();

        binary_pattern_matched_assets
    } else {
        let re_os = get_binary_pattern_by_os()?;
        let os_matched_assets: Vec<&GitHubReleaseAsset> = assets
            .into_iter()
            .filter(|asset| re_os.is_match(&asset.name))
            .collect();

        let re_arch = get_binary_pattern_by_arch()?;
        let os_arch_matched_assets: Vec<&GitHubReleaseAsset> = os_matched_assets
            .clone()
            .into_iter()
            .filter(|asset| re_arch.is_match(&asset.name))
            .collect();

        if os_arch_matched_assets.len() > 0usize {
            os_arch_matched_assets
        } else if os_matched_assets.len() > 0usize {
            os_matched_assets
        } else {
            assets.into_iter().collect()
        }
    };

    Ok(matched_assets.first().unwrap().browser_download_url.clone())
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
    let repo_name = clone_repo(&payload_config_dir, &repo)?;
    let resource_path = payload_config_dir.join(&repo_name);
    set_resource_as_current(&resource_path, &current_install_dir)?;

    // checkout branch/tag
    if let Some(ver) = &repo.ver {
        assert!(std::env::set_current_dir(&current_install_dir).is_ok());
        run_cmd(&format!("git checkout -q {}", &ver))?;
    };

    Ok(())
}

fn get_resource_repo(
    payload_config_dir: &Path,
    current_install_dir: &Path,
    repo: &Repo,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let asset_path = if let Some(is_release) = repo.is_release {
        if is_release {
            // repo release
            let url = get_repo_release_asset_url(&repo)?;
            Some(get_asset(&payload_config_dir, &current_install_dir, &url)?)
        } else {
            clone_and_checkout_repo(&repo, &payload_config_dir, &current_install_dir)?;
            None
        }
    } else {
        clone_and_checkout_repo(&repo, &payload_config_dir, &current_install_dir)?;
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
        Resource::Repo(repo) => get_resource_repo(&payload_config_dir, &current_install_dir, repo),
        Resource::Location(url) => {
            get_resource_location(&payload_config_dir, &current_install_dir, &url, init_result)
        }
    }
}

pub fn get_adaptive_resource(
    payload: &Payload,
    init_result: Option<&str>,
) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
    let payload_config_dir = get_payload_config_dir_path(&payload)?;
    let current_install_dir = get_payload_current_install_dir_path(&payload)?;

    let asset_path = match &payload.resource {
        AdaptiveResource::Repo(repo) => {
            get_resource_repo(&payload_config_dir, &current_install_dir, repo)?
        }
        AdaptiveResource::Location(url) => {
            get_resource_location(&payload_config_dir, &current_install_dir, url, init_result)?
        }
        AdaptiveResource::OSSpecific {
            linux,
            macos,
            windows,
        } => {
            let os = std::env::consts::OS;
            match os {
                "linux" => {
                    if let Some(resource) = &linux {
                        get_resource(
                            &payload_config_dir,
                            &current_install_dir,
                            &resource,
                            init_result,
                        )?
                    } else {
                        None
                    }
                }
                "macos" => {
                    if let Some(resource) = &macos {
                        get_resource(
                            &payload_config_dir,
                            &current_install_dir,
                            &resource,
                            init_result,
                        )?
                    } else {
                        None
                    }
                }
                "windows" => {
                    if let Some(resource) = &windows {
                        get_resource(
                            &payload_config_dir,
                            &current_install_dir,
                            &resource,
                            init_result,
                        )?
                    } else {
                        None
                    }
                }
                _ => {
                    println!("unsupported os os={}", os);

                    None
                }
            }
        }
    };

    Ok(asset_path)
}
