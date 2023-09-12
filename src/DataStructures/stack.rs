/*자료구조 -Stack
스택이란?

스택은 객체와 그 객체가 저장되는 순서를 기억하는 방법에 관한 추상 자료형입니다.
LIFO(last in, first out)이라고 하며 






 */
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

pub fn main() {
    let mut stack: Stack<i32> = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack size: {}", stack.size());

    while let Some(item) = stack.pop() {
        println!("Popped item: {}", item);
    }

    println!("Stack is empty: {}", stack.is_empty());
}

