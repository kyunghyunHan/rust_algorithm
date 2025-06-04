# Sort

- 중진 데이터를 값의 크기 순서에 따라 재배치 하는것

## Internal Sorting
- 정렬할 모든 데이터를 주 기억장치에 저장한 후 정렬하는 방식
✓ Method where all data can be stored in main memory and then sorted
## stable sort
- 같은 값을 갖는 여러개의 데이터에 대한 정렬 전의 상대적 순서가 정렬 후에도 그대로 유지되는 정렬
## in-place sort
- 입력 배열 이외에 별도로 필요한 저장공간이 상수개를 넘지 않는 정렬
## comparison based sort
- 두 데이터의 값 전체를 직접적으러 비교하여 어떤 값이 큰지 또는 작은지를 결정하여 정렬하는 방식
## External Sorting
✓ When all data cannot be stored in main memory,
All data is stored in auxiliary memory first,
Then only some portions of data are selectively brought into main memory for sorting
- 모든 데이터를 주기억장치에 저장할수 없는 경우
- 모든 데이터를 보조기억장치에 저장해 두고
- 그 중 일부 데이터만을 반복적으로 주 기억장치에 옮겨와서 정렬을 수행
#### 1)Comparison-Based Algorithms

- Selection Sort
- Bubble Sort
- Insertion Sort
- Shell Sort
- Quick Sort
- Merge Sort
- Heap Sort

####  2)Data Distribution Algorithms

- Counting Sort
- Radix Sort
- Bucket Sort
## [Selection sort](./selection/select.md)
- 입력 배열에서 가장 작은 값 부터 순서대로 선택해서 나열하는 방식
## [Bubble Sort](./bubble/bubble_sort.md)

## [Insertion Sort](./insertion/insertion.md)

## [Shell Sort]()