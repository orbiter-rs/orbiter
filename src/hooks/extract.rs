use infer;
use std::path::Path;

use crate::lib::script::*;

pub fn extract(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    run_cmd(&cmd)?;

    Ok(())
}

pub fn extract_asset(asset_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let asset_path_string = &asset_path.display().to_string();
    let infer_kind = infer::get_from_path(asset_path)?;
    if let Some(kind) = &infer_kind {
        match kind.extension() {
            "zip" => {
                run_cmd(&format!("unzip {}", asset_path_string))?;
            }
            "gz" => {
                run_cmd(&format!("tar xvf {}", asset_path_string))?;
            }
            "deb" => {
                run_cmd(&format!(
                    "ar xv {}; ls *gz | xargs -n 1 tar xvf",
                    asset_path_string
                ))?;
            }
            _ => {
                println!(
                    "unsupported archive ext={} path={}",
                    kind.extension(),
                    asset_path_string
                );
            }
        }
    }

    Ok(())
}
