# Grade Calculator

Automatically calculate weighted grades and statistics for a class.

## Student Scores

| Student   | Homework   | Midterm   | Final   | Weighted   |
| --------- | ---------- | --------- | ------- | ---------- |
| Alice     | 95         | 88        | 92      | 91.7       |
| Bob       | 78         | 82        | 85      | 82.0       |
| Carol     | 92         | 95        | 90      | 92.1       |
| David     | 85         | 79        | 88      | 84.4       |
| Eve       | 88         | 91        | 94      | 91.3       |
<!-- md-table: E_ = B_ * 0.3 + C_ * 0.3 + D_ * 0.4 -->
<!-- md-table: id="scores" -->

## Class Statistics

| Metric     | Homework   | Midterm   | Final   | Overall   |
| ---------- | ---------- | --------- | ------- | --------- |
| Average    | 87.60      | 87        | 89.80   | 88.3      |
| Minimum    | 78         | 79        | 85      | 82.0      |
| Maximum    | 95         | 95        | 94      | 92.1      |
| Count      | 5          | 5         | 5       | 5         |
<!-- md-table: let homework = from("scores", B_) -->
<!-- md-table: let midterm = from("scores", C_) -->
<!-- md-table: let final= from("scores", D_) -->
<!-- md-table: let weighted = from("scores", E_) -->
<!-- md-table: B1 = avg(homework); C1 = avg(midterm); D1 = avg(final); E1 = avg(weighted); -->
<!-- md-table: B2 = min(homework); C2 = min(midterm); D2 = min(final); E2 = min(weighted); -->
<!-- md-table: B3 = max(homework); C3 = max(midterm); D3 = max(final); E3 = max(weighted); -->
<!-- md-table: B4 = count(homework); C4 = count(midterm); D4 = count(final); E4 = count(weighted) -->

## Grading Scale

Weighted grade calculation:
- Homework: 30%
- Midterm: 30%
- Final: 40%

Letter grades:
- A: 90-100
- B: 80-89
- C: 70-79
- D: 60-69
- F: 0-59

Run `md table < grades.md` to update all calculations.
