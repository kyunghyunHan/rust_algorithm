struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

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
impl<T> Stack<T> {
    // Stack 생성자
    pub fn new() -> Self {
        Stack { top: None }
    }
    /*Push:adds key to collection */
    fn push(&mut self, item: T) {
        let mut t = Node::new(item);
        t.next = self.top.take(); //소유권 가져옴
        self.top = Some(t);
    }
    fn top() {}
    fn pop(&mut self) -> Option<T> {
        match self.top.take() {
            Some(node) => {
                //다음을 탑으로
                self.top = node.next;
                Some(node.data)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(node) => Some(&node.data),
            None => None,
        }
    }

    fn is_empty(&self) -> bool {
        self.top.is_none()
    }
}

pub fn example() {
    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    s.push(4);

    println!("{:?}", s.pop());
    println!("{:?}", s.pop());
    println!("{:?}", s.peek());
    println!("{:?}", s.pop());
    println!("{:?}", s.is_empty());
    println!("{:?}", s.pop());
    println!("{:?}", s.is_empty());
}
