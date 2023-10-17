/*트리 
최소 신장트리,허프만 코드에 사용
Root: Tree구조의 최상단 Node
Edge :Node와 Node의 연결
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
