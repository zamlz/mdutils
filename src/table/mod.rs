mod formatter;
mod formula;
mod parser;

use formatter::format_table_row;
use formula::apply_formulas;
use parser::{
    extract_formulas_from_comment, is_formula_comment, is_md_table_comment, is_table_row,
    parse_table_row,
};

/// Creates a new empty markdown table with the specified dimensions
pub fn create_table(rows: usize, cols: usize) -> String {
    if rows == 0 || cols == 0 {
        return String::new();
    }

    let mut table_rows = Vec::new();

    // Create header row (empty)
    let header: Vec<String> = vec![String::new(); cols];
    table_rows.push(header);

    // Create separator row (not counted in the row count)
    let separator: Vec<String> = vec!["---".to_string(); cols];
    table_rows.push(separator);

    // Create data rows
    for _ in 0..rows {
        let row: Vec<String> = vec![String::new(); cols];
        table_rows.push(row);
    }

    // Calculate column widths
    let mut col_widths = vec![0; cols];
    for row in &table_rows {
        for (col_idx, cell) in row.iter().enumerate() {
            col_widths[col_idx] = col_widths[col_idx].max(cell.len());
        }
    }

    // Format each row
    let formatted_rows: Vec<String> = table_rows
        .iter()
        .map(|row| format_table_row(row, &col_widths))
        .collect();

    formatted_rows.join("\n")
}

/// Parses a table specification string into row and column counts.
///
/// The specification must follow the format `"table:R:C"` where:
/// - `R` is the number of data rows (must be > 0)
/// - `C` is the number of columns (must be > 0)
///
/// # Arguments
///
/// * `spec` - The specification string to parse
///
/// # Returns
///
/// * `Ok((rows, cols))` if the spec is valid
/// * `Err(String)` with an error message if the spec is invalid
///
/// # Examples
///
/// ```
/// let (rows, cols) = parse_table_spec("table:2:3").unwrap();
/// assert_eq!(rows, 2);
/// assert_eq!(cols, 3);
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Format is not `"table:R:C"`
/// - Row or column count is not a valid number
/// - Row or column count is zero
pub fn parse_table_spec(spec: &str) -> Result<(usize, usize), String> {
    // Expected format: "table:R:C" where R is rows and C is columns
    let parts: Vec<&str> = spec.split(':').collect();

    if parts.len() != 3 || parts[0] != "table" {
        return Err(format!("Invalid spec format '{}'. Expected format: table:R:C (e.g., table:2:3)", spec));
    }

    let rows = parts[1].parse::<usize>()
        .map_err(|_| format!("Invalid row count '{}'", parts[1]))?;
    let cols = parts[2].parse::<usize>()
        .map_err(|_| format!("Invalid column count '{}'", parts[2]))?;

    if rows == 0 || cols == 0 {
        return Err("Row and column counts must be greater than 0".to_string());
    }

    Ok((rows, cols))
}

/// Formats markdown tables in the input text and returns the full text with aligned tables
pub fn format_tables(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut output = Vec::new();
    let mut current_table_lines = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if is_table_row(lines[i]) {
            // Start collecting table lines
            current_table_lines.push(lines[i]);
            i += 1;

            // Collect all consecutive table lines
            while i < lines.len() && is_table_row(lines[i]) {
                current_table_lines.push(lines[i]);
                i += 1;
            }

            // Check for HTML comments with formulas after the table
            let mut formulas = Vec::new();
            let start_comment_idx = i;

            // Look for <!-- md-table: --> comment
            if i < lines.len() && is_md_table_comment(lines[i]) {
                formulas.extend(extract_formulas_from_comment(lines[i]));
                i += 1;

                // Collect additional formula comments on following lines
                while i < lines.len() && is_formula_comment(lines[i]) {
                    formulas.extend(extract_formulas_from_comment(lines[i]));
                    i += 1;
                }
            }

            // Format the table with formulas
            let formatted = format_table_with_formulas(&current_table_lines, &formulas);
            output.push(formatted);

            // Add the comments back to output
            for comment_idx in start_comment_idx..i {
                if comment_idx < lines.len() {
                    output.push(lines[comment_idx].to_string());
                }
            }

            current_table_lines.clear();
        } else {
            // Regular line, pass through as-is
            output.push(lines[i].to_string());
            i += 1;
        }
    }

    let mut result = output.join("\n");

    // Preserve trailing newline if the original input had one
    if text.ends_with('\n') {
        result.push('\n');
    }

    result
}

