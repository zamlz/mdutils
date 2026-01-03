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
sys.stderr.write(f"First 15 Fibonacci numbers:\n{result}\n")
```
<!-- md-code: id="fibonacci"; bin="python3" -->

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
sys.stderr.write(f"Prime numbers from 1 to 50:\n{primes}\n")
sys.stderr.write(f"\nTotal count: {len(primes)}\n")
```
<!-- md-code: id="primes"; bin="python3" -->

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

sys.stderr.write("Matrix A (2x3):\n")
for row in A:
    sys.stderr.write(f"  {row}\n")

sys.stderr.write("\nMatrix B (3x2):\n")
for row in B:
    sys.stderr.write(f"  {row}\n")

sys.stderr.write("\nResult C = A × B (2x2):\n")
for row in C:
    sys.stderr.write(f"  {row}\n")
```
<!-- md-code: id="matrix"; bin="python3" -->

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

sys.stderr.write(f"Approximate integral: {approximate:.4f}\n")
sys.stderr.write(f"Exact value: {exact:.4f}\n")
sys.stderr.write(f"Error: {abs(approximate - exact):.6f}\n")
```
<!-- md-code: id="integration"; bin="python3" -->

## Usage

Run `md code < scientific-computation.md` to execute all computations and see results.
