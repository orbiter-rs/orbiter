use glob::glob;
use std::fs;

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
    let resolved_func = fs::canonicalize(glob(&func).unwrap().next().unwrap().unwrap()).unwrap();
    let resolved_bin_dir =
        fs::canonicalize(glob(&bin_dir).unwrap().next().unwrap().unwrap()).unwrap();

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
        func = resolved_func.to_str().unwrap(),
        bin_dir = resolved_bin_dir.to_str().unwrap()
    ))
}
