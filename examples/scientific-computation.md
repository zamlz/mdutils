# Scientific Computation Example

Using Python for mathematical calculations within markdown documents.

## Fibonacci Sequence

Generate the first 15 Fibonacci numbers:

```python
import sys

def fibonacci(n):
    fib = [0, 1]
    for i in range(2, n):
        fib.append(fib[i-1] + fib[i-2])
    return fib

result = fibonacci(15)
print(f"First 15 Fibonacci numbers:\n\t{result}")
```
<!-- md-code: id="fibonacci"; bin="python3" -->

Output:
```
First 15 Fibonacci numbers:
	[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377]

```
<!-- md-code-output: id="fibonacci" -->

## Prime Number Detection

Check which numbers from 1 to 50 are prime:

```python
import sys

def is_prime(n):
    if n < 2:
        return False
    for i in range(2, int(n**0.5) + 1):
        if n % i == 0:
            return False
    return True

primes = [n for n in range(1, 51) if is_prime(n)]
print(f"Prime numbers from 1 to 50:{primes}")
print(f"Total count: {len(primes)}")
```
<!-- md-code: id="primes"; bin="python3" -->

Output:
```
Prime numbers from 1 to 50:[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
Total count: 15

```
<!-- md-code-output: id="primes" -->

## Matrix Operations

Perform basic matrix multiplication:

```python
import sys

def matrix_multiply(A, B):
    rows_A, cols_A = len(A), len(A[0])
    rows_B, cols_B = len(B), len(B[0])

    if cols_A != rows_B:
        return None

    result = [[0 for _ in range(cols_B)] for _ in range(rows_A)]

    for i in range(rows_A):
        for j in range(cols_B):
            for k in range(cols_A):
                result[i][j] += A[i][k] * B[k][j]

    return result

A = [[1, 2, 3],
     [4, 5, 6]]

B = [[7, 8],
     [9, 10],
     [11, 12]]

C = matrix_multiply(A, B)

print("Matrix A (2x3):")
for row in A:
    print(f"  {row}")

print("Matrix B (3x2):")
for row in B:
    print(f"  {row}")

print("Result C = A × B (2x2):")
for row in C:
    print(f"  {row}")
```
<!-- md-code: id="matrix"; bin="python3" -->

Output:
```
Matrix A (2x3):
  [1, 2, 3]
  [4, 5, 6]
Matrix B (3x2):
  [7, 8]
  [9, 10]
  [11, 12]
Result C = A × B (2x2):
  [58, 64]
  [139, 154]

```
<!-- md-code-output: id="matrix" -->

## Numerical Integration

Approximate the integral of f(x) = x² from 0 to 10 using the trapezoidal rule:

```python
import sys

def trapezoidal_rule(f, a, b, n):
    h = (b - a) / n
    result = 0.5 * (f(a) + f(b))

    for i in range(1, n):
        result += f(a + i * h)

    return h * result

def f(x):
    return x ** 2

# Exact integral of x² from 0 to 10 is [x³/3] = 1000/3 ≈ 333.333
approximate = trapezoidal_rule(f, 0, 10, 1000)
exact = 1000 / 3

print(f"Approximate integral: {approximate:.4f}")
print(f"Exact value: {exact:.4f}")
print(f"Error: {abs(approximate - exact):.6f}")
```
<!-- md-code: id="integration"; bin="python3" -->

Output:
```
Approximate integral: 333.3335
Exact value: 333.3333
Error: 0.000167

```
<!-- md-code-output: id="integration" -->

## Usage

Run `md code < scientific-computation.md` to execute all computations and see results.
