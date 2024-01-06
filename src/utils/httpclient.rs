use regex::Regex;

use reqwest::{header::CONTENT_DISPOSITION, Url};

pub fn get_resource_name(
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

pub fn get_resource_name_from_url(url: &Url) -> Result<String, Box<dyn std::error::Error>> {
    let resource_name = url
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    Ok(resource_name.to_string())
}

pub fn get_binary_pattern_by_os() -> Result<regex::Regex, Box<dyn std::error::Error>> {
    let os = std::env::consts::OS;
    Ok(match os {
        "linux" => Regex::new(r"(linux|linux-gnu)")?,
        "macos" => Regex::new(r"(darwin|mac|macos|osx|os-x)")?,
        "windows" => Regex::new(r"(windows|cygwin|[-_]win|win64|win32)")?,
        _ => panic!("Unsupported os: {}", os),
    })
}

pub fn get_binary_pattern_by_arch() -> Result<regex::Regex, Box<dyn std::error::Error>> {
    let machine_arch = std::env::consts::ARCH;
    Ok(match machine_arch {
        "x86_64" | "amd64" => Regex::new(r"(x86_64|amd64|intel|linux64)")?,
        "arm64" | "aarch64" => Regex::new(r"(arm64|aarch64)")?,
        _ => panic!("Unsupported architecture: {}", machine_arch),
    })
}
