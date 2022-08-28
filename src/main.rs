use log::error;
use std::process;

use orbiter::utils::config;
use orbiter::utils::pipeline;

fn main() {
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
