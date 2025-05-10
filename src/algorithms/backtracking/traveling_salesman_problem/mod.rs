fn hamiltonian(i: usize, w: &Vec<Vec<bool>>, vindex: &mut Vec<usize>, n: usize) {
    if promising(i, w, vindex, n) {
        if i == n - 1 {
            // 출력: n개의 정점까지만 출력
            println!("{:?}", &vindex[0..n]);
        } else {
            for j in 2..=n {
                vindex[i + 1] = j;
                hamiltonian(i + 1, w, vindex, n);
            }
        }
    }
}

fn promising(i: usize, w: &Vec<Vec<bool>>, vindex: &Vec<usize>, n: usize) -> bool {
    if i == n - 1 && !w[vindex[n - 1]][vindex[0]] {
        return false;
    }
    if i > 0 && !w[vindex[i - 1]][vindex[i]] {
        return false;
    }
    for j in 1..i {
        if vindex[i] == vindex[j] {
            return false;
        }
    }
    true
}

pub fn example() {
    let n = 4;
    let edges = vec![
        vec![1, 2],
        // vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ];

    // 그래프 인접 행렬
    let mut w = vec![vec![false; n + 1]; n + 1];
    for edge in edges {
        let u = edge[0];
        let v = edge[1];
        w[u][v] = true;
        w[v][u] = true;
    }

    let mut vindex = vec![0; n + 1];
    vindex[0] = 1;

    hamiltonian(0, &w, &mut vindex, n);
}
