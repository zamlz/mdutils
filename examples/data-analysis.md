# Data Analysis Notebook

This example demonstrates using code execution for data analysis tasks.

<!-- md-toc: -->
<!-- md-toc: end -->

## Dataset Summary

Let's analyze a simple dataset using Python.

```python
import sys

data = [23, 45, 67, 89, 34, 56, 78, 90, 12, 43]

sys.stderr.write(f"Count: {len(data)}\n")
sys.stderr.write(f"Mean: {sum(data)/len(data):.2f}\n")
sys.stderr.write(f"Min: {min(data)}\n")
sys.stderr.write(f"Max: {max(data)}\n")
sys.stderr.write(f"Range: {max(data) - min(data)}\n")
```
<!-- md-code: id="summary_stats"; bin="python3" -->

## Data Transformation

Apply a transformation (e.g., normalize to 0-100 scale).

```python
import sys

data = [23, 45, 67, 89, 34, 56, 78, 90, 12, 43]
min_val = min(data)
max_val = max(data)

normalized = [(x - min_val) / (max_val - min_val) * 100 for x in data]

sys.stderr.write("Normalized values:\n")
for i, (orig, norm) in enumerate(zip(data, normalized)):
    sys.stderr.write(f"  {orig} -> {norm:.1f}\n")
```
<!-- md-code: id="normalize"; bin="python3" -->

## Statistical Tests

Calculate variance and standard deviation.

```python
import sys
import math

data = [23, 45, 67, 89, 34, 56, 78, 90, 12, 43]
mean = sum(data) / len(data)

variance = sum((x - mean) ** 2 for x in data) / len(data)
std_dev = math.sqrt(variance)

sys.stderr.write(f"Mean: {mean:.2f}\n")
sys.stderr.write(f"Variance: {variance:.2f}\n")
sys.stderr.write(f"Standard Deviation: {std_dev:.2f}\n")
```
<!-- md-code: id="stats"; bin="python3" -->

## Usage

Run `md code < data-analysis.md` to execute all code blocks and see the results.

The code blocks write to stderr, which is captured in the output blocks below each code section.
