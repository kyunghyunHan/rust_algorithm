use std::collections::HashMap;

pub fn example() {
    let n: usize = 3;
    let m: usize = 10;
    
    // Create graph
    let graph = make_graph(n, m);
    
    // Print adjacency list
    for i in 0..n * m {
        print!("{}: [", i);
        for j in 0..graph[&i].len() {
            print!("{}", graph[&i][j]);
            if j < graph[&i].len() - 1 {
                print!(", ");
            }
        }
        println!("]");
    }
    
    // Initialize markers
    let mut mark = vec![0; n * m];
    let s = 0; // starting vertex
    mark[s] = 1; // mark starting vertex
    
    // Count solutions
    let mut count = 0;
    tour(1, s, s, n, m, &graph, &mut mark, &mut count);
    
    println!("{}", count);
}

fn make_graph(n: usize, m: usize) -> HashMap<usize, Vec<usize>> {
    let imove = [-2, -1, 1, 2, 2, 1, -1, -2];
    let jmove = [1, 2, 2, 1, -1, -2, -2, -1];
    
    // Initialize graph
    let mut graph = HashMap::new();
    for i in 0..n * m {
        graph.insert(i, Vec::new());
    }
    
    // Add all possible knight moves
    for i in 0..n {
        for j in 0..m {
            for k in 0..8 {
                let ni = i as i32 + imove[k];
                let nj = j as i32 + jmove[k];
                
                if 0 <= ni && ni < n as i32 && 0 <= nj && nj < m as i32 {
                    graph.get_mut(&(i * m + j)).unwrap().push((ni as usize) * m + (nj as usize));
                }
            }
        }
    }
    
    graph
}

fn tour(
    k: usize,
    v: usize,
    s: usize,
    n: usize,
    m: usize,
    graph: &HashMap<usize, Vec<usize>>,
    mark: &mut Vec<usize>,
    count: &mut usize,
) {
    if k == n * m && graph[&v].contains(&s) {
        *count += 1;
        mark[v] = k;
        
        for i in 0..n {
            for j in 0..m {
                print!("{:>2} ", mark[i * m + j]);
            }
            println!();
        }
    } else {
        for &u in &graph[&v] {
            if mark[u] == 0 {
                mark[u] = k + 1;
                tour(k + 1, u, s, n, m, graph, mark, count);
                mark[u] = 0;
            }
        }
    }
}