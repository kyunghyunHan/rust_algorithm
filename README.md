# Rust Algorithms and Data Structures

Rust로 알고리즘, 자료구조, 암호학 및 코딩 테스트 문제를 구현하고 정리하는 학습용 저장소입니다.

<img src="./assets/kka.jpeg" alt="Project mascot" width="200" />

## 시작하기

```bash
cargo check --all-targets
cargo test --all-targets
cargo run
```

`src/main.rs`에서 실행할 예제의 주석을 해제한 뒤 `cargo run`을 사용합니다.

## 프로젝트 구조

```text
.
├── assets/              # 이미지와 예제 입력 데이터
├── docs/                # 프로젝트 공통 문서
├── src/
│   ├── algorithms/      # 알고리즘 구현과 설명
│   ├── coding_test/     # 코딩 테스트 풀이
│   ├── cryptography/    # 암호학 예제
│   ├── data_structure/  # 자료구조 구현과 설명
│   ├── finance/         # 금융 계산 예제
│   ├── space/           # 우주·통신 관련 학습 자료
│   ├── c/               # C 개념을 Rust로 연습한 예제
│   └── utils/           # 공용 유틸리티
├── Cargo.toml
└── README.md
```

## 알고리즘

| 분류 | 문서 및 구현 |
| --- | --- |
| 정렬 | [개요](./src/algorithms/sort/sort.md), [버블](./src/algorithms/sort/bubble/bubble_sort.md), [선택](./src/algorithms/sort/selection/selection_sort.md), [삽입](./src/algorithms/sort/insertion/insertion.md), [병합](./src/algorithms/sort/merge/merge.md), [퀵](./src/algorithms/sort/quick/quick.md), [힙](./src/algorithms/sort/heap/heap.md), [계수](./src/algorithms/sort/counting/counting.md), [기수](./src/algorithms/sort/radix/radix.md), [버킷](./src/algorithms/sort/bucket/bucket.md), [셸](./src/algorithms/sort/shell/shell.md) |
| 탐색 | [개요](./src/algorithms/search/search.md), [선형 탐색](./src/algorithms/search/linear_search/linear_search.md), [순차 탐색](./src/algorithms/search/sequential_search/sequential_search.md), [이진 탐색](./src/algorithms/search/binary_search/binary_search.md) |
| 그래프 | [개요](./src/algorithms/graph/graph.md), [다익스트라](./src/algorithms/graph/dijkstra/dijkstra.md), [벨만–포드](./src/algorithms/graph/bellman_ford/bellman_ford.md), [플로이드](./src/algorithms/graph/floyd/floyd.md), [크루스칼](./src/algorithms/graph/kruskal/kruskal.md), [프림](./src/algorithms/graph/prim/prim.md), [오일러 경로](./src/algorithms/graph/eulerian/eulerian.md) |
| 문자열 | [개요](./src/algorithms/string/string.md), [브루트 포스](./src/algorithms/string/brute_force/brute_force.md), [KMP](./src/algorithms/string/kmp/kmp.md), [Rabin–Karp](./src/algorithms/string/rabin_karp/rabin_karp.md), [Boyer–Moore](./src/algorithms/string/boyer_moore/boyer_moore.md), [허프만 코딩](./src/algorithms/string/huffman_coding/huffman_coding.md) |
| 동적 계획법 | [개요](./src/algorithms/dynamic_programming/dynamic_programming.md), [최장 공통 부분 수열](./src/algorithms/dynamic_programming/longest_common_subsequence/longest_common_subsequence.md), [행렬 곱셈 순서](./src/algorithms/dynamic_programming/matrix_chain_multiplication/matrix_chain_multiplication.md) |
| 분할 정복 | [개요](./src/algorithms/divide_and_conquer/divide_and_conquer.md) |
| 백트래킹 | [Knight's Tour](./src/algorithms/backtracking/knights_tour/knights_tour.md), [외판원 문제](./src/algorithms/backtracking/traveling_salesman_problem/traveling_salesman_problem.md) |
| 탐욕법 | [개요](./src/algorithms/greedy/greedy.md), [Knapsack](./src/algorithms/greedy/knapsack/knapsack.md) |
| 수학 | [진법 변환](./src/algorithms/math/number_base_conversion/number_base_conversion.md), [소인수 분해](./src/algorithms/math/prime_factors/prime_factors.md), [에라토스테네스의 체](./src/algorithms/math/sieve_of_eratosthenes/sieve_of_eratosthenes.md), [팩토리얼](./src/algorithms/math/factorial/factorial.md), [비트 연산](./src/algorithms/math/bits/bits.md) |
| NP-완전 | [개요](./src/algorithms/np_complete/np_complete.md) |

## 자료구조

| 자료구조 | 문서 |
| --- | --- |
| 배열 | [Array](./src/data_structure/array/array.md) |
| 벡터 | [Vector](./src/data_structure/vector/vector.md) |
| 연결 리스트 | [Linked List](./src/data_structure/linked_list/linked_list.md), [Doubly Linked List](./src/data_structure/linked_list/doubly_linked_list.md) |
| 스택과 큐 | [Stack](./src/data_structure/stack/stack.md), [Queue](./src/data_structure/queue/queue.md) |
| 트리 | [개요](./src/data_structure/tree/tree.md), [Binary Tree](./src/data_structure/tree/binary_tree/binary_tree.md) |
| 해시 테이블 | [구현](./src/data_structure/hash_table/hash_table.rs) |
| 트라이 | [Trie](./src/data_structure/trie/trie.md) |

## 코딩 테스트

- [Coders' Cup](./src/coding_test/coders_cup/)
- [Baekjoon](./src/coding_test/baekjoon/)
- [문제집 풀이](./src/coding_test/books/)

## 참고 문서

- [NASA Power of 10](./docs/nasa_power_of_10.md)
- [Finance](./src/finance/finance.md)
- [Space](./src/space/space.md)
- [Delay/Disruption Tolerant Networking](./src/space/dtn/dtn.md)

## 복잡도 요약

일반적인 증가 순서는 다음과 같습니다.

```text
O(1) < O(log n) < O(n) < O(n log n) < O(n²) < O(n³) < O(2ⁿ)
```

공간 복잡도는 알고리즘 실행에 필요한 정적 공간과 동적 공간을 함께 고려합니다.
