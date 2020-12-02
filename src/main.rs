use glob::glob;
use std::fs;
use std::fs::File;
use std::io::BufReader;

mod config;
use crate::config::*;

mod download;
use crate::download::*;

mod shimmer;
use crate::shimmer::*;

mod paths;
use crate::paths::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("orbiter.config.yml")?;
    let mut reader = BufReader::new(file);
    let items: Vec<InitItem> = from_reader(&mut reader);
    println!("{:?}", items);
    let item = &items[0];

    // create a working dir
    let dir_path = get_item_dir_path(item);
    fs::create_dir_all(&dir_path)?;
    // save file
    // download_file(item)?;

    // change current dir
    assert!(std::env::set_current_dir(&dir_path).is_ok());

    // create shim
    let base_shim = match &item.exec {
        Executable::Run(cmd) => get_basic_shim(&cmd, &cmd),
        Executable::Command { run, alias } => Ok(format!("{} {}", run, alias.as_ref().unwrap())),
    };

    println!("{}", &base_shim.unwrap());

    Ok(())
}
