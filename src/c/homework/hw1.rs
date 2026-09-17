use std::{alloc::alloc, ptr::null_mut};

use crate::data_structure::queue;

struct Data {
    id: i32,
    score: i32,
}

struct Node {
    data: Data,
    next_node: *mut Node,
}

struct Queue {
    front: *mut Node,
    rear: *mut Node,
    nodes: *mut Node,
    count: i32,
    size: i32,
}

// fn init(s: i32) -> *mut Queue {
//     if s <= 0 {
//         return null_mut();
//     }
// }
