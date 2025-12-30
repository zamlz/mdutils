/// Common utilities shared across modules

/// Validates that an ID is not empty.
///
/// Valid IDs must:
/// - Not be empty
///
/// Any non-empty string is a valid ID.
///
/// # Examples
///
/// ```
/// use mdutils::common::validate_id;
///
/// assert!(validate_id("my_table_123").is_ok());
/// assert!(validate_id("MyTable").is_ok());
/// assert!(validate_id("my-table").is_ok());
/// assert!(validate_id("my table").is_ok());
/// assert!(validate_id("table!").is_ok());
/// assert!(validate_id("2024-sales").is_ok());
///
/// assert!(validate_id("").is_err());
/// ```
pub fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("ID cannot be empty".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ids() {
        // Alphanumeric and underscores
        assert!(validate_id("table1").is_ok());
        assert!(validate_id("my_table").is_ok());
        assert!(validate_id("MyTable123").is_ok());
        assert!(validate_id("_table").is_ok());
        assert!(validate_id("table_").is_ok());
        assert!(validate_id("TABLE").is_ok());
        assert!(validate_id("t").is_ok());
        assert!(validate_id("_").is_ok());
        assert!(validate_id("___").is_ok());
        assert!(validate_id("a1b2c3").is_ok());

        // Whitespace is now allowed
        assert!(validate_id("my table").is_ok());
        assert!(validate_id("table ").is_ok());
        assert!(validate_id(" table").is_ok());
        assert!(validate_id("my\ttable").is_ok());

        // Special characters are now allowed
        assert!(validate_id("my-table").is_ok());
        assert!(validate_id("my.table").is_ok());
        assert!(validate_id("my@table").is_ok());
        assert!(validate_id("my!table").is_ok());
        assert!(validate_id("my#table").is_ok());
        assert!(validate_id("my$table").is_ok());
        assert!(validate_id("my%table").is_ok());
        assert!(validate_id("2024-sales-data").is_ok());
        assert!(validate_id("user@domain").is_ok());
        assert!(validate_id("v1.0.0").is_ok());
    }

    #[test]
    fn test_invalid_ids_empty() {
        assert!(validate_id("").is_err());

        let err = validate_id("").unwrap_err();
        assert_eq!(err, "ID cannot be empty");
    }
}
