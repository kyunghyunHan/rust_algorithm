use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }

    fn append(&mut self, d: i32) {
        //맨 마지막에 추가할 노드
        let end = Node::new(d);

        //포인터
        let mut n = self;

        //마지막 노드를 찾을 떄 까지 반복
        while let Some(ref mut next) = n.next {
            n = next;
        }
        //맨 마지막 노드의 next 값에 추가할 노드의 포인터 추가
        n.next = Some(Box::new(end));
    }

    fn delete(&mut self, d: i32) {
        //포인터
        let mut n = self;

        //마지막 노드까지 반복
        while let Some(ref mut next) = n.next {
            if next.val == d {
                n.next = next.next.take();
            } else {
                n = n.next.as_mut().unwrap();
            }
        }
    }
    fn retrieve(&self) {
        let mut n = self;
        while let Some(ref next) = n.next {
            println!("{}", n.val);
            n = next;
        }
        println!("{}", n.val);
    }
}
pub fn example() {
    let mut head = Node::new(1);
    head.append(2);
    head.append(3);
    head.retrieve();  // 1->2->3
    head.delete(2);
    head.retrieve();  // 1->3
}
