# Grade Calculator

Automatically calculate weighted grades and statistics for a class.

## Student Scores

| Student | Homework | Midterm | Final | Weighted | Letter |
|---------|----------|---------|-------|----------|--------|
| Alice   | 95       | 88      | 92    | 0        |        |
| Bob     | 78       | 82      | 85    | 0        |        |
| Carol   | 92       | 95      | 90    | 0        |        |
| David   | 85       | 79      | 88    | 0        |        |
| Eve     | 88       | 91      | 94    | 0        |        |
<!-- md-table: E_ = B_ * 0.3 + C_ * 0.3 + D_ * 0.4 -->

## Class Statistics

| Metric   | Homework | Midterm | Final | Overall |
|----------|----------|---------|-------|---------|
| Average  | 0        | 0       | 0     | 0       |
| Minimum  | 0        | 0       | 0     | 0       |
| Maximum  | 0        | 0       | 0     | 0       |
| Count    | 0        | 0       | 0     | 0       |
<!-- md-table: B1 = avg(B1:B5); C1 = avg(C1:C5); D1 = avg(D1:D5); E1 = avg(E1:E5); B2 = min(B1:B5); C2 = min(C1:C5); D2 = min(D1:D5); E2 = min(E1:E5); B3 = max(B1:B5); C3 = max(C1:C5); D3 = max(D1:D5); E3 = max(E1:E5); B4 = count(B1:B5); C4 = count(C1:C5); D4 = count(D1:D5); E4 = count(E1:E5) -->

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
