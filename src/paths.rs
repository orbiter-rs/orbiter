use std::path::PathBuf;

use crate::config::*;

pub fn get_config_path() -> PathBuf {
    let logic_config_path = format!(
        "{}/.orbiter.config.yml",
        dirs::home_dir().as_ref().unwrap().to_str().unwrap(),
    );

    PathBuf::from(&logic_config_path)
}

pub fn get_item_dir_path(item: &InitItem) -> PathBuf {
    let logic_dir_path = format!(
        "{}/.orbiter/items/{}",
        dirs::home_dir().as_ref().unwrap().to_str().unwrap(),
        item.id.as_ref().unwrap()
    );

    PathBuf::from(&logic_dir_path)
}
