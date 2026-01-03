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
<!-- md-code: id="table_demo"; execute; bin="cargo run --quiet -- table" -->

Output:
```
| A   | B   | C   |
| --- | --- | --- |
| 5   | 10  | 15  |
| 3   | 7   | 10  |
<!-- md-table: C_ = A_ + B_ -->
```
<!-- md-code-output: id="table_demo" -->

## Example 2: md toc generating a TOC

```markdown
<!-- md-toc: -->
<!-- md-toc: end -->

# Section One
## Subsection A
## Subsection B
# Section Two
```
<!-- md-code: id="toc_demo"; execute; bin="cargo run --quiet -- toc" -->

Output:
```
<!-- md-toc: -->
- [Section One](#section-one)
  - [Subsection A](#subsection-a)
  - [Subsection B](#subsection-b)
- [Section Two](#section-two)
<!-- md-toc: end -->

# Section One
## Subsection A
## Subsection B
# Section Two

```
<!-- md-code-output: id="toc_demo" -->
