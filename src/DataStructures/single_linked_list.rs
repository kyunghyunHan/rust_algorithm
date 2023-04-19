/*Single linked list 
연결리스트란 하나의 노드의 값과 다음 노드를 가르키는 노드로 이루어진 자료구조

- 노드가 다음 노드만을 가르키기 때문에 단일 연결리스트라함
- 배열의 경우 원소에 접근하는것은 효율적
- 데이터를 추가하거나 삭제하기 위해서는 그 위치에 요소를 삭제한 다음 그 뒤에 있는 모든 원소를 움직여야 한다.
*/
//단일 연결리스트 노드정의

use std::option::Option;
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node{
    //새로운 노드생성
      // 새로운 노드를 생성하는 함수
      fn new(data: i32) -> Node {
        Node { data: data, next: None }
    }
}

//단일 연결리스트 구현
struct LinkedList{
    head:Option<Box<Node>>
}
impl LinkedList{
    //빈리스트 생성함수
    fn new()->Self{
        Self { head: None }
    }
    //리스트에 새로운 노드를 추가하는 함수
    fn push(&mut self, data: i32) {
        let mut new_node = Box::new(Node::new(data));
        match self.head.take() {
            Some(old_head) => {
                new_node.next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }


    // 리스트에서 노드를 제거하고 해당 노드의 데이터를 반환하는 함수
    fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.data)
            }
            None => None,
        }
    }
    
     //리스트가 비어 있는지 확인하는 함수
    fn is_empty(&self)->bool{

        self.head.is_none()
    }
}

pub fn main(){



}