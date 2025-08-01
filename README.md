# 🦀Rust Algorithms and Data Structures
## 
<img src="./assets/kka.jpeg" style="width: 200px;" />

>This repository contains examples of various algorithms and data structures implemented in Rust.
## Complexity Analysis

## Space complexity
- Memory usage = static space + dynamic space
## Time complexity
- 
```rust
// T (n)  = 3n + 5 => O(n)
fn sum_average(a : Vec<i32> ,n:i32){
    let mut sum = 0;//1
    let mut i = 0;//1
    while i < n {//n+1
        sum  = sum + a[i];//n
        i = i + 1;//n
    }
    let average = sum / n ;//1
    println!("{} {}",sum,average); //1
}
```
-  time taken from algorithm start to completion


## Asymptotic Performance

- O(1) < O(log n) < O(n) < O(n log n) < O(n²) < O(n³) < O(2ⁿ)

## Data Structures

A data structure is a particular way of organizing and storing data in a computer so that it can be accessed and modified efficiently.


- [Array](./src/data_structure/array/array.md)
- [Vector](./src/data_structure/vector/vector.md)
- [Linked List](./src/data_structure/linked_list/linked_list.md)
- [Stack](./src/data_structure/stack/stack.md)
- [Queue](./src/data_structure/queue/queue.md)
- [Trie](./src/data_structure/trie/trie.md)
- [Tree](./src/data_structure/tree/tree.md)
- - [Bineary Search Tree]()
- - [Binary Tree](./src/data_structure/tree/binary_tree/binary_tree.md)
- - [AVL Tree](./src)
- [Hash Table]()

## Algorithm
- [Math]()
- - [Bits]() 
- - [Sieve Of Eratosthenes]()
- - [Euclidean algorithm]()
- - [Fast Powering]()
- - [Factorial](./src/algorithms/math/factorial/factorial.md)
- - [Prime Facotrs](./src/algorithms/math/prime_factors/prime_factors.md)
- - [Fast Powering]()
- [String](./src/algorithms/string/string.md)
- [Search](./src/algorithms/search/search.md)
- - [Binary Search](./src/algorithms/search/binary_search/binary_search.md)
- - [Linear Search](./src/algorithms/search/linear_search/lenear_search.md) and sequential search
- [Sort](./src/algorithms/sort/sort.md)
- - [Selection Sort](./src/algorithms/sort/select/select.md)
- - [Bubble Sort](./src/algorithms/sort/bubble/bubble_sort.md)
- - [Insertion Sort](./src/algorithms/sort/insertion/insertion.md)
- - [Merge Sort](./src/algorithms/sort/merge/merge.md)
- - [Quick Sort](./src/algorithms/sort/quick/quick.md)

- [Graph](./src/algorithms/graph/graph.md)
- - [Eulerian](./src/algorithms/graph/eulerian/eulerian.md)
- - [Knight's Tour](./src/algorithms/backtracking/knights_tour/knights_tour.md)
## Algorithm Method
- [Divide And Conquer](./src/algorithms/divide_and_conquer/divide_and_conquer.md)
- [Dynamic Programming](./src/algorithms/dynamic_programming/dynamic_programming.md)
- - [fibonacci](./src/algorithms/math/factorial/factorial.md)
- - [Matrix Chain Multiplication]()
- - [Longest Common Subsequence]()
- [Greedy](./src/algorithms/greedy/greedy.md) 
- - [Coin Change](./src/algorithms/greedy/coin_change/mod.rs)
- - [Knapsack](./src/algorithms/greedy/knapsack/knapsack.md) not 0/1 problem
- [NP-C](./src/algorithms/np_complete/np_complete.md) NP-complete problem
- - [Knapsack](./src/algorithms/greedy/knapsack/knapsack.md) For the 0/1 dividend problem
- Recursion
- Design Patterns
- Cash
- [Reinforcement Learning](https://github.com/kyunghyunHan/reinforcement_learning)
- AI
- - 
- - 

## 🏆Coders's Cup
- [combining_lists](./src/conding_test/coders_cup/combining_lists/combining_lists.md)
- [Maxinum Score From Removing Stones](./src/conding_test/coders_cup/maxinum_score_from_removing_stones/maxinum_score_from_removing_stones.md)
- [Palindrome Checking](./src/conding_test/coders_cup/palindrome_checking/palindrome_checking.md)
- [Higher or Lower](./src/conding_test/coders_cup/higher_or_lower/higher_or_lower.md)
- [Sorting]()
## Coding Test
- [LeetCode](https://leetcode.com/)

## Game

## Interview

## [Nasa Power of 10]()

