# Monthly Budget Tracker

Track your monthly expenses and income with automatic calculations.

## Income

| Source      | Amount |
|-------------|--------|
| Salary      | 5000   |
| Freelance   | 1200   |
| Investments | 300    |
| **Total**   | 0      |
<!-- md-table: B4 = sum(B1:B3) -->

## Expenses

| Category       | Budgeted | Actual | Difference |
|----------------|----------|--------|------------|
| Rent           | 1500     | 1500   | 0          |
| Groceries      | 600      | 680    | 0          |
| Transportation | 300      | 250    | 0          |
| Utilities      | 200      | 215    | 0          |
| Entertainment  | 400      | 520    | 0          |
| Savings        | 2000     | 2000   | 0          |
| **Totals**     | 0        | 0      | 0          |
<!-- md-table: B7 = sum(B1:B6); C7 = sum(C1:C6); D_ = B_ - C_ -->

## Summary

| Item            | Amount |
|-----------------|--------|
| Total Income    | 0      |
| Total Expenses  | 0      |
| **Net Savings** | 0      |
<!-- md-table: B1 = 6500; B2 = 5165; B3 = B1 - B2 -->

## Analysis

Run `md table < budget.md > budget.md` to update all calculations.

- Budget variance: Check the Difference column to see where you overspent
- Savings rate: Calculate as (Net Savings / Total Income) * 100
