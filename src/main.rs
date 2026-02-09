mod code;
mod common;
mod done;
mod table;
mod toc;

use clap::{Parser, Subcommand};
use code::process_code_blocks;
use common::{ExitCode, ProcessingResult};
use done::process_done;
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
    /// Mark checklist items as done with strikethrough and timestamp
    Done,
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

/// Handles a ProcessingResult: prints output, reports errors, and returns exit code
fn handle_result(result: ProcessingResult) -> ExitCode {
    print!("{}", result.output);

    for error in &result.errors {
        eprintln!("error: {}", error);
    }

    if result.has_errors() {
        ExitCode::DataErr
    } else {
        ExitCode::Success
    }
}

fn run() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Table => {
            let input = match read_stdin() {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::IoErr;
                }
            };
            handle_result(format_tables(&input))
        }
        Commands::Code => {
            let input = match read_stdin() {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::IoErr;
                }
            };
            handle_result(process_code_blocks(&input))
        }
        Commands::Toc => {
            let input = match read_stdin() {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::IoErr;
                }
            };
            handle_result(process_toc(&input))
        }
        Commands::Done => {
            let input = match read_stdin() {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("{}", e);
                    return ExitCode::IoErr;
                }
            };
            handle_result(process_done(&input))
        }
        Commands::New { spec } => match parse_table_spec(&spec) {
            Ok((rows, cols)) => {
                let table = create_table(rows, cols);
                print!("{}", table);
                ExitCode::Success
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::Usage
            }
        },
    }
}

fn main() -> ExitCode {
    run()
}
