mod code;
mod table;

use clap::{Parser, Subcommand};
use code::process_code_blocks;
use std::io::{self, Read};
use table::{create_table, format_tables};

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
    /// Execute code blocks with md-code directives
    Code,
    /// Create a new markdown table
    New {
        /// Table specification in format "table:R:C" (e.g., "table:2:3")
        spec: String,
    },
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
        Commands::Code => {
            let stdin = io::stdin();
            let mut input = String::new();

            if let Err(e) = stdin.lock().read_to_string(&mut input) {
                eprintln!("Error reading input: {}", e);
                std::process::exit(1);
            }

            match process_code_blocks(&input) {
                Ok(output) => print!("{}", output),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::New { spec } => {
            match parse_table_spec(&spec) {
                Ok((rows, cols)) => {
                    let table = create_table(rows, cols);
                    print!("{}", table);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn parse_table_spec(spec: &str) -> Result<(usize, usize), String> {
    // Expected format: "table:R:C" where R is rows and C is columns
    let parts: Vec<&str> = spec.split(':').collect();

    if parts.len() != 3 || parts[0] != "table" {
        return Err(format!("Invalid spec format '{}'. Expected format: table:R:C (e.g., table:2:3)", spec));
    }

    let rows = parts[1].parse::<usize>()
        .map_err(|_| format!("Invalid row count '{}'", parts[1]))?;
    let cols = parts[2].parse::<usize>()
        .map_err(|_| format!("Invalid column count '{}'", parts[2]))?;

    if rows == 0 || cols == 0 {
        return Err("Row and column counts must be greater than 0".to_string());
    }

    Ok((rows, cols))
}
