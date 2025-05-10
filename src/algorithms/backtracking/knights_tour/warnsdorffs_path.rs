use std::io;

fn main() {
    // 체스보드 크기
    let n: usize = 3;
    let m: usize = 10;
    
    // 그래프 생성
    let graph = make_graph(n, m);
    
    // 인접 리스트 출력
    for v in 0..n * m {
        print!("{}: ", v);
        for &j in &graph[v] {
            print!("{} ", j);
        }
        println!();
    }
    
    // 시작 위치
    let s = 0;
    
    // 방문 표시 배열 초기화
    let mut tour_path = vec![0; n * m];
    let mut visited = vec![false; n * m];
    visited[s] = true;
    tour_path[0] = s;
    
    // Warnsdorff의 알고리즘으로 해밀턴 경로 찾기
    let success = warnsdorff_path(n, m, &graph, &mut visited, &mut tour_path);
    
    if success {
        println!("해밀턴 경로를 찾았습니다:");
        // 경로 출력
        for i in 0..n {
            for j in 0..m {
                // tour_path 배열에서 현재 위치의 단계(몇 번째 방문인지)를 찾기
                let pos = i * m + j;
                let step = tour_path.iter().position(|&x| x == pos).unwrap() + 1;
                print!("{:2} ", step);
            }
            println!();
        }
        
        // 경로 순서대로 출력
        println!("방문 순서:");
        for i in 0..n * m {
            print!("{} ", tour_path[i]);
            if (i + 1) % 10 == 0 {
                println!();
            }
        }
        println!();
    } else {
        println!("해밀턴 경로를 찾지 못했습니다");
    }
}

// 그래프 생성 함수
fn make_graph(n: usize, m: usize) -> Vec<Vec<usize>> {
    // 나이트의 이동 방향 (8가지 L자 이동)
    let imov = [-2, -1, 1, 2, 2, 1, -1, -2];
    let jmov = [1, 2, 2, 1, -1, -2, -2, -1];
    
    // 그래프 초기화
    let mut graph = vec![Vec::new(); n * m];
    
    // 모든 셀을 순회하며 가능한 이동을 그래프에 추가
    for i in 0..n {
        for j in 0..m {
            for k in 0..8 {
                let ni = i as i32 + imov[k];
                let nj = j as i32 + jmov[k];
                
                if ni >= 0 && nj >= 0 && ni < n as i32 && nj < m as i32 {
                    graph[i * m + j].push((ni as usize) * m + (nj as usize));
                }
            }
        }
    }
    
    graph
}

// 미방문 이웃의 수를 계산하는 함수
fn count_unvisited_neighbors(pos: usize, graph: &Vec<Vec<usize>>, visited: &Vec<bool>) -> usize {
    graph[pos].iter().filter(|&&neighbor| !visited[neighbor]).count()
}

// Warnsdorff의 알고리즘으로 해밀턴 경로 찾기
fn warnsdorff_path(
    n: usize, 
    m: usize, 
    graph: &Vec<Vec<usize>>, 
    visited: &mut Vec<bool>, 
    tour_path: &mut Vec<usize>
) -> bool {
    // 모든 칸을 방문했는지 확인 - 해밀턴 경로 완성!
    if visited.iter().all(|&v| v) {
        return true;
    }
    
    // 현재 위치 (마지막으로 방문한 칸)
    let current_pos = *tour_path.iter().take_while(|&&pos| visited[pos]).last().unwrap();
    
    // 다음에 방문할 수 있는 모든 위치 찾기
    let mut next_moves: Vec<(usize, usize)> = graph[current_pos]
        .iter()
        .filter(|&&next_pos| !visited[next_pos])
        .map(|&next_pos| {
            // (위치, 미방문 이웃 수) 튜플 반환
            (next_pos, count_unvisited_neighbors(next_pos, graph, visited))
        })
        .collect();
    
    // 미방문 이웃이 가장 적은 순으로 정렬
    next_moves.sort_by_key(|&(_, count)| count);
    
    // 가능한 모든 다음 이동을 시도
    for (next_pos, _) in next_moves {
        // 방문 표시
        visited[next_pos] = true;
        
        // 경로에 추가
        let step = visited.iter().filter(|&&v| v).count() - 1;
        tour_path[step] = next_pos;
        
        // 재귀적으로 다음 단계 시도
        if warnsdorff_path(n, m, graph, visited, tour_path) {
            return true;
        }
        
        // 실패하면 백트래킹
        visited[next_pos] = false;
    }
    
    // 모든 시도가 실패하면 false 반환
    false
}