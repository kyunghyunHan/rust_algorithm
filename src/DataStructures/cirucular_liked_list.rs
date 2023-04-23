/*
원형연결리스트

 */

struct Node<T>{
    data:T,
    next:Option<Box<Node<T>>>,
}

impl<T>Node<T>{
  //새로운 노드를 생성하는 생성자 
  fn new(data:T)->Self{
    Node{data,next:Node}
  }
}

struct CirucularLikendList<T>{
    head:Option<Box<Node<T>>>,
    tail:Option<Box<Node<T>>>,
}

impl<T>CirucularLikendList<T>{
fn new()->Self{
    CirucularLikendList{head:Nonbe}
}
    
}