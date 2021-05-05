use crate::lib::script::*;

pub fn src(files: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for f in files {
        let cmd = format!("source {}", &f);
        run_cmd(&cmd)?;
    }

    Ok(())
}
