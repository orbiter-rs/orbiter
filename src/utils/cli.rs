use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// update item(s)
    Init {
        shell: String,
    },
    Update {
        item: Option<String>,
    },
    // List,
}
