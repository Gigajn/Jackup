use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "jackup",
    about = "Cloud repatriation and self-hosting orchestrator"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Status,
}

fn main() {
    match Cli::parse().command {
        Command::Status => println!("Jackup foundation is ready; no servers paired yet."),
    }
}
