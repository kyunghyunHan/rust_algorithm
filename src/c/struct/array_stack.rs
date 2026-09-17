use std::{
    alloc::{alloc, dealloc, Layout},
    ptr::null_mut,
};

use crate::data_structure::array::Array;

type ElementType = i32;

struct Node {
    data: ElementType,
}
struct Stack {
    capacity: usize,
    top: i32,
    nodes: *mut Node,
}

fn create_stack(stack: *mut *mut Stack, capacity: usize) {
    unsafe {
        if stack.is_null() || capacity <= 0 {
            return;
        }

        *stack = null_mut();

        *stack = alloc(Layout::new::<Stack>()) as *mut Stack;

        if stack.is_null() {
            return;
        }
        (**stack).nodes = alloc(Layout::array::<Node>(capacity).unwrap()) as *mut Node;
        if ((**stack).nodes.is_null()) {
            dealloc(*stack as *mut u8, Layout::new::<Node>());
            *stack = null_mut();
            return;
        }
        (**stack).capacity = capacity;
        (**stack).top = -1;
    }
}

fn destroy_stack(stack: *mut Stack) {
    unsafe {
        if stack.is_null() {
            return;
        }

        dealloc(
            (*stack).nodes as *mut u8,
            Layout::array::<Node>((*stack).capacity).unwrap(),
        );
        dealloc((stack) as *mut u8, Layout::new::<Stack>());
    }
}

fn push(stack: *mut Stack, data: ElementType) {
    unsafe {
        if stack.is_null() || (*stack).top + 1 >= (*stack).capacity.try_into().unwrap() {
            return;
        }
        (*stack).top += 1;
        (*(*stack).nodes.add((*stack).top as usize)).data = data;
    }
}

fn pop(stack: *mut Stack) -> i32 {
    unsafe {
        if stack.is_null() {
            return 0;
        }
        let position = (*stack).top;
        let result = (*(*stack).nodes.add(position as usize)).data;
        (*stack).top -= 1;

        return result;
    }
}
fn print_stack(stack: *mut Stack) {
    unsafe {
        if stack.is_null() {
            return;
        }

        for i in 0..(*stack).top + 1 {
            println!("popped: {}, ", pop(stack));
        }
    }
}
pub fn example() {
    let mut i = 0;
    let mut stack = null_mut();

    create_stack(&mut stack, 10);

    if (stack.is_null()) {
        return;
    }

    push(stack, 3);
    push(stack, 3);
    push(stack, 3);
    push(stack, 3);
    push(stack, 27);
    print_stack(stack);
}
