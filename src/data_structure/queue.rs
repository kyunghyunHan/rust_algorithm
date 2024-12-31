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
impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            first: None,
            last: None,
        }
    }
    fn add(&mut self, item: T) {
        let t = Node::new(item);

        if let Some(mut last_node) = self.last.take() {
            last_node.next = Some(t);
            self.last = Some(last_node);
        } else {
            self.first = Some(t.clone());
            self.last = Some(t);
        }
    }

    fn remove(&mut self) -> Option<T> {
        if let Some(first_node) = self.first.take() {
            self.first = first_node.next;

            // first가 None이면 last도 None으로
            if self.first.is_none() {
                self.last = None;
            }

            Some(first_node.data)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.first {
            Some(node) => Some(node.data.clone()),
            None => None,
        }
    }
}

pub fn example() {
    let mut s: Queue<i32> = Queue::new();
    s.add(1);
    s.add(2);
    s.add(3);
    s.add(4);

    println!("{:?}", s.remove());
    println!("{:?}", s.remove());
    println!("{:?}", s.peek());
    println!("{:?}", s.remove());
    // println!("{:?}", s.is_empty());
    println!("{:?}", s.remove());
    // println!("{:?}", s.is_empty());
}
