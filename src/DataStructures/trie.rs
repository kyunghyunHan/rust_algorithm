/*트리 
최소 신장트리,허프만 코드에 사용
Root: Tree구조의 최상단 Node
Edge :Node와 Node의 연결
Parent:LeafNode를 제외한 모든 Node ,Edge로 연결된 Node를 하위에 보유한 모든 노드
Child:Root Node를 제외한 모든 Node즉, Parent를 갖는 Node
Leaf : Tree의 구조에서 Child를 갖지 않는 모든 최하단 Node
Height	: 전체 Tree 구조에서 가장 긴 경로
Depth	: 특정 Node에서 Root Node 까지의 경로(깊이)
Sub Tree :	Tree 구조 내에 있는 모든 부분적인 Tree들
Sibling	 : 동일한 Parent / Level 갖는 관계인 Node
- 트리는 그래프의 일종
- 방향성이 없으며 순환하지 않음
- 무방향 그래프
- 하나의 루트 노드 존재
- 계층모델
- 간선수:N개의 Node라면 N-1q
*/

use std::cmp::Ordering;

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: i32,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            left: None,
            right: None,
            data,
        }
    }

    fn insert(&mut self, data: i32) {
        match data.cmp(&self.data) {
            Ordering::Less => {
                if let Some(ref mut left) = self.left {
                    left.insert(data);
                } else {
                    self.left = Some(Box::new(Node::new(data)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(data);
                } else {
                    self.right = Some(Box::new(Node::new(data)));
                }
            }
            _ => {}
        }
    }

    fn search(&self, want_word: i32) -> String {
        match want_word.cmp(&self.data) {
            Ordering::Less => {
                if let Some(ref left) = self.left {
                    left.search(want_word)
                } else {
                    format!("{} Not Found", want_word)
                }
            }
            Ordering::Greater => {
                if let Some(ref right) = self.right {
                    right.search(want_word)
                } else {
                    format!("{} Not Found", want_word)
                }
            }
            _ => format!("{} is Found", self.data),
        }
    }

    pub fn print_tree(&self) {
        if let Some(ref left) = self.left {
            left.print_tree();
        }
        println!("{}", self.data);
        if let Some(ref right) = self.right {
            right.print_tree();
        }
    }
}

pub fn main() {
    let mut root = Node::new(10);
    root.insert(1);
    root.insert(3);
    root.insert(6);
    root.insert(6);
    root.insert(9);

    root.insert(13);

    root.insert(22);

    root.insert(17);
    root.insert(10);
    root.insert(2);


    println!("{}", root.search(4));

    println!("{}", root.search(17));
    println!("{}", root.search(22));
    println!("{:?}",root.print_tree());
}

    /*
     *         10
     *       /        \
     *   20      30
     *  /  \
     * 50   60
     * 이런 구조의 트리 완성 root노드는 n10임
     */
