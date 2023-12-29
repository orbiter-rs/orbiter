use clap::Parser;
use log::error;
use orbiter::utils::paths::update_path;
use orbiter::utils::shells::SupportedShell;
use std::process;

use orbiter::utils::cli;
use orbiter::utils::config;
use orbiter::utils::pipeline;

fn main() {
    let cmd = cli::Cli::parse();

    let mut current_shell: SupportedShell = SupportedShell::Sh;
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cmd.command {
        Some(cli::Commands::Init { shell }) => {
            current_shell = SupportedShell::from_str(shell);

            // update PATH env var to enable shims
            update_path(&current_shell);
        }
        Some(cli::Commands::Update { item }) => {
            if let Some(item_id) = item {
                println!("update item={:?}", &item_id);
            } else {
                println!("Printing all items...");
            }
        }
        // Some(cli::Commands::List) => {
        //     println!("Printing list");
        // }
        None => {}
    }

    if let Err(e) = run(&current_shell) {
        println!("Orbiter has encountered an error: {e}");

        process::exit(1);
    }
}

fn run(current_shell: &SupportedShell) -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let payloads = config::get_payloads()?;
    payloads.iter().for_each(|payload| {
        let result = pipeline::process_payload(current_shell, &payload);
        match &result {
            Ok(_) => (),
            Err(err) => error!("error processing payload [{:#?}]: {}", &payload, &err),
        }
    });

    // replace with stub
    println!("autoload -Uz compinit");
    println!("compinit");

    Ok(())
}
