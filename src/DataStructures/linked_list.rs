use std::fmt::{self,Display,Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T>{
    val:T,
    next:Option<NonNull<Node<T>>>,
    prev:Option<NonNull<Node<T>>>,
}

