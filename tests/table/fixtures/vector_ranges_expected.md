# Vector Range Integration Test

## Test 1: Column Range (A_:C_)

| A  | B  | C  | Total |
| -- | -- | -- | ----- |
| 10 | 20 | 30 | 450   |
| 40 | 50 | 60 | 50    |
| 70 | 80 | 90 | 90    |
<!-- md-table: D1 = sum(A_:C_); D2 = avg(A_:C_); D3 = max(A_:C_) -->

## Test 2: Row Range (_1:_2)

| Col1 | Col2 | Col3 |
| ---- | ---- | ---- |
| 5    | 10   | 15   |
| 20   | 25   | 30   |
| 105  | 0    | 0    |
<!-- md-table: A3 = sum(_1:_2) -->

## Test 3: Column Range with Matrix Operations

| Vec1 | Vec2 | Vec3 | Sum | Product |
| ---- | ---- | ---- | --- | ------- |
| 1    | 2    | 3    | 21  | 720     |
| 4    | 5    | 6    | 0   | 0       |
<!-- md-table: D1 = sum(A_:C_); E1 = prod(A_:C_) -->

## Test 4: Single Vector Range (Equivalent to A_)

| Data | Result |
| ---- | ------ |
| 100  | 600    |
| 200  | 0      |
| 300  | 0      |
<!-- md-table: B1 = sum(A_:A_) -->
