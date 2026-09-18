use std::alloc::{alloc, dealloc, Layout};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::ptr::null_mut;
struct Node {
    r: i32,
    c: i32,
}
struct Queue {
    capacity: usize,
    rear: usize,
    front: usize,
    nodes: *mut Node,
}

fn init_queue(queue: *mut Queue, s: usize) {
    unsafe {
        let nodes_layout = Layout::array::<Node>(5).unwrap();

        let queue_layout = Layout::new::<Queue>();

        let (layout, nodes_offset) = queue_layout.extend(nodes_layout).unwrap();
        let mut queue = alloc(layout) as *mut Queue;

        (*queue).capacity = s;
        (*queue).front = 0;
        (*queue).rear = 0;
        (*queue).nodes = queue.add(1) as *mut Node;
    }
}

fn enqueue(queue: *mut Queue, node: Node) {
    unsafe {
        if ((*queue).rear >= (*queue).capacity) {
            println!("OverFlow");
            return;
        }
        *((*queue).nodes.add((*queue).rear)) = node;
        (*queue).rear += 1;
    }
}
fn dequeue(queue: *mut Queue) {
    unsafe {
        if (*queue).front >= (*queue).rear {
            println!("UnderFlow");
            return;
        }
        (*queue).front += 1;
        
    }
}

fn printf(queue: *mut Queue) {
    unsafe {
        for i in (*queue).front..(*queue).rear {
            println!(
                "{} : {} {}",
                i - (*queue).front,
                (*(*queue).nodes.add(i)).r,
                (*(*queue).nodes.add(i)).c
            )
        }
    }
}
pub fn example() {
    let reader: Box<dyn BufRead> = match File::open("data.txt") {
        Ok(file) => Box::new(BufReader::new(file)),
        Err(_) => Box::new(BufReader::new(std::io::stdin())),
    };

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();

        let Some(cmd) = iter.next() else {
            continue;
        };

        let cmd = cmd.chars().next().unwrap();

        if cmd == 'P' {
            let Some(r) = iter.next() else {
                continue;
            };

            let Some(c) = iter.next() else {
                continue;
            };
            let r: i32 = r.parse().unwrap();
            let c: i32 = c.parse().unwrap();
            let data = Node { r, c };
        }

        // let mut new_node = null_mut();
        // new_node = create_node(data);
        // if (new_node.is_null()) {
        //     exit(0);
        // }
        // append_node(&mut head, new_node);
        // println!("id: {}, score: {}", id, score);
    }
}
