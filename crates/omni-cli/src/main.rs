//! Example CLI: parse a `PubId` and echo it normalized. Demonstrates the
//! workspace pattern — binaries compose on crates, never duplicate them.

use clap::Parser;

/// Validate an identifier using omni-core.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The identifier to validate.
    id: String,
}

fn main() {
    let args = Args::parse();
    match omni_core::text::PubId::parse(&args.id) {
        Ok(id) => println!("valid: {id}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
