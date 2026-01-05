Command: `new` (New Markdown Element Creation)
==============================================

<!-- md-toc: -->
- [New Table](#new-table)
<!-- md-toc: end -->

## New Table

The `new` subcommand creates a new empty markdown table with the specified
dimensions and outputs it to STDOUT.

**Create a table with specified rows and columns:**

Here is how to create a table with 3 rows and 2 columns. The format is
`table:R:C` where R is the number of rows and C is the number of columns.

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
