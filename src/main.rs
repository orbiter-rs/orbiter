mod config;
use crate::config::*;
mod download;
mod paths;
mod process;
use crate::process::*;
mod shimmer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payloads = get_payloads()?;
    println!("{:?}", payloads);
    payloads.iter().for_each(|payload| {
        let result = process_payload(&payload);
        match &result {
            Ok(_) => (),
            Err(err) => println!("error processing payload [{:#?}]: {}", &payload, &err),
        }
    });

    Ok(())
}
