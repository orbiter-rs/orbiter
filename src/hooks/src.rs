use std::fs;

pub fn src(files: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for f in files {
        let f_path = fs::canonicalize(&f).unwrap().display().to_string();
        let cmd = format!(". {}", &f_path);

        println!("{}", &cmd);
    }

    Ok(())
}
