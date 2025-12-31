mod code;
mod common;
mod table;
mod toc;

use clap::{Parser, Subcommand};
use code::process_code_blocks;
use std::io::{self, Read};
use table::{create_table, format_tables, parse_table_spec};
use toc::process_toc;

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
    /// Generate or update table of contents
    Toc,
    /// Create a new markdown table
    New {
        /// Table specification in format "table:R:C" (e.g., "table:2:3")
        spec: String,
    },
}

/// Reads all input from stdin into a String
fn read_stdin() -> Result<String, String> {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin
        .lock()
        .read_to_string(&mut input)
        .map_err(|e| format!("Error reading input: {}", e))?;
    Ok(input)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Table => {
            let input = match read_stdin() {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let output = format_tables(&input);
            print!("{}", output);
        }
        Commands::Code => {
            let input = match read_stdin() {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            match process_code_blocks(&input) {
                Ok(output) => print!("{}", output),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Toc => {
            let input = match read_stdin() {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let output = process_toc(&input);
            print!("{}", output);
        }
        Commands::New { spec } => match parse_table_spec(&spec) {
            Ok((rows, cols)) => {
                let table = create_table(rows, cols);
                print!("{}", table);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
    }
}
