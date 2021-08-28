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
            _ => {}
        }
    } else if let Some(ext) = &asset_path.extension() {
        println!("ext {:?}", ext);
        match ext.to_str().unwrap() {
            "dmg" => {
                run_cmd(&format!(
                    r#"
                    _extract_dmg() {{
                      local dmg_name="{}"
                      echo "dmg_name $dmg_name"

                      local attached_vol_info=$(eval "hdiutil attach $(realpath -m $dmg_name)" | tail -n1)
                      local attached_vol=$(echo $attached_vol_info | awk -F " " '{{print $1}}')
                      local attached_vol_mnt_pt=$(echo $attached_vol_info | awk -F " " '{{print $3}}')
                      cp -R $(realpath -m $attached_vol_mnt_pt)/ .
                      echo "attached vol $(realpath -m $attached_vol)"
                      eval "hdiutil detach \"$attached_vol\""
                    }}

                    _extract_dmg

                    "#,
                    asset_path_string
                ))?;
            }
            _ => {}
        }
    }

    Ok(())
}
