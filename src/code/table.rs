//! Rendering of delimited code output as a markdown table

use crate::code::error::CodeError;
use crate::table::parser::is_separator_cell;

/// How the output of a code block is split into cells
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    /// Tab separated values
    Tsv,
    /// Comma separated values, with `"` quoting
    Csv,
    /// Pipe separated values, optionally with leading/trailing pipes
    Pipe,
    /// Any other literal string
    Custom(String),
}

/// Settings for turning code output into a table (from the md-code directive)
#[derive(Debug, Clone)]
pub struct TableSpec {
    pub delimiter: Delimiter,
    /// Explicit header cells; when set, every output row is data
    pub columns: Option<Vec<String>>,
    /// Treat the first output row as the header (ignored when `columns` is set)
    pub header: bool,
}

/// Renders delimited output as aligned markdown table lines
pub fn render_table(output: &str, spec: &TableSpec) -> Result<Vec<String>, CodeError> {
    let mut rows: Vec<(usize, Vec<String>)> = Vec::new();

    for (idx, line) in output.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let cells = split_line(line, &spec.delimiter);
        // Allow the output itself to already be a markdown table
        if spec.delimiter == Delimiter::Pipe && cells.iter().all(|c| is_separator_cell(c)) {
            continue;
        }
        rows.push((idx + 1, cells));
    }

    let header = match (&spec.columns, spec.header) {
        (Some(columns), _) => columns.clone(),
        (None, true) => {
            if rows.is_empty() {
                return Err(CodeError::TableError(
                    "output has no rows to use as the header".to_string(),
                ));
            }
            rows.remove(0).1
        }
        (None, false) => vec![String::new(); rows.first().map_or(0, |(_, r)| r.len())],
    };

    for (line, row) in &rows {
        if row.len() != header.len() {
            return Err(CodeError::TableError(format!(
                "output line {line} has {} columns, expected {}",
                row.len(),
                header.len()
            )));
        }
    }

    let header: Vec<String> = header.iter().map(|c| escape_cell(c)).collect();
    let body: Vec<Vec<String>> = rows
        .iter()
        .map(|(_, row)| row.iter().map(|c| escape_cell(c)).collect())
        .collect();

    // Widths use the same rule as `md table` so a later format pass is a no-op
    let mut widths = vec![3; header.len()];
    for row in std::iter::once(&header).chain(body.iter()) {
        for (width, cell) in widths.iter_mut().zip(row) {
            *width = (*width).max(cell.len());
        }
    }

    let mut lines = vec![format_row(&header, &widths)];
    let separator: Vec<String> = widths.iter().map(|&w| "-".repeat(w)).collect();
    lines.push(format_row(&separator, &widths));
    lines.extend(body.iter().map(|row| format_row(row, &widths)));
    Ok(lines)
}

fn format_row(cells: &[String], widths: &[usize]) -> String {
    let padded: Vec<String> = cells
        .iter()
        .zip(widths)
        .map(|(cell, &width)| format!("{cell:<width$}"))
        .collect();
    format!("| {} |", padded.join(" | "))
}

/// Pipes inside a cell would otherwise start a new column
fn escape_cell(cell: &str) -> String {
    cell.replace('|', "\\|")
}

fn split_line(line: &str, delimiter: &Delimiter) -> Vec<String> {
    match delimiter {
        Delimiter::Tsv => split_trimmed(line, "\t"),
        Delimiter::Csv => split_csv(line),
        Delimiter::Pipe => {
            let trimmed = line.trim();
            let inner = trimmed.strip_prefix('|').unwrap_or(trimmed);
            let inner = inner.strip_suffix('|').unwrap_or(inner);
            split_trimmed(inner, "|")
        }
        Delimiter::Custom(d) => split_trimmed(line, d),
    }
}

fn split_trimmed(line: &str, delimiter: &str) -> Vec<String> {
    line.split(delimiter)
        .map(|c| c.trim().to_string())
        .collect()
}

/// Splits one CSV record, honouring `"` quoting and `""` escapes
fn split_csv(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if in_quotes && chars.peek() == Some(&'"') => {
                cell.push('"');
                chars.next();
            }
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => cells.push(std::mem::take(&mut cell).trim().to_string()),
            _ => cell.push(c),
        }
    }
    cells.push(cell.trim().to_string());
    cells
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(delimiter: Delimiter) -> TableSpec {
        TableSpec {
            delimiter,
            columns: None,
            header: true,
        }
    }

    #[test]
    fn test_tsv_with_header() {
        let lines = render_table("host\tport\nserver1\t3000\n", &spec(Delimiter::Tsv)).unwrap();
        assert_eq!(
            lines,
            vec![
                "| host    | port |",
                "| ------- | ---- |",
                "| server1 | 3000 |",
            ]
        );
    }

    #[test]
    fn test_explicit_columns_keep_all_rows_as_data() {
        let mut s = spec(Delimiter::Tsv);
        s.columns = Some(vec!["A".to_string(), "B".to_string()]);
        let lines = render_table("1\t2\n3\t4\n", &s).unwrap();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0], "| A   | B   |");
        assert_eq!(lines[2], "| 1   | 2   |");
    }

    #[test]
    fn test_no_header_uses_empty_header() {
        let mut s = spec(Delimiter::Tsv);
        s.header = false;
        let lines = render_table("1\t2\n", &s).unwrap();
        assert_eq!(lines[0], "|     |     |");
        assert_eq!(lines[2], "| 1   | 2   |");
    }

    #[test]
    fn test_csv_quoting() {
        let lines =
            render_table("a,b\n\"x, y\",\"say \"\"hi\"\"\"\n", &spec(Delimiter::Csv)).unwrap();
        assert_eq!(lines[2], "| x, y | say \"hi\" |");
    }

    #[test]
    fn test_pipe_skips_separator_and_outer_pipes() {
        let lines = render_table(
            "| a | b |\n| --- | --- |\n| 1 | 2 |\n",
            &spec(Delimiter::Pipe),
        )
        .unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[2], "| 1   | 2   |");
    }

    #[test]
    fn test_custom_delimiter() {
        let lines =
            render_table("a::b\n1::2\n", &spec(Delimiter::Custom("::".to_string()))).unwrap();
        assert_eq!(lines[2], "| 1   | 2   |");
    }

    #[test]
    fn test_pipe_in_cell_is_escaped() {
        let lines = render_table("a,b\nx|y,z\n", &spec(Delimiter::Csv)).unwrap();
        assert_eq!(lines[2], "| x\\|y | z   |");
    }

    #[test]
    fn test_blank_lines_skipped() {
        let lines = render_table("a\tb\n\n1\t2\n", &spec(Delimiter::Tsv)).unwrap();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_ragged_row_errors_with_line() {
        let err = render_table("a\tb\n1\t2\t3\n", &spec(Delimiter::Tsv)).unwrap_err();
        assert!(err.to_string().contains("line 2"));
        assert!(err.to_string().contains("3 columns, expected 2"));
    }

    #[test]
    fn test_header_only_output() {
        let lines = render_table("a\tb\n", &spec(Delimiter::Tsv)).unwrap();
        assert_eq!(lines.len(), 2);
    }
}
