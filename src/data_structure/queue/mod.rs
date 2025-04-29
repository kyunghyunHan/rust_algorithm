#[derive(Debug)]
struct Queue<T> {
    size: usize,
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>, // 원시 포인터 사용
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>, // prev 포인터 제거
}

impl<T> Node<T> {
    fn new(val: T) -> Box<Node<T>> {
        Box::new(Node { val, next: None })
    }
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            size: 0,
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn enqueue(&mut self, x: T) {
        let mut new_node = Node::new(x);
        let raw_node = Box::into_raw(new_node);

        unsafe {
            if !self.tail.is_null() {
                (*self.tail).next = Some(Box::from_raw(raw_node));
                self.tail = raw_node;
            } else {
                self.head = Some(Box::from_raw(raw_node));
                self.tail = raw_node;
            }
        }

        self.size += 1;
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            self.size -= 1;

            if let Some(next) = head.next {
                self.head = Some(next);
            } else {
                self.tail = std::ptr::null_mut();
            }

            head.val
        })
    }
}
pub fn example() {
    let mut q: Queue<i32> = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.enqueue(4);
    println!("{:?}", q);

    println!("{:?}", q.dequeue()); // Some(1)
    println!("{:?}", q.dequeue()); // Some(2)
    println!("{:?}", q.peek()); // Some(3)
    println!("{:?}", q.dequeue()); // Some(3)
    println!("{:?}", q.is_empty()); // false
    println!("{:?}", q.dequeue()); // Some(4)
    println!("{:?}", q.is_empty()); // true
}
