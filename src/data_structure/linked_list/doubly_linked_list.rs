use std::fmt::Display;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

// 이중 연결 리스트의 노드
#[derive(Debug)]
struct DoublyLinkedNode<T> {
    data: T,
    next: Option<NonNull<DoublyLinkedNode<T>>>,
    prev: Option<NonNull<DoublyLinkedNode<T>>>,
}

impl<T> DoublyLinkedNode<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            prev: None,
        }
    }
}

// 이중 연결 리스트
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<NonNull<DoublyLinkedNode<T>>>,
    tail: Option<NonNull<DoublyLinkedNode<T>>>,
    length: usize,
    // PhantomData로 T의 소유권을 명시
    marker: PhantomData<Box<DoublyLinkedNode<T>>>,
}

impl<T> DoublyLinkedList<T> {
    // 새로운 빈 리스트 생성
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
            marker: PhantomData,
        }
    }

    // 리스트 길이 반환
    pub fn len(&self) -> usize {
        self.length
    }

    // 리스트가 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // 앞쪽에 노드 추가
    pub fn push_front(&mut self, data: T) {
        unsafe {
            let new_node =
                NonNull::new_unchecked(Box::into_raw(Box::new(DoublyLinkedNode::new(data))));

            match self.head {
                None => {
                    // 빈 리스트인 경우
                    self.head = Some(new_node);
                    self.tail = Some(new_node);
                }
                Some(old_head) => {
                    // 기존 head 앞에 추가
                    (*new_node.as_ptr()).next = Some(old_head);
                    (*old_head.as_ptr()).prev = Some(new_node);
                    self.head = Some(new_node);
                }
            }

            self.length += 1;
        }
    }

    // 뒤쪽에 노드 추가
    pub fn push_back(&mut self, data: T) {
        unsafe {
            let new_node =
                NonNull::new_unchecked(Box::into_raw(Box::new(DoublyLinkedNode::new(data))));

            match self.tail {
                None => {
                    // 빈 리스트인 경우
                    self.head = Some(new_node);
                    self.tail = Some(new_node);
                }
                Some(old_tail) => {
                    // 기존 tail 뒤에 추가
                    (*old_tail.as_ptr()).next = Some(new_node);
                    (*new_node.as_ptr()).prev = Some(old_tail);
                    self.tail = Some(new_node);
                }
            }

            self.length += 1;
        }
    }

    // 앞쪽 노드 제거
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|old_head| {
                let boxed_node = Box::from_raw(old_head.as_ptr());

                match boxed_node.next {
                    None => {
                        // 마지막 노드였던 경우
                        self.head = None;
                        self.tail = None;
                    }
                    Some(new_head) => {
                        // 새로운 head 설정
                        (*new_head.as_ptr()).prev = None;
                        self.head = Some(new_head);
                    }
                }

                self.length -= 1;
                boxed_node.data
            })
        }
    }

    // 뒤쪽 노드 제거
    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.tail.map(|old_tail| {
                let boxed_node = Box::from_raw(old_tail.as_ptr());

                match boxed_node.prev {
                    None => {
                        // 마지막 노드였던 경우
                        self.head = None;
                        self.tail = None;
                    }
                    Some(new_tail) => {
                        // 새로운 tail 설정
                        (*new_tail.as_ptr()).next = None;
                        self.tail = Some(new_tail);
                    }
                }

                self.length -= 1;
                boxed_node.data
            })
        }
    }

    // 특정 인덱스의 값 조회
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        unsafe {
            let mut current = self.head?;

            // 중간 지점을 기준으로 앞/뒤에서 시작할지 결정 (최적화)
            if index < self.length / 2 {
                // 앞에서부터 탐색
                for _ in 0..index {
                    current = (*current.as_ptr()).next?;
                }
            } else {
                // 뒤에서부터 탐색
                current = self.tail?;
                for _ in 0..(self.length - 1 - index) {
                    current = (*current.as_ptr()).prev?;
                }
            }

            Some(&(*current.as_ptr()).data)
        }
    }

    // 특정 인덱스에 값 삽입
    pub fn insert(&mut self, index: usize, data: T) -> Result<(), &'static str> {
        if index > self.length {
            return Err("Index out of bounds");
        }

        if index == 0 {
            self.push_front(data);
            return Ok(());
        }

        if index == self.length {
            self.push_back(data);
            return Ok(());
        }

        unsafe {
            let new_node =
                NonNull::new_unchecked(Box::into_raw(Box::new(DoublyLinkedNode::new(data))));

            // 삽입 위치의 노드 찾기
            let mut current = self.head.unwrap();
            for _ in 0..index {
                current = (*current.as_ptr()).next.unwrap();
            }

            let prev_node = (*current.as_ptr()).prev.unwrap();

            // 새 노드 연결
            (*new_node.as_ptr()).next = Some(current);
            (*new_node.as_ptr()).prev = Some(prev_node);
            (*current.as_ptr()).prev = Some(new_node);
            (*prev_node.as_ptr()).next = Some(new_node);

            self.length += 1;
        }

        Ok(())
    }

    // 특정 인덱스의 노드 제거
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.length {
            return None;
        }

        if index == 0 {
            return self.pop_front();
        }

        if index == self.length - 1 {
            return self.pop_back();
        }

        unsafe {
            let mut current = self.head.unwrap();
            for _ in 0..index {
                current = (*current.as_ptr()).next.unwrap();
            }

            let prev_node = (*current.as_ptr()).prev.unwrap();
            let next_node = (*current.as_ptr()).next.unwrap();

            // 이전 노드와 다음 노드 연결
            (*prev_node.as_ptr()).next = Some(next_node);
            (*next_node.as_ptr()).prev = Some(prev_node);

            self.length -= 1;

            let boxed_node = Box::from_raw(current.as_ptr());
            Some(boxed_node.data)
        }
    }

    // 리스트를 Vec으로 변환 (순방향)
    pub fn to_vec(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut current = self.head;

        unsafe {
            while let Some(node) = current {
                result.push(&(*node.as_ptr()).data);
                current = (*node.as_ptr()).next;
            }
        }

        result
    }

    // 리스트를 Vec으로 변환 (역방향)
    pub fn to_vec_reverse(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut current = self.tail;

        unsafe {
            while let Some(node) = current {
                result.push(&(*node.as_ptr()).data);
                current = (*node.as_ptr()).prev;
            }
        }

        result
    }
}

