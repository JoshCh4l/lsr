use crate::cli::Cli;
use clap::Parser;

mod cli;
mod entry_file;

fn main() {
    println!();
    let cli = Cli::parse();
    Cli::lsr(cli);
}
