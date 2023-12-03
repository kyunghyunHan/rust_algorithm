use std::rc::Rc;
use std::cell::RefCell;
/*스레드 이진트리
1.
*/
struct Node {
   data: i32,
   left:Option<Rc<RefCell<Node>>>,
   right:Option<Rc<RefCell<Node>>>,
   is_thread_right:bool,
}


impl Node{
   fn new(key:i32)->Rc<RefCell<Node>>{
      Rc::new(RefCell::new(Node { 
        data: key, 
        left: None, 
        right: None, 
        is_thread_right: false }))
   }
}

fn create_threaded_BST(root:&Option<Rc<RefCell<Node>>>,prev:&mut Option<Rc<RefCell<Node>>>){
    if let Some(node_ref)=root{
        let mut node= node_ref.borrow_mut();
        create_threaded_BST(&node.left, prev);

        if let Some(prev_node_ref)=prev{
            // if  prev_node.is_some()
        }
    }

}
pub fn main(){

}