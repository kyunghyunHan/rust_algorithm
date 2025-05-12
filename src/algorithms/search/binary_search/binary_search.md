# Binary Search
- 정렬된 리스트 형태로 주어진 원소들을 절반씩 줄여가면서 원하는 값을 가진 원소를 찾는 방법
#### )Recurrence Equation
- T(n) = ⎰ Θ(1),          n = 1 ⎱
         ⎱ T(n/2) + Θ(1), n ≥ 2 ⎰
#### )Closed Type

- T(n) = Θ(log n)