use log::error;

mod hooks;
mod lib;
use crate::lib::config::*;
use crate::lib::process::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let payloads = get_payloads()?;
    payloads.iter().for_each(|payload| {
        let result = process_payload(&payload);
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
