use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Queue<T> {
    size: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            size: 0,
            head: None,
            tail: None,
        }
    }

    fn enqueue(&mut self, x: T) {
        let new_node = Node::new(x);
        if let Some(ref tail) = self.tail {
            // Link the new node to the current tail
            tail.borrow_mut().next = Some(new_node.clone());
        } else {
            // If the queue is empty, set head as well
            self.head = Some(new_node.clone());
        }
        self.tail = Some(new_node);
        self.size += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.size -= 1;

            // Take the next node while dropping the borrow immediately
            let next_opt = {
                let mut head_ref = old_head.borrow_mut();
                head_ref.next.take()
            };

            // Update head/tail pointers
            if let Some(next) = next_opt {
                self.head = Some(next);
            } else {
                self.tail = None;
            }

            // Now we can safely unwrap and move the value out
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.as_ref().map(|node| node.borrow().val.clone())
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

pub fn example() {
    let mut q: Queue<i32> = Queue::new();

    println!("--- Enqueue 4 numbers ---");
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.enqueue(4);
    println!("After enqueue: size={}, peek={:?}", q.size(), q.peek());

    println!("\n--- Dequeue two elements ---");
    println!("dequeue -> {:?}", q.dequeue()); // 1
    println!("dequeue -> {:?}", q.dequeue()); // 2
    println!("size now = {}, peek = {:?}", q.size(), q.peek()); // size=2, peek=3

    println!("\n--- Enqueue more elements ---");
    q.enqueue(5);
    q.enqueue(6);
    println!("After enqueue: size={}, peek={:?}", q.size(), q.peek());

    println!("\n--- Dequeue until empty ---");
    while let Some(v) = q.dequeue() {
        println!("dequeue -> {}", v);
    }
    println!("Queue empty? {}", q.is_empty());

    println!("\n--- Test dequeue/peek on empty queue ---");
    println!("dequeue -> {:?}", q.dequeue());
    println!("peek -> {:?}", q.peek());

    println!("\n--- Reuse the queue ---");
    q.enqueue(42);
    q.enqueue(99);
    println!("After enqueue: size={}, peek={:?}", q.size(), q.peek());
    println!("dequeue -> {:?}", q.dequeue());
    println!("dequeue -> {:?}", q.dequeue());
    println!("Queue empty? {}", q.is_empty());
}


