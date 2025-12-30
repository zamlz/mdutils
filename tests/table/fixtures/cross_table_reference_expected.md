# Cross-Table Reference Test

First table with some data:

| Value |
| ----- |
| 10    |
| 20    |
| 30    |
<!-- md-table: id="source" -->

Second table that references the first:

| Sum  | Avg  |
| ---- | ---- |
| 60   | 20   |
<!-- md-table: id="result"; A1 = sum(from("source", A_)); B1 = avg(from("source", A_)) -->
