# Doubly Linked List

A doubly linked list implementation in Rust that supports bidirectional traversal and efficient insertion/deletion operations at both ends.

## Data Structures

### DoublyLinkedNode<T>
Internal node structure representing each element in the list.

```rust
struct DoublyLinkedNode<T> {
    data: T,
    next: Option<NonNull<DoublyLinkedNode<T>>>,
    prev: Option<NonNull<DoublyLinkedNode<T>>>,
}
```

### DoublyLinkedList<T>
Main doubly linked list structure.

```rust
pub struct DoublyLinkedList<T> {
    head: Option<NonNull<DoublyLinkedNode<T>>>,
    tail: Option<NonNull<DoublyLinkedNode<T>>>,
    length: usize,
    marker: PhantomData<Box<DoublyLinkedNode<T>>>,
}
```

## Methods

### `new() -> Self`
Creates a new empty doubly linked list.

```rust
let mut list = DoublyLinkedList::new();
```

**Returns:** A new empty `DoublyLinkedList<T>`

---

### `len(&self) -> usize`
Returns the number of elements in the list.

```rust
let len = list.len();
```

**Returns:** The length of the list as `usize`

---

### `is_empty(&self) -> bool`
Checks if the list is empty.

```rust
if list.is_empty() {
    println!("List is empty");
}
```

**Returns:** `true` if the list has no elements, `false` otherwise

---

### `push_front(&mut self, data: T)`
Adds an element to the front of the list.

```rust
list.push_front(10);
list.push_front(5);
// List is now [5, 10]
```

**Parameters:**
- `data: T` - The value to add to the front of the list

**Time Complexity:** O(1)

---

### `push_back(&mut self, data: T)`
Adds an element to the end of the list.

```rust
list.push_back(10);
list.push_back(20);
// List is now [10, 20]
```

**Parameters:**
- `data: T` - The value to add to the end of the list

**Time Complexity:** O(1)

---

### `pop_front(&mut self) -> Option<T>`
Removes and returns the first element of the list.

```rust
let first = list.pop_front();
match first {
    Some(value) => println!("Removed: {}", value),
    None => println!("List is empty"),
}
```

**Returns:** `Some(T)` if the list is not empty, `None` otherwise

**Time Complexity:** O(1)

---

### `pop_back(&mut self) -> Option<T>`
Removes and returns the last element of the list.

```rust
let last = list.pop_back();
match last {
    Some(value) => println!("Removed: {}", value),
    None => println!("List is empty"),
}
```

**Returns:** `Some(T)` if the list is not empty, `None` otherwise

**Time Complexity:** O(1)

---

### `get(&self, index: usize) -> Option<&T>`
Returns a reference to the element at the specified index.

```rust
if let Some(value) = list.get(2) {
    println!("Value at index 2: {}", value);
}
```

**Parameters:**
- `index: usize` - The index of the element to retrieve

**Returns:** `Some(&T)` if the index is valid, `None` otherwise

**Time Complexity:** O(n/2) - Optimized to start from the closer end (head or tail)

**Note:** This method optimizes traversal by starting from whichever end (head or tail) is closer to the target index.

---

### `insert(&mut self, index: usize, data: T) -> Result<(), &'static str>`
Inserts an element at the specified index.

```rust
list.insert(1, 15)?; // Insert 15 at index 1
```

**Parameters:**
- `index: usize` - The position where to insert the element
- `data: T` - The value to insert

**Returns:** `Ok(())` on success, `Err(&'static str)` if index is out of bounds

**Time Complexity:** 
- O(1) if index is 0 or length
- O(n) for middle positions

---

### `remove(&mut self, index: usize) -> Option<T>`
Removes and returns the element at the specified index.

```rust
if let Some(removed) = list.remove(1) {
    println!("Removed: {}", removed);
}
```

**Parameters:**
- `index: usize` - The index of the element to remove

**Returns:** `Some(T)` if the index is valid, `None` otherwise

**Time Complexity:** 
- O(1) if index is 0 or length-1
- O(n) for middle positions

---

### `to_vec(&self) -> Vec<&T>`
Converts the list to a vector of references (forward direction).

```rust
let vec = list.to_vec();
println!("List contents: {:?}", vec);
```

