use std::collections::HashMap;

struct Graph {
    graph: HashMap<i32, Vec<i32>>,
    vertices: Vec<i32>,
}

impl Graph {
    fn new(vertices: Vec<i32>) -> Self {
        let mut graph = HashMap::new();
        for vertex in &vertices {
            graph.insert(*vertex, Vec::new());
        }
        Graph { graph, vertices }
    }

    fn add_edge(&mut self, u: i32, v: i32) {
        self.graph.get_mut(&u).unwrap().push(v);
        self.graph.get_mut(&v).unwrap().push(u);
    }

    fn dfs_count(&self, i: i32, visited: &mut HashMap<i32, bool>) -> i32 {
        let mut count = 1;
        visited.insert(i, true);

        for j in self.graph.get(&i).unwrap() {
            if !*visited.get(j).unwrap_or(&false) {
                count += self.dfs_count(*j, visited);
            }
        }

        count
    }

    fn is_connected(&self) -> bool {
        let mut visited: HashMap<i32, bool> = HashMap::new();
        for vertex in &self.vertices {
            visited.insert(*vertex, false);
        }

        // Start DFS from the first vertex
        if let Some(&first_vertex) = self.vertices.first() {
            for i in &self.vertices {
                if !self.graph.get(i).unwrap().is_empty() {
                    self.dfs_count(*i, &mut visited);
                    break;
                }
            }

            // Check if all vertices are visited
            for vertex in &self.vertices {
                if !*visited.get(vertex).unwrap_or(&false)
                    && !self.graph.get(vertex).unwrap().is_empty()
                {
                    return false;
                }
            }
        }

        true
    }

    fn is_eulerian(&self) -> i32 {
        // Check if the graph is connected
        if !self.is_connected() {
            return 0;
        }

        // Count vertices with odd degree
        let mut odd = 0;
        for i in &self.vertices {
            if self.graph.get(i).unwrap().len() % 2 != 0 {
                odd += 1;
            }
        }

        if odd == 0 {
            return 2; // Eulerian Circuit
        } else if odd == 2 {
            return 1; // Eulerian Path
        } else {
            return 0; // Not Eulerian
        }
    }
}

pub fn example() {
    let mut g = Graph::new(vec![0, 1, 2, 3, 4, 5]);
    g.add_edge(0, 1);
    g.add_edge(1, 2);
    g.add_edge(2, 3);
    g.add_edge(3, 4);
    g.add_edge(4, 5);

    let res = g.is_eulerian();

    if res == 0 {
        println!("Graph is not Eulerian");
    } else if res == 1 {
        println!("Graph has an Euler path");
    } else {
        println!("Graph has an Euler circuit");
    }
}
