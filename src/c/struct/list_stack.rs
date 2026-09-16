use std::{
    alloc::{alloc, Layout},
    ptr::null_mut,
};

use crate::data_structure::queue;

struct Node {
    data: *mut u8,
    next_node: *mut Node,
}
struct LinkedListStack {
    list: *mut Node,
    top: *mut Node,
}

fn create_stack(stack: *mut *mut LinkedListStack) {
    unsafe {
        if stack.is_null() {
            return;
        }
        *stack = null_mut();
        let layout = Layout::new::<LinkedListStack>();
        let stack = alloc(layout) as *mut LinkedListStack;

        if stack.is_null() {
            return;
        }
        (*stack).list = null_mut();
        (*stack).top = null_mut();
    }
}

fn pop(stack: *mut LinkedListStack) -> *mut Node {
    unsafe {
        if stack.is_null() || is_empty((stack)) {
            return null_mut();
        } else {
            let top_node = (*stack).top;
            if (*stack).list == (*stack).top {
                (*stack).list = null_mut();
                (*stack).top = null_mut();
            } else {
                let mut current_top = (*stack).list;
                while !current_top.is_null() && (*current_top).next_node != (*stack).top {
                    current_top = (*current_top).next_node;
                }

                (*stack).top = current_top;
                if !(*stack).top.is_null() {
                    (*(*stack).top).next_node = null_mut();
                }
            }
            top_node
        }
    }
}
fn destroy_stack(stack: *mut LinkedListStack) {
    if stack.is_null() {
        return;
    }
    while !is_empty(stack) {
        // let popped =
    }
}
fn is_empty(stack: *mut LinkedListStack) -> bool {
    unsafe {
        return stack.is_null() || (*stack).list.is_null();
    }
}
pub fn example() {}
