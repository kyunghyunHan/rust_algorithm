/*
!!이진탐색트리
전위순회 뿌리먼저방문
중위순회 왼쪽하위트리 후 뿌리방문
후위순회 하위 트리 모두 방문 후 뿌리방문
층별 순회 위쪽 node들  부터 아래방향으로 차례로 방문
*/

#[derive(Debug,Clone)]
struct Node {
    key: i32,
    content: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32, content: String) -> Self {
        Node {
            key,
            content,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, key: i32, content: String) {
        if key <= self.key {
            if let Some(ref mut left) = self.left {
                left.insert(key, content);
            } else {
                self.left = Some(Box::new(Node::new(key, content)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(key, content);
            } else {
                self.right = Some(Box::new(Node::new(key, content)));
            }
        }
    }

    fn search(&self, key: i32) -> Option<&String> {
        if self.key == key {
            return Some(&self.content);
        } else if key < self.key {
            if let Some(ref left) = self.left {
                return left.search(key);
            }
        } else {
            if let Some(ref right) = self.right {
                return right.search(key);
            }
        }
        None
    }

    fn remove(&mut self, key: i32) -> Option<Box<Node>> {
        if key == self.key {
            if self.left.is_none() {
                return self.right.take();
            } else if self.right.is_none() {
                return self.left.take();
            } else {
                let mut right_min = self.right.as_mut().unwrap();
                while let Some(ref mut next) = right_min.left {
                    right_min = next;
                }

                let mut new_node = right_min.clone();
                self.key = new_node.key;
                self.content = new_node.content;

                self.right = self.right.take().and_then(|mut r| {
                    r.remove(new_node.key)
                });
            }
        } else if key < self.key {
            self.left = self.left.take().and_then(|mut l| {
                l.remove(key)
            });
        } else {
            self.right = self.right.take().and_then(|mut r| {
                r.remove(key)
            });
        }
        Some(Box::new(Node {
            key: self.key,
            content: self.content.clone(),
            left: self.left.take(),
            right: self.right.take(),
        }))
    }
}

#[derive(Debug)]
struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn insert(&mut self, key: i32, content: String) {
        if let Some(ref mut root) = self.root {
            root.insert(key, content);
        } else {
            self.root = Some(Box::new(Node::new(key, content)));
        }
    }

    fn search(&self, key: i32) -> Option<&String> {
        if let Some(ref root) = self.root {
            return root.search(key);
        }
        None
    }

    fn remove(&mut self, key: i32) -> Option<()> {
        if let Some(ref mut root) = self.root {
            self.root = root.remove(key);
            return Some(());
        }
        None
    }
}

pub fn main() {
    let mut bst = BinarySearchTree::new();
    bst.insert(10, String::from("Content for key 10"));
    bst.insert(5, String::from("Content for key 5"));
    bst.insert(15, String::from("Content for key 15"));
    bst.insert(7, String::from("Content for key 7"));

    println!("{:?}", bst.search(5)); // Some("Content for key 5")
    println!("{:?}", bst.search(12)); // None

    bst.remove(5);
    println!("{:?}", bst.search(5)); // None
}
