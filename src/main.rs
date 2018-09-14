#![feature(crate_visibility_modifier)]

mod block;
mod chain;

use crate::chain::Blockchain;
use std::process;

fn main() {
    println!("Rusty Mining Operations are now open for business!\n");

    run().unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1)
    })
}

fn run() -> Result<(), block::MiningError> {
    let mut chain = Blockchain::genesis();
    println!("{}", chain);
    chain.add_block("This is the first real block")?;
    println!("{}", chain);
    chain.add_block("And this is the next one")?;
    println!("{}", chain);
    Ok(())
}
