use std::io;

pub fn example() {
    // 입력 받기
    let (n, m) = read_input();
    
    // 그래프 생성
    let graph = make_graph(n, m);
    
    // 각 정점의 인접 리스트 출력
    for v in 0..n * m {
        print!("{}: ", v);
        for &j in &graph[v] {
            print!("{} ", j);
        }
        println!();
    }
    
    // 모든 가능한 시작점에서 탐색
    let mut total = 0;
    for s in 0..n * m {
        // 방문 표시 배열 초기화
        let mut mark = vec![0; n * m];
        mark[s] = 1; // 시작 정점 표시
        
        // 해결책 카운터 초기화
        let mut cnt = 0;
        
        // 탐색 시작
        tour(1, s, n, m, &graph, &mut mark, &mut cnt);
        
        // 결과 출력 (해결책이 있는 경우만)
        if cnt > 0 {
            println!("{}: {}", s, cnt);
        }
        
        total += cnt;
    }
    
    println!("total = {}", total);
}

// 입력을 읽어 n과 m 값을 반환하는 함수
fn read_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다");
    
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("n을 파싱하는 데 실패했습니다");
    let m: usize = iter.next().unwrap().parse().expect("m을 파싱하는 데 실패했습니다");
    
    (n, m)
}

// 그래프 생성 함수
fn make_graph(n: usize, m: usize) -> Vec<Vec<usize>> {
    // 기사의 이동 방향 (8가지 L자 이동)
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

// 기사의 여행 탐색 함수
fn tour(
    kth: usize,        // 현재까지의 이동 횟수
    v: usize,          // 현재 위치
    n: usize,          // 체스판 가로 크기
    m: usize,          // 체스판 세로 크기
    graph: &Vec<Vec<usize>>, // 인접 리스트
    mark: &mut Vec<usize>,   // 방문 표시 배열
    cnt: &mut usize,         // 해결책 카운터
) {
    if kth == n * m {
        // 모든 칸을 방문했다면 해결책 카운트 증가
        *cnt += 1;
        
        // 해결책 출력 (주석 처리됨)
        println!("solution #{}: ", cnt);
        for i in 0..n {
            for j in 0..m {
                print!("{:2} ", mark[i * m + j]);
            }
            println!();
        }
    } else {
        // 현재 위치에서 이동할 수 있는 모든 다음 위치 시도
        for &u in &graph[v] {
            // 아직 방문하지 않은 위치라면
            if mark[u] == 0 {
                // 이 위치를 방문 표시
                mark[u] = kth + 1;
                
                // 다음 단계 재귀 호출
                tour(kth + 1, u, n, m, graph, mark, cnt);
                
                // 백트래킹 (방문 표시 취소)
                mark[u] = 0;
            }
        }
    }
}