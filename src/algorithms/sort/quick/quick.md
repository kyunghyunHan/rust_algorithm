# Quick Sort

- 특정 데이터를 기준으로 주어진 배열을 2개의 부분 배열로 분할하고,각 부분 배열에 대해서 퀵정렬을 순한적으로 적용하는 방식

## 성능과 특징
- 퀵정렬의 수행 시간은 분할되는 두 부분배열의 크기게 따라 달라짐
## Pivot
- 주어진 배열을 두 부분배열로 분할할 때 기준이 되는 특정 데이터
- 보통 배열의 첫 번째 원소를 피벗으로 지정

### Best-case time complexity
#### )Recurrence Equation
- T(n) = ⎰ Θ(1),           n = 1 ⎱
         ⎱ 2T(n/2) + Θ(n), n ≥ 2 ⎰
#### )Closed Type
- T(n) = Θ(n log n)

### Worst-case time complexity
#### )Recurrence Equation
- T(n) = ⎰ Θ(1),           n = 1 ⎱
         ⎱ T(n-1) + Θ(n),  n ≥ 2 ⎰
#### )Closed Type
- T(n) = Θ(n²)

