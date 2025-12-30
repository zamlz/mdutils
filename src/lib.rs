/// Library interface for mdutils functionality
/// Exposes table, code, and toc processing modules for use in tests and as a library
pub mod table;
pub mod code;
pub mod toc;
pub mod common;

// Re-export commonly used functions for convenience
pub use table::format_tables;
pub use code::process_code_blocks;
pub use toc::process_toc;
