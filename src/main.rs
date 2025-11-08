mod cli;

use clap::Parser;

fn main() {
    if let Err(e) = cli::run(cli::Cli::parse()) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
