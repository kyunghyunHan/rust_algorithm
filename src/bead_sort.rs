pub fn bead_sort(a:&mut [usize]){

    let mut max= a[0];
    for i in 1..a.len(){
        if a[i]>max{
            max= a[i];
        }
    }
}