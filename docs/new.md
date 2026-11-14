Command: `new` (New Markdown Element Creation)
==============================================

<!-- md-toc: -->
- [New Table](#new-table)
<!-- md-toc: end -->

## New Table

The `new` subcommand creates a new empty markdown table with the specified
dimensions and outputs it to STDOUT.

**Create a table with specified rows and columns:**

Here is how to create a table with 2 rows and 3 columns. The format is
`table:R:C` where R is the number of data rows and C is the number of columns.
The header and separator rows are always added on top of the data rows.

**Example:**

```bash
md new table:2:3
```
<!-- md-code: id="new-table-example"; bin="bash"; syntax="markdown" -->

Output:
```markdown
|     |     |     |
| --- | --- | --- |
|     |     |     |
|     |     |     |
```
<!-- md-code-output: id="new-table-example" -->

All cells are empty and ready to be filled in.
