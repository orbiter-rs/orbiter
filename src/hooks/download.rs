use crate::config::*;
use crate::paths::*;
use reqwest;
use std::fs::File;
use std::io;

pub fn download_payload(item: &Payload) -> Result<(), Box<dyn std::error::Error>> {
    let payload_config_dir = get_payload_config_dir_path(item)?;

    match &item.download {
        Download::Repo(_) => todo!(),
        Download::RepoRelease(_) => todo!(),
        Download::Location(url) => {
            let res = reqwest::blocking::get(url)?;
            let mut dest = {
                let fname = res
                    .url()
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .and_then(|name| if name.is_empty() { None } else { Some(name) })
                    .unwrap_or("tmp.bin");

                println!("file to download: '{}'", fname);
                let fname = payload_config_dir.join(fname);
                println!("will be located under: '{:?}'", fname);
                File::create(fname)?
            };

            let file_content = res.bytes()?;
            io::copy(&mut file_content.as_ref(), &mut dest)?;
        }
    };

    Ok(())
}
