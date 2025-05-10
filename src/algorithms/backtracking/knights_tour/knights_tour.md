# Knight's Tour

## Overview
The Knight's Tour is a mathematical problem where a knight chess piece must visit every square on an m × n chessboard exactly once. This is a specific instance of finding a Hamiltonian path or cycle in graph theory.

## Hamiltonian Paths and Cycles

### Hamiltonian Cycle
- In an undirected graph G = (V, E), a cycle that visits every vertex **exactly once** and returns to the starting vertex
- Forms a cyclic structure connecting all nodes

### Hamiltonian Path
- A path that visits every vertex **exactly once**
- Does not need to return to the starting point

## Example Chessboard

A 3×4 chessboard with positions labeled:

|         |  j=0  |  j=1  |  j=2   |  j=3  |
| :-----: | :---: | :---: | :----: | :---: |
| **i=0** |   0   | **1** |   2    |   3   |
| **i=1** |   4   |   5   |   6    | **7** |
| **i=2** |   8   |   9   | **10** |  11   |

## Adjacency List

List of possible knight moves from each position:

```
0: [6, 9]      // From position 0, knight can move to positions 6 and 9
1: [7, 10, 8]  // From position 1, knight can move to positions 7, 10, and 8
2: [11, 9, 4]
3: [10, 5]
4: [2, 10]
5: [3, 11]
6: [8, 0]
7: [9, 1]
8: [1, 6]
9: [2, 7, 0]
10: [3, 4, 1]
11: [5, 2]
```

## Knight's Movement Pattern

The knight in chess moves in an 'L' shape:
- 2 squares horizontally and 1 square vertically
- 2 squares vertically and 1 square horizontally

This unique movement pattern makes finding a tour that visits every square exactly once an interesting graph traversal problem.

## Solution Methods

Common approaches to solving the Knight's Tour problem:
1. **Backtracking**: Exploring all possible move sequences to find a solution
2. **Warnsdorff's Algorithm**: A heuristic approach where the knight moves to the square with the fewest onward moves
3. **Divide and Conquer**: Breaking larger boards into smaller sections and solving each separately

The Knight's Tour has been a fascinating problem for mathematicians, computer scientists, and chess puzzle enthusiasts for centuries.


## Code
### BackTracking
- [Cycle]()
- [Path]()