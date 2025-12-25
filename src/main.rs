use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    if let Err(e) = stdin.lock().read_to_string(&mut input) {
        eprintln!("Error reading input: {}", e);
        std::process::exit(1);
    }

    let tables = extract_tables(&input);
    for table in tables {
        println!("{}", table);
    }
}

/// Extracts all markdown tables from the input text
fn extract_tables(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut tables = Vec::new();
    let mut current_table = Vec::new();

    for line in lines {
        if is_table_row(line) {
            current_table.push(line);
        } else {
            // If we were building a table and hit a non-table line, save it
            if !current_table.is_empty() {
                tables.push(current_table.join("\n"));
                current_table.clear();
            }
        }
    }

    // Don't forget the last table if file ends with one
    if !current_table.is_empty() {
        tables.push(current_table.join("\n"));
    }

    tables
}

/// Checks if a line looks like a markdown table row
fn is_table_row(line: &str) -> bool {
    let trimmed = line.trim();

    // Must start and contain pipe characters
    if !trimmed.starts_with('|') || !trimmed.contains('|') {
        return false;
    }

    // Must have at least 2 pipe characters (start + at least one separator)
    trimmed.matches('|').count() >= 2
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_extract_single_table() {
        let input = r#"# Document Title

Some text before the table.

| Name | Age |
|------|-----|
| John | 30  |
| Jane | 25  |

Some text after the table."#;

        let tables = extract_tables(input);
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0], "| Name | Age |\n|------|-----|\n| John | 30  |\n| Jane | 25  |");
    }

    #[test]
    fn test_extract_multiple_tables() {
        let input = r#"First table:

| Col1 | Col2 |
|------|------|
| A    | B    |

Some text in between.

| Col3 | Col4 |
|------|------|
| C    | D    |"#;

        let tables = extract_tables(input);
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0], "| Col1 | Col2 |\n|------|------|\n| A    | B    |");
        assert_eq!(tables[1], "| Col3 | Col4 |\n|------|------|\n| C    | D    |");
    }

    #[test]
    fn test_no_tables() {
        let input = r#"# Just a document

With some text but no tables.

- List item 1
- List item 2"#;

        let tables = extract_tables(input);
        assert_eq!(tables.len(), 0);
    }

    #[test]
    fn test_table_without_separator_row() {
        let input = r#"| Header 1 | Header 2 |
| Data 1   | Data 2   |"#;

        let tables = extract_tables(input);
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0], "| Header 1 | Header 2 |\n| Data 1   | Data 2   |");
    }

    #[test]
    fn test_empty_input() {
        let tables = extract_tables("");
        assert_eq!(tables.len(), 0);
    }

    #[test]
    fn test_table_at_end_of_file() {
        let input = r#"Some text.

| Header |
|--------|
| Data   |"#;

        let tables = extract_tables(input);
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0], "| Header |\n|--------|\n| Data   |");
    }
}
