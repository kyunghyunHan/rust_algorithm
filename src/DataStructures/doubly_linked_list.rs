//이중연결리스트
struct Node<T>{
    data:T,
    prev:Option<Box<Node<T>>>,
    next:Option<Box<Node<T>>>,
}




//이중연결리스트 구조체 정의
pub struct DoublyLinkedList<T>{
    head:Option<Box<Node<T>>>,
    tail:Option<Box<Node<T>>>,
    size:usize,
}

impl<T>DoublyLinkedList<T>{


    
}