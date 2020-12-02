use reqwest;
use std::fs::File;
use std::io;

// mod config;
use crate::config::*;
use crate::paths::*;

pub fn download_file(item: &InitItem) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_path = get_item_dir_path(item);
    file_path.push("download_file.tar.gz");

    match &item.download {
        Download::Release(_) => todo!(),
        Download::Location(url) => {
            let resp = reqwest::blocking::get(url)?.bytes()?;
            let mut resp = resp.as_ref();
            let mut out = File::create(file_path)?;
            io::copy(&mut resp, &mut out)?;
        }
    }

    Ok(())
}
