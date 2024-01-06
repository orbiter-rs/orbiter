use super::paths::*;
use super::script::*;
use super::shells::SupportedShell;

pub fn create_symlink(
    current_shell: &SupportedShell,
    file_path: &str,
    alias: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = if let Some(alias) = alias {
        alias.to_string()
    } else {
        get_file_name(file_path)?
    };

    let resolved_bin_path = resolve_single_path(file_path)?;
    // set exec mode
    run_cmd_in_shell(
        current_shell,
        &format!("chmod +x {}", &resolved_bin_path.display().to_string()),
    )?;

    println!(
        "ln -sf {} {}",
        &resolved_bin_path.display().to_string(),
        get_bin_file_path(&file_name)?.display().to_string()
    );

    Ok(())
}
