
/*
선형 큐 
- 삽입과 삭제 연산이 선입선출 FIFO로 이뤄지는 자료구조 입니다.
- 스택과 다르게 먼저 들어온 데이터가 먼저 나갑니다.
- 그래서 삽입과 삭제가 양방향에서 이루어집니다.
 - 새값 추가는 rear에서 이뤄지고 삭제는 큐의 front에서 이뤄집니다.
 - 일반적으로 deque를 이용하여 큐를 구현합니다.
 - rear:큐에서 가장 끝 데이터를 가리키는 영역
 - front큐에서 가장 앞의 데이터를 가리키는 영역
- 삽입을 위해서는 계속해서 요소들을 이동시켜야 함
- front, rear 는 증가만 하는 방식, 실제로는 front 앞쪽에 공간이 있더라도 삽입할 수 없는 경우가 발생할 수 있음

 -큐를 구현할 때 항상 빠지지 않고 나오는 Front와 Rear는 바로 앞과 뒤를 가리키는 인덱스이다. 처음에는 Front와 Rear가 모두 0 혹은 -1(빈 것을 구분하기 위해)에 존재하지만, 데이터를 넣을때, Rear가 점점 커지고, 데이터를 빼면 Front가 점점 거진다. 위 그림을 보면 Rear와 Front가 쉽게 이해할 수 있다.
 

 - 직선 큐의 안좋은점은 삭제할 경우 그자리가 비게 되는 현상발생,자료들을 이동시켜주는 작업을해야하는 번거로움이 있다.
선입후출 */
struct Queue {
    arr: Vec<i32>,  // Queue elements stored in a vector
    capacity: usize,  // Maximum capacity of the queue
    front: usize,  // Index pointing to the front element (if present)
    rear: usize,   // Index pointing to the rear element (if present)
    count: usize,  // Current number of elements in the queue
}

impl Queue {
    //큐 생성
    fn new(size: usize) -> Self {
        Queue {
            arr: vec![0; size],
            capacity: size,
            front: 0,
            rear: size - 1,
            count: 0,
        }
    }
   //Queue의 뒤쪽에 새 요소를 삽입
   //큐에 데이터를 추가하려 할 때는 큐가 full 인지 확인 후 진행
    fn enqueue(&mut self, item: i32) {
        // Check for queue overflow
        if self.is_full() {
            println!("Overflow\nProgram Terminated");
            std::process::exit(1);
        }

        println!("Inserting {}", item);

        // Increment the rear index and wrap around if necessary
        self.rear = (self.rear + 1) % self.capacity;
        self.arr[self.rear] = item;
        self.count += 1;
    }
    //Queue의 앞쪽 요소를 제거하고 반환
    fn dequeue(&mut self) -> Option<i32> {
        // Check for queue underflow
        if self.is_empty() {
            println!("Underflow\nProgram Terminated");
            std::process::exit(1);
        }

        let x = self.arr[self.front];
        println!("Removing {}", x);

        // Increment the front index and wrap around if necessary
        self.front = (self.front + 1) % self.capacity;
        self.count -= 1;

        Some(x)
    }  
//Queue의 전면 요소 반환
    fn peek(&self) -> Option<i32> {
        if self.is_empty() {
            println!("Underflow\nProgram Terminated");
            std::process::exit(1);
        }

        Some(self.arr[self.front])
    }
    //Queue에 있는 요소의 총 수
    fn size(&self) -> usize {
        self.count
    }
   //Queue이 비어 있는지 확인
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
    // Queue이 가득 찼는지 확인
    fn is_full(&self) -> bool {
        self.size() == self.capacity
    }
}

pub fn main() {
    // Create a queue with a capacity of 5
    //용량이 5인 큐 생성
    let mut q = Queue::new(5);
    //큐에 삽입
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    println!("The front element is {:?}", q.peek());
    q.dequeue();

    q.enqueue(4);
    println!("The front element is {:?}", q.peek());

    println!("The queue size is {}", q.size());

    q.dequeue();
    q.dequeue();
    q.dequeue();

    if q.is_empty() {
        println!("The queue is empty");
    } else {
        println!("The queue is not empty");
    }
}






