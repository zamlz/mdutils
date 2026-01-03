# Meta-Programming: md Processing Markdown

This example demonstrates `md` processing markdown that contains `md` directives. This is useful for creating dynamic documentation that processes itself.

## Example 1: Processing a Table

Here's a markdown table with formulas:

```markdown
| Product  | Price | Quantity | Total |
|----------|-------|----------|-------|
| Apples   | 1.50  | 10       | 0     |
| Oranges  | 2.00  | 5        | 0     |
| Bananas  | 0.80  | 12       | 0     |
| **Sum**  |       |          | 0     |
<!-- md-table:
  D_ = B_ * C_
  D4 = sum(D1:D3)
-->
```
<!-- md-code: id="table_example"; bin="md table" -->

## Example 2: Generating a Table of Contents

Here's a document that needs a TOC:

```markdown
# User Guide

<!-- md-toc: -->
<!-- md-toc: end -->

# Installation

Instructions for installing the software.

## Requirements

List of prerequisites.

## Steps

Step-by-step installation guide.

# Configuration

How to configure the application.

## Basic Settings

Common configuration options.

## Advanced Settings

Advanced configuration for power users.

# Usage

How to use the application.
```
<!-- md-code: id="toc_example"; bin="md toc" -->

## Example 3: Data Analysis Pipeline

Process data through multiple stages:

~~~markdown
Calculate statistics on a dataset:

```python
import sys
data = [10, 20, 30, 40, 50]
mean = sum(data) / len(data)
sys.stderr.write(f"Mean: {mean}\n")
sys.stderr.write(f"Sum: {sum(data)}\n")
```
<!-- md-code: id="stats"; bin="python3" -->
~~~
<!-- md-code: id="pipeline"; bin="md code" -->

## Usage

To process this meta-programming example:

```bash
# Process all the examples
md code < meta-programming.md
```

This will execute the `md` commands on the embedded markdown examples, showing how `md` can process markdown that contains `md` directives.
