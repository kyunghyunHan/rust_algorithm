use std::fmt::{self,Display,Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T>{
    val:T,
    next:Option<NonNull<Node<T>>>,
    prev:Option<NonNull<Node<T>>>,
}

impl<T>Node<T>{
    fn new(t:T)->Node<T>{
        Node{
            val:t,
            prev:None,
            next:None,
        }
    }
}
pub struct LinkedList<T>{
    length:u32,
    head:Option<NonNull<Node<T>>>,
    tail:Option<NonNull<Node<T>>>,
    marker:PhantomData<Box<Node<T>>>
}

impl<T>Default for LinkedList<T>{
    fn default()->Self{
        Self::new()
    }
}