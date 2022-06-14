use crate::lib::paths::*;
use crate::lib::script::*;
use glob::glob;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

pub fn get_func_name(func: &str) -> Result<String, Box<dyn std::error::Error>> {
    let func_name = Path::new(&func)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    Ok(String::from(func_name))
}

pub fn get_shim(
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
    let func_name = Path::new(&func)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap();

    let globbed = glob(&bin_dir)?
        .next()
        .ok_or(format!("unable to locate {}", bin_dir))??;
    let mut resolved_bin_path = fs::canonicalize(&globbed).unwrap();
    // set exec mode
    run_cmd(&format!(
        "chmod +x {}",
        &resolved_bin_path.display().to_string()
    ))?;

    //
    resolved_bin_path.pop();
    let bin_dir = resolved_bin_path.to_str().unwrap();

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

pub fn persist_shim(cmd: &str, shim_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let shim_fname = get_func_name(&cmd)?;
    fs::create_dir_all(&get_bin_dir_path()?)?;
    let shim_path = get_bin_file_path(&shim_fname)?;
    let mut dest = File::create(&shim_path)?;
    io::copy(&mut shim_content.as_bytes(), &mut dest)?;

    // set shim mode
    run_cmd(&format!("chmod +x {}", &shim_path.display().to_string()))?;

    Ok(())
}
