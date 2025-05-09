fn make_graph(n: usize, m: usize) -> Vec<Vec<usize>> {
    // 기사의 이동 방향 (8가지 L자 이동)
    let imove = [-2, -1, 1, 2, 2, 1, -1, -2];
    let jmove = [1, 2, 2, 1, -1, -2, -2, -1];
    
    // 그래프 초기화: n*m 개의 노드에 대해 빈 인접 리스트 생성
    let mut graph = vec![Vec::new(); n * m];
    
    // 모든 셀을 순회하며 가능한 이동을 그래프에 추가
    for i in 0..n {
        for j in 0..m {
            for k in 0..8 {
                // 다음 위치 계산
                let ni = i as i32 + imove[k];
                let nj = j as i32 + jmove[k];
                
                // 다음 위치가 체스판 내에 있는지 확인
                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    // 현재 위치 (i,j)의 인덱스는 i*m + j
                    // 다음 위치 (ni,nj)의 인덱스는 ni*m + nj
                    graph[i * m + j].push((ni as usize) * m + (nj as usize));
                }
            }
        }
    }
    
    graph
}