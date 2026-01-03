# Meta-Programming: md Processing Markdown

This example demonstrates `md` processing markdown that contains `md`
directives. This is useful for creating dynamic documentation that
processes itself.

## Example 1: Processing a Table

Here's a markdown table with formulas:

```markdown
| Product  | Price | Quantity | Total |
|----------|-------|----------|-------|
| Apples   | 1.50  | 10       | 0     |
| Oranges  | 2.00  | 5        | 0     |
| Bananas  | 0.80  | 12       | 0     |
| **Sum**  |       |          | 0     |
<!-- md-table: D_ = B_ * C_; D4 = sum(D1:D3) -->
```
<!-- md-code: id="table_example"; bin="md table" -->

Output:
```
| Product    | Price   | Quantity   | Total   |
| ---------- | ------- | ---------- | ------- |
| Apples     | 1.50    | 10         | 15.00   |
| Oranges    | 2.00    | 5          | 10.00   |
| Bananas    | 0.80    | 12         | 9.60    |
| **Sum**    |         |            | 34.60   |
<!-- md-table: D_ = B_ * C_; D4 = sum(D1:D3) -->
```
<!-- md-code-output: id="table_example" -->

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

Output:
```
# User Guide

<!-- md-toc: -->
- [Installation](#installation)
  - [Requirements](#requirements)
  - [Steps](#steps)
- [Configuration](#configuration)
  - [Basic Settings](#basic-settings)
  - [Advanced Settings](#advanced-settings)
- [Usage](#usage)
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
<!-- md-code-output: id="toc_example" -->

## Example 3: Data Analysis Pipeline

Process data through multiple stages:

~~~markdown
Calculate statistics on a dataset:

```python
import sys
data = [10, 20, 30, 40, 50]
mean = sum(data) / len(data)
print(f"Mean: {mean}\n")
print(f"Sum: {sum(data)}\n")
```
<!-- md-code: id="stats"; bin="python3" -->
~~~
<!-- md-code: id="pipeline"; bin="md code" -->

Output:
~~~
Calculate statistics on a dataset:

```python
import sys
data = [10, 20, 30, 40, 50]
mean = sum(data) / len(data)
print(f"Mean: {mean}\n")
print(f"Sum: {sum(data)}\n")
```
<!-- md-code: id="stats"; bin="python3" -->

Output:
```
Mean: 30.0

Sum: 150


```
<!-- md-code-output: id="stats" -->
~~~
<!-- md-code-output: id="pipeline" -->

## Usage

To process this meta-programming example:

```bash
# Process all the examples
md code < meta-programming.md
```

This will execute the `md` commands on the embedded markdown examples,
showing how `md` can process markdown that contains `md` directives.
