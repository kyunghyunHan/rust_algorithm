use ndarray;
use ndarray::{arr1,arr2 ,Array1, Array2};

pub fn main(){
    
    let as_list = vec![1, 2, 3];
    let as_array = Array1::from(as_list);
    let row_vec = Array2::from_shape_vec((1, 3), vec![1, 3, 3]).unwrap();
    let col_vec = Array2::from_shape_vec((3, 1), vec![1, 2, 3]).unwrap();

    println!("asList:  {:?}", as_array.shape());
    println!("asArray: {:?}", as_array.shape());
    println!("rowVec:  {:?}", row_vec.shape());
    println!("colVec:  {:?}", col_vec.shape());

        // Define scalar 's' and an array 'a' as in the Python code
        let s = 2;
        let a = arr1(&[3, 4, 5]); // Create a 1D ndarray
    
        // Multiply 's' by 'a' in Rust
        let result_a = a * s;
        
        // Print the results
        println!("{:?}", result_a);
    
        let v = arr2(&[[4.0, 5.0, 6.0]]);
        let w = arr2(&[[10.0], [20.0], [30.0]]);
    
        let result = v + w;
    
        println!("{:?}", result);

}