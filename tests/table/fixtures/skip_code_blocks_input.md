# Test: Tables Inside Code Blocks Should Be Skipped

This table should be formatted:

| A   | B   | C   |
| --- | --- | --- |
| 5   | 10  | 0   |
<!-- md-table: C1 = A1 + B1 -->

This markdown table inside a code block should NOT be processed:

```markdown
| x   | y   | z   |
| --- | --- | --- |
| 3   | 2   | 0   |
| 3   | 2   | 0   |
<!-- md-table: C_ = A_ + B_ -->
```

This table should also be formatted:

| X | Y | Z |
|---|---|---|
| 1 | 2 | 0 |
<!-- md-table: C1 = A1 * B1 -->
