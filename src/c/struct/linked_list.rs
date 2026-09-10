use std::{
    panic::Location,
    ptr::{self, null_mut},
};
/*
링크드 리스트
Head - 첫번쨰 노드
Tail - 마지막 노드
단일 연결 리스트는 노드가 자기 이전 노드를 몰라
*/
//Node
struct Node<T> {
    data: T,
    next_node: *mut Node<T>,
}
//새로운 노드
fn create_node<T>(new_data: T) -> *mut Node<T> {
    let new_node = Box::new(Node {
        data: new_data,
        next_node: ptr::null_mut(),
    });
    //Box 가 힙메모리를 넘기고 주소의 raw포인터 얻음
    Box::into_raw(new_node)
}

fn destory_node<T>(node: *mut Node<T>) {
    drop(node);
}

fn append_node<T>(head: *mut *mut Node<T>, new_node: *mut Node<T>) {
    unsafe {
        if *head == null_mut() {
            *head = new_node;
        } else {
            //
            let mut tail = *head;
            while (*tail).next_node != null_mut() {
                tail = (*tail).next_node;
            }

            (*tail).next_node = new_node;
        }
    }
}

fn get_node_at<T>(head: *mut Node<T>, location: usize) {
    unsafe {
        let mut current = head;

        while current != null_mut() && location >= 0 {
            current = (*current).next_node;
        }
    }
}
