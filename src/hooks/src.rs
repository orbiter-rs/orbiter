use std::fs;

pub fn src(files: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for f in files {
        if let Ok(f_path) = fs::canonicalize(&f) {
            let cmd = format!(". {}", &f_path.display().to_string());
            println!("{}", &cmd);
        };
    }

    Ok(())
}
