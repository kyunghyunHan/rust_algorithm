

/*
Node
Requires space to store data and a space to indicate the next address.
- Stores user-input data in the Data field.
- Connects nodes together using the Next address.
- Null indicates the last node.
*/
struct Node<T> {
    value: T,// Data to store
    next: Option<Box<Node<T>>>,// Pointer to the next node
}

/*
- A linked list is represented by a LinkedList structure.
- The head points to the first node's address.
- The tail points to the last node's address.
- The pointer Next is null.
*/
struct LinkedList<T: PartialEq> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq> LinkedList<T> {
    // Function to create a new empty linked list
    fn new() -> Self {
        LinkedList { head: None }
    }
    /*
    The push function adds a value to the front of the linked list. It creates a new node and sets it as the head of the linked list.

    - 1) When adding a node to the front:
    - - The new node's Next points to the current head's address.
    - - The head points to the new node.

    - 2) When inserting at the end:
    - - Use tail instead of head.
    - - If there's no tail node, adding elements will require traversing the list from the beginning each time.
    - - This results in O(n) time complexity for each insertion.
    - - The new node's Next points to null since it's the last node.
    - - The tail node's Next points to the new node.
    - - The tail node now points to the new address.

    - 3) Inserting at a specific position:
    - - Requires finding the position with the cur node.
    - - 1. Use traversal to make cur point to node 4.
    - - 2. Set the Next address of the new node 5 to be the same as what node 4 points to.
    - - 3. Set the Next address of node 4 to point to the new node 5.
    */
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    /*
    The pop function retrieves a value from the front of the linked list. It takes the current head node, updates the head to the next node, and returns the value of the removed node.
    */    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

     /*
    The remove function deletes a specific value from the linked list. It searches for the node with the value to be removed and updates the previous node's pointer to bypass the node to be removed.

    - Requires the pre node.
    - Deleting node 1:
    - 1. Traverse to make cur point to node 1 for deletion, and pre points to the node just before it.
    - 2. Set the Next address of the node pointed to by pre to be the same as what node 1 points to.
    - 3. The node pointed to by node 1 is freed.
    */
    fn remove(&mut self, value: T) {
        let mut cur = &mut self.head;

        // Check if the head node contains the value to be deleted
        if let Some(node) = cur {
            if node.value == value {
                self.head = node.next.take();
                return;
            }
        }

        // Traverse the linked list to find and remove the node with the specified value
        while let Some(node) = cur {
            if let Some(next_node) = &mut node.next {
                if next_node.value == value {
                    node.next = next_node.next.take();
                    break;
                }
            }
            cur = &mut node.next;
        }
    }

   
    /*
    The is_empty function checks if the linked list is empty by verifying if the head is None.
    */

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub fn main() {
    //this provides a typical constructor and we can use it to create a new node like so;
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push(3);
    list.push(2);
    list.push(1);

    list.remove(2);

    while let Some(value) = list.pop() {
        println!("{}", value);
    }
}