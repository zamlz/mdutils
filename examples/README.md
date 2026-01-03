# mdutils Examples

This directory contains real-world examples demonstrating the capabilities of `md` (mdutils). Each example showcases different features and use cases.

## Examples Overview

### 1. Budget Tracker (`budget.md`)

**Features:** Table formulas, arithmetic operations, vector operations

A monthly budget tracker that automatically calculates:
- Total income from multiple sources
- Budget vs. actual expenses
- Net savings

**Usage:**
```bash
md table < budget.md
```

**Key Features Demonstrated:**
- `sum()` function for totaling values
- Vector operations (`D_ = B_ - C_`) for calculating differences across rows
- Cross-table references for summary calculations

---

### 2. Grade Calculator (`grades.md`)

**Features:** Weighted calculations, statistical functions

A classroom grade calculator that computes:
- Weighted final grades (30% homework, 30% midterm, 40% final)
- Class statistics (average, min, max, count)

**Usage:**
```bash
md table < grades.md
```

**Key Features Demonstrated:**
- Weighted formula calculations
- Statistical functions: `avg()`, `min()`, `max()`, `count()`
- Vector operations for batch calculations

---

### 3. Data Analysis Notebook (`data-analysis.md`)

**Features:** Code execution, Python integration, TOC generation

A data analysis notebook that performs:
- Statistical calculations (mean, variance, standard deviation)
- Data normalization
- Multiple analysis stages

**Usage:**
```bash
# Generate table of contents
md toc < data-analysis.md > temp.md

# Execute code blocks
md code < temp.md > data-analysis.md
```

**Key Features Demonstrated:**
- Python code execution
- Writing output to stderr for capture
- Organizing analysis into sections with TOC
- Multi-stage data processing

---

### 4. Project Documentation (`project-docs.md`)

**Features:** Table of contents generation, document organization

Comprehensive project documentation with:
- Automatically generated table of contents
- Multi-level section hierarchy
- API reference structure

**Usage:**
```bash
md toc < project-docs.md
```

**Key Features Demonstrated:**
- Automatic TOC generation with `<!-- md-toc: -->`
- GitHub-style anchor links
- Deep nesting support
- Duplicate header handling

---

### 5. Scientific Computation (`scientific-computation.md`)

**Features:** Advanced Python code execution, mathematical operations

Scientific computing examples including:
- Fibonacci sequence generation
- Prime number detection
- Matrix multiplication
- Numerical integration

**Usage:**
```bash
md code < scientific-computation.md
```

**Key Features Demonstrated:**
- Complex Python algorithms
- Mathematical computations
- Multiple independent code blocks
- Reproducible computational results

---

### 6. Invoice Generator (`invoice.md`)

**Features:** Table formulas, variables, currency calculations

Professional invoice with:
- Automatic line item totals (quantity × price)
- Subtotal calculation
- Tax computation
- Grand total

**Usage:**
```bash
md table < invoice.md
```

**Key Features Demonstrated:**
- `let` statements for reusable values
- Vector operations for line items
- Percentage calculations
- Currency-style formatting

---

### 7. Meta-Programming (`meta-programming.md`)

**Features:** Self-referential markdown processing

Demonstrates `md` processing markdown that contains `md` directives:
- Processing table formulas within code blocks
- Generating TOCs for embedded documents
- Multi-level processing pipelines

**Usage:**
```bash
md code < meta-programming.md
```

**Key Features Demonstrated:**
- Nested markdown processing
- Different fence types (`` ``` `` vs `~~~`)
- Multi-stage transformations
- Self-documenting examples

---

## Quick Start

### Prerequisites

Build and install `md`:
```bash
nix build
# The binary is at ./result/bin/md
```

Or use it through nix develop:
```bash
nix develop --command md --help
```

### Running Examples

Each example can be processed with the appropriate subcommand:

```bash
# Process tables
md table < budget.md > budget-updated.md
md table < grades.md > grades-updated.md
md table < invoice.md > invoice-updated.md

# Execute code
md code < data-analysis.md > data-analysis-results.md
md code < scientific-computation.md > sci-comp-results.md
md code < meta-programming.md > meta-prog-output.md

# Generate TOC
md toc < project-docs.md > project-docs-toc.md
md toc < data-analysis.md > data-analysis-toc.md
```

### In-place Updates

To update a file in place:
```bash
md table < budget.md > temp.md && mv temp.md budget.md
```

Or using shell redirection (careful - test first!):
```bash
md table < budget.md | sponge budget.md  # requires moreutils
```

## Idempotency Property

All examples are designed to be **idempotent** - running the same command on the output produces unchanged results:

```bash
# First run
md table < budget.md > output1.md

# Second run (should be identical to output1.md)
md table < output1.md > output2.md

# Verify
diff output1.md output2.md  # No differences!
```

This property ensures:
- Deterministic, reproducible results
- Safe to run multiple times
- Version control friendly (no spurious changes)

## Tips and Best Practices

### Table Formulas

1. **Avoid self-referential formulas**: Don't write to a cell while reading from its row/column
   - Bad: `C1 = _1 @ A_` (C1 is in row 1)
   - Good: `C3 = _1 @ A_` (C3 is in row 3)

2. **Use variables for clarity**:
   ```
   let tax_rate = 0.085
   B2 = B1 * tax_rate
   ```

3. **Vector operations for efficiency**:
   ```
   D_ = B_ * C_  # Instead of D1 = B1 * C1, D2 = B2 * C2, etc.
   ```

### Code Execution

1. **Write to stderr for output**: Output blocks capture stderr, not stdout
   ```python
   import sys
   sys.stderr.write(f"Result: {value}\n")
   ```

2. **Use unique IDs**: Each code block needs a unique identifier
   ```markdown
   <!-- md-code: id="analysis_step1"; bin="python3" -->
   ```

3. **Specify timeout for long-running code**:
   ```markdown
   <!-- md-code: id="complex"; bin="python3"; timeout=60 -->
   ```

### Table of Contents

1. **Place marker before content**: The TOC generates links only for headers after the marker
2. **Update when structure changes**: Re-run `md toc` after adding/removing sections
3. **Use meaningful header names**: They become URL-friendly anchor links

## Creating Your Own Examples

Start with a template:

**For tables:**
```markdown
| Column1 | Column2 | Result |
|---------|---------|--------|
| 10      | 20      | 0      |
<!-- md-table: C1 = A1 + B1 -->
```

**For code:**
```markdown
```python
import sys
sys.stderr.write("Hello from Python!\n")
```
<!-- md-code: id="hello"; bin="python3" -->
```

**For TOC:**
```markdown
<!-- md-toc: -->
<!-- md-toc: end -->

# Your sections here...
```

Then process with:
```bash
md table < yourfile.md    # For tables
md code < yourfile.md     # For code
md toc < yourfile.md      # For TOC
```

## Contributing

Have a great example? Submit a pull request! Good examples:
- Solve a real-world problem
- Demonstrate specific features clearly
- Include helpful comments
- Are idempotent

## Learn More

- See the main [README](../README.md) for complete documentation
- Check [CLAUDE.md](../CLAUDE.md) for architecture details
- Read the [integration tests](../tests/) for more examples
