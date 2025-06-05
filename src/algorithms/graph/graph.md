# Graph

## 연결할 객체를 나타내는 정점(vertex)의 집합과 정점을 연결하는 간선(edge)의 집합으로 구성된 비선형 자료구조

## G=(V,E)
- V:Vertex의 집합
- E:edge의 집합

## Tree

- 1. 무방향
- 2. 모든 vertex connected
- 3. 사이클 x

## 인접 행렬(adjacency matrix)
- 정점의 집합 V={1,2,…,n}인 그래프 G에 대해서 2차원 배열 A=(n×n)으로 표현하는 방법

## 인접 리스트(adjacency list)
- 정점의 집합 V={1,2,…,n}인 그래프 G에 대해서 n개의 연결 리스트로 표현하는 방법으로, 각 연결 리스트는 임의의 정점 u에 대해서 인접한 모든 정점을 표현함

그래프 순회(graph traversal)
- 그래프의 모든 정점을 체계적으로 한 번씩 방문하는 방법으로, 깊이 우선 탐색과 너비 우선 탐색이 있음

깊이 우선 탐색(depth first search)
- 한 정점을 시작으로 매번 인접한 정점 중 한 곳으로 이동하며 탐색하는 방법
- 최근의 주변 정점을 우선으로 탐색하는 방법

너비 우선 탐색(breadth first search)
- 시작 정점을 기준으로 거리가 가장 가깝게 인접한 정점을 우선으로 모두 방문한 후 시작 정점과의 거리가 점점 멀어지는 순서로 인접 정점들을 탐색하는 방법
- 주변 정점 중에서 가장 오래된 것부터 우선 방문하는 방법

위상 정렬(topological sort)
- 무사이클 방향 그래프에서 모든 간선이 한 방향으로만 향하도록 정점을 한 줄로 나열하는 것

연결 성분(connected component)
- 주어진 무방향 그래프에서 임의의 두 정점 간의 경로가 존재하는 최대 부분 그래프

강연결 성분(strongly connected component)
- 주어진 방향 그래프에서 임의의 두 정점 사이에 양방향의 경로가 존재하는 최대 부분 그래프


### Degree
- The gegree of a vertex is the number of edges connected to that vertex.
- In other words, it represents how many edges are incident to a given vertex in the graph.

  