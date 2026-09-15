use std::alloc::{alloc, alloc_zeroed};
use std::mem::size_of;
use std::ptr::null_mut;
use std::{alloc, alloc::Layout};
type data_t = i32;
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
        if head == null_mut() {
            return;
        }
        (*new_node).prev = *head;
        (*new_node).next = (*(*head)).next;
    }
}
pub fn example() {}
