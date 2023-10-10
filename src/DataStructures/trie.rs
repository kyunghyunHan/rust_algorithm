/*트리 
최소 신장트리,허프만 코드에 사용

*/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Node {
    data: i32,
    left_node: Option<Rc<RefCell<Node>>>,
    right_node: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node {
            data,
            left_node: None,
            right_node: None,
        }
    }

    fn set_left_node(&mut self, node: Option<Rc<RefCell<Node>>>) {
        self.left_node = node;
    }

    fn set_right_node(&mut self, node: Option<Rc<RefCell<Node>>>) {
        self.right_node = node;
    }

    fn get_left_node(&self) -> Option<Rc<RefCell<Node>>> {
        self.left_node.clone()
    }

    fn get_right_node(&self) -> Option<Rc<RefCell<Node>>> {
        self.right_node.clone()
    }
}

struct TreeClass {
    new_node: Option<Rc<RefCell<Node>>>,
}

impl TreeClass {
    fn make_node(&mut self, data: i32, left_node: Option<Rc<RefCell<Node>>>, right_node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        self.new_node = Some(Rc::new(RefCell::new(Node::new(data))));
        if let Some(node) = self.new_node.clone() {
            node.borrow_mut().set_left_node(left_node);
            node.borrow_mut().set_right_node(right_node);
        }
        self.new_node.clone()
    }
}

pub fn main() {
    let mut tc = TreeClass { new_node: None };

    let n50 = tc.make_node(50, None, None);
    let n60 = tc.make_node(60, None, None);
    let n20 = tc.make_node(20, n50.clone(), n60.clone());
    let n30 = tc.make_node(30, None, None);
    let n10 = tc.make_node(10, n20.clone(), n30.clone());

    print!("{:?}",n10);

    /*
     *         10
     *       /        \
     *   20      30
     *  /  \
     * 50   60
     * 이런 구조의 트리 완성 root노드는 n10임
     */
}