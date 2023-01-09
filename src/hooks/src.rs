use glob::glob;

use std::fs;

pub fn src(files: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    for f in files {
        if f.contains("*") {
            // handle globs
            for entry in glob(&f).expect(&format!("unable to locate {}", f)) {
                if let Ok(entry_path) = entry {
                    print_src_path_canonical(&entry_path.display().to_string());
                }
            }
        } else {
            print_src_path_canonical(&f);
        }
    }

    Ok(())
}

fn print_src_path_canonical(path: &str) {
    if let Ok(canonical_path) = fs::canonicalize(path) {
        println!(". {}", &canonical_path.display().to_string());
    };
}
