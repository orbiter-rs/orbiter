use regex::Regex;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::config::Repo;
use crate::utils::httpclient::get_binary_pattern_by_arch;
use crate::utils::httpclient::get_binary_pattern_by_os;

use super::Providers;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubReleaseAsset {
    name: String,
    content_type: String,
    browser_download_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub assets: Vec<GitHubReleaseAsset>,
}

pub fn get_repo_release_asset_url(repo: &Repo) -> Result<String, Box<dyn std::error::Error>> {
    let all_release_url = match Providers::from(&repo.provider) {
        Providers::GitHub => Url::parse(&format!(
            "https://api.github.com/repos/{}/releases",
            &repo.repo
        ))?,
        Providers::GitLab => Url::parse(&format!(
            "https://gitlab.com/api/v4/projects/{}/releases",
            &repo.repo
        ))?,
        Providers::Gitee => Url::parse(&format!(
            "https://gitee.com/api/v5/repos/{}/releases",
            &repo.repo
        ))?,
    };
    let client = reqwest::blocking::Client::builder().timeout(None).build()?;
    let res = client
        .get(all_release_url)
        .header("Accept", "*/*")
        .header("User-Agent", "orbiter")
        .send()?;

    let releases: Vec<GitHubRelease> = res.json()?;
    let release = if let Some(ver) = &repo.ver {
        releases
            .iter()
            .find(|release| release.tag_name.eq_ignore_ascii_case(ver))
            .unwrap()
    } else {
        releases
            .iter()
            .find(|release| !release.tag_name.contains("nightly"))
            .unwrap()
    };

    get_matched_asset_url(&repo.binary_pattern, release)
}

fn get_matched_asset_url(
    binary_pattern: &Option<String>,
    release: &GitHubRelease,
) -> Result<String, Box<dyn std::error::Error>> {
    let assets = release
        .assets
        .iter()
        .filter(|asset| !asset.name.contains("sha256"))
        .collect::<Vec<&GitHubReleaseAsset>>();

    let matched_assets = if let Some(binary_pattern) = binary_pattern {
        let re = Regex::new(binary_pattern)?;
        assets
            .into_iter()
            .filter(|asset| re.is_match(&asset.name))
            .collect::<Vec<&GitHubReleaseAsset>>()
    } else {
        let re_os = get_binary_pattern_by_os()?;
        let re_arch = get_binary_pattern_by_arch()?;
        let matched_assets = assets
            .clone()
            .into_iter()
            .filter(|asset| {
                re_os.is_match(&asset.name.to_lowercase())
                    || re_arch.is_match(&asset.name.to_lowercase())
            })
            .collect::<Vec<&GitHubReleaseAsset>>();

        if !matched_assets.is_empty() {
            matched_assets
        } else {
            assets.into_iter().collect()
        }
    };

    Ok(matched_assets
        .first()
        .unwrap()
        .browser_download_url
        .to_owned())
}
