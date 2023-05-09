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
fn merge(left:&[i32],right:&[i32],arr:&mut[i32],buffer:&mut[i32]){

    let mut i =0;
    let mut j= 0;
    let mut k= 0;
    while i <left.len()&& j<right.len(){

        if left[i]<=right[j]{
            //comparison between the left and right partitions
        }
    }
}

fn main(){}