/// Formats a table with formula evaluation
fn format_table_with_formulas(lines: &[&str], formulas: &[String]) -> String {
    if lines.is_empty() {
        return String::new();
    }

    // Parse all rows into cells
    let mut rows: Vec<Vec<String>> = lines
        .iter()
        .map(|line| parse_table_row(line))
        .collect();

    if rows.is_empty() {
        return lines.join("\n");
    }

    // Apply formulas if any
    if !formulas.is_empty() {
        apply_formulas(&mut rows, formulas);
    }

    // Find the maximum width for each column
    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut col_widths = vec![0; num_cols];

    for row in &rows {
        for (col_idx, cell) in row.iter().enumerate() {
            col_widths[col_idx] = col_widths[col_idx].max(cell.len());
        }
    }

    // Format each row
    let formatted_rows: Vec<String> = rows
        .iter()
        .map(|row| format_table_row(row, &col_widths))
        .collect();

    formatted_rows.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_single_table() {
        let input = r#"# Document Title

Some text before the table.

| Name | Age |
|------|-----|
| John | 30  |
| Jane | 25  |

Some text after the table."#;

        let output = format_tables(input);

        assert!(output.contains("# Document Title"));
        assert!(output.contains("Some text before the table."));
        assert!(output.contains("Some text after the table."));
        assert!(output.contains("Name"));
        assert!(output.contains("Age"));
        assert!(output.contains("John"));
        assert!(output.contains("Jane"));
        // Check that table rows start and end with pipes
        assert!(output.lines().any(|line| line.trim().starts_with("| Name")));
    }

    #[test]
    fn test_format_multiple_tables() {
        let input = r#"First table:

| Col1 | Col2 |
|------|------|
| A    | B    |

Some text in between.

| Col3 | Col4 |
|------|------|
| C    | D    |"#;

        let output = format_tables(input);

        assert!(output.contains("First table:"));
        assert!(output.contains("Some text in between."));
        assert!(output.contains("Col1"));
        assert!(output.contains("Col2"));
        assert!(output.contains("Col3"));
        assert!(output.contains("Col4"));
        assert!(output.contains("A"));
        assert!(output.contains("B"));
        assert!(output.contains("C"));
        assert!(output.contains("D"));
    }

    #[test]
    fn test_no_tables() {
        let input = r#"# Just a document

With some text but no tables.

- List item 1
- List item 2"#;

        let output = format_tables(input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_table_alignment() {
        let input = "| A | B |\n|---|---|\n| Short | VeryLongContent |";
        let output = format_tables(input);

        // Should align columns based on widest content
        assert!(output.contains("VeryLongContent"));
        // All column widths should be consistent
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_empty_input() {
        let output = format_tables("");
        assert_eq!(output, "");
    }

    #[test]
    fn test_table_at_end_of_file() {
        let input = r#"Some text.

| Header |
|--------|
| Data   |"#;

        let output = format_tables(input);
        assert!(output.contains("Some text."));
        assert!(output.contains("Header"));
        assert!(output.contains("Data"));
        // Verify it's formatted as a table
        assert!(output.lines().any(|line| line.trim().starts_with("| Header")));
    }

    #[test]
    fn test_table_with_formulas() {
        let input = r#"| A | B | C |
|---|---|---|
| 5 | 10 | 0 |
<!-- md-table: C1 = A1 + B1 -->"#;

        let output = format_tables(input);
        assert!(output.contains("15"));
    }

    #[test]
    fn test_table_with_multiple_formulas() {
        let input = r#"| Item | Price | Quantity | Total |
|---|---|---|---|
| Apple | 1.50 | 10 | 0 |
| Banana | 0.75 | 20 | 0 |
| Orange | 2.00 | 5 | 0 |
<!-- md-table: D1 = B1 * C1; D2 = B2 * C2; D3 = B3 * C3 -->"#;

        let output = format_tables(input);
        // Check that all formulas were evaluated
        assert!(output.contains("15")); // 1.50 * 10
        assert!(output.contains("10")); // 2.00 * 5
    }

    #[test]
    fn test_table_with_vector_formulas() {
        let input = r#"| Product | Base | Tax | Total |
|---|---|---|---|
| Laptop | 1000 | 0 | 0 |
| Mouse | 50 | 0 | 0 |
| Keyboard | 150 | 0 | 0 |
<!-- md-table: C_ = B_ * 0.08; D_ = B_ + C_ -->"#;

        let output = format_tables(input);

        // Check vector formula results
        assert!(output.contains("80"));   // 1000 * 0.08 (tax on laptop)
        assert!(output.contains("1080")); // 1000 + 80 (total for laptop)
        assert!(output.contains("4"));    // 50 * 0.08 (tax on mouse)
        assert!(output.contains("54"));   // 50 + 4 (total for mouse)
        assert!(output.contains("12"));   // 150 * 0.08 (tax on keyboard)
        assert!(output.contains("162"));  // 150 + 12 (total for keyboard)
    }

    #[test]
    fn test_table_with_sum_function() {
        let input = r#"| Item | Price | Quantity | Total |
|---|---|---|---|
| A | 10 | 5 | 0 |
| B | 20 | 3 | 0 |
| C | 15 | 2 | 0 |
| TOTAL | 0 | 0 | 0 |
<!-- md-table: D_ = B_ * C_; A4 = sum(B_); B4 = sum(C_); D4 = sum(D_) -->"#;

        let output = format_tables(input);

        // Check that sum formulas were evaluated
        assert!(output.contains("| A    | 10    | 5        | 50"));   // First row calculated
        assert!(output.contains("| B    | 20    | 3        | 60"));   // Second row calculated
        assert!(output.contains("| C    | 15    | 2        | 30"));   // Third row calculated
        assert!(output.contains("| 45   | 10    | 0        | 140"));  // Sum row (A4=45, B4=10, C4=0, D4=140)
    }

    #[test]
    fn test_table_with_exponentiation() {
        let input = r#"| Base | Squared |
|---|---|
| 2 | 0 |
| 3 | 0 |
| 4 | 0 |
<!-- md-table: B_ = A_ ^ 2 -->"#;

        let output = format_tables(input);

        // Check exponentiation results
        assert!(output.contains("| 2    | 4"));  // 2^2 = 4
        assert!(output.contains("| 3    | 9"));  // 3^2 = 9
        assert!(output.contains("| 4    | 16")); // 4^2 = 16
    }
}
