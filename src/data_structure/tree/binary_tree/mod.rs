use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow(); // Ref<TreeNode>
                helper(n.left.clone(), result);
                result.push(n.val);
                helper(n.right.clone(), result);
            }
        }

        let mut result = vec![];
        helper(root, &mut result);
        result
    }
}
