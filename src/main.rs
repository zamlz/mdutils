mod table;

use clap::{Parser, Subcommand};
use std::io::{self, Read};
use table::format_tables;

#[derive(Parser)]
#[command(name = "md")]
#[command(about = "Markdown utilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format and align markdown tables in the input
    Table,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Table => {
            let stdin = io::stdin();
            let mut input = String::new();

            if let Err(e) = stdin.lock().read_to_string(&mut input) {
                eprintln!("Error reading input: {}", e);
                std::process::exit(1);
            }

            let output = format_tables(&input);
            print!("{}", output);
        }
    }
}
