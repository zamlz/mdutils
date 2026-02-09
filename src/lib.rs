//! Library interface for mdutils functionality
//!
//! Exposes table, code, toc, and done processing modules for use in tests and as a library.
//!
//! All processing functions return a [`ProcessingResult`] which contains:
//! - The processed output (always produced, even with errors)
//! - A list of any errors encountered
//!
//! This allows callers to always get output while still being able to detect and report errors.

pub mod code;
pub mod common;
pub mod done;
pub mod table;
pub mod toc;

// Re-export commonly used types for convenience
pub use common::{ProcessingError, ProcessingResult};

// Re-export commonly used functions for convenience
pub use code::process_code_blocks;
pub use done::{process_done, process_done_with_timestamp};
pub use table::format_tables;
pub use toc::process_toc;
