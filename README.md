Markdown Utils
==============

A Rust CLI tool for markdown processing with multiple subcommands. This
is heavily based on some of the useful features I used when I used to
use GNU/Emacs and Org-mode. Spreadsheet and literate programming were
really cool features but were limited to GNU/Emacs. Emacs is realy
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

### Creating New Tables

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

### Table Formatting

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

### Table Formulas (Spreadsheet Functionality)

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

### Vector and Matrix Operations

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
   - Vector: `sum([10, 20, 30])` → `60`
   - Complex: `sum(A_ + B_)` → Sum of element-wise addition

2. **`avg(expr)`** - Average of all elements
   - Scalar: `avg(5)` → `5`
   - Vector: `avg([10, 20, 30])` → `20`
   - Complex: `avg(A_ * 2)` → Average of doubled values

3. **`min(expr)`** - Minimum value across all elements
   - Scalar: `min(5)` → `5`
   - Vector: `min([30, 10, 20])` → `10`
   - Complex: `min(A_ + B_)` → Minimum of element-wise addition

4. **`max(expr)`** - Maximum value across all elements
   - Scalar: `max(5)` → `5`
   - Vector: `max([30, 10, 20])` → `30`
   - Complex: `max(A_ - B_)` → Maximum of element-wise subtraction

5. **`count(expr)`** - Number of elements
   - Scalar: `count(5)` → `1`
   - Vector: `count([10, 20, 30])` → `3`
   - Usage: `count(A_)` → Number of rows in column A

6. **`prod(expr)`** - Product of all elements
   - Scalar: `prod(5)` → `5`
   - Vector: `prod([2, 3, 4])` → `24` (2 × 3 × 4)
   - Complex: `prod(A_ + 1)` → Product of incremented values

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

### Matrix Multiplication and Transpose Operator

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

### Formula Error Handling

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
<!-- md-error: Failed to parse formula 'this is invalid': invalid syntax (expected format: TARGET = EXPRESSION) -->
```

**Error Behavior:**
- Errors don't stop processing of subsequent formulas
- Each formula is evaluated independently
- Successful formulas update their cells even if others fail
- Error messages are descriptive and include the failing formula

### Code Block Execution

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
