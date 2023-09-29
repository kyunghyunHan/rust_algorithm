/*자료구조 -Stack
- 두번쨰 데이터 구조는 스택입니다.
- 스택은 마치 접시가 쌓이는 것과 비슷한 삽입과 삭제연산이 후입선출로 이루어지는 자료구조 입니다.
- 후입선출은 LIFO(last in, first out)이라고 하며  삽입과 삭제가 한쪽에서만 일어나는 특징이 있습니다.
- 새값이 스택이 들어가게 되면 Top이 새 값을 가르킵니다.스택에서 값을 빼낼떄 pop이 top이 가르키는 값을 스택에서 빼게되어 있으므로 결과적으로는 가장 마지막에 넣엇던 값이 나오게 됩니다.
- 스택은 깊이 우선 탐색,백트래킹 종류의 코딩 테스트에 효과적이므로 반드시 알아 두어야합니다.

장점
- 구조가 단순해서,구현이 쉽다.
- 데이터 저장/읽기 속도가 빠르다.

단점
- 데이터의 최대개수를 정해나야한다.(파이썬의 경우 재귀함수는 1000번까지만 호출이 가능하기때문에 1000번까지 공간을 미리 확보해둔다.)
 - 맨 위의 원소만 접근해야 하기 때문에 삽입과 삭제에 있어 매우 비효율적이다.
 - 저장 공간의 낭비가 발생할수 있다.(미리 최대 개수만큼 저장 공간 확보해야 함)
 */
//스택 구조체
struct Stack<T> {
    data: Vec<T>,
}
//
impl<T> Stack<T> {
    //스택 생성
    fn new() -> Self {
        Stack { data: Vec::new() }
    }
    /*스택에 값을 추가하는 함수 */
    fn push(&mut self, item: T) {
        self.data.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
   //스택이 비어 있는지확인
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    //스택의 크기를 반환
    fn size(&self) -> usize {
        self.data.len()
    }
    /* top위치에 있는 값을 가져오는 함수 */
    fn peek(&self) -> Option<&T> {
        self.data.last()
    }
}

pub fn main() {
    let mut stack: Stack<i32> = Stack::new();
    //스택에 1을 추가
    stack.push(1);
    //스택에 2를 추가
    stack.push(2);
    //스택에 3를 추가
    stack.push(3);
   //스택을 위에서부터 제거
    while let Some(item) = stack.pop() {
        println!("Popped item: {}", item);
    }
     //스택에 값이 있으면 false 없으면 true
    println!("Stack is empty: {}", stack.is_empty());
}

