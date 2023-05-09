//mert sort


fn merge_sort(mut arr:&mut [i32]){
    if arr.len()<=1{
        //if we have one or None elements, it is sorted
        return;
    }
    //divides arr in two halves
    let middle= arr.len()/2;

    merge_sort(&mut arr[..middle]);
    merge_sort(&mut arr[middle]);
    //allocates(할당) buffer for saving arr  
    let mut buffer= Vec::with_capacity(arr.len());
    
}


fn main(){}

