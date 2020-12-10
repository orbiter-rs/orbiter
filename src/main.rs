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
        process_payload(&payload);
    });

    Ok(())
}
