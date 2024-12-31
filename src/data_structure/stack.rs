/*
스택

- 스택과 큐는 Delete연산에 의해 집합에서 삭제되는 원소가 미리 정해져있는 동적집합
- 스택에서는 가장 최근에 삽입된 원소가 먼저 삭제
- 후입선출(last-in-fist-out)

- push:insert
- POP:Delete

- S[1:S.top]
- S[1은 스택에맨 밑에 원소를 나타내고] S[S.top]은 맨 위에 있는 원소를 나타낸다

- S.top=0일떄 스택은 원소를 포함하지 않고 비었디(empty)라고한다
- 빈 스택에서 원소를 추출하려는 경우 스택부족(underflow)라고한다
- s.top이 원소의개수 S.size를 초과하면 overflow라고한다
*/

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

    fn push(&mut self, item: T) {
        let mut t = Node::new(item);
        t.next = self.top.take(); //소유권 가져옴
        self.top = Some(t);
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
