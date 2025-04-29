pub struct LinkedListNode<T> {
    pub value: T,
    pub next: Option<Box<LinkedListNode<T>>>,
}