**Returns:** `Vec<&T>` containing references to all elements in forward order

**Time Complexity:** O(n)

---

### `to_vec_reverse(&self) -> Vec<&T>`
Converts the list to a vector of references (reverse direction).

```rust
let reversed = list.to_vec_reverse();
println!("Reversed list: {:?}", reversed);
```

**Returns:** `Vec<&T>` containing references to all elements in reverse order

**Time Complexity:** O(n)

---

### `to_string(&self) -> String` (for T: Display)
Returns a string representation of the list.

```rust
let list_str = list.to_string();
println!("{}", list_str); // "1 <-> 2 <-> 3"
```

**Returns:** `String` representation with elements separated by " <-> "

**Requirements:** Type `T` must implement the `Display` trait

---

## Iterator Implementation

The doubly linked list implements several iterator traits:

### `IntoIterator`
Allows the list to be consumed by a for loop.

```rust
for value in list {
    println!("Value: {}", value);
}
```

### `Iterator`
Forward iteration through the list.

```rust
let mut iter = list.into_iter();
while let Some(value) = iter.next() {
    println!("Value: {}", value);
}
```

### `DoubleEndedIterator`
Bidirectional iteration support.

```rust
let mut iter = list.into_iter();
// Iterate from front
let first = iter.next();
// Iterate from back
let last = iter.next_back();
```

### `ExactSizeIterator`
Provides exact size information for the iterator.

```rust
let iter = list.into_iter();
println!("Iterator length: {}", iter.len());
```

## Usage Examples

### Basic Operations

```rust
use doubly_linked_list::DoublyLinkedList;

let mut list = DoublyLinkedList::new();

// Add elements
list.push_back(1);
list.push_back(2);
list.push_back(3);
list.push_front(0);

println!("List: {}", list.to_string()); // "0 <-> 1 <-> 2 <-> 3"
println!("Length: {}", list.len()); // 4

// Access elements
if let Some(value) = list.get(2) {
    println!("Index 2: {}", value); // 2
}

// Insert in middle
list.insert(2, 99).unwrap();
println!("After insert: {}", list.to_string()); // "0 <-> 1 <-> 99 <-> 2 <-> 3"

// Remove element
if let Some(removed) = list.remove(2) {
    println!("Removed: {}", removed); // 99
}

// Remove from ends
println!("Pop front: {:?}", list.pop_front()); // Some(0)
println!("Pop back: {:?}", list.pop_back()); // Some(3)
```

### Using Iterator

```rust
let mut list = DoublyLinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_back(3);

// Forward iteration
for value in &list {
    println!("Value: {}", value);
}

// Consuming the list
for value in list {
    println!("Consumed: {}", value);
}
```

## Memory Safety

The implementation uses `unsafe` code with `NonNull<T>` pointers for performance and to enable bidirectional links. However, memory safety is ensured through:

1. **Proper Drop implementation** - Automatically cleans up all nodes when the list is dropped
2. **Careful pointer management** - All raw pointers are kept valid through the list's lifetime
3. **Length tracking** - Prevents out-of-bounds access

## Time Complexity Summary

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| `push_front` | O(1) | Always constant time |
| `push_back` | O(1) | Always constant time |
| `pop_front` | O(1) | Always constant time |
| `pop_back` | O(1) | Always constant time |
| `get` | O(n/2) | Optimized traversal |
| `insert` | O(1) or O(n) | O(1) for ends, O(n) for middle |
| `remove` | O(1) or O(n) | O(1) for ends, O(n) for middle |
| `to_vec` | O(n) | Must traverse all elements |

## Advantages over Singly Linked List

1. **Bidirectional traversal** - Can iterate forwards and backwards
2. **Efficient tail operations** - O(1) removal from the end
3. **Optimized middle access** - Can start from closer end (head/tail)
4. **Better for certain algorithms** - Useful for LRU caches, undo/redo systems

## When to Use

**Choose Doubly Linked List when:**
- You need bidirectional traversal
- Frequent insertion/deletion at both ends
- Implementing LRU cache or similar algorithms
- Middle element access with optimization

**Choose other data structures when:**
- Random access is primary concern (use `Vec<T>`)
- Memory efficiency is critical (use singly linked list)
- Mostly append operations (use `Vec<T>` or `VecDeque<T>`)