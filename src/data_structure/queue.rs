/*
Queue

- 스택과 큐는 Delete연산에 의해 집합에서 삭제되는 원소가 미리 정해져있는 동적집합
- 큐에서는 집합에서 가장 오랜시간동안 존재한 원소가 삭제
- First-in First out

-  Enqueue: Insert
- Dequeue:Delete

- 큐는 head와 tail인자를 갖는다


*/
#[derive(Clone)]
struct Queue<T> {
    // top: Option<Box<Node<T>>>,
    first: Option<Box<Node<T>>>,
    last: Option<Box<Node<T>>>,
}
#[derive(Clone)]

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(data: T) -> Box<Node<T>> {
        Box::new(Node {
            data: data,
            next: None,
        })
    }
}
impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            first: None,
            last: None,
        }
    }
    fn add(&mut self, item: T) {
        let mut t = Node::new(item);

        match self.last.take() {
            Some(mut last) => {
                last.next = Some(t);
                // self.last = Some(t);
            }
            None => {
                self.first = Some(t);
                // self.last = self.first
            }
        }
        // if !self.last.is_none() {
        //     self.last = Some(t);
        // }
        // self.last = Some(t);
        // if self.first.is_none() {
        //     self.first = self.last.take()
        // }
    }

    fn remove() {}
    fn peek() {}
}

pub fn example() {}
