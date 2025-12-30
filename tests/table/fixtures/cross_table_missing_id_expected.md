# Cross-Table Missing ID Error

Only table:

| Result |
| ------ |
| 0      |
<!-- md-table: A1 = sum(from("nonexistent")) -->
<!-- md-error: Failed to evaluate expression 'sum(from("nonexistent"))': table 'nonexistent' not found (tables must have an id attribute) -->
