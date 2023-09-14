
/*
큐 
- 삽입과 삭제 연산이 선입선출 FIFO로 이뤄지는 자료구조 입니다.
- 스택과 다르게 먼저 들어온 데이터가 먼저 나갑니다.
- 그래서 삽입과 삭제가 양방향에서 이루어집니다.
 -새값 추가는 rear에서 이뤄지고 삭제는 큐의 front에서 이뤄집니다.
 - 일반저긍로 deque를 이용하여 큐를 구현합니다.
 - rear:큐에서 가장 끝 데이터를 가리키는 영역
 - front큐에서 가장 앞의 데이터를 가리키는 영역


 우선순위 큐 :힙
선입후출 */
struct Queue<T> {
    items:Vec<T>,
}

impl<T>Queue <T> {
    fn new()->Self {
        Queue {items:Vec::new()}
    }

    fn enqueue(&mut self,item:T){
        self.items.push(item);
    }

    fn dequeue(&mut self)->Option<T>{
        if self.is_empty(){
            None
        }else {
            Some(self.items.remove(0))

        }
    }

    fn is_empty(&self)->bool {
        self.items.is_empty()
    }
}






/* medium 
new
remove
is_empty
*/


