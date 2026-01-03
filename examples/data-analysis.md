# Data Analysis Notebook

This example demonstrates using code execution for data analysis tasks.

<!-- md-toc: -->
- [Dataset Summary](#dataset-summary)
- [Data Transformation](#data-transformation)
- [Statistical Tests](#statistical-tests)
- [Usage](#usage)
<!-- md-toc: end -->

## Dataset Summary

Let's analyze a simple dataset using Python.

```python
import sys

data = [23, 45, 67, 89, 34, 56, 78, 90, 12, 43]

print(f"Count: {len(data)}")
print(f"Mean: {sum(data)/len(data):.2f}")
print(f"Min: {min(data)}")
print(f"Max: {max(data)}")
print(f"Range: {max(data) - min(data)}")
```
<!-- md-code: id="summary_stats"; bin="python3" -->

Output:
```
Count: 10
Mean: 53.70
Min: 12
Max: 90
Range: 78

```
<!-- md-code-output: id="summary_stats" -->

## Data Transformation

Apply a transformation (e.g., normalize to 0-100 scale).

```python
import sys

data = [23, 45, 67, 89, 34, 56, 78, 90, 12, 43]
min_val = min(data)
max_val = max(data)

normalized = [(x - min_val) / (max_val - min_val) * 100 for x in data]

print("Normalized values:")
for i, (orig, norm) in enumerate(zip(data, normalized)):
    print(f"  {orig} -> {norm:.1f}")
```
<!-- md-code: id="normalize"; bin="python3" -->

Output:
```
Normalized values:
  23 -> 14.1
  45 -> 42.3
  67 -> 70.5
  89 -> 98.7
  34 -> 28.2
  56 -> 56.4
  78 -> 84.6
  90 -> 100.0
  12 -> 0.0
  43 -> 39.7

```
<!-- md-code-output: id="normalize" -->

## Statistical Tests

Calculate variance and standard deviation.

```python
import sys
import math

data = [23, 45, 67, 89, 34, 56, 78, 90, 12, 43]
mean = sum(data) / len(data)

variance = sum((x - mean) ** 2 for x in data) / len(data)
std_dev = math.sqrt(variance)

print(f"Mean: {mean:.2f}")
print(f"Variance: {variance:.2f}")
print(f"Standard Deviation: {std_dev:.2f}")
```
<!-- md-code: id="stats"; bin="python3" -->

Output:
```
Mean: 53.70
Variance: 659.61
Standard Deviation: 25.68

```
<!-- md-code-output: id="stats" -->

## Usage

Run `md code < data-analysis.md` to execute all code blocks and see the results.

The code blocks write to stderr, which is captured in the output blocks below each code section.
