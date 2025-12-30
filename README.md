Markdown Utils
==============

<!-- md-toc: -->
- [Why?](#why)
- [Disclaimer](#disclaimer)
- [Usage](#usage)
  - [Command: `new` (New Markdown Element Creation)](#command-new-new-markdown-element-creation)
  - [Command: `toc` (Table-of-Contents Generation)](#command-toc-table-of-contents-generation)
  - [Command: `table` (Auto-formatting and Spreadsheet Formulas)](#command-table-auto-formatting-and-spreadsheet-formulas)
    - [Table Formatting](#table-formatting)
    - [Table Formulas (Spreadsheet Functionality)](#table-formulas-spreadsheet-functionality)
    - [Vector and Matrix Operations](#vector-and-matrix-operations)
    - [Variables (Let Statements)](#variables-let-statements)
    - [Cell Range References](#cell-range-references)
    - [Matrix Assignments](#matrix-assignments)
    - [Matrix Multiplication and Transpose Operator](#matrix-multiplication-and-transpose-operator)
    - [Formula Error Handling](#formula-error-handling)
    - [Table IDs](#table-ids)
    - [Cross-Table References](#cross-table-references)
  - [Command: `code` (Code Execution)](#command-code-code-execution)
- [Development](#development)
  - [Build](#build)
  - [Run](#run)
  - [Test](#test)
  - [Debug](#debug)
<!-- md-toc: end -->

## Why?

A Rust CLI tool for markdown processing with multiple subcommands. This
is heavily based on some of the useful features I used when I used to
use GNU/Emacs and Org-mode. Spreadsheet and literate programming were
really cool features but were limited to GNU/Emacs. Emacs is really
awesome, but if I needed to edit markdown files, I wanted to be able to
do some of the same things.  And even more so, I wanted to do so in any
text editor! Editors like VIM, NeoVIM, Kakoune, Helix, etc. allow you
to take a selection of text and pipe it to some program in your path
and then replaces the highlighted text with it's output. This tool
does just that. It is not editor specific and utilizes HTML comments in
markdown (which is valid syntax) to tag and extend markdown with some
of these capabilities.

## Disclaimer

This tool is entirely vibe-coded. It started off as an experiment in
vibe-coding and I figured it was simple enough to define and structurely
build up in an interative manner. I had the features and design in my
head but I just didn't have the time to work on a project like this. It's
simple enough where I can define everything exactly to what I need,
but also complex enough where it would take me a while.

In any case, you have been warned!

## Usage

`md` supports the following commands:
- `new`: Create simple elements like new tables
- `table`: Format and apply formulas to existing tables
- `code`: Evaluation of code blocks
- `toc`: Generation of table of contents

### Command: `new` (New Markdown Element Creation)

The `new` subcommand creates a new empty markdown table with the specified
dimensions and outputs it to STDOUT.

**Create a table with specified rows and columns:**
```bash
./result/bin/md new table:3:2
```

This creates a table with 3 rows and 2 columns. The format is `table:R:C`
where R is the number of rows and C is the number of columns.

**Example:**

```bash
./result/bin/md new table:2:3
```

Output:
```markdown
|     |     |     |
| --- | --- | --- |
|     |     |     |
|     |     |     |
```

All cells are empty and ready to be filled in.

### Command: `toc` (Table-of-Contents Generation)

The `toc` subcommand automatically generates or updates a table of contents from markdown headers.

**Basic usage:**
```bash
./result/bin/md toc < document.md
```

**How it works:**
- Add a `<!-- md-toc: -->` marker where you want the TOC
- The command scans all headers in the document
- Generates clickable links with proper indentation
- Adds `<!-- md-toc: end -->` after the TOC
- Re-running updates the existing TOC

**Example:**

Input:
```markdown
# My Documentation
<!-- md-toc: -->

## Introduction
### Background
### Goals

## Implementation
### Architecture
### Testing

## Conclusion
```

Output:
```markdown
# My Documentation
<!-- md-toc: -->
- [Introduction](#introduction)
  - [Background](#background)
  - [Goals](#goals)
- [Implementation](#implementation)
  - [Architecture](#architecture)
  - [Testing](#testing)
- [Conclusion](#conclusion)
<!-- md-toc: end -->

## Introduction
### Background
### Goals

## Implementation
### Architecture
### Testing

## Conclusion
```

**Features:**
- **GitHub-style slugs**: Links use the same anchor format as GitHub
- **Automatic indentation**: H2 sections indented, H3 further indented, etc.
- **Duplicate handling**: Headers with the same text get unique slugs (e.g., `#section`, `#section-1`, `#section-2`)
- **Update support**: Re-running replaces the old TOC with updated content
- **Smart parsing**: Only includes headers after the TOC marker (prevents self-reference)

**Updating an existing TOC:**

```markdown
# Document
<!-- md-toc: -->
- [Old Section](#old-section)
<!-- md-toc: end -->

## New Section 1
## New Section 2
```

After running `md toc`:
```markdown
# Document
<!-- md-toc: -->
- [New Section 1](#new-section-1)
- [New Section 2](#new-section-2)
<!-- md-toc: end -->

## New Section 1
## New Section 2
```

**Note:** If no `<!-- md-toc: -->` marker is found, the document is returned unchanged.

### Command: `table` (Auto-formatting and Spreadsheet Formulas)

#### Table Formatting

The `table` subcommand reads markdown from STDIN, formats and aligns any
markdown tables it finds, and outputs the entire document to STDOUT with
nicely formatted tables.

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

All content is preserved, but tables are properly aligned based on
column widths.

#### Table Formulas (Spreadsheet Functionality)

Tables can include spreadsheet-like formulas using HTML comments with the
`<!-- md-table: -->` marker.

**Basic formula syntax:**
- Formulas are defined in HTML comments after the table
- Format: `CELL = EXPRESSION` (e.g., `C1 = A1 + B1`)
- Cell references use spreadsheet notation: A1, B2, C3, etc. (Column letter + Row number)
- Row 1 is the first data row (header rows are not addressable in formulas)
- Multiple formulas can be separated by semicolons in one comment or placed on separate comment lines

**Supported operators:**
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Parentheses: `()` for grouping expressions (e.g., `(A1 + B1) * C1`)

**Example:**

Input:
```markdown
| Item | Price | Quantity | Total |
|---|---|---|---|
| Apple | 1.50 | 10 | 0 |
| Banana | 0.75 | 20 | 0 |
<!-- md-table: D1 = B1 * C1; D2 = B2 * C2 -->
```

Output:
```markdown
| Item   | Price | Quantity | Total |
| ------ | ----- | -------- | ----- |
| Apple  | 1.50  | 10       | 15    |
| Banana | 0.75  | 20       | 15    |
<!-- md-table: D1 = B1 * C1; D2 = B2 * C2 -->
```

**Multiple comment lines example:**
```markdown
| Product | Price | Tax | Total |
|---|---|---|---|
| Laptop | 1000 | 0 | 0 |
<!-- md-table: C1 = B1 * 0.08 -->
<!-- D1 = B1 + C1 -->
```

Formulas are evaluated in order, so later formulas can reference cells
updated by earlier formulas.


#### Vector and Matrix Operations

The table formula system supports vector operations, allowing you to
apply formulas to entire columns or rows at once.

**Vector Reference Syntax:**
- `A_` - Column A as a vector (all data rows)
- `_1` - Row 1 as a vector (all columns)
- Note: Column vectors must use the underscore suffix (`A_`), no shorthand

**Vector Assignment:**
```markdown
| Product  | Base | Tax | Total |
|----------|------|-----|-------|
| Laptop   | 1000 | 0   | 0     |
| Mouse    | 50   | 0   | 0     |
| Keyboard | 150  | 0   | 0     |
<!-- md-table: C_ = B_ * 0.08; D_ = B_ + C_ -->
```

Output:
```markdown
| Product  | Base | Tax  | Total  |
| -------- | ---- | ---- | ------ |
| Laptop   | 1000 | 80   | 1080   |
| Mouse    | 50   | 4    | 54     |
| Keyboard | 150  | 12   | 162    |
<!-- md-table: C_ = B_ * 0.08; D_ = B_ + C_ -->
```

**Broadcasting:**

Scalars automatically broadcast to vectors:
- `C_ = A_ * 0.5` multiplies every value in column A by 0.5
- `D_ = A_ + 100` adds 100 to every value in column A

**Exponentiation Operator:**

Use `^` for exponentiation (Excel-style):
```markdown
| Base | Squared |
|------|---------|
| 2    | 0       |
| 3    | 0       |
| 4    | 0       |
<!-- md-table: B_ = A_ ^ 2 -->
```

Output:
```markdown
| Base | Squared |
| ---- | ------- |
| 2    | 4       |
| 3    | 9       |
| 4    | 16      |
<!-- md-table: B_ = A_ ^ 2 -->
```

**Operator Precedence:**
1. Parentheses `()` (highest)
2. Exponentiation `^`
3. Multiplication `*` and Division `/`
4. Addition `+` and Subtraction `-` (lowest)

Example: `2 + 3 ^ 2 * 4` evaluates as `2 + ((3^2) * 4)` = `2 + (9 * 4)` = `2 + 36` = `38`

**Formula Functions:**

The formula system provides aggregate functions that work with both scalars and vectors/matrices.
All functions reduce matrices to scalar values.

**Available Functions:**

1. **`sum(expr)`** - Sum of all elements
   - Scalar: `sum(5)` → `5`
   - Vector: `sum(A_)` → `60` (where A_ contains values 10, 20, 30)
   - Complex: `sum(A_ + B_)` → Sum of element-wise addition

2. **`avg(expr)`** - Average of all elements
   - Scalar: `avg(5)` → `5`
   - Vector: `avg(A_)` → `20` (where A_ contains values 10, 20, 30)
   - Complex: `avg(A_ * 2)` → Average of doubled values

3. **`min(expr)`** - Minimum value across all elements
   - Scalar: `min(5)` → `5`
   - Vector: `min(A_)` → `10` (where A_ contains values 30, 10, 20)
   - Complex: `min(A_ + B_)` → Minimum of element-wise addition

4. **`max(expr)`** - Maximum value across all elements
   - Scalar: `max(5)` → `5`
   - Vector: `max(A_)` → `30` (where A_ contains values 30, 10, 20)
   - Complex: `max(A_ - B_)` → Maximum of element-wise subtraction

5. **`count(expr)`** - Number of elements
   - Scalar: `count(5)` → `1`
   - Vector: `count(A_)` → `3` (where A_ contains 3 values: 10, 20, 30)
   - Usage: `count(A_)` → Number of rows in column A

6. **`prod(expr)`** - Product of all elements
   - Scalar: `prod(5)` → `5`
   - Vector: `prod(A_)` → `24` (where A_ contains values 2, 3, 4; result is 2 × 3 × 4)
   - Complex: `prod(A_ + 1)` → Product of incremented values

7. **`from("table_id")` or `from("table_id", range)` or `from(variable)`** - Cross-table reference and variable access
   - Whole table: `from("sales")` → Entire table as matrix
   - Column: `from("sales", A_)` → Column A from sales table
   - Cell: `from("sales", B2)` → Cell B2 from sales table
   - Range: `from("sales", A1:C3)` → Range from sales table
   - Variable: `from(x)` → Access matrix variable (variables must be matrices)
   - See [Cross-Table References](#cross-table-references) and [Variables](#variables-let-statements) for details

**Example - Multiple Functions:**

Input:
```markdown
| Values | Sum | Avg | Min | Max | Count | Prod   |
| ------ | --- | --- | --- | --- | ----- | ------ |
| 10     | 0   | 0   | 0   | 0   | 0     | 0      |
| 20     | 0   | 0   | 0   | 0   | 0     | 0      |
| 30     | 0   | 0   | 0   | 0   | 0     | 0      |
| 40     | 0   | 0   | 0   | 0   | 0     | 0      |
<!-- md-table: B1 = sum(A_); C1 = avg(A_); D1 = min(A_); E1 = max(A_); F1 = count(A_); G1 = prod(A_) -->
```

Output:
```markdown
| Values | Sum | Avg  | Min | Max | Count | Prod   |
| ------ | --- | ---- | --- | --- | ----- | ------ |
| 10     | 100 | 25   | 10  | 40  | 4     | 240000 |
| 20     | 0   | 0    | 0   | 0   | 0     | 0      |
| 30     | 0   | 0    | 0   | 0   | 0     | 0      |
| 40     | 0   | 0    | 0   | 0   | 0     | 0      |
<!-- md-table: B1 = sum(A_); C1 = avg(A_); D1 = min(A_); E1 = max(A_); F1 = count(A_); G1 = prod(A_) -->
```

**Complex Function Expressions:**

All functions support complex nested expressions:
- `C1 = sum(A_ + B_)` - Sum of element-wise addition
- `D1 = avg(A_ * 2)` - Average of doubled values
- `E1 = max((A_ + B_ * 0.8) ^ 2)` - Maximum of squared weighted values
- `F1 = prod(A_ / 10)` - Product of scaled values

**Empty and Non-Numeric Cells:**

Empty or non-numeric cells in vectors are treated as zero:
```markdown
| Values |
|--------|
| 10     |
|        |
| text   |
| 30     |
<!-- md-table: A1 = sum(A_) -->
```
Result: A1 = 40 (only 10 and 30 are counted)

**Vector Operations:**
- `C_ = A_ + B_` - Element-wise addition
- `C_ = A_ - B_` - Element-wise subtraction
- `C_ = A_ * B_` - Element-wise multiplication
- `C_ = A_ / B_` - Element-wise division
- `C_ = A_ ^ B_` - Element-wise exponentiation

#### Variables (Let Statements)

The formula system supports variables that can store scalar values or matrices for reuse in multiple formulas.
Variables are defined using `let` statements and are scoped to the table's formula comment.

**Variable Syntax:**

```
let variable_name = expression
```

Variables can then be referenced in subsequent formulas within the same `md-table` directive.

**Example 1: Basic Scalar Variable**

Input:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 5   | 10  | 0   |
<!-- md-table: let x = 15; C1 = x -->
```

Output:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 5   | 10  | 15  |
<!-- md-table: let x = 15; C1 = x -->
```

**Example 2: Vector Variable**

Store column vectors in variables:

Input:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 0   |
| 3   | 4   | 0   |
| 5   | 6   | 0   |
<!-- md-table: let sum_col = A_ + B_; C_ = sum_col -->
```

Output:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 3   |
| 3   | 4   | 7   |
| 5   | 6   | 11  |
<!-- md-table: let sum_col = A_ + B_; C_ = sum_col -->
```

**Example 3: Multiple Variables with Expressions**

Variables can reference other variables defined earlier:

Input:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 5   | 10  | 0   | 0   |
<!-- md-table: let x = A1; let y = B1; let product = x * y; C1 = product; D1 = x + y -->
```

Output:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 5   | 10  | 50  | 15  |
<!-- md-table: let x = A1; let y = B1; let product = x * y; C1 = product; D1 = x + y -->
```

**Example 4: Variables with Functions**

Combine variables with aggregate functions:

Input:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 1   | 10  | 0   | 0   |
| 2   | 20  | 0   | 0   |
| 3   | 30  | 0   | 0   |
<!-- md-table: let col_a = A_; let col_b = B_; let total_a = sum(col_a); let total_b = sum(col_b); C1 = total_a; D1 = total_b -->
```

Output:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 1   | 10  | 6   | 60  |
| 2   | 20  | 0   | 0   |
| 3   | 30  | 0   | 0   |
<!-- md-table: let col_a = A_; let col_b = B_; let total_a = sum(col_a); let total_b = sum(col_b); C1 = total_a; D1 = total_b -->
```

**Example 5: Variables with Complex Expressions**

Variables work with all operators including transpose and matrix multiplication:

Input:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 2   | 3   | 0   |
| 4   | 5   | 0   |
<!-- md-table: let factor = 2; C_ = (A_ + B_) * factor -->
```

Output:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 2   | 3   | 10  |
| 4   | 5   | 18  |
<!-- md-table: let factor = 2; C_ = (A_ + B_) * factor -->
```

**Variable Naming Rules:**

- Variable names must not look like cell references (e.g., `A1`, `B_`, `_2` are invalid)
- Valid examples: `x`, `total`, `sum_col`, `result`, `factor`
- Invalid examples: `A1` (looks like cell), `B_` (looks like column), `_2` (looks like row)

**Variable Scope:**

- Variables are scoped to a single table's formula comment
- Each table has its own independent variable namespace
- Variables must be defined before they are used (sequential processing)
- Variables from one table cannot be accessed by another table

**Error Handling:**

Undefined variable usage produces clear error messages:

```markdown
| A   | B   |
| --- | --- |
| 5   | 0   |
<!-- md-table: B1 = undefined_var + 10 -->
<!-- md-error: Failed to evaluate expression 'undefined_var + 10': undefined variable: 'undefined_var' -->
```

Invalid variable names are caught during parsing:

```markdown
| A   | B   |
| --- | --- |
| 5   | 0   |
<!-- md-table: let A1 = 10 -->
<!-- md-error: Failed to parse statement 'let A1 = 10': invalid syntax (expected format: 'let VAR = EXPRESSION' or 'TARGET = EXPRESSION') -->
```

**Use Cases:**

- **Simplify complex formulas**: Define intermediate results once and reuse them
- **Improve readability**: Give meaningful names to values and calculations
- **Avoid repetition**: Store commonly used values or expressions
- **Build pipelines**: Chain multiple transformations step by step

#### Cell Range References

The formula system supports powerful range syntax for selecting rectangular regions
of cells, making it easy to work with matrices and perform operations on multiple
rows and columns at once.

**Range Syntax Types:**

1. **Scalar Ranges** (`A1:C5`) - Rectangular cell ranges
   - Creates a matrix from the specified rectangular region
   - Format: `START_CELL:END_CELL` (e.g., `A1:C5`, `B2:D10`)
   - Dimensions: `(end_row - start_row + 1) × (end_col - start_col + 1)`

2. **Column Ranges** (`A_:C_`) - Multiple column vectors
   - Selects all data rows across multiple columns
   - Format: `START_COL_:END_COL_` (e.g., `A_:C_`, `B_:E_`)
   - Uses all rows without needing to specify the table size
   - Creates an `n×m` matrix (n = number of data rows, m = number of columns)

3. **Row Ranges** (`_1:_5`) - Multiple row vectors
   - Selects all columns across multiple rows
   - Format: `_START_ROW:_END_ROW` (e.g., `_1:_3`, `_2:_5`)
   - Uses all columns without needing to specify the table width
   - Creates an `m×n` matrix (m = number of rows, n = number of columns)

**Range Restrictions:**
- Cannot mix different reference types: `A_:_5` is invalid
- Start must be ≤ end for all ranges
- Both start and end must use the same reference type

**Example 1 - Scalar Range (A1:C3):**

Creates a 3×3 matrix:

```markdown
| A | B | C | Sum |
|---|---|---|-----|
| 1 | 2 | 3 | 0   |
| 4 | 5 | 6 | 0   |
| 7 | 8 | 9 | 0   |
<!-- md-table: D1 = sum(A1:C3) -->
```

Output:
```markdown
| A | B | C | Sum |
|---|---|---|-----|
| 1 | 2 | 3 | 45  |
| 4 | 5 | 6 | 0   |
| 7 | 8 | 9 | 0   |
<!-- md-table: D1 = sum(A1:C3) -->
```

Result: 1+2+3+4+5+6+7+8+9 = 45

**Example 2 - Column Range (A_:C_):**

Selects all data rows for columns A through C:

```markdown
| A  | B  | C  | Total |
|----|----|----|-------|
| 10 | 20 | 30 | 0     |
| 40 | 50 | 60 | 0     |
| 70 | 80 | 90 | 0     |
<!-- md-table: D1 = sum(A_:C_); D2 = avg(A_:C_); D3 = max(A_:C_) -->
```

Output:
```markdown
| A  | B  | C  | Total |
|----|----|----|-------|
| 10 | 20 | 30 | 450   |
| 40 | 50 | 60 | 50    |
| 70 | 80 | 90 | 90    |
<!-- md-table: D1 = sum(A_:C_); D2 = avg(A_:C_); D3 = max(A_:C_) -->
```

**Example 3 - Row Range (_1:_3):**

Selects all columns for rows 1 through 3:

```markdown
| A | B | C |
|---|---|---|
| 1 | 2 | 3 |
| 4 | 5 | 6 |
| 7 | 8 | 9 |
| 0 | 0 | 0 |
<!-- md-table: A4 = sum(_1:_3) -->
```

Output:
```markdown
| A  | B | C |
|----|---|---|
| 1  | 2 | 3 |
| 4  | 5 | 6 |
| 7  | 8 | 9 |
| 45 | 0 | 0 |
<!-- md-table: A4 = sum(_1:_3) -->
```

Result: 1+2+3+4+5+6+7+8+9 = 45

**Example 4 - Range in Complex Expression:**

Ranges work seamlessly with all operators and functions:

```markdown
| Q1 | Q2 | Q3 | Q4 | Total |
|----|----|----|----|-------|
| 100| 150| 200| 250| 0     |
| 80 | 120| 160| 200| 0     |
| 0  | 0  | 0  | 0  | 0     |
<!-- md-table: E1 = sum(A1:D1); E2 = sum(A2:D2); E3 = sum(A_:D_) -->
```

Output:
```markdown
| Q1  | Q2  | Q3  | Q4  | Total |
|-----|-----|-----|-----|-------|
| 100 | 150 | 200 | 250 | 700   |
| 80  | 120 | 160 | 200 | 560   |
| 0   | 0   | 0   | 0   | 1260  |
<!-- md-table: E1 = sum(A1:D1); E2 = sum(A2:D2); E3 = sum(A_:D_) -->
```

**Range Equivalences:**

- `A_:A_` is equivalent to `A_` (single column)
- `_1:_1` is equivalent to `_1` (single row)
- `A1:A3` is a 3×1 column vector
- `A1:C1` is a 1×3 row vector
- `A1:A1` is a scalar (1×1 matrix automatically converted)

**Using Ranges with All Functions:**

All aggregate functions work with ranges:
- `sum(A1:C3)` - Sum of all cells in range
- `avg(A_:D_)` - Average across multiple columns
- `max(_1:_5)` - Maximum across multiple rows
- `min(A1:C3 * 2)` - Minimum of doubled values in range
- `count(A_:C_)` - Total number of cells in column range
- `prod(A1:A3)` - Product of cells in range

**Performance Note:**

Column ranges (`A_:C_`) and row ranges (`_1:_5`) are especially useful
when working with dynamic tables where the number of rows or columns may
change. They automatically adapt to the table size without requiring
formula updates.

#### Matrix Assignments

The formula system supports assigning entire matrices, ranges, and vectors in a single formula.
This enables powerful bulk operations where you can update multiple cells at once with properly
dimensioned results.

**Supported Assignment Types:**

1. **Scalar Assignment** - `A1 = expr` (single cell)
2. **Column Vector Assignment** - `A_ = expr` (entire column)
3. **Row Vector Assignment** - `_1 = expr` (entire row)
4. **Range Assignment** - `A1:C3 = expr` (rectangular region)
5. **Column Range Assignment** - `A_:C_ = expr` (multiple columns)
6. **Row Range Assignment** - `_1:_3 = expr` (multiple rows)

**Important:** The dimensions of the result must exactly match the dimensions of the assignment target.

**Example 1: Row Vector Assignment**

Assign the result of a row vector operation to an entire row:

Input:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 3   |
| 0   | 0   | 0   |
<!-- md-table: _2 = _1 * 2 -->
```

Output:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 3   |
| 2   | 4   | 6   |
<!-- md-table: _2 = _1 * 2 -->
```

**Example 2: Range Assignment**

Assign a matrix result to a rectangular range:

Input:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 1   | 2   | 0   | 0   |
| 3   | 4   | 0   | 0   |
<!-- md-table: C1:D2 = A1:B2 + 10 -->
```

Output:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 1   | 2   | 11  | 12  |
| 3   | 4   | 13  | 14  |
<!-- md-table: C1:D2 = A1:B2 + 10 -->
```

**Example 3: Column Range Assignment**

Update multiple columns at once:

Input:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 1   | 2   | 0   | 0   |
| 3   | 4   | 0   | 0   |
| 5   | 6   | 0   | 0   |
<!-- md-table: C_:D_ = A_:B_ * 10 -->
```

Output:
```markdown
| A   | B   | C   | D   |
| --- | --- | --- | --- |
| 1   | 2   | 10  | 20  |
| 3   | 4   | 30  | 40  |
| 5   | 6   | 50  | 60  |
<!-- md-table: C_:D_ = A_:B_ * 10 -->
```

**Example 4: Row Range Assignment**

Update multiple rows in one formula:

Input:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 3   |
| 4   | 5   | 6   |
| 0   | 0   | 0   |
| 0   | 0   | 0   |
<!-- md-table: _3:_4 = _1:_2 * 10 -->
```

Output:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 3   |
| 4   | 5   | 6   |
| 10  | 20  | 30  |
| 40  | 50  | 60  |
<!-- md-table: _3:_4 = _1:_2 * 10 -->
```

**Dimension Validation:**

Matrix assignments require exact dimension matches. If the dimensions don't match, you'll get a descriptive error:

```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 0   |
| 3   | 4   | 0   |
<!-- md-table: C_ = A1:B2 -->
<!-- md-error: Assignment failed: expected column vector but got matrix result -->
```

**Practical Use Cases:**

- **Data transformation**: `C_:D_ = A_:B_ * conversion_factor`
- **Row copying**: `_5:_10 = _1:_6`
- **Region updates**: `D1:F3 = A1:C3 + B1:C3`
- **Bulk calculations**: Compute multiple related values in one formula

**Notes:**
- Matrix assignments follow the same dimension rules as matrix operations
- Scalars cannot be assigned to matrix targets (use broadcasting instead: `C_ = A_ * 2`)
- Assignment targets can use any valid range syntax from [Cell Range References](#cell-range-references)

#### Matrix Multiplication and Transpose Operator

The formula system supports matrix multiplication using the `@` operator,
following standard linear algebra rules. Vectors are treated as matrices
with dimension tracking.

**Matrix Dimensions:**
- Column vectors (`A_`, `B_`, etc.) are n×1 matrices
- Row vectors (`_1`, `_2`, etc.) are 1×n matrices
- Matrix multiplication follows the rule: (m×n) @ (n×p) = (m×p)
- Dimensions must match for multiplication to work

**Transpose Operator (`.T`):**

The `.T` operator transposes vectors, converting between row and column vectors:
- `A_.T` - Transpose column A to a row vector (n×1 → 1×n)
- `_1.T` - Transpose row 1 to a column vector (1×n → n×1)

**Dot Product Example:**

Computing a dot product using transpose and matrix multiplication:

```markdown
| A | B | Result |
|---|---|--------|
| 1 | 2 | 0      |
| 4 | 5 | 0      |
| 7 | 8 | 0      |
<!-- md-table: C1 = A_.T @ B_ -->
```

Output:
```markdown
| A | B | Result |
|---|---|--------|
| 1 | 2 | 78     |
| 4 | 5 | 0      |
| 7 | 8 | 0      |
<!-- md-table: C1 = A_.T @ B_ -->
```

**Calculation:** A_.T @ B_ = [1, 4, 7] @ [2; 5; 8] = 1×2 + 4×5 + 7×8 = 2 + 20 + 56 = 78

**Row-Column Multiplication:**

```markdown
| A | B | C |
|---|---|---|
| 1 | 2 | 0 |
| 3 | 4 | 0 |
| 5 | 6 | 0 |
<!-- md-table: C1 = _1 @ A_ -->
```

Result: C1 = _1 @ A_ = [1, 2, 0] @ [1; 3; 5] = 1×1 + 2×3 + 0×5 = 7

**Complex Matrix Expressions:**

Matrix operations can be combined with other operators:

```markdown
| A | B | Result |
|---|---|--------|
| 1 | 2 | 0      |
| 3 | 4 | 0      |
| 5 | 6 | 0      |
<!-- md-table: C1 = (A_.T @ B_) + 10 -->
```

Result: C1 = (1×2 + 3×4 + 5×6) + 10 = 44 + 10 = 54

**Operator Precedence (Updated):**
1. Parentheses `()` and Transpose `.T` (highest)
2. Exponentiation `^`
3. Matrix multiplication `@`, Multiplication `*`, and Division `/`
4. Addition `+` and Subtraction `-` (lowest)

**All operators and functions supported:**
- Arithmetic: `+`, `-`, `*`, `/`, `^`
- Matrix multiplication: `@`
- Transpose: `.T`
- Parentheses: `()`
- Functions: `sum()`, `avg()`, `min()`, `max()`, `count()`, `prod()`

#### Formula Error Handling

When formulas contain errors, the system reports them using HTML comments
with the `<!-- md-error: -->` marker. Error comments are inserted directly
after the formula that failed.

**Enhanced Error Messages with Position Tracking:**

Error messages now include visual indicators pointing to the exact location
of the error in the expression, making debugging faster and easier.

**Error Types:**

1. **Parse Errors** - Invalid formula syntax
2. **Evaluation Errors** - Invalid expressions or references
3. **Assignment Errors** - Type mismatches or out-of-bounds assignments

**Example - Unknown Function Error:**

Input:
```markdown
| A | B |
|---|---|
| 1 | 0 |
| 2 | 0 |
<!-- md-table: B1 = foo(A_) -->
```

Output:
```markdown
| A   | B   |
| --- | --- |
| 1   | 0   |
| 2   | 0   |
<!-- md-table: B1 = foo(A_) -->
<!-- md-error: Failed to evaluate expression:
unknown function: 'foo' (supported functions: sum, avg, min, max, count, prod)
foo(A_)
^^^ -->
```

Notice the `^^^` indicator pointing to the exact location of the unknown function.

**Example - Column Out of Bounds Error:**

Input:
```markdown
| A | B | C |
|---|---|---|
| 1 | 2 | 0 |
| 3 | 4 | 0 |
<!-- md-table: C_ = A_ + B_; D_ = X_ + Y_ -->
```

Output:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 3   |
| 3   | 4   | 7   |
<!-- md-table: C_ = A_ + B_; D_ = X_ + Y_ -->
<!-- md-error: Failed to evaluate expression:
column vector X_ is out of bounds: column X does not exist (table has 3 columns)
X_ + Y_
^ -->
```

Note: The first formula (`C_ = A_ + B_`) succeeded and updated column C,
while the second formula (`D_ = X_ + Y_`) failed with a precise error message
showing exactly where the problem occurs. Each formula is evaluated independently.

**Example - Parse Error:**

Input:
```markdown
| A | B | C |
|---|---|---|
| 1 | 2 | 0 |
| 3 | 4 | 0 |
<!-- md-table: this is invalid -->
```

Output:
```markdown
| A   | B   | C   |
| --- | --- | --- |
| 1   | 2   | 0   |
| 3   | 4   | 0   |
<!-- md-table: this is invalid -->
<!-- md-error: Failed to parse statement 'this is invalid': invalid syntax (expected format: 'let VAR = EXPRESSION' or 'TARGET = EXPRESSION') -->
```

**Error Behavior:**
- Errors don't stop processing of subsequent formulas
- Each formula is evaluated independently
- Successful formulas update their cells even if others fail
- Error messages are descriptive and include the failing formula

#### Table IDs

Tables can be assigned optional identifiers using the `id` attribute in the `md-table` directive.
This allows tables to reference each other's data using the `from()` function in formulas.

**ID Syntax:**

Table IDs are specified as an attribute in the `md-table` directive, similar to code blocks:

```markdown
| Name | Age |
|------|-----|
| Alice| 30  |
| Bob  | 25  |
<!-- md-table: id="employee_data" -->
```

**ID Validation Rules:**

Table IDs must follow these rules:
- Cannot be empty
- Any other non-empty string is valid

**Valid ID examples:**
- `id="sales_data"`
- `id="table1"`
- `id="my_table_123"`
- `id="_results"`
- `id="sales data"` (whitespace is allowed)
- `id="sales-data"` (hyphens are allowed)
- `id="sales.data"` (periods are allowed)
- `id="2024-Q1-sales"` (any characters are allowed)
- `id="user@domain"` (special characters are allowed)

**Invalid ID examples:**
- `id=""` (empty)

**Combining IDs with formulas:**

You can use both an ID and formulas in the same directive:

```markdown
| Product | Price | Tax | Total |
|---------|-------|-----|-------|
| Laptop  | 1000  | 0   | 0     |
| Mouse   | 25    | 0   | 0     |
<!-- md-table: id="product_table"; C_ = B_ * 0.08; D_ = B_ + C_ -->
```

The `id` attribute comes first, followed by formulas separated by semicolons.

**Error handling:**

Empty table IDs produce descriptive error messages:

Input:
```markdown
| A | B |
|---|---|
| 1 | 2 |
<!-- md-table: id="" -->
```

Output:
```markdown
| A   | B   |
| --- | --- |
| 1   | 2   |
<!-- md-table: id="" -->
<!-- md-error: Invalid table ID: ID cannot be empty -->
```

**Notes:**
- Table IDs are optional (unlike code blocks where IDs are required for execution)
- IDs can be any non-empty string
- IDs should be unique within the document (not currently enforced - planned future enhancement)
- The ID validation logic is shared with code block IDs for consistency

#### Cross-Table References

Tables with IDs can reference data from other tables in the same document using the `from()` function.
This enables powerful data flows between tables, allowing you to build dashboards, summaries, and aggregations.

**The `from()` function:**

The `from()` function retrieves data from another table by its ID. It has two forms:

1. **Full table reference:** `from("table_id")` - Returns the entire data portion of the table as a matrix
2. **Range reference:** `from("table_id", range)` - Returns a specific cell, column, row, or range from the table

**Syntax:**
```
from("table_id")          # Entire table as matrix
from("table_id", A_)      # Column A from the table
from("table_id", _1)      # Row 1 from the table
from("table_id", A1)      # Single cell from the table
from("table_id", A1:C3)   # Range from the table
```

**Example 1: Sum data from another table**

Input:
```markdown
| Value |
| ----- |
| 10    |
| 20    |
| 30    |
<!-- md-table: id="source" -->

| Total |
| ----- |
| 0     |
<!-- md-table: A1 = sum(from("source", A_)) -->
```

Output:
```markdown
| Value |
| ----- |
| 10    |
| 20    |
| 30    |
<!-- md-table: id="source" -->

| Total |
| ----- |
| 60    |
<!-- md-table: A1 = sum(from("source", A_)) -->
```

**Example 2: Reference entire table**

Input:
```markdown
| A   | B   |
| --- | --- |
| 1   | 2   |
| 3   | 4   |
<!-- md-table: id="data" -->

| Sum |
| --- |
| 0   |
<!-- md-table: A1 = sum(from("data")) -->
```

Output:
```markdown
| A   | B   |
| --- | --- |
| 1   | 2   |
| 3   | 4   |
<!-- md-table: id="data" -->

| Sum |
| --- |
| 10  |
<!-- md-table: A1 = sum(from("data")) -->
```

**Example 3: Multiple aggregations**

```markdown
| Sales |
| ----- |
| 100   |
| 200   |
| 150   |
<!-- md-table: id="sales" -->

| Metric  | Value |
| ------- | ----- |
| Total   | 0     |
| Average | 0     |
| Maximum | 0     |
<!-- md-table: id="summary"; B1 = sum(from("sales", A_)); B2 = avg(from("sales", A_)); B3 = max(from("sales", A_)) -->
```

**Error handling:**

If you reference a table that doesn't exist, an error is displayed:

```markdown
| Result |
| ------ |
| 0      |
<!-- md-table: A1 = from("missing") -->
<!-- md-error: Failed to evaluate expression 'from("missing")': table 'missing' not found (tables must have an id attribute) -->
```

**Supported functions with `from()`:**

All aggregate functions work with cross-table references:
- `sum(from("id", range))` - Sum of values
- `avg(from("id", range))` - Average of values
- `min(from("id", range))` - Minimum value
- `max(from("id", range))` - Maximum value
- `count(from("id", range))` - Count of values
- `prod(from("id", range))` - Product of values

**Notes:**
- Table IDs should be unique within the document (duplicate IDs will cause the later table to override the earlier one)
- The referenced table must appear before the formula is evaluated (tables are processed top-to-bottom)
- Empty or non-numeric cells in the source table are treated as 0
- String literals in formulas (like `"table_id"`) must be enclosed in double quotes
- Cross-table references can be combined with other operations and functions

### Command: `code` (Code Execution)

The `code` subcommand allows you to execute code blocks in markdown
files and automatically capture their output. This is useful for creating
executable documentation, tutorials, or notebooks.

**Basic usage:**
```bash
./result/bin/md code < document.md
```

**How it works:**
- Code blocks with `md-code` directives are executed
- Output is automatically captured and inserted into the document
- Regular code blocks without directives are left untouched
- All content is preserved exactly as-is

**Basic directive syntax:**

Code blocks are marked with HTML comments using the `<!-- md-code: -->` marker immediately after the code fence.

~~~markdown
```python
print("Hello, world!")
```
<!-- md-code: id="hello"; execute; bin="python3" -->
~~~

**Directive parameters:**
- `id="unique-id"` (required) - Unique identifier for the code block
- `execute` (flag, required for execution) - Marks the code block for execution
- `bin="command"` (required if execute is set) - The command to run (e.g., `"python3"`, `"node"`, `"bash"`)
- `timeout=N` (optional) - Timeout in seconds (default: 30)

**Example - Python code execution:**

Input:
~~~markdown
```python
x = 10
y = 20
print(f"The sum is {x + y}")
```
<!-- md-code: id="sum"; execute; bin="python3" -->
~~~

Output:
~~~markdown
```python
x = 10
y = 20
print(f"The sum is {x + y}")
```
<!-- md-code: id="sum"; execute; bin="python3" -->

Output:
```
The sum is 30
```
<!-- md-code-output: id="sum" -->
~~~

**Example - With custom timeout:**

~~~markdown
```python
import time
time.sleep(2)
print("Done!")
```
<!-- md-code: id="slow"; execute; bin="python3"; timeout=5 -->
~~~

**Example - Bash script:**

~~~markdown
```bash
echo "Current directory: $(pwd)"
ls -la | head -5
```
<!-- md-code: id="pwd"; execute; bin="bash" -->
~~~

**Example - Command with arguments:**

~~~markdown
```python
print("unbuffered output")
```
<!-- md-code: id="unbuf"; execute; bin="python3 -u" -->
~~~

**Output block management:**
- Output blocks are automatically created after code blocks when they produce output
- If you run the command again, existing output blocks are updated
- Empty output (no stdout/stderr) does not create an output block
- Output blocks are marked with `<!-- md-code-output: id="..." -->` for tracking

**Multiple executions:**

You can have multiple code blocks in the same document, each with unique IDs:

~~~markdown
```python
print("First block")
```
<!-- md-code: id="first"; execute; bin="python3" -->

```python
print("Second block")
```
<!-- md-code: id="second"; execute; bin="python3" -->
~~~

**Important notes:**
- Each code block must have a unique `id`
- Regular code blocks without `md-code` directives are completely ignored
- Both stdout and stderr are captured in the output
- The tool is idempotent - running it multiple times updates the same output blocks

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
