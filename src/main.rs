use clap::Parser;
use log::error;
use orbiter::utils::completion::load_completion;
use orbiter::utils::config::Payload;
use orbiter::utils::listing::get_listing;
use orbiter::utils::listing::ListingScope;
use orbiter::utils::paths::update_path;
use orbiter::utils::shells::SupportedShell;

use orbiter::utils::cli;
use orbiter::utils::config;
use orbiter::utils::pipeline;
use orbiter::utils::update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cmd = cli::Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let result = match &cmd.command {
        cli::Commands::Init { shell } => {
            let current_shell = SupportedShell::from_str(shell);

            // update PATH env var to enable shims
            update_path(&current_shell);

            init_shell(&current_shell)
        }
        cli::Commands::Update { id } => {
            let payloads = config::get_payloads()?;
            if let Some(payload_id) = id {
                println!("Updating payload: {:?}", &payload_id);
                update_payload(&payloads, payload_id)?
            } else {
                update::self_update()?
            };

            println!("Restart terminal to take effect");

            Ok(())
        }
        cli::Commands::List { scope } => {
            let payloads = config::get_payloads()?;
            // list items
            let listing_scope = ListingScope::from(scope);
            let listing = get_listing(&payloads, &listing_scope)?;
            println!("{:?} payloads: {:?}", &listing_scope, &listing);

            Ok(())
        }
    };

    Ok(if let Err(e) = result {
        error!("Orbiter has encountered an error: {e}");
    })
}

fn init_shell(current_shell: &SupportedShell) -> Result<(), Box<dyn std::error::Error>> {
    config::get_payloads()?.iter().for_each(|payload| {
        pipeline::process_payload(current_shell, &payload)
            .unwrap_or_else(|err| error!("error processing payload [{:#?}]: {}", &payload, &err))
    });

    // enables completion for shells that require it
    load_completion(current_shell);

    Ok(())
}

fn update_payload(
    payloads: &Vec<Payload>,
    payload_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(
        if let Some(payload) = payloads.iter().find(|p| p.id == payload_id) {
            update::update_payload(payload)?
        } else {
            error!("Payload with id {} not found", &payload_id)
        },
    )
}
