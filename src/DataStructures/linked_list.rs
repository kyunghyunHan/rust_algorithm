use std::thread::current;


/*연결리스트 
각 노드가 데이터와 포인터를 가지고 한 줄로 연결되어 있는 방식으로 데이터를 저정하는 자료구조
- 동적으로 메모리 사용 가능
- 메모리 호율적으로 사용
- 데이터 재구성 용이
- 대용량 데이터 처리 적합

 =======단점 ==========
 - 특정 위치 데이터 검색할 떄 느리다
 - 메모리를 추가적으로  사용해야 한다.
 - 첫번째 혹은 마지막 노드를 탐색할 때는 쉽게 찾을수 있지만 중간 노드를 탐색할 경우 첫 노드부터 순차적으로 탐색해야 하기때문에 느리고 구현이 까다롭다

*/
/*
노드
Data를 저장할 공간과 다음주소를 가리킬 공간이 필요
- 사용자가 입력하는 정보를 Data영역에 담고 노드가 추가될떄마다 Next address를 이용하여 다음노드와 연결
- Null이면 마지막 노드

 */
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/*
맨처음 노드의 주소를 기리킬 노드 haed
마지막 노드를 가리킬 노드 tail
다음주소가 가리키는 포인터는 null


 */
struct LinkedList<T: PartialEq> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq> LinkedList<T> {
    // 새로운 빈 연결 리스트를 생성하는 함수
    fn new() -> Self {
        LinkedList { head: None }
    }
    /*
    push 함수: 연결 리스트의 맨 앞에 값을 추가하는 함수입니다. 새로운 노드를 생성하고, 해당 노드를 연결 리스트의 헤드로 지정합니다.

    - 1)맨앞에 노드를 추가하는 경우
    - - 새로 추가되는 노드의 다음주소 ->현재 head가 가리키는 주소
    - - head 가 가리키는 주소 ->새로 추가된 노드

    - 2)맨 마지막에 삽입하는 방법
    - - head대신 tail사용
    -- tail노드가 없다면 매번삽입할떄마다 처음부터 끝까지 삽입하는 번거로움이 발생
    - - 매번 O(n)의 시간복잡도가 발생한다
    - - 새로 추가되는 노드의 다음주소 ->Null(마지막 노드이기 때문에)
    - - tail이 가리키는 노드의 다음주소 ->새로 추가되는 다음노드
    - - tail이 가리키는 주소 ->새로 추가된 주소

    - -3)원하는 위치에 삽입하는 방법
    - - 삽입할 위치를 찾는 cur이 필요
    - - 1.탐색을 통해 cur노드가 4를 가리키게 만든다
    - - 2.새로운 노드5가 가리키는 주소->cur이 가리키는 노드4가 가리키는 다음노드
    - - cur이 가리키는 노드4가 가리키는 다음노드->새로운노드5
     */
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    /* pop 함수: 연결 리스트의 맨 앞에서 값을 가져오는 함수입니다. 현재 헤드 노드를 가져온 뒤, 헤드를 다음 노드로 업데이트하고 해당 노드의 값을 반환합니다.*/
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    /*
    remove 함수: 연결 리스트에서 특정 값을 삭제하는 함수입니다. 헤드 노드부터 순회하면서 삭제할 값을 가진 노드를 찾고, 이전 노드의 포인터를 업데이트하여 삭제합니다.
    - pre노드필요
    - 삭제할 노드를 1
    1.탐색을 통해 삭제할 노드를 cur이 가리키게 하고 삭제할 노드의 바로 전 노드를 pre가 가리키게 된다
    2.pre가 가리키는 노드의 다음주소 ->cur이 가리키는 다음주소
    3.cur이 가리키는 노드는 free하다

 */
    fn remove(&mut self, value: T) {
        let mut cur = &mut self.head;

        // 헤드 노드가 삭제할 값을 가지고 있는지 확인
        if let Some(node) = cur {
            if node.value == value {
                self.head = node.next.take();
                return;
            }
        }

        // 연결 리스트를 순회하면서 삭제할 노드를 찾고, 포인터를 업데이트하여 노드를 삭제함
        while let Some(node) = cur {
            if let Some(next_node) = &mut node.next {
                if next_node.value == value {
                    node.next = next_node.next.take();
                    break;
                }
            }
            cur = &mut node.next;
        }
    }

   /*
   is_empty 함수: 연결 리스트가 비어 있는지 확인하는 함수입니다. 헤드가 None인지 여부를 확인하여 비어 있는지 판단합니다.
   */
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub fn main() {
    //this provides a typical constructor and we can use it to create a new node like so;
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push(3);
    list.push(2);
    list.push(1);

    list.remove(2);

    while let Some(value) = list.pop() {
        println!("{}", value);
    }
}
/*medium */

impl <T>Node<T> {

    fn new(value:T)->Node<T>{
        Node{value,next:None}
    }
    fn from(mut v:Vec<T>)-> Node<T> {
        let mut current= Node::new(v.remove(v.len()-1));
        while v.len() >0 {
            let mut node = Node::new(v.remove(v.len()-1));
            node.next = Some(Box::new(current));
            current = node;
        }
        current

    }

    fn to_vec(self)->Vec<T> {
        let mut v= vec![];
        let mut current = Some(Box::new(self));
        while let Some(node)= current {
            v.push(node.value);
            current = node.next;
        }
        v
    }

    fn rev(self)->Node<T>{
        let mut next_node = None;
        let mut node= self;
        let mut old_next_node_option = node.next;
        node.next = next_node;
        while let Some(mut old_next_node) = old_next_node_option {
            old_next_node_option = old_next_node.next;
            old_next_node.next = Some(Box::new(node));
            node = *old_next_node;
        }
        node
    }

}

struct NodeIterator<'a, T> {
    e: Option<&'a Node<T>>,
  }
  impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      return if let Some(e) = self.e {
        self.e = if let Some(n) = &e.next {
          Some(n.as_ref())
        } else {
          None
        };
        Some(&e.value)
      } else {
        None
      };
    }
  }
  