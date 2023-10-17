use ndarray;
use ndarray::{Array, Array1, Array2};

pub fn main(){
    
    let as_list = vec![1, 2, 3];
    let as_array = Array1::from(as_list);
    let row_vec = Array2::from_shape_vec((1, 3), vec![1, 3, 3]).unwrap();
    let col_vec = Array2::from_shape_vec((3, 1), vec![1, 2, 3]).unwrap();

    println!("asList:  {:?}", as_array.shape());
    println!("asArray: {:?}", as_array.shape());
    println!("rowVec:  {:?}", row_vec.shape());
    println!("colVec:  {:?}", col_vec.shape());
}