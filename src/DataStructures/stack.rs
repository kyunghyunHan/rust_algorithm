/*자료구조 -Stack
스택구조
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

