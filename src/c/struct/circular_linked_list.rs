use std::alloc::{alloc, alloc_zeroed, dealloc};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::mem::size_of;
use std::process::exit;
use std::ptr::null_mut;
use std::{alloc, alloc::Layout};
struct Data {
    id: i32,
    score: i32,
}
type data_t = Data;

struct Node {
    data: data_t,
    prev: *mut Node,
    next: *mut Node,
}
fn init(head: *mut *mut Node) {
    unsafe {
        // let size = size_of::<Node>();
        let layout = Layout::new::<Node>();
        *head = alloc_zeroed(layout) as *mut Node;
        (**head).next = *head;
        (**head).prev = *head;
    }
}

//새로운 노드 생성
fn create_node(new_data: data_t) -> *mut Node {
    unsafe {
        let layout = Layout::new::<Node>();
        let new_node = alloc(layout) as *mut Node;
        if new_node == null_mut() {
            return null_mut();
        } else {
            (*new_node).data = new_data;
            (*new_node).prev = null_mut();
            (*new_node).next = null_mut();
        }
        return new_node;
    }
}
fn append_node(head: *mut *mut Node, new_node: *mut Node) {
    unsafe {
        if head.is_null() || (*head).is_null() || new_node.is_null() {
            return;
        }
        let current = (**head).prev;

        (*new_node).prev = current;
        (*new_node).next = (*current).next;

        (*(*current).next).prev = new_node;
        (*current).next = new_node;
    }
}

fn insert_afte(current: *mut Node, new_node: *mut Node) {
    unsafe {
        (*new_node).prev = current;
        (*new_node).next = (*current).next;
        (*(*current).next).prev = new_node;
        (*current).next = new_node;
    }
}

fn print_node(head: *mut Node) {
    unsafe {
        let mut current = (*head).next;
        while (current != head) {
            println!("{} {}", (*current).data.id, (*current).data.score);
            current = (*current).next;
        }
        println!();
    }
}

fn remove_node(target: *mut Node) {
    unsafe {
        (*(*target).prev).next = (*target).next;
        (*(*target).next).prev = (*target).prev;
        (*target).prev = null_mut();
        (*target).next = null_mut();
        let layout = Layout::new::<Node>();
        dealloc(target as *mut u8, layout);
    }
}
pub fn example() {
    let mut head = null_mut();
    init(&mut head);

    let reader: Box<dyn BufRead> = match File::open("data.txt") {
        Ok(file) => Box::new(BufReader::new(file)),
        Err(_) => Box::new(BufReader::new(io::stdin())),
    };

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();

        let Some(id) = iter.next() else {
            continue;
        };

        let Some(score) = iter.next() else {
            continue;
        };

        let id: i32 = id.parse().unwrap();
        let score: i32 = score.parse().unwrap();
        let data = data_t { id, score };
        let mut new_node = null_mut();
        new_node = create_node(data);
        if (new_node.is_null()) {
            exit(0);
        }
        append_node(&mut head, new_node);
        // println!("id: {}, score: {}", id, score);
    }
    print_node(head);
}
