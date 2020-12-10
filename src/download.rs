use reqwest;
use std::fs::File;
use std::io;

// mod config;
use crate::config::*;
use crate::paths::*;

pub fn download_payload(item: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    let payload_config_dir = get_payload_config_dir_path(item)?;
    let file_path = payload_config_dir.join("download_file.tar.gz");

    match &item.download {
        Download::Repo(_) => todo!(),
        Download::RepoRelease(_) => todo!(),
        Download::Location(url) => {
            let resp = reqwest::blocking::get(url)?.bytes()?;
            let mut resp = resp.as_ref();
            let mut out = File::create(file_path)?;
            io::copy(&mut resp, &mut out)?;
        }
    }

    Ok(())
}
