use clap::Parser;
use log::error;
use std::process;

use orbiter::utils::cli;
use orbiter::utils::config;
use orbiter::utils::pipeline;

fn main() {
    let cmd = cli::Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cmd.command {
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

    if let Err(e) = run() {
        println!("Orbiter has encountered an error: {e}");

        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let payloads = config::get_payloads()?;
    payloads.iter().for_each(|payload| {
        let result = pipeline::process_payload(&payload);
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
