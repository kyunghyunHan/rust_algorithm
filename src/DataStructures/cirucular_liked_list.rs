// 원형 연결 리스트의 노드를 정의합니다.
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    // 새로운 노드를 생성하는 생성자 함수입니다.
    fn new(data: T) -> Self {
        Node { data, next: None }
    }
}

// 원형 연결 리스트를 구현한 구조체입니다.
struct CircularLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T> CircularLinkedList<T> {
    // 빈 원형 연결 리스트를 생성하는 생성자 함수입니다.
    fn new() -> Self {
        CircularLinkedList { head: None, tail: None }
    }

    // 원형 연결 리스트의 맨 뒤에 노드를 추가하는 메서드입니다.
    fn push(&mut self, data: T) {
        let new_node = Box::new(Node::new(data));
        if self.head.is_none() {
            // 리스트가 비어있을 경우, 새로 추가되는 노드를 head와 tail로 설정합니다.
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            // 리스트에 이미 노드가 있는 경우, tail의 next를 새로 추가되는 노드로 설정하고,
            // tail을 새로 추가되는 노드로 갱신합니다.
            self.tail.as_mut().unwrap().next = Some(new_node.clone());
            self.tail = Some(new_node);
        }
    }

    // 원형 연결 리스트의 첫 번째 노드를 반환하는 메서드입니다.
    fn get_first(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    // 원형 연결 리스트를 순환하며 모든 노드의 값을 출력하는 메서드입니다.
    fn print_all(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{:?}", node.data);
            current = &node.next;
            if current == &self.head {
                // 원형 연결 리스트가 순환하므로, tail 노드의 next가 head를 가리킬 경우 순환을 종료합니다.
                break;
            }
        }
    }
}
