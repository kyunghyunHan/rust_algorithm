# Linked List

## new(comparator: Option<fn(&T, &T) -> bool>) -> Self
Creates a new linked list.
The comparator function is optional.

## prepend(&mut self, value: T)
Adds a value to the front of the list.
Creates a new node and places it before the current head.
If there is no tail, the newly created node is also set as the tail.

## append(&mut self, value: T)
Adds a value to the end of the list.
Uses the tail pointer to quickly insert at the end.

## Insert(&mut self, index: usize, value: T)
Inserts a value at the specified index.
If the index is 0, it calls prepend.
Otherwise, it traverses the list to insert the node at the correct position.

## delete(&mut self, value: T)
Deletes all nodes with the specified value.
Checks and removes nodes from the head, middle, and tail.

## find(&self, value: Option<&T>, callback: Option<fn(&T) -> bool>)
Finds a node that matches either the given value or satisfies a callback condition.
This mirrors JavaScript's find({ value, callback }) pattern, adapted to Rust.

## delete_head() / delete_tail()
Deletes the head or tail node of the list.

## reverse()
Reverses the list.
Flips the direction of pointers and updates the head and tail accordingly.

## from_array(vec: Vec<T>)
Creates a linked list from a Vec<T>.

## to_array() -> Vec<T>
Converts the current linked list into a vector.

## to_string()
Returns a string representation of the list (for debugging purposes).




## [code](https://github.com/kyunghyunHan/rust_algorithm/blob/main/src/data_structure/linked_list/mod.rs)