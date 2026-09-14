use std::ptr::{self, null_mut};
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
//뒤에 node 저장
fn append_node<T>(head: *mut *mut Node<T>, new_node: *mut Node<T>) {
    unsafe {
        //head == null 이면
        if *head == null_mut() {
            //head에 새로운 노드 저장
            *head = new_node;
        } else {
            //
            //null 이 아니라면
            //head는 tail
            let mut tail = *head;
            //테일이 nulldl 아닐때까지 tail을 next node
            while (*tail).next_node != null_mut() {
                tail = (*tail).next_node;
            }
            //테일의 다음노드는 new node
            (*tail).next_node = new_node;
        }
    }
}

fn get_node_at<T>(head: *mut Node<T>, mut location: usize) -> *mut Node<T> {
    unsafe {
        let mut current = head;

        while current != null_mut() && location >= 0 {
            current = (*current).next_node;
            location -= 1;
        }
        current
    }
}

pub fn example() {
    unsafe {
        let mut i = 0;
        let mut cnt = 0;
        let mut list: *mut Node<i32> = null_mut();
        let mut current: *mut Node<i32> = null_mut();
        let mut new_node: *mut Node<i32> = null_mut();

        for i in 0..5 {
            new_node = create_node(i);
            append_node(&mut list, new_node);
        }
    }
}
