use crate::utils::{
    config::ShellSpecificEvaluatable, evaluatable::process_evaluatable, shells::SupportedShell,
};

pub fn load(
    current_shell: &SupportedShell,
    load_evaluatable: &ShellSpecificEvaluatable,
) -> Result<(), Box<dyn std::error::Error>> {
    process_evaluatable(current_shell, load_evaluatable)
}
