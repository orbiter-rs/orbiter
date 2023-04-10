use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use crate::utils::paths::*;
use crate::utils::script::*;

pub fn get_func_name(func: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(get_file_name(func)?)
}

pub fn get_shim_content(
    func: &str,
    bin_dir: &str,
    env: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    match env {
        None => get_basic_shim(func, bin_dir),
        Some("base") => get_basic_shim(func, bin_dir),
        Some(&_) => get_basic_shim(func, bin_dir),
    }
}

pub fn get_basic_shim(func: &str, bin_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    let func_name = get_func_name(func)?;
    let resolved_bin_path = resolve_single_path(bin_dir)?;

    // set exec mode
    run_cmd(&format!(
        "chmod +x {}",
        &resolved_bin_path.display().to_string()
    ))?;

    let bin_dir = get_dir(&resolved_bin_path)?.display().to_string();

    Ok(format!(
        r##"
#!/bin/sh

{internal_func}() {{
    local bindir="{bin_dir}"


    local PATH="$bindir":"$PATH"
    "$bindir"/"{func}" "$@"

}}

{internal_func} "$@"
"##,
        func = func_name,
        internal_func = func_name.replace("-", "_"),
        bin_dir = bin_dir
    ))
}

fn get_shim_path(cmd: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_bin_file_path(&get_func_name(&cmd)?)?)
}

pub fn create_shim(cmd: &str, shim_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(&get_bin_dir_path()?)?;
    let shim_path = get_shim_path(&cmd)?;
    let mut dest = File::create(&shim_path)?;
    io::copy(&mut shim_content.as_bytes(), &mut dest)?;

    // set shim mode
    run_cmd(&format!("chmod +x {}", &shim_path.display().to_string()))?;

    Ok(())
}

pub fn remove_shim(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(fs::remove_file(get_shim_path(&cmd)?)?)
}
