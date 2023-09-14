/*자료구조 -Stack
스택이란?
스택은 삽입과 삭제연산이 후입선출로 이뤄지는 자료구조입니다.후입선출은 LIFO(last in, first out)이라고 하며  삽입과 삭제가 한쪽에서만 일어나는 특징이 있습니다.
- 새값이 스택이 들어가게 되면 Top이 새 값을 가르킵니다.스택에서 값을 빼낼떄 pop이 top이 가르키는 값을 스택에서 빼게되어 있으므로 결과적으로는 가장 마지막에 넣엇던 값이 나오게 됩니다.
- 스택은 깊이 우선 탐색,백트래킹 종류의 코딩 테스트에 효과적이므로 반드시 알아 두어야합니다.








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

