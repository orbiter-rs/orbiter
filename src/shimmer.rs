use glob::glob;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

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
    let globbed = glob(&bin_dir).unwrap().next().unwrap().unwrap();
    let mut resolved_bin_dir = fs::canonicalize(&globbed).unwrap();
    resolved_bin_dir.pop();

    Ok(format!(
        r##"
#!/bin/sh

{func}() {{
    local bindir="{bin_dir}"


    local -x PATH="$bindir":"$PATH"
    "$bindir"/"{func}" "$@"

}}

{func} "$@"
"##,
        func = func_name,
        bin_dir = resolved_bin_dir.to_str().unwrap()
    ))
}
