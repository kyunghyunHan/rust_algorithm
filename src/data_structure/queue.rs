/*
Queue

- 스택과 큐는 Delete연산에 의해 집합에서 삭제되는 원소가 미리 정해져있는 동적집합
- 큐에서는 집합에서 가장 오랜시간동안 존재한 원소가 삭제
- First-in First out

-  Enqueue: Insert
- Dequeue:Delete

- 큐는 head와 tail인자를 갖는다


*/
#[derive(Clone, Debug)]
struct Queue<T> {
    // top: Option<Box<Node<T>>>,
    first: Option<Box<Node<T>>>,
    last: Option<Box<Node<T>>>,
}
#[derive(Clone, Debug)]

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
        let new_node = Node::new(item);

        if self.first.is_none() {
            self.first = Some(new_node.clone());
            self.last = Some(new_node);
            return;
        }

        let mut current = &mut self.first;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node.clone());
                self.last = Some(new_node);
                break;
            }
            current = &mut node.next;
        }
    }
    pub fn remove(&mut self) -> Option<T> {
        if let Some(first_node) = self.first.take() {
            self.first = first_node.next; // 첫 번째 노드를 제거하고 그 다음 노드를 가리킴

            // 큐가 비면 last도 None으로 설정
            if self.first.is_none() {
                self.last = None;
            }

            Some(first_node.data) // 제거된 첫 번째 노드의 데이터를 반환
        } else {
            None // 큐가 비어 있으면 None 반환
        }
    }
    fn peek(&self) -> Option<T> {
        match &self.first {
            Some(node) => Some(node.data.clone()),
            None => None,
        }
    }
    fn is_empty(&self) -> bool {
        self.first.is_none()
    }
}

pub fn example() {
    let mut q: Queue<i32> = Queue::new();
    q.add(1);
    q.add(2);
    q.add(3);
    q.add(4);
    println!("{:?}", q);

    println!("{:?}", q.remove());
    println!("{:?}", q.remove());
    println!("{:?}", q.peek());
    println!("{:?}", q.remove());
    println!("{:?}", q.is_empty());
    println!("{:?}", q.remove());
    println!("{:?}", q.is_empty());
}
