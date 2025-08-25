use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct LinkedListNode<T> {
    pub value: T,
    pub next: Option<Box<LinkedListNode<T>>>,
}

impl<T: Display> LinkedListNode<T> {
    pub fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
    tail: Option<*mut LinkedListNode<T>>,
    length: usize,
}

impl<T: PartialEq + Display + Clone> LinkedList<T> {
    //create new Linked List
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.length
    }

    pub fn empty(&self) -> bool {
        self.head.is_none()
    }
    //can use insert above at index0
    pub fn prepend(&mut self, value: T) {
        let mut new_node = Box::new(LinkedListNode {
            value,
            next: self.head.take(),
        });
        let raw_ptr: *mut _ = &mut *new_node;

        if self.tail.is_none() {
            self.tail = Some(raw_ptr);
        }
        self.head = Some(new_node)
    }

    //리스트 끝에 값을 추가
    pub fn append(&mut self, value: T) {
        let mut new_node = Box::new(LinkedListNode { value, next: None });
        let raw_ptr: *mut _ = &mut *new_node;

        match self.tail {
            None => {
                self.head = Some(new_node);
                self.tail = Some(raw_ptr);
            }
            Some(tail_ptr) => unsafe {
                (*tail_ptr).next = Some(new_node);
                self.tail = Some(raw_ptr);
            },
        }
    }
    //지정한 인덱스 위치에 값을 추가

    pub fn insert(&mut self, value: T, mut index: usize) {
        if index == 0 {
            self.prepend(value);
            return;
        }

        let mut current = &mut self.head;
        while index > 1 {
            match current {
                Some(ref mut node) => {
                    current = &mut node.next;
                    index -= 1;
                }
                None => break,
            }
        }

        // 👇 Clone해서 사용
        let new_node = Box::new(LinkedListNode {
            value: value.clone(),
            next: current.as_mut().and_then(|n| n.next.take()),
        });

        if let Some(ref mut node) = current {
            node.next = Some(new_node);
        } else {
            self.append(value);
        }
    }
    //특정한 값을 가진 노드를 모두 삭제
    pub fn delete(&mut self, value: T) -> Option<T> {
        let mut deleted = None;

        while let Some(ref mut node) = self.head {
            if node.value == value {
                let old_head = self.head.take();
                self.head = old_head.and_then(|n| n.next);
                deleted = Some(value.clone());
            } else {
                break;
            }
        }

        let mut current = &mut self.head;

        while let Some(ref mut node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.value == value {
                    node.next = next_node.next.take();
                    deleted = Some(value.clone());
                    break;
                }
            }
            current = &mut node.next;
        }

        // update tail
        if let Some(ref mut node) = self.tail {
            unsafe {
                if (**node).value == value {
                    self.tail = None;
                }
            }
        }

        deleted
    }
    //리스트 앞또는 뒤의 노드삭제
    pub fn delete_head(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.head.is_none() {
                self.tail = None;
            }
            node.value
        })
    }

    pub fn delete_tail(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            if node.next.is_some() && node.next.as_ref().unwrap().next.is_none() {
                let tail_val = node.next.take().map(|n| n.value);
                self.tail = Some(&mut **node as *mut _);
                return tail_val;
            }
            current = &mut node.next;
        }

        self.head.take().map(|node| {
            self.tail = None;
            node.value
        })
    }

    //특정 값이나 콜백 조건에 맞는 노드를 찾기
    pub fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        let mut current = &self.head;

        while let Some(ref node) = current {
            if predicate(&node.value) {
                return Some(&node.value);
            }
            current = &node.next;
        }

        None
    }

    //벡터로부터 연결리스트 생성
    pub fn from_vec(&mut self, values: Vec<T>) {
        for value in values {
            self.append(value);
        }
    }
    //연결리스트를 벡터로 생성
    pub fn to_vec(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut current = &self.head;

        while let Some(ref node) = current {
            result.push(&node.value);
            current = &node.next;
        }

        result
    }

    //리스트를 문자열로출력
    pub fn to_string(&self) -> String {
        self.to_vec()
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(", ")
    }
    //리스트 뒤집기
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        self.tail = None;

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        // set new head and tail
        self.head = prev;

        // find new tail
        let mut current_tail = &mut self.head;
        while let Some(ref mut node) = current_tail {
            if node.next.is_none() {
                self.tail = Some(&mut **node as *mut _);
            }
            current_tail = &mut node.next;
        }
    }
}
pub fn example() {
    let mut list = LinkedList::new();
    list.append(10);
    list.append(20);
    list.prepend(5);

    println!("{:?}", list.to_vec()); // [5, 10, 20]

    list.delete(10);
    println!("{:?}", list.to_vec()); // [5, 20]

    list.reverse();
    println!("{:?}", list.to_vec()); // [20, 5]
}
