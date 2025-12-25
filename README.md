Markdown Utils (Name Pending)
=============================

A Rust CLI tool for markdown processing with multiple subcommands.

## Usage

### Table Formatting

The `table` subcommand reads markdown from STDIN, formats and aligns any markdown tables it finds, and outputs the entire document to STDOUT with nicely formatted tables.

**Format tables in a markdown file:**
```bash
./result/bin/md table < document.md
```

**Format tables from piped input:**
```bash
cat document.md | ./result/bin/md table
```

**View available commands:**
```bash
./result/bin/md --help
```

**Example:**

Input:
```markdown
# My Document

Some introductory text.

| Name | Age | City |
|---|---|---|
| Alice | 30 | New York |
| Bob | 25 | LA |

More text here.
```

Output:
```markdown
# My Document

Some introductory text.

| Name  | Age | City     |
| ----- | --- | -------- |
| Alice | 30  | New York |
| Bob   | 25  | LA       |

More text here.
```

All content is preserved, but tables are properly aligned based on column widths.

### Table Formulas (Spreadsheet Functionality)

Tables can include spreadsheet-like formulas using HTML comments with the `<!-- md-table: -->` marker.

**Basic formula syntax:**
- Formulas are defined in HTML comments after the table
- Format: `CELL = EXPRESSION` (e.g., `C2 = A2 + B2`)
- Cell references use spreadsheet notation: A1, B2, C3, etc. (Column letter + Row number)
- Row 1 is the header row, Row 2 is the first data row
- Multiple formulas can be separated by semicolons in one comment or placed on separate comment lines

**Supported operators:**
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`

**Example:**

Input:
```markdown
| Item | Price | Quantity | Total |
|---|---|---|---|
| Apple | 1.50 | 10 | 0 |
| Banana | 0.75 | 20 | 0 |
<!-- md-table: D2 = B2 * C2; D3 = B3 * C3 -->
```

Output:
```markdown
| Item   | Price | Quantity | Total |
| ------ | ----- | -------- | ----- |
| Apple  | 1.50  | 10       | 15    |
| Banana | 0.75  | 20       | 15    |
<!-- md-table: D2 = B2 * C2; D3 = B3 * C3 -->
```

**Multiple comment lines example:**
```markdown
| Product | Price | Tax | Total |
|---|---|---|---|
| Laptop | 1000 | 0 | 0 |
<!-- md-table: C2 = B2 * 0.08 -->
<!-- D2 = B2 + C2 -->
```

Formulas are evaluated in order, so later formulas can reference cells updated by earlier formulas.

## Development

This project uses Nix flakes for reproducible builds and development environments.

### Build

Build the project using Nix:
```bash
nix build
```

The binary will be available at `./result/bin/md`.

### Run

Run the built binary:
```bash
./result/bin/md table < input.md
```

Or run directly through the development shell:
```bash
nix develop --command cargo run -- table < input.md
```

### Test

Run tests:
```bash
nix develop --command cargo test
```

### Debug

Enter the development shell for interactive development:
```bash
nix develop
```

Inside the shell, you can use standard Cargo commands:
```bash
cargo build              # Build the project
cargo run -- table       # Run the project with table subcommand
cargo test               # Run tests
cargo check              # Check for errors without building
cargo clippy             # Run linter
cargo fmt                # Format code
```

For debugging with GDB or LLDB:
```bash
nix develop --command cargo build
nix develop --command gdb ./target/debug/md
```
