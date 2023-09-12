/* array 

Why should I learn data structures?

Data structures aim to efficiently use memory while processing data quickly and reliably. Since data structures are designed to be useful in specific situations, they can be slow and unreliable in other situations. Therefore, the ability to choose the right data structure for a given situation is crucial.

Algorithms, on the other hand, aim to solve specific problems with memory-efficient and fast performance. They are a formalized representation of a set of procedures or methods, often expressed in a mathematical form. In other words, algorithms can be mathematically defined. Like data structures, algorithms can be highly effective in specific situations but may not be suitable in others.




The first data structure in an array.

an array is a collection of data that is stored sequentially in contiguous memory space
in most programming languages.it typically stores data of the same data type.For example,
if an array is of type "int,"it can only store integer elements,and elements of other types like double,float.char,etc.,cannot be stored in it.

Each individual value that makes up an array is called an "element",and the number that indicates the position of an elements within the array is referred to as the "index"



- A linear datga structure that stoires multiple elements of the same data type.
- Data is stored sequentially in contiguous memory space.
- Arrays have a fixed size when declared,and this size cannot be changed.
- Once values are declared in an array,they cannot be changed without redeclaring the array.
- Access to elements in an array is done through indices.
- indices start at O,and the last index is equal to the number of elements in the array minusone.
- Time complexity for accessing an element is O(1).
- To insert data into an array,existing data must be shifted,resulting in a time complexity of O(n)


- Advantages:
- Easy to implement
- Memory management is straightforward due to contiguous stroage.
- Fast for searching
- Disadvantages:
-  Array size cannot be changed 
-  Memory wastage can occur.

ㅇㅇdd
*/

struct Array {
    arr:Vec<i32> //Vector to store the elements
}

impl Array {
    //Constructor to create a new Array
    fn new(size:usize)->Self {
        let arr = vec![0;size];//Initialize the vector with zeros
        Array {arr}
    }

    //Get the element at a specific index
    fn get_element(&self,index:usize)->Option<i32> {

        if index <self.arr.len(){
            Some(self.arr[index])
        }else {
            None
        }

    }
   //Add a value at a specific index
    fn add(&mut self,index:usize,value:i32){
        if index <=self.arr.len(){
            self.arr.insert(index,value);
        }else {
            println!("-1");
        }
    } 
    //Remove the element at a specific index
    fn remove(&mut self,index:usize){
        if index <self.arr.len(){
            self.arr.remove(index);
        }
    }
    //Set the value of an element at a specific index
    fn set(&mut self,index:usize,value :i32 ) {
        if index <self.arr.len(){
            self.arr[index]= value;
        }
    }
   //print all elements in the array
    fn print(&self){
        for &element in &self.arr {
            print!("{}",element);
        }
        println!();
    }
}

pub fn main(){
    //new Array
    let mut arr = Array::new(5);
    
    arr.add(0,1);
    arr.add(1,1);
    arr.add(1,2);
    arr.print();
    
}   