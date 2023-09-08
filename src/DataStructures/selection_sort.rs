use std::cmp::Ord;


fn selection_sort<T:Ord>(array:&mut [T]){

    let len = array.len();
    for i in 0..len {
        let mut min_index = i;
        for j in i + 1..len {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        if min_index != i {
            array.swap(i, min_index);
        }
    }




}

pub fn main(){

let mut numbers = vec![5,2,9,1,5,6];

selection_sort(&mut numbers);
println!("Sorted numbers: {:?}", numbers);

let mut words = vec!["apple", "banana", "kiwi", "grape", "orange"];
selection_sort(&mut words);
println!("Sorted words: {:?}", words);
}