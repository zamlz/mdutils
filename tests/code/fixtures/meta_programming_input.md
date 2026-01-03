# Meta-Programming Test: md running itself

This demonstrates md-code executing md commands on markdown examples.

## Example 1: md table processing a table

```markdown
| A | B | C |
|---|---|---|
| 5 | 10 | 0 |
| 3 | 7 | 0 |
<!-- md-table: C_ = A_ + B_ -->
```
<!-- md-code: id="table_demo"; bin="cargo run --quiet -- table" -->

## Example 2: md toc generating a TOC

```markdown
<!-- md-toc: -->
<!-- md-toc: end -->

# Section One
## Subsection A
## Subsection B
# Section Two
```
<!-- md-code: id="toc_demo"; bin="cargo run --quiet -- toc" -->

## Example 3: md code executing code

~~~markdown
```python
print(f"3^2 = {3**2}")
```
<!-- md-code: id="python_test"; bin="python3" -->
~~~
<!-- md-code: id="code_demo"; bin="cargo run --quiet -- code" -->
