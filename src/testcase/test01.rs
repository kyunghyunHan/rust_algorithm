use std::alloc::{alloc, dealloc, Layout};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ptr::null_mut;
/*
stack 만들기
*/
struct Data {
    id: i32,
    score: i32,
}
struct Node {
    data: Data,
    next_node: *mut Node,
}
struct Stack {
    top: *mut Node,
    size: usize,
    count: i32,
}
fn init(s: usize) -> *mut Stack {
    unsafe {
        let layout = Layout::new::<Stack>();
        let stack = alloc(layout) as *mut Stack;
        (*stack).top = null_mut();
        (*stack).size = s;
        (*stack).count = 0;
        stack
    }
}
fn create_node(data: Data) -> *mut Node {
    unsafe {
        let layout = Layout::new::<Node>();
        let new_node = alloc(layout) as *mut Node;
        if new_node.is_null() {
            return null_mut();
        } else {
            (*new_node).data = data;
            (*new_node).next_node = null_mut();
        }
        return new_node;
    }
}

fn push(stack: *mut Stack, new_node: *mut Node) {
    unsafe {
        if stack.is_null() || new_node.is_null() {
            return;
        }
        (*new_node).next_node = (*stack).top;
        (*stack).top = new_node;
        (*stack).count += 1;
    }
}

fn pop(stack: *mut Stack) -> *mut Node {
    unsafe {
        if stack.is_null() {
            return null_mut();
        } else {
            let mut target = (*stack).top;
            (*stack).top = (*(*stack).top).next_node;
            (*target).next_node = null_mut();
            target
        }
    }
}
fn destroy_node(node: *mut Node) {
    unsafe {
        if node.is_null() {
            return;
        } else {
            let layout = Layout::new::<Node>();
            let node = node as *mut u8;
            dealloc(node, layout);
        }
    }
}
fn destroy_stack(stack: *mut Stack) {
    unsafe {
        if stack.is_null() {
            return;
        } else {
            let layout = Layout::new::<Stack>();
            let node = stack as *mut u8;
            dealloc(node, layout);
        }
    }
}
fn print_stack(stack: *mut Stack) {
    unsafe {
        if stack.is_null() {
            return;
        }
        let mut i = 0;
        let mut target = (*stack).top;
        while i < (*stack).count {
            println!("{} {}", (*target).data.id, (*target).data.score);
            target = (*target).next_node;
            i += 1;
        }
    }
}
pub fn example() {
    // let (s, n, r, c);
    let mut count = 0;

    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let cmd = line.next().unwrap();

        if cmd == "P" {
        } else if cmd == "D" {
        }
    }
}
