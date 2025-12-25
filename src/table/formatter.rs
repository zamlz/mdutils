use super::parser::is_separator_cell;

/// Formats a table row with proper padding based on column widths
pub fn format_table_row(cells: &[String], col_widths: &[usize]) -> String {
    let formatted_cells: Vec<String> = cells
        .iter()
        .enumerate()
        .map(|(idx, cell)| {
            let width = col_widths.get(idx).copied().unwrap_or(0);

            // Check if this is a separator row (contains only dashes, colons, and spaces)
            if is_separator_cell(cell) {
                format_separator_cell(cell, width)
            } else {
                // Regular cell - left-aligned with padding
                format!("{:<width$}", cell, width = width)
            }
        })
        .collect();

    format!("| {} |", formatted_cells.join(" | "))
}

/// Formats a separator cell with the appropriate width
fn format_separator_cell(cell: &str, width: usize) -> String {
    let has_left_colon = cell.starts_with(':');
    let has_right_colon = cell.ends_with(':');

    match (has_left_colon, has_right_colon) {
        (true, true) => format!(":{:-<width$}:", "", width = width.saturating_sub(2)),
        (true, false) => format!(":{:-<width$}", "", width = width.saturating_sub(1)),
        (false, true) => format!("{:-<width$}:", "", width = width.saturating_sub(1)),
        (false, false) => format!("{:-<width$}", "", width = width),
    }
}
