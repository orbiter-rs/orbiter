mod hooks;
mod lib;
use crate::lib::config::*;
use crate::lib::process::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let payloads = get_payloads()?;
    payloads.iter().for_each(|payload| {
        let result = process_payload(&payload);
        match &result {
            Ok(_) => (),
            Err(err) => println!("error processing payload [{:#?}]: {}", &payload, &err),
        }
    });

    Ok(())
}
