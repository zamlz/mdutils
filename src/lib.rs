pub mod code;
pub mod common;
/// Library interface for mdutils functionality
/// Exposes table, code, and toc processing modules for use in tests and as a library
pub mod table;
pub mod toc;

// Re-export commonly used functions for convenience
pub use code::process_code_blocks;
pub use table::format_tables;
pub use toc::process_toc;
