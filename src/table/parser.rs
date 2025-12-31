/// Parses a table row into individual cells
pub fn parse_table_row(line: &str) -> Vec<String> {
    let trimmed = line.trim();

    // Remove leading and trailing pipes
    let content = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let content = content.strip_suffix('|').unwrap_or(content);

    // Split by pipe and trim each cell
    content
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

/// Checks if a line looks like a markdown table row
pub fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();

    // Must start and contain pipe characters
    if !trimmed.starts_with('|') || !trimmed.contains('|') {
        return false;
    }

    // Must have at least 2 pipe characters (start + at least one separator)
    trimmed.matches('|').count() >= 2
}

/// Checks if a cell is part of a separator row
pub fn is_separator_cell(cell: &str) -> bool {
    !cell.is_empty() && cell.chars().all(|c| c == '-' || c == ':' || c == ' ')
}

/// Checks if a line is an HTML comment with md-table marker
pub fn is_md_table_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("<!--") && trimmed.contains("md-table:")
}

/// Checks if a line is an HTML comment (for continuation formulas)
pub fn is_formula_comment(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("<!--") && trimmed.ends_with("-->")
}

/// Parses md-table directive and extracts optional ID and formulas
/// Format: <!-- md-table: id="table_name"; A1 = B1 + C1; D1 = sum(C_) -->
/// Returns (optional_id, formulas)
pub fn extract_formulas_from_comment(line: &str) -> Result<(Option<String>, Vec<String>), String> {
    use crate::common::validate_id;

    let trimmed = line.trim();

    // Remove <!-- and -->
    let content = trimmed
        .strip_prefix("<!--")
        .unwrap_or(trimmed)
        .strip_suffix("-->")
        .unwrap_or(trimmed)
        .trim();

    // Remove md-table: prefix if present
    let content = content.strip_prefix("md-table:").unwrap_or(content).trim();

    let mut id = None;
    let mut formulas = Vec::new();

    // Split by semicolon
    for part in content.split(';') {
        let part = part.trim();

        if part.is_empty() {
            continue;
        }

        // Check if this part is an ID attribute
        if part.starts_with("id=") {
            // Extract ID value from quotes
            let value = part.strip_prefix("id=").unwrap().trim();

            if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                let extracted_id = value[1..value.len() - 1].to_string();

                // Validate ID format
                validate_id(&extracted_id).map_err(|e| format!("Invalid table ID: {}", e))?;

                id = Some(extracted_id);
            } else {
                return Err("Table ID must be enclosed in double quotes".to_string());
            }
        } else {
            // This is a formula
            formulas.push(part.to_string());
        }
    }

    Ok((id, formulas))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_table_row() {
        assert_eq!(parse_table_row("| A | B | C |"), vec!["A", "B", "C"]);
        assert_eq!(parse_table_row("|A|B|C|"), vec!["A", "B", "C"]);
        assert_eq!(parse_table_row("  | A | B |  "), vec!["A", "B"]);
    }

    #[test]
    fn test_is_table_row() {
        assert!(is_table_row("| Header 1 | Header 2 |"));
        assert!(is_table_row("|---|---|"));
        assert!(is_table_row("| Data | More Data |"));
        assert!(is_table_row("  | Indented | Table |  "));

        assert!(!is_table_row("Not a table"));
        assert!(!is_table_row("| Only one pipe"));
        assert!(!is_table_row(""));
        assert!(!is_table_row("# Header"));
    }

    #[test]
    fn test_is_separator_cell() {
        assert!(is_separator_cell("---"));
        assert!(is_separator_cell(":---"));
        assert!(is_separator_cell("---:"));
        assert!(is_separator_cell(":---:"));
        assert!(!is_separator_cell("data"));
        assert!(!is_separator_cell(""));
    }

    #[test]
    fn test_extract_formulas_from_comment() {
        // Test with formulas only (no ID)
        let (id, formulas) =
            extract_formulas_from_comment("<!-- md-table: A1 = B1 + C1 -->").unwrap();
        assert_eq!(id, None);
        assert_eq!(formulas, vec!["A1 = B1 + C1"]);

        let (id, formulas) =
            extract_formulas_from_comment("<!-- md-table: A1 = 5; B1 = 10 -->").unwrap();
        assert_eq!(id, None);
        assert_eq!(formulas, vec!["A1 = 5", "B1 = 10"]);

        let (id, formulas) = extract_formulas_from_comment("<!-- A1 = B1 + C1 -->").unwrap();
        assert_eq!(id, None);
        assert_eq!(formulas, vec!["A1 = B1 + C1"]);
    }

    #[test]
    fn test_extract_formulas_with_id() {
        // Test with ID and formulas
        let (id, formulas) =
            extract_formulas_from_comment("<!-- md-table: id=\"sales_data\"; A1 = B1 + C1 -->")
                .unwrap();
        assert_eq!(id, Some("sales_data".to_string()));
        assert_eq!(formulas, vec!["A1 = B1 + C1"]);

        let (id, formulas) =
            extract_formulas_from_comment("<!-- md-table: id=\"my_table\"; A1 = 5; B1 = 10 -->")
                .unwrap();
        assert_eq!(id, Some("my_table".to_string()));
        assert_eq!(formulas, vec!["A1 = 5", "B1 = 10"]);

        // Test with ID only (no formulas)
        let (id, formulas) =
            extract_formulas_from_comment("<!-- md-table: id=\"table1\" -->").unwrap();
        assert_eq!(id, Some("table1".to_string()));
        assert_eq!(formulas.len(), 0);
    }

    #[test]
    fn test_extract_formulas_id_validation() {
        // Whitespace is now allowed
        assert!(extract_formulas_from_comment("<!-- md-table: id=\"my table\" -->").is_ok());

        // Special characters are now allowed
        assert!(extract_formulas_from_comment("<!-- md-table: id=\"my-table\" -->").is_ok());
        assert!(extract_formulas_from_comment("<!-- md-table: id=\"my.table\" -->").is_ok());
        assert!(extract_formulas_from_comment("<!-- md-table: id=\"2024-sales\" -->").is_ok());

        // Invalid ID - missing quotes
        assert!(extract_formulas_from_comment("<!-- md-table: id=mytable -->").is_err());

        // Empty ID
        assert!(extract_formulas_from_comment("<!-- md-table: id=\"\" -->").is_err());
    }
}
