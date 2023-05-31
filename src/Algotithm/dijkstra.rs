use std::collections::BinaryHeap;
use std::cmp::Reverse;
/*선택되지 않는 노드중 최소거리를 가진 노드를 찾기
1부터 V까지의 모든 노드 순회
 */

fn find_shortest_node(select: &[bool], dist: &[i32]) -> usize {
    let mut min_dist = std::i32::MAX;
    let mut min_idx = 0;

    for (i, &is_selected) in select.iter().enumerate() {
        if is_selected {
            continue;
        }
        if dist[i] < min_dist {
            min_dist = dist[i];
            min_idx = i;
        }
    }
    min_idx
}

fn update_dist(new_node: usize, select: &[bool], dist: &mut [i32], map: &[Vec<i32>]) {
    for (i, &is_selected) in select.iter().enumerate() {
        if is_selected {
            continue;
        }
        if let Some(new_dist) = dist[new_node].checked_add(map[new_node][i]) {
            if new_dist < dist[i] {
                dist[i] = new_dist;
            }
        }
    }
}

fn dijkstra(start: usize, select: &mut [bool], dist: &mut [i32], map: &[Vec<i32>]) {
    let v = select.len();

    for i in 0..v {
        if i != start {
            dist[i] = std::i32::MAX;
        }
    }

    for _ in 0..v - 1 {
        let new_node = find_shortest_node(select, dist);
        select[new_node] = true;
        update_dist(new_node, select, dist, map);
    }
}

pub fn main() {
    const V: usize = 45; // 노드의 개수
    const START: usize = 1; // 시작 노드

    let mut select = vec![false; V];
    let mut dist = vec![std::i32::MAX; V];
    let mut map: Vec<Vec<i32>> = vec![vec![std::i32::MAX; V]; V];
    // 그래프의 가중치 값을 가지고 MAP 설정
    // 예시: map[0][1] = 5; // 노드 0과 노드 1 사이의 간선 가중치는 5입니다.
    map[0][1] = 5;

    dijkstra(START, &mut select, &mut dist, &map);

    println!("최단 거리: {:?}", dist);
}








//우선순위 큐 사용
// fn dijkstra_using_heap(start: usize, dist: &mut [i32], vertex: &[Vec<(usize, i32)>]) {
//     let mut pq = BinaryHeap::new();
//     pq.push((Reverse(0), start));
//     dist[start] = 0;

//     while let Some((Reverse(cost), cur)) = pq.pop() {
//         if cost > dist[cur] {
//             continue;
//         }

//         for &(next, n_cost) in &vertex[cur] {
//             let new_cost = cost + n_cost;

//             if new_cost < dist[next] {
//                 dist[next] = new_cost;
//                 pq.push((Reverse(new_cost), next));
//             }
//         }
//     }

//     for &d in dist.iter().skip(1) {
//         if d == std::i32::MAX {
//             println!("INF");
//         } else {
//             println!("{}", d);
//         }
//     }
// }

// pub fn main() {
//     let v = 10; // 노드의 개수
//     let start = 1; // 시작 노드

//     let mut dist = vec![std::i32::MAX; v + 1];
//     let mut vertex = vec![vec![]; v + 1];

//     // 그래프의 가중치 값을 가지고 Vertex 설정
//     // 예시: vertex[1].push((2, 5)); // 노드 1과 노드 2 사이의 간선 가중치는 5입니다.
//     vertex[1].push((2, 5));
//     vertex[2].push((3, 4));
//     dijkstra_using_heap(start, &mut dist, &vertex);
// }