impl<T: Display> DoublyLinkedList<T> {
    // 문자열로 변환
    pub fn to_string(&self) -> String {
        self.to_vec()
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(" <-> ")
    }
}

// 메모리 안전성을 위한 Drop 구현
impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// 기본 Default 구현
impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Iterator 구현 (순방향)
pub struct IntoIter<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.length, Some(self.list.length))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

// 사용 예제
pub fn example() {
    let mut list = DoublyLinkedList::new();

    // 값 추가
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_front(0);

    println!("List: {}", list.to_string()); // 0 <-> 1 <-> 2 <-> 3
    println!("Length: {}", list.len()); // 4

    // 특정 인덱스 값 조회
    if let Some(value) = list.get(2) {
        println!("Index 2: {}", value); // 2
    }

    // 중간에 값 삽입
    list.insert(2, 99).unwrap();
    println!("After insert: {}", list.to_string()); // 0 <-> 1 <-> 99 <-> 2 <-> 3

    // 값 제거
    if let Some(removed) = list.remove(2) {
        println!("Removed: {}", removed); // 99
    }
    println!("After remove: {}", list.to_string()); // 0 <-> 1 <-> 2 <-> 3

    // 앞뒤에서 제거
    println!("Pop front: {:?}", list.pop_front()); // Some(0)
    println!("Pop back: {:?}", list.pop_back()); // Some(3)
    println!("Final: {}", list.to_string()); // 1 <-> 2

    // 역방향 출력
    println!("Reverse vec: {:?}", list.to_vec_reverse()); // [&2, &1]

    // Iterator 사용
    for value in list {
        println!("Value: {}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = DoublyLinkedList::new();

        list.push_front(1);
        list.push_back(2);
        list.push_front(0);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn test_get_insert_remove() {
        let mut list = DoublyLinkedList::new();

        list.push_back(1);
        list.push_back(3);
        list.insert(1, 2).unwrap();

        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));

        assert_eq!(list.remove(1), Some(2));
        assert_eq!(list.len(), 2);
    }
}
