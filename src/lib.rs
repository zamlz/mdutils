pub mod code;
pub mod common;
pub mod done;
/// Library interface for mdutils functionality
/// Exposes table, code, toc, and done processing modules for use in tests and as a library
pub mod table;
pub mod toc;

// Re-export commonly used functions for convenience
pub use code::process_code_blocks;
pub use done::{process_done, process_done_with_timestamp};
pub use table::format_tables;
pub use toc::process_toc;
