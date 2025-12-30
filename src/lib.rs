/// Library interface for mdutils functionality
/// Exposes table and code processing modules for use in tests and as a library

pub mod table;
pub mod code;

// Re-export commonly used functions for convenience
pub use table::format_tables;
pub use code::process_code_blocks;
