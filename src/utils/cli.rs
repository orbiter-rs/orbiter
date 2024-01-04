use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialise shell
    Init {
        /// Name of shell (Options: Zsh, Bash, PowerShell, etc.)
        shell: String,
    },
    /// Update a payload
    Update {
        /// ID of the payload to update
        id: Option<String>,
    },
    /// List configured payloads
    List {
        /// Scope of the payloads to list (effective(default)/all)
        scope: Option<String>,
    },
}
