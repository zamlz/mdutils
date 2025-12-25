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

/// Extracts formulas from an HTML comment
pub fn extract_formulas_from_comment(line: &str) -> Vec<String> {
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

    // Split by semicolon for multiple formulas
    content
        .split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
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
        let formulas = extract_formulas_from_comment("<!-- md-table: A1 = B1 + C1 -->");
        assert_eq!(formulas, vec!["A1 = B1 + C1"]);

        let formulas = extract_formulas_from_comment("<!-- md-table: A1 = 5; B1 = 10 -->");
        assert_eq!(formulas, vec!["A1 = 5", "B1 = 10"]);

        let formulas = extract_formulas_from_comment("<!-- A1 = B1 + C1 -->");
        assert_eq!(formulas, vec!["A1 = B1 + C1"]);
    }
}
