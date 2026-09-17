type VElementType = i32;
enum VisitMode {
    Visited,
    NotVisted,
}
struct Vertex {
    data: VElementType,
    visited: i32,
    index: i32,

    next: *mut Vertex,
    edge: *mut Vertex,
    adjacenc_list: Edge,
}

struct Edge {
    weight: i32,
    next: *mut Edge,
    from: *mut Vertex,
    target: *mut Vertex,
}

struct Graph {
    vertices: *mut Vertex,
    vertex_count: i32,
}

pub fn example() {}
