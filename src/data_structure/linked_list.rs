/*
Linked List (단방향)

- 일렬로 데이터를 저장할떄 사용
- 다음 데이터의 주소를 가지고 있음
- 배열보다 속도가 느림
- 길이가 정해지지 않을떄는 배열보다 효과적
*/
use std::fmt::Display;

// Node 구조체 정의
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>, //다음노드의 주소
}

// LinkedList 구조체 정의
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T>
where
    T: PartialEq + Display,
{
    // 새로운 빈 리스트 생성
    fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    // 리스트의 앞쪽에 새로운 노드 추가
    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    // 리스트의 뒤쪽에 새로운 노드 추가
    fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            while let Some(ref mut node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = &mut node.next;
            }
        }
        self.size += 1;
    }

    // 리스트의 앞쪽에서 노드 제거
    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    // 특정 값 검색
    fn contains(&self, data: &T) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == *data {
                return true;
            }
            current = &node.next;
        }
        false
    }

    // 리스트의 크기 반환
    fn len(&self) -> usize {
        self.size
    }

    // 리스트가 비어있는지 확인
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 리스트의 모든 요소 출력
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

// 사용 예시
pub fn example() {
    let mut list: LinkedList<i32> = LinkedList::new();
}
